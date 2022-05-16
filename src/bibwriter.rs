use anyhow::anyhow;
use anyhow::Context;

/// BibWriter Trait
pub trait BibWrite {
    fn write(&mut self, entry: &biblatex::Entry) -> Result<(), anyhow::Error>;
    fn get_num_written_entries(&self) -> usize;
}

/// Biblatex type implementing bibwriter trait
pub struct BiblatexWriter<W: std::io::Write> {
    writer: W,
    counter: usize,
}

impl<W: std::io::Write> BiblatexWriter<W> {
    pub fn new(writer: W) -> Self {
        Self { writer, counter: 0 }
    }
}

impl<W: std::io::Write> BibWrite for BiblatexWriter<W> {
    fn write(&mut self, entry: &biblatex::Entry) -> Result<(), anyhow::Error> {
        write!(self.writer, "{}\n\n", entry.to_biblatex_string())
            .context("Could not write entry to file")?;
        self.counter += 1;

        Ok(())
    }
    fn get_num_written_entries(&self) -> usize {
        self.counter
    }
}

/// Bibtex type implementing bibwriter trait
pub struct BibtexWriter<W: std::io::Write> {
    writer: W,
    counter: usize,
}

impl<W: std::io::Write> BibtexWriter<W> {
    pub fn new(writer: W) -> Self {
        Self { writer, counter: 0 }
    }
}

impl<W: std::io::Write> BibWrite for BibtexWriter<W> {
    fn write(&mut self, entry: &biblatex::Entry) -> Result<(), anyhow::Error> {
        write!(
            self.writer,
            "{}\n\n",
            entry
                .to_bibtex_string()
                .map_err(|e| anyhow!("TypeError: {}", e))?
        )
        .context("Could not write entry to file")?;
        self.counter += 1;

        Ok(())
    }
    fn get_num_written_entries(&self) -> usize {
        self.counter
    }
}
