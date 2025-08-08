use std::io::Write;

use tantivy::directory::TerminatingWrite;

pub struct NotWritingWriter;

impl Write for NotWritingWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl TerminatingWrite for NotWritingWriter {
    fn terminate_ref(&mut self, _: tantivy::directory::AntiCallToken) -> std::io::Result<()> {
        Ok(())
    }
}
