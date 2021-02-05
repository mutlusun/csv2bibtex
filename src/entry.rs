use std::str::FromStr;

/// BibLaTex Writer
#[derive(Debug)]
pub struct Entry {
    item: biblatex::Entry,
}

impl AsRef<biblatex::Entry> for Entry {
    fn as_ref(&self) -> &biblatex::Entry {
        &self.item
    }
}

impl std::ops::Deref for Entry {
    type Target = biblatex::Entry;
    fn deref(&self) -> &biblatex::Entry {
        &self.item
    }
}

impl Default for Entry {
    fn default() -> Self {
        Self::new()
    }
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
