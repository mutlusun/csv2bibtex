/// Input file does have a different number of fields in a line, in addition tabulators are used as
/// delimiter. Both are tested here.

#[cfg(test)]
mod test_input2 {
    use log::error;

    #[test]
    #[should_panic]
    fn fail_because_of_different_length() {
        // build config structure
        let config = csv2bibtex::args::Config {
            file_input: std::path::PathBuf::from("./tests/test2-input1.csv"),
            file_output: std::path::PathBuf::from("./tests/tmp-test2-output1.bib"),
            csv_delimiter: String::from("\t"),
            ..Default::default()
        };

        // run main function
        csv2bibtex::run(&config).unwrap();
    }

    #[test]
    fn should_work() {
        // build config structure
        let mut config = csv2bibtex::args::Config {
            file_input: std::path::PathBuf::from("./tests/test2-input1.csv"),
            file_output: std::path::PathBuf::from("./tests/tmp-test2-output1.bib"),
            csv_delimiter: String::from("\t"),
            csv_lazy: true,
            ..Default::default()
        };

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
        if let Err(e) = csv2bibtex::run(&config) {
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
        assert!(left.iter().eq(right.iter()));

        // clean up
        std::fs::remove_file("./tests/tmp-test2-output1.bib").unwrap();
    }
}
