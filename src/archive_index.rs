use std::{path::Path, sync::Arc};

use tantivy::{TantivyError, error::DataCorruption};

use crate::{TarDirectory, tar_directory::TarDirectoryError};

pub trait IdxBuilderTarExt {
    fn from_archive(self, archive_path: impl AsRef<Path>) -> tantivy::Result<tantivy::Index>;
}
impl IdxBuilderTarExt for tantivy::IndexBuilder {
    fn from_archive(self, archive_path: impl AsRef<Path>) -> tantivy::Result<tantivy::Index> {
        let dir = TarDirectory::open(archive_path).map_err(|e| e.to_tantivy_error())?;
        //let tar_directory: Box<dyn Directory> = Box::new(dir);

        self.open_or_create(dir)
    }
}
impl TarDirectoryError {
    fn to_tantivy_error(self) -> TantivyError {
        match self {
            TarDirectoryError::Io(error) => TantivyError::IoError(Arc::new(error)),
            TarDirectoryError::ParseInt(parse_int_error) => {
                let cause = format!("Failed to Read TAR-Header: {}", parse_int_error);
                TantivyError::DataCorruption(DataCorruption::comment_only(cause))
            }
            TarDirectoryError::Utf8(utf8_error) => {
                let cause = format!("Failed to Read UTF8 from TAR-Header: {}", utf8_error);
                TantivyError::DataCorruption(DataCorruption::comment_only(cause))
            }
        }
    }
}
/*
use std::{fs::File, io, path::Path, sync::Arc};

use tantivy::{
    Directory, Index, TantivyError,
    directory::{
        MmapDirectory, RamDirectory,
        error::{Incompatibility, OpenDirectoryError},
    },
    error::DataCorruption,
};
impl MmapArchivationExt for RamDirectory {}
impl MmapArchivationExt for MmapDirectory {
    fn get_path(&self) -> tantivy::Result<()> {
        //self.inner.root_path; //field inner of struct MmapDirectory is private
        Ok(())
    }
}
pub trait MmapArchivationExt {
    fn get_path(&self) -> tantivy::Result<()> {
        Err(TantivyError::IndexBuilderMissingArgument(
            "Only Mmap indices can be Archived!",
        ))
    }
}

pub trait TarArchivationExt {
    fn archive(&self, path: impl AsRef<Path>) -> io::Result<()>;
}

impl TarArchivationExt for Index {
    fn archive(&self, path: impl AsRef<Path>) -> io::Result<()> {
        todo!()
    }
}

/*pub trait ArchiveIndexExt {
    fn archive_index(&mut self, path: impl AsRef<Path>) -> std::io::Result<()>;
}*/

/*impl ArchiveIndexExt for tar::Builder<File> {
    fn archive_index(&mut self, path: impl AsRef<Path>) -> std::io::Result<()> {
        self.append_dir_all(".", path)?;
        self.finish()?;
        Ok(())
    }
}*/
 */
