use anyhow::anyhow;
use clap::{crate_authors, crate_description, crate_name, crate_version};

/// Main Config
pub struct Config {
    pub csv_delimiter: String,
    pub csv_field_mapping: std::collections::HashMap<String, String>,
    pub file_input: std::path::PathBuf,
    pub file_output: std::path::PathBuf,
    pub log_level: log::LevelFilter,
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
                clap::Arg::with_name("csv-delimiter")
                    .help("Delimiter between cells in CSV file")
                    .long("csv-delimiter")
                    .short("c")
                    .takes_value(true)
                    .value_name("DELIMITER"),
            )
            .arg(
                clap::Arg::with_name("field-csv-to-bib")
                    .help("Assignment of csv fields to bibtex fields")
                    .long("field-csv-to-bib")
                    .short("f")
                    .takes_value(true)
                    .multiple(true)
                    .number_of_values(1)
                    .value_name("FIELD"),
            )
            .get_matches();

        // input / output files
        let file_input = std::path::PathBuf::from(matches.value_of("input-file").unwrap());
        let file_output = std::path::PathBuf::from(matches.value_of("output-file").unwrap());

        // handle field assignments
        let mut csv_field_mapping = std::collections::HashMap::new();
        if let Some(x) = matches.values_of("field-csv-to-bib") {
            for field in x {
                let result: Vec<&str> = field.split('=').collect();
                csv_field_mapping.insert(String::from(result[0]), String::from(result[1]));
            }
        }

        // csv options
        let csv_delimiter = if let Some(x) = matches.value_of("csv-delimiter") {
            String::from(x)
        } else {
            String::from(",")
        };

        // logging handling
        let log_level = if let Some(x) = matches.value_of("log-level") {
            match x.to_lowercase().as_str() {
                "debug" => log::LevelFilter::Debug,
                "info" => log::LevelFilter::Info,
                "warn" => log::LevelFilter::Warn,
                "error" => log::LevelFilter::Error,
                _ => return Err(anyhow!("Unknown log level given")),
            }
        } else {
            log::LevelFilter::Info
        };

        Ok(Self {
            file_input,
            file_output,
            csv_delimiter,
            csv_field_mapping,
            log_level,
        })
    }
}
