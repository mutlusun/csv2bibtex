use biblatex;
use std::str::FromStr;

/// BibLaTex Writer
struct Entry {
    item: biblatex::Entry,
}

impl Entry {
    pub fn new() -> Self {
        let item = biblatex::Entry::new(String::from("none"), biblatex::EntryType::Article);

        Self { item }
    }

    pub fn from_hashmap(map: std::collections::HashMap<String, String>) -> Self {
        let key = map
            .get("key")
            .map_or_else(|| String::from("none"), |x| x.to_owned());
        let entrytype = map
            .get("entrytype")
            .map_or_else(|| String::from("article"), |x| x.to_owned());
        let mut item =
            biblatex::Entry::new(key, biblatex::EntryType::from_str(&entrytype).unwrap());

        for (k, v) in map {
            if k == "key" || k == "entrytype" {
                continue;
            }

            item.set(&k, vec![biblatex::Chunk::Normal(v)]);
        }

        Self { item }
    }
}
