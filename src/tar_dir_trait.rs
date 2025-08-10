use std::{io::BufWriter, path::PathBuf, sync::Arc};

use tantivy::{Directory, directory::error::OpenReadError};

use crate::{TarDirectory, not_writing_writer::NotWritingWriter, tar_handle::TarHandle};

impl Directory for TarDirectory {
    fn get_file_handle(
        &self,
        path: &std::path::Path,
    ) -> Result<
        std::sync::Arc<dyn tantivy::directory::FileHandle>,
        tantivy::directory::error::OpenReadError,
    > {
        let name = path.to_string_lossy();
        let (offset, length) = self
            .toc
            .get(name.as_ref())
            .ok_or_else(|| OpenReadError::FileDoesNotExist(PathBuf::from(path)))?;

        let handle = TarHandle {
            mmap: Arc::from(&self.mmap[..]),
            off: *offset,
            len: *length,
        };
        Ok(Arc::new(handle))
    }

    fn delete(&self, path: &std::path::Path) -> Result<(), tantivy::directory::error::DeleteError> {
        let _ = path;
        Ok(())
    }

    fn exists(
        &self,
        path: &std::path::Path,
    ) -> Result<bool, tantivy::directory::error::OpenReadError> {
        let name = path.to_string_lossy();
        Ok(self.toc.contains_key(name.as_ref()))
    }

    fn open_write(
        &self,
        path: &std::path::Path,
    ) -> Result<tantivy::directory::WritePtr, tantivy::directory::error::OpenWriteError> {
        let _ = path;
        Ok(BufWriter::new(Box::new(NotWritingWriter)))
        /*Err(OpenWriteError::wrap_io_error(
            std::io::Error::other("TAR Archive is read only!"),
            PathBuf::from(path),
        ))*/
    }

    fn atomic_read(
        &self,
        path: &std::path::Path,
    ) -> Result<Vec<u8>, tantivy::directory::error::OpenReadError> {
        self.read_atomic(path)
            .ok_or(OpenReadError::FileDoesNotExist(PathBuf::from(path)))
    }

    fn atomic_write(&self, path: &std::path::Path, data: &[u8]) -> std::io::Result<()> {
        let _ = (path, data);
        Ok(())
    }

    fn sync_directory(&self) -> std::io::Result<()> {
        Ok(())
    }

    fn watch(
        &self,
        watch_callback: tantivy::directory::WatchCallback,
    ) -> tantivy::Result<tantivy::directory::WatchHandle> {
        let _ = watch_callback;
        tantivy::Result::Err(tantivy::TantivyError::InternalError(String::from(
            "TAR IS READ ONLY",
        )))
    }
}
