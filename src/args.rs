use anyhow;
use clap::{crate_authors, crate_description, crate_name, crate_version};

/// Main Config
pub struct Config {
    csv_separator: char,
    csv_field_mapping: String,
    file_input: std::path::PathBuf,
    file_output: std::path::PathBuf,
}

impl Config {
    pub fn new() -> Result<Self, anyhow::Error> {
        let matches = clap::App::new(crate_name!())
            .version(crate_version!())
            .about(crate_description!())
            .author(crate_authors!())
            .arg(
                clap::Arg::with_name("input-file")
                    .help("Input file to use")
                    .takes_value(true)
                    .value_name("INPUT")
                    .required(true)
                    .index(1),
            )
            .arg(
                clap::Arg::with_name("output-file")
                    .help("Output file to use")
                    .takes_value(true)
                    .value_name("OUTPUT")
                    .required(true)
                    .index(2),
            )
            .arg(
                clap::Arg::with_name("log-level")
                    .help("Verbosity level, either DEBUG, INFO, WARN, or ERROR")
                    .long("verbosity")
                    .short("v")
                    .takes_value(true)
                    .value_name("LEVEL"),
            )
            .arg(
                clap::Arg::with_name("csv-separator")
                    .help("Separator between cells in CSV file")
                    .long("csv-separator")
                    .short("c")
                    .takes_value(true)
                    .value_name("SEPARATOR"),
            )
            .get_matches();

        let file_input = std::path::PathBuf::from(matches.value_of("input-file").unwrap());
        let file_output = std::path::PathBuf::from(matches.value_of("output-file").unwrap());
        let csv_separator = if let Some(x) = matches.value_of("csv-separator") {
            x.chars().next().unwrap()
        } else {
            ','
        };

        Ok(Self {
            file_input,
            file_output,
            csv_separator,
            csv_field_mapping: String::from(""),
        })
    }
}
