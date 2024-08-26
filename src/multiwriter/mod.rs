use std::io::{self, Write};

pub struct MultiWriter<W: Write> {
    writers: Vec<W>,
}

impl<W: Write> MultiWriter<W> {
    pub fn new(writers: Vec<W>) -> Self {
        MultiWriter { writers }
    }
}

impl<W: Write> Write for MultiWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut min_written = buf.len();
        for writer in &mut self.writers {
            match writer.write(buf) {
                Ok(written) => min_written = min_written.min(written),
                Err(e) => return Err(e),
            }
        }
        Ok(min_written)
    }

    fn flush(&mut self) -> io::Result<()> {
        for writer in &mut self.writers {
            writer.flush()?;
        }
        Ok(())
    }
}
