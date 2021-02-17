/// Convert Fields According to Command Line Arguments
pub struct FieldConverter<'a> {
    map: &'a mut std::collections::HashMap<String, String>,
    // compile regex only once
    regex: regex::Regex,
}

impl<'a> FieldConverter<'a> {
    pub fn new(replacement_list: &'a mut std::collections::HashMap<String, String>) -> Self {
        Self {
            map: replacement_list,
            //map_postprocess,
            regex: regex::Regex::new("\\[\\[(.+?)\\]\\]").unwrap(),
        }
    }

    pub fn add_defaults(self) -> Self {
        // insert some defaults that may fit to the given column names in the csv file
        // insert only if key doesn't exist already
        self.map
            .entry(String::from("entrytype"))
            .or_insert_with(|| String::from("[[type]]"));
        self.map
            .entry(String::from("bibtexkey"))
            .or_insert_with(|| String::from("[[bibtexkey]]"));
        self.map
            .entry(String::from("title"))
            .or_insert_with(|| String::from("[[titles]]"));
        self.map
            .entry(String::from("author"))
            .or_insert_with(|| String::from("[[authors]]"));

        self
    }

    pub fn convert_fields(
        &self,
        input: std::collections::HashMap<String, String>,
    ) -> std::collections::HashMap<String, String> {
        // TODO use with_capacity length of self.map
        let mut ret = std::collections::HashMap::new();

        // TODO optimize:
        // 1. remove keys/entries that are non-existent in csv files
        // 2. for every entry/row in csv file, the same happens. Probably we can preprocess all
        //    replacements ... Maybe we can even process one column at a time?
        // 3. Maybe it's best to iterate over the fields in the init function and collect only the
        //    captures. Then we can iterate over the captures here.
        for (k, v) in self.map.iter() {
            // replace fields and save them in the `ret` map. This is the output of the current
            // function and will be printed later
            let result = self.regex.replace_all(v, |caps: &regex::Captures| {
                if let Some(x) = input.get(&caps[1]) {
                    x
                } else {
                    ""
                }
            });

            if result != "" {
                ret.insert(String::from(k), result.into_owned());
            }
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_default_fields() {
        let mut input = std::collections::HashMap::new();
        input.insert(String::from("authors"), String::from("author1, author2"));
        input.insert(String::from("titles"), String::from("My eloquent title"));

        let mut output = std::collections::HashMap::new();
        output.insert(String::from("author"), String::from("author1, author2"));
        output.insert(String::from("title"), String::from("My eloquent title"));

        let mut replacement_list = std::collections::HashMap::new();

        let converter = FieldConverter::new(&mut replacement_list).add_defaults();
        let ret = converter.convert_fields(input);

        assert_eq!(ret, output);
    }

    #[test]
    fn test_own_replacement_list() {
        let mut input = std::collections::HashMap::new();
        input.insert(String::from("Start Page"), String::from("1200"));
        input.insert(String::from("ISBNs"), String::from("XXXXX-XXXXX"));

        let mut output = std::collections::HashMap::new();
        output.insert(String::from("pages"), String::from("1200"));
        output.insert(String::from("isbn"), String::from("XXXXX-XXXXX"));

        let mut replacement_list = std::collections::HashMap::new();
        replacement_list.insert(String::from("pages"), String::from("[[Start Page]]"));
        replacement_list.insert(String::from("isbn"), String::from("[[ISBNs]]"));

        let converter = FieldConverter::new(&mut replacement_list).add_defaults();
        let ret = converter.convert_fields(input);

        assert_eq!(ret, output);
    }

    #[test]
    fn test_combined_fields() {
        let mut input = std::collections::HashMap::new();
        input.insert(String::from("Start Page"), String::from("1200"));
        input.insert(String::from("End Page"), String::from("1212"));

        let mut output = std::collections::HashMap::new();
        output.insert(String::from("pages"), String::from("1200--1212"));

        let mut replacement_list = std::collections::HashMap::new();
        replacement_list.insert(
            String::from("pages"),
            String::from("[[Start Page]]--[[End Page]]"),
        );

        let converter = FieldConverter::new(&mut replacement_list).add_defaults();
        let ret = converter.convert_fields(input);

        assert_eq!(ret, output);
    }

    #[test]
    fn test_combined_fields_same_field_multiple_times() {
        let mut input = std::collections::HashMap::new();
        input.insert(String::from("Start Page"), String::from("1200"));

        let mut output = std::collections::HashMap::new();
        output.insert(String::from("pages"), String::from("1200--1200"));

        let mut replacement_list = std::collections::HashMap::new();
        replacement_list.insert(
            String::from("pages"),
            String::from("[[Start Page]]--[[Start Page]]"),
        );

        let converter = FieldConverter::new(&mut replacement_list).add_defaults();
        let ret = converter.convert_fields(input);

        assert_eq!(ret, output);
    }
}
