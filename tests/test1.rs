#[cfg(test)]
mod test_input1 {
    use csv2bibtex::args;
    use log::error;

    #[test]
    fn only_with_defaults() {
        // build config structure
        let mut config = args::Config {
            csv_delimiter: String::from(","),
            csv_field_mapping: std::collections::HashMap::new(),
            file_input: std::path::PathBuf::from("./tests/test1-input1.csv"),
            file_output: std::path::PathBuf::from("./tests/tmp-test1-output1.bib"),
            log_level: log::LevelFilter::Error,
        };

        // run main function
        if let Err(e) = csv2bibtex::run(&mut config) {
            error!("{:#}.", e);
            std::process::exit(1);
        }

        // compare our output with expected output
        let left = biblatex::Bibliography::parse(
            &std::fs::read_to_string("./tests/test1-output1.bib").unwrap(),
        )
        .unwrap();
        let right = biblatex::Bibliography::parse(
            &std::fs::read_to_string("./tests/tmp-test1-output1.bib").unwrap(),
        )
        .unwrap();
        assert_eq!(left.iter().eq(right.iter()), true);

        // clean up
        std::fs::remove_file("./tests/tmp-test1-output1.bib").unwrap();
    }

    #[test]
    fn with_custom_fields() {
        // build field hash map
        let mut csv_field_mapping = std::collections::HashMap::new();
        csv_field_mapping.insert(String::from("title"), String::from("[[Document Title]]"));
        csv_field_mapping.insert(String::from("author"), String::from("[[Authors]]"));
        csv_field_mapping.insert(
            String::from("journal"),
            String::from("[[Publication Title]]"),
        );
        csv_field_mapping.insert(String::from("year"), String::from("[[Publication Year]]"));
        csv_field_mapping.insert(String::from("volume"), String::from("[[Volume]]"));
        csv_field_mapping.insert(String::from("number"), String::from("[[Issue]]"));
        csv_field_mapping.insert(
            String::from("pages"),
            String::from("[[Start Page]]--[[End Page]]"),
        );
        csv_field_mapping.insert(String::from("abstract"), String::from("[[Abstract]]"));
        csv_field_mapping.insert(String::from("issn"), String::from("[[ISSN]]"));
        csv_field_mapping.insert(String::from("isbn"), String::from("[[ISBNs]]"));
        csv_field_mapping.insert(String::from("doi"), String::from("[[DOI]]"));
        csv_field_mapping.insert(
            String::from("keywords"),
            String::from("[[Author Keywords]]"),
        );

        // build config structure
        let mut config = args::Config {
            csv_delimiter: String::from(","),
            csv_field_mapping,
            file_input: std::path::PathBuf::from("./tests/test1-input1.csv"),
            file_output: std::path::PathBuf::from("./tests/tmp-test1-output2.bib"),
            log_level: log::LevelFilter::Error,
        };

        // run main function
        if let Err(e) = csv2bibtex::run(&mut config) {
            error!("{:#}.", e);
            std::process::exit(1);
        }

        // compare our output with expected output
        let left = biblatex::Bibliography::parse(
            &std::fs::read_to_string("./tests/test1-output2.bib").unwrap(),
        )
        .unwrap();
        let right = biblatex::Bibliography::parse(
            &std::fs::read_to_string("./tests/tmp-test1-output2.bib").unwrap(),
        )
        .unwrap();
        assert_eq!(left.iter().eq(right.iter()), true);

        // clean up
        std::fs::remove_file("./tests/tmp-test1-output2.bib").unwrap();
    }
}
