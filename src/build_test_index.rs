use std::path::Path;

use tantivy::{
    Document, Index, TantivyDocument,
    collector::TopDocs,
    query::QueryParser,
    schema::{STORED, Schema, TEXT},
};

use crate::TarDirectory;

#[allow(unused)]
pub fn tar_test_index(index_path: impl AsRef<Path>) -> std::io::Result<()> {
    use std::fs::File;
    use tar::Builder;

    let file = File::create("archive.tar")?;

    // Create a tar builder
    let mut tar_builder = Builder::new(file);

    // Append the directory (recursively)
    tar_builder.append_dir_all(".", index_path)?;

    // Finish writing
    tar_builder.finish()?;
    Ok(())
}
#[allow(unused)]
pub fn schema() -> Schema {
    let mut schema_builder = Schema::builder();

    schema_builder.add_text_field("title", TEXT | STORED);
    schema_builder.add_text_field("body", TEXT);
    let schema = schema_builder.build();
    schema
}
#[allow(unused)]
pub fn build_test_index(index_path: impl AsRef<Path>) -> tantivy::Result<()> {
    /*use tantivy::collector::TopDocs;
    use tantivy::query::QueryParser;*/
    use tantivy::schema::*;
    use tantivy::{Index, IndexWriter, doc};

    //let index_path = "test_index";
    let schema = schema();
    let index = Index::create_in_dir(&index_path, schema.clone())?;
    let mut index_writer: IndexWriter = index.writer(50_000_000)?;
    let title = schema.get_field("title").unwrap();
    let body = schema.get_field("body").unwrap();

    let mut old_man_doc = TantivyDocument::default();
    old_man_doc.add_text(title, "The Old Man and the Sea");
    old_man_doc.add_text(
        body,
        "He was an old man who fished alone in a skiff in the Gulf Stream and he had gone \
              eighty-four days now without taking a fish.",
    );

    index_writer.add_document(old_man_doc)?;
    for _ in (0..50_000) {
        index_writer.add_document(doc!(
        title => "Of Mice and Men",
        body => "A few miles south of Soledad, the Salinas River drops in close to the hillside \
                bank and runs deep and green. The water is warm too, for it has slipped twinkling \
                over the yellow sands in the sunlight before reaching the narrow pool. On one \
                side of the river the golden foothill slopes curve up to the strong and rocky \
                Gabilan Mountains, but on the valley side the water is lined with trees—willows \
                fresh and green with every spring, carrying in their lower leaf junctures the \
                debris of the winter’s flooding; and sycamores with mottled, white, recumbent \
                limbs and branches that arch over the pool"
        ))?;
    }

    index_writer.add_document(doc!(
    title => "Frankenstein",
    title => "The Modern Prometheus",
    body => "You will rejoice to hear that no disaster has accompanied the commencement of an \
             enterprise which you have regarded with such evil forebodings.  I arrived here \
             yesterday, and my first task is to assure my dear sister of my welfare and \
             increasing confidence in the success of my undertaking."
    ))?;

    index_writer.commit()?;

    Ok(())
}
#[allow(unused)]
pub fn test_tar_dir(dir: TarDirectory) -> tantivy::Result<()> {
    let index = Index::open(dir)?;
    println!("Opened index!");
    let reader = index
        .reader_builder()
        .reload_policy(tantivy::ReloadPolicy::Manual)
        .try_into()?;
    println!("Opened reader!");
    let searcher = reader.searcher();
    let schema = schema();
    let title = schema
        .get_field("title")
        .expect("couldnt find title of schema");
    let body = schema
        .get_field("body")
        .expect("couldnt find body of schema");
    let query_parser = QueryParser::for_index(&index, vec![title, body]);
    let query = query_parser.parse_query("sea whale")?;
    println!("Built query!");
    let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;
    for (_score, doc_address) in top_docs {
        let retrieved_doc: TantivyDocument = searcher.doc(doc_address)?;
        println!("{}", retrieved_doc.to_json(&schema));
    }
    println!("Searched successfully!");
    /*let query = query_parser.parse_query("title:sea^20 body:whale^70")?;

    let (_score, doc_address) = searcher
        .search(&query, &TopDocs::with_limit(1))?
        .into_iter()
        .next()
        .unwrap();

    let explanation = query.explain(&searcher, doc_address)?;

    println!("{}", explanation.to_pretty_json());*/
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{fs::File, path::PathBuf};

    use tempfile::TempDir;

    use super::*;
    #[test]
    fn test_index_in_tar() -> Result<(), Box<dyn std::error::Error>> {
        let test_directory = TempDir::new()?;
        let index_directory = TempDir::new_in(&test_directory)?;
        let archive_path = PathBuf::from(test_directory.path()).join("archive.tar");
        let test_archive = File::create(&archive_path)?;

        //Build index
        build_test_index(&index_directory)?;

        //Archive index
        let mut tar = tar::Builder::new(test_archive);
        tar.append_dir_all(".", &index_directory)?;
        tar.finish()?;

        //open archive
        let tardir = TarDirectory::open(archive_path)?;
        test_tar_dir(tardir)?;

        Ok(())
    }
}
