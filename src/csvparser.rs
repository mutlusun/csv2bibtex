use csv;
use log::{debug, error, info};

/// CSV Parser
pub struct Parser<R> {
    iterator: csv::DeserializeRecordsIntoIter<R, std::collections::HashMap<String, String>>,
}

impl<R: std::io::Read> Parser<R> {
    pub fn new(data: R, delimiter: &str) -> Self {
        // TODO there has to be a better way
        let delimiter = if delimiter == "\\t" { "\t" } else { delimiter };

        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .delimiter(delimiter.as_bytes()[0])
            .flexible(true)
            .from_reader(data);

        info!("CSV file has {} columns.", reader.headers().unwrap().len());
        // TODO pretty printing
        info!("CSV columns: {:#?}.", reader.headers().unwrap());

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
                debug!(
                    "Read CSV file, line: {}",
                    // I'm not sure why there is an offset of one here.
                    &self.iterator.reader().position().line() - 1
                );
                match x {
                    Ok(x) => Some(x),
                    Err(e) => {
                        // TODO recover from errors
                        error!("{}", e);
                        None
                    }
                }
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
        let mut parser = Parser::new(data, ",");
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
