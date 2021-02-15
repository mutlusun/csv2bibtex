/// Input file does have a different number of fields in a line, in addition tabulators are used as
/// delimiter. Both are tested here.

#[cfg(test)]
mod test_input2 {
    use csv2bibtex::args;
    use log::error;

    #[test]
    #[should_panic]
    fn fail_because_of_different_length() {
        // build config structure
        let mut config = args::Config::default();
        config.file_input = std::path::PathBuf::from("./tests/test2-input1.csv");
        config.file_output = std::path::PathBuf::from("./tests/tmp-test2-output1.bib");
        config.csv_delimiter = String::from("\t");

        // run main function
        csv2bibtex::run(&mut config).unwrap();
    }

    #[test]
    fn should_work() {
        // build config structure
        let mut config = args::Config::default();
        config.file_input = std::path::PathBuf::from("./tests/test2-input1.csv");
        config.file_output = std::path::PathBuf::from("./tests/tmp-test2-output1.bib");
        config.csv_delimiter = String::from("\t");
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

        // run main function
        if let Err(e) = csv2bibtex::run(&mut config) {
            error!("{:#}.", e);
            std::process::exit(1);
        }

        // compare our output with expected output
        let left = biblatex::Bibliography::parse(
            &std::fs::read_to_string("./tests/test2-output1.bib").unwrap(),
        )
        .unwrap();
        let right = biblatex::Bibliography::parse(
            &std::fs::read_to_string("./tests/tmp-test2-output1.bib").unwrap(),
        )
        .unwrap();
        assert_eq!(left.iter().eq(right.iter()), true);

        // clean up
        std::fs::remove_file("./tests/tmp-test2-output1.bib").unwrap();
    }
}
