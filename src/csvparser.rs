use csv;

/// CSV Parser
pub struct Parser<R> {
    reader: csv::Reader<R>,
}

impl<R: std::io::Read> Parser<R> {
    pub fn new(data: R) -> Self {
        let reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .delimiter(b',')
            .from_reader(data);

        Self { reader }
    }
}

impl<R: std::io::Read> Iterator for Parser<R> {
    type Item = std::collections::HashMap<String, String>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = self.reader.deserialize();
        let result = result.next();

        match result {
            Some(x) => {
                let record: Option<Self::Item> = x.ok();
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
