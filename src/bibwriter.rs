use anyhow::Context;

/// Writer
pub struct Writer<W: std::io::Write> {
    writer: W,
    counter: usize,
}

impl<W: std::io::Write> Writer<W> {
    pub fn new(writer: W) -> Self {
        Self { writer, counter: 0 }
    }

    pub fn write(&mut self, data: &str) -> Result<(), anyhow::Error> {
        write!(self.writer, "{}\n\n", data).context("Could not write entry to file.")?;

        self.counter += 1;

        Ok(())
    }

    pub fn get_num_written_entries(&self) -> usize {
        self.counter
    }
}
