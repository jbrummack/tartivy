use std::{ops::Deref, sync::Arc};

use tantivy::directory::{FileHandle, OwnedBytes};

#[derive(Debug)]
pub struct TarHandle {
    pub(crate) mmap: Arc<[u8]>,
    pub(crate) off: usize,
    pub(crate) len: usize,
}

impl Deref for TarHandle {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.mmap[self.off..self.off + self.len]
    }
}

unsafe impl stable_deref_trait::StableDeref for TarHandle {}

/*impl HasLen for TarHandle {
    fn len(&self) -> usize {
        self.len
    }
}*/

impl FileHandle for TarHandle {
    #[doc = " Reads a slice of bytes."]
    #[doc = ""]
    #[doc = " This method may panic if the range requested is invalid."]
    fn read_bytes(&self, range: core::ops::Range<usize>) -> std::io::Result<OwnedBytes> {
        if range.end > self.len {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "range out of bounds",
            ));
        }
        Ok(
            OwnedBytes::new(Arc::clone(&self.mmap))
                .slice(self.off..self.off + self.len) // file slice
                .slice(range.start..range.end), // requested range
        )
    }
}
