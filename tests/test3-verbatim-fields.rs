/// Test the output of verbatim fields

#[cfg(test)]
mod test_input3 {
    use csv2bibtex::args;
    use log::error;

    #[test]
    fn test_verbatim_fields() {
        // build config structure
        let mut config = args::Config::default();
        config.file_input = std::path::PathBuf::from("./tests/test3-input1.csv");
        config.file_output = std::path::PathBuf::from("./tests/tmp-test3-output1.bib");
        config.csv_delimiter = String::from(";");
        config.csv_lazy = true;

        // build field hash map
        config
            .csv_field_mapping
            .insert(String::from("entrytype"), String::from("article"));
        config
            .csv_field_mapping
            .insert(String::from("author"), String::from("[[AU]]"));
        config
            .csv_field_mapping
            .insert(String::from("title"), String::from("[[TI]]"));
        config
            .csv_field_mapping
            .insert(String::from("url"), String::from("[[DI]]"));

        // run main function
        if let Err(e) = csv2bibtex::run(&mut config) {
            error!("{:#}.", e);
            std::process::exit(1);
        }

        // compare our output with expected output
        let left = biblatex::Bibliography::parse(
            &std::fs::read_to_string("./tests/test3-output1.bib").unwrap(),
        )
        .unwrap();
        let right = biblatex::Bibliography::parse(
            &std::fs::read_to_string("./tests/tmp-test3-output1.bib").unwrap(),
        )
        .unwrap();
        assert_eq!(left.iter().eq(right.iter()), true);

        // clean up
        std::fs::remove_file("./tests/tmp-test3-output1.bib").unwrap();
    }
}
