use csv;
use log::debug;

/// CSV Parser
pub struct Parser<R> {
    iterator: csv::DeserializeRecordsIntoIter<R, std::collections::HashMap<String, String>>,
}

impl<R: std::io::Read> Parser<R> {
    pub fn new(data: R) -> Self {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .delimiter(b',')
            .from_reader(data);

        Self {
            iterator: reader.into_deserialize(),
        }
    }
}

impl<R: std::io::Read> Iterator for Parser<R> {
    type Item = std::collections::HashMap<String, String>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.iterator.next();

        match result {
            Some(x) => {
                let record: Option<Self::Item> = x.ok();
                debug!(
                    "Read csv line: {}",
                    // I'm not sure why there is an offset of one here.
                    &self.iterator.reader().position().line() - 1
                );
                record
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csv_small() {
        let data = "author,year,title\nalice,2000,my title".as_bytes();
        let mut parser = Parser::new(data);
        let result: std::collections::HashMap<String, String> = [
            (String::from("author"), String::from("alice")),
            (String::from("year"), String::from("2000")),
            (String::from("title"), String::from("my title")),
        ]
        .iter()
        .cloned()
        .collect();

        assert_eq!(parser.next(), Some(result));
    }
}
