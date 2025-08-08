use std::{collections::HashMap, fs::File, path::Path, sync::Arc};

use memmap2::Mmap;

#[derive(Debug, thiserror::Error)]
pub enum TarDirectoryError {
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("{0}")]
    Utf8(#[from] std::str::Utf8Error),
}

#[derive(Debug, Clone)]
pub struct TarDirectory {
    pub(crate) mmap: Arc<Mmap>,
    pub(crate) toc: HashMap<String, (usize, usize)>,
}

impl TarDirectory {
    pub(crate) fn read_atomic(&self, p: &std::path::Path) -> Option<Vec<u8>> {
        let str = p.to_str()?;
        let (off, len) = *self.toc.get(str)?;
        let slc: &[u8] = &self.mmap[off..off + len].to_vec();
        Some(slc.to_vec())
    }
    pub fn open(path: impl AsRef<Path>) -> Result<Self, TarDirectoryError> {
        let file = File::open(path)?;
        let mmap = unsafe { Arc::new(Mmap::map(&file)?) };
        let mut toc = HashMap::new();
        let mut pos = 0;
        while pos + 512 <= mmap.len() {
            let header = &mmap[pos..pos + 512];
            if header[0] == 0 {
                break; // End of archive
            }
            let name = std::str::from_utf8(&header[..100])?
                .trim_end_matches('\0')
                .to_string();
            //println!("LOADED: {name}");
            let size = usize::from_str_radix(
                std::str::from_utf8(&header[124..136])?.trim_end_matches('\0'),
                8,
            )?;
            let file_offset = pos + 512;
            toc.insert(name, (file_offset, size));

            let size_padded = ((size + 511) / 512) * 512;
            pos += 512 + size_padded;
        }

        Ok(Self { mmap, toc })
    }
}
