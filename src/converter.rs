/// Convert Fields According to Command Line Arguments
pub struct FieldConverter<'a> {
    // Collection of bibtex entries and their corresponding CSV fields. CSV fields can also be
    // combined and mixed with other characters, e.g., "[[page_beginning]]--[[page_end]]".
    map: &'a mut std::collections::HashMap<String, String>,
    // The regex to replace CSV fields with their corresponding entry. Saved here to compile the
    // regex only once.
    regex: regex::Regex,
    // Fields that should stay as they are -- verbatim mode
    verbatim_fields: &'a mut Vec<String>,
}

impl<'a> FieldConverter<'a> {
    pub fn new(
        replacement_list: &'a mut std::collections::HashMap<String, String>,
        verbatim_fields: &'a mut Vec<String>,
    ) -> Self {
        Self {
            map: replacement_list,
            regex: regex::Regex::new("\\[\\[(.+?)\\]\\]").unwrap(),
            verbatim_fields,
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
            .or_insert_with(|| String::from("[[title]]"));
        self.map
            .entry(String::from("author"))
            .or_insert_with(|| String::from("[[author]]"));
        self.map
            .entry(String::from("abstract"))
            .or_insert_with(|| String::from("[[abstract]]"));
        self.map
            .entry(String::from("journal"))
            .or_insert_with(|| String::from("[[journal]]"));
        self.map
            .entry(String::from("volume"))
            .or_insert_with(|| String::from("[[volume]]"));
        self.map
            .entry(String::from("number"))
            .or_insert_with(|| String::from("[[issue]]"));

        // Insert into verbatim fields
        let tmp_verbfields = [
            "url", "file", "doi", "pdf", "eprint", "verba", "verbb", "verbc", "urlraw",
        ];
        for item in tmp_verbfields {
            if !self.verbatim_fields.contains(&String::from(item)) {
                self.verbatim_fields.push(String::from(item));
            }
        }

        self
    }

    pub fn convert_fields(
        &self,
        input: std::collections::HashMap<String, String>,
        default_key: String,
    ) -> biblatex::Entry {
        // Check whether we have a key specified in `self.map` and use this one or set a default
        // value.
        //
        // TODO that looks ugly!
        let key: String = if let Some(x) = self.map.get("bibtexkey") {
            self.regex
                .replace_all(x, |caps: &regex::Captures| {
                    if let Some(y) = input.get(&caps[1]) {
                        y.to_string()
                    } else {
                        default_key.clone()
                    }
                })
                .into_owned()
        } else {
            default_key
        };

        // Check whether we have a entry type specified in `self.map` and use this one or set a
        // default value.
        //
        // TODO that looks ugly!
        let entrytype: String = if let Some(x) = self.map.get("entrytype") {
            self.regex
                .replace_all(x, |caps: &regex::Captures| {
                    if let Some(y) = input.get(&caps[1]) {
                        y.to_string()
                    } else {
                        String::from("article")
                    }
                })
                .into_owned()
        } else {
            String::from("article")
        };

        // Create a return entry
        let mut ret = biblatex::Entry::new(key, biblatex::EntryType::new(&entrytype));

        // TODO optimize:
        // 1. remove keys/entries that are non-existent in csv files
        // 2. for every entry/row in csv file, the same happens. Probably we can preprocess all
        //    replacements ... Maybe we can even process one column at a time?
        // 3. Maybe it's best to iterate over the fields in the init function and collect only the
        //    captures. Then we can iterate over the captures here.
        for (k, v) in self.map.iter() {
            if k == "bibtexkey" || k == "entrytype" {
                continue;
            }
            // replace fields and save them in the `ret` entry. This is the output of the current
            // function and will be printed later
            let result = self.regex.replace_all(v, |caps: &regex::Captures| {
                if let Some(x) = input.get(&caps[1]) {
                    x
                } else {
                    ""
                }
            });

            if result.is_empty() {
                continue;
            }

            if self.verbatim_fields.contains(k) {
                ret.set(
                    k,
                    vec![biblatex::Spanned::detached(biblatex::Chunk::Verbatim(
                        result.into_owned(),
                    ))],
                );
            } else {
                ret.set(
                    k,
                    vec![biblatex::Spanned::detached(biblatex::Chunk::Normal(
                        result.into_owned(),
                    ))],
                );
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
        input.insert(String::from("author"), String::from("author1, author2"));
        input.insert(String::from("title"), String::from("My eloquent title"));

        let mut output = biblatex::Entry::new(String::from("test1"), biblatex::EntryType::Article);
        output.set(
            "author",
            vec![biblatex::Spanned::detached(biblatex::Chunk::Normal(
                String::from("author1, author2"),
            ))],
        );
        output.set(
            "title",
            vec![biblatex::Spanned::detached(biblatex::Chunk::Normal(
                String::from("My eloquent title"),
            ))],
        );

        let mut replacement_list = std::collections::HashMap::new();
        let mut verbatim_fields = std::vec::Vec::new();

        let converter =
            FieldConverter::new(&mut replacement_list, &mut verbatim_fields).add_defaults();
        let ret = converter.convert_fields(input, String::from("test1"));

        assert_eq!(ret, output);
    }

    #[test]
    fn test_entrykey() {
        // First use only the key
        let mut input = std::collections::HashMap::new();
        input.insert(String::from("key"), String::from("entry1"));

        let output = biblatex::Entry::new(String::from("entry1"), biblatex::EntryType::Article);

        let mut replacement_list = std::collections::HashMap::new();
        replacement_list.insert(String::from("bibtexkey"), String::from("[[key]]"));
        let mut verbatim_fields = std::vec::Vec::new();

        let converter = FieldConverter::new(&mut replacement_list, &mut verbatim_fields);
        let ret = converter.convert_fields(input, String::from("test1"));

        assert_eq!(ret, output);

        // Now, add a prefix
        let mut input = std::collections::HashMap::new();
        input.insert(String::from("key"), String::from("entry1"));

        let output =
            biblatex::Entry::new(String::from("prefix_entry1"), biblatex::EntryType::Article);

        let mut replacement_list = std::collections::HashMap::new();
        replacement_list.insert(String::from("bibtexkey"), String::from("prefix_[[key]]"));
        let mut verbatim_fields = std::vec::Vec::new();

        let converter = FieldConverter::new(&mut replacement_list, &mut verbatim_fields);
        let ret = converter.convert_fields(input, String::from("test1"));

        assert_eq!(ret, output);
    }

    #[test]
    fn test_own_replacement_list() {
        let mut input = std::collections::HashMap::new();
        input.insert(String::from("Start Page"), String::from("1200"));
        input.insert(String::from("ISBNs"), String::from("XXXXX-XXXXX"));

        let mut output = biblatex::Entry::new(String::from("test1"), biblatex::EntryType::Article);
        output.set(
            "pages",
            vec![biblatex::Spanned::detached(biblatex::Chunk::Normal(
                String::from("1200"),
            ))],
        );
        output.set(
            "isbn",
            vec![biblatex::Spanned::detached(biblatex::Chunk::Normal(
                String::from("XXXXX-XXXXX"),
            ))],
        );

        let mut replacement_list = std::collections::HashMap::new();
        replacement_list.insert(String::from("pages"), String::from("[[Start Page]]"));
        replacement_list.insert(String::from("isbn"), String::from("[[ISBNs]]"));
        let mut verbatim_fields = std::vec::Vec::new();

        let converter =
            FieldConverter::new(&mut replacement_list, &mut verbatim_fields).add_defaults();
        let ret = converter.convert_fields(input, String::from("test1"));

        assert_eq!(ret, output);
    }

    #[test]
    fn test_combined_fields() {
        let mut input = std::collections::HashMap::new();
        input.insert(String::from("Start Page"), String::from("1200"));
        input.insert(String::from("End Page"), String::from("1212"));

        let mut output = biblatex::Entry::new(String::from("test1"), biblatex::EntryType::Article);
        output.set(
            "pages",
            vec![biblatex::Spanned::detached(biblatex::Chunk::Normal(
                String::from("1200--1212"),
            ))],
        );

        let mut replacement_list = std::collections::HashMap::new();
        replacement_list.insert(
            String::from("pages"),
            String::from("[[Start Page]]--[[End Page]]"),
        );
        let mut verbatim_fields = std::vec::Vec::new();

        let converter =
            FieldConverter::new(&mut replacement_list, &mut verbatim_fields).add_defaults();
        let ret = converter.convert_fields(input, String::from("test1"));

        assert_eq!(ret, output);
    }

    #[test]
    fn test_combined_fields_same_field_multiple_times() {
        let mut input = std::collections::HashMap::new();
        input.insert(String::from("Start Page"), String::from("1200"));

        let mut output = biblatex::Entry::new(String::from("test1"), biblatex::EntryType::Article);
        output.set(
            "pages",
            vec![biblatex::Spanned::detached(biblatex::Chunk::Normal(
                String::from("1200--1200"),
            ))],
        );

        let mut replacement_list = std::collections::HashMap::new();
        replacement_list.insert(
            String::from("pages"),
            String::from("[[Start Page]]--[[Start Page]]"),
        );
        let mut verbatim_fields = std::vec::Vec::new();

        let converter =
            FieldConverter::new(&mut replacement_list, &mut verbatim_fields).add_defaults();
        let ret = converter.convert_fields(input, String::from("test1"));

        assert_eq!(ret, output);
    }

    #[test]
    fn test_verbatim_fields() {
        let mut input = std::collections::HashMap::new();
        input.insert(String::from("author"), String::from("author1, author2"));
        input.insert(String::from("title"), String::from("My eloquent title"));
        input.insert(
            String::from("testfield"),
            String::from("Test: 1234$%?_]';p[\\]"),
        );
        input.insert(String::from("type"), String::from("misc"));

        let mut output = biblatex::Entry::new(String::from("test1"), biblatex::EntryType::Misc);
        output.set(
            "author",
            vec![biblatex::Spanned::detached(biblatex::Chunk::Normal(
                String::from("author1, author2"),
            ))],
        );
        output.set(
            "title",
            vec![biblatex::Spanned::detached(biblatex::Chunk::Normal(
                String::from("My eloquent title"),
            ))],
        );
        output.set(
            "testfield",
            vec![biblatex::Spanned::detached(biblatex::Chunk::Verbatim(
                String::from("Test: 1234$%?_]';p[\\]"),
            ))],
        );

        let mut replacement_list = std::collections::HashMap::new();
        replacement_list.insert(String::from("testfield"), String::from("[[testfield]]"));
        let mut verbatim_fields = std::vec::Vec::new();
        verbatim_fields.push(String::from("testfield"));

        let converter =
            FieldConverter::new(&mut replacement_list, &mut verbatim_fields).add_defaults();
        let ret = converter.convert_fields(input, String::from("test1"));

        assert_eq!(ret, output);

        // Test output of a verbatim field
        let tmp = String::from("@misc{test1,\nauthor = {author1, author2},\ntestfield = {{Test\\: 1234\\$\\%?\\_]';p[\\\\]}},\ntitle = {My eloquent title},\n}");
        assert_eq!(ret.to_biblatex_string(), tmp);
    }
}
