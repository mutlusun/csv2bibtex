use log::info;

/// CSV Parser
pub struct Reader<R> {
    iterator: csv::DeserializeRecordsIntoIter<R, std::collections::HashMap<String, String>>,
}

impl<R: std::io::Read> Reader<R> {
    pub fn new(data: R, delimiter: &str, error_recover: bool) -> Self {
        // TODO there has to be a better way
        let delimiter = if delimiter == "\\t" { "\t" } else { delimiter };

        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .delimiter(delimiter.as_bytes()[0])
            .flexible(error_recover)
            .from_reader(data);

        info!("CSV file has {} columns.", reader.headers().unwrap().len());
        // TODO pretty printing
        info!("CSV columns: {:#?}.", reader.headers().unwrap());

        Self {
            iterator: reader.into_deserialize(),
        }
    }
}

impl<R: std::io::Read> Iterator for Reader<R> {
    type Item = Result<std::collections::HashMap<String, String>, csv::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csv_small() {
        let data = "author,year,title\nalice,2000,my title".as_bytes();
        let mut parser = Reader::new(data, ",", false);
        let result: std::collections::HashMap<String, String> = [
            (String::from("author"), String::from("alice")),
            (String::from("year"), String::from("2000")),
            (String::from("title"), String::from("my title")),
        ]
        .iter()
        .cloned()
        .collect();

        assert_eq!(parser.next().unwrap().unwrap(), result);
    }
}
