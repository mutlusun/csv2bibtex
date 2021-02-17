use anyhow::anyhow;
use clap::{crate_authors, crate_description, crate_name, crate_version};

/// Output Type (BibTex vs. BibLaTeX)
#[derive(Debug, Clone)]
pub enum OutputType {
    Bibtex,
    Biblatex,
}

impl Default for OutputType {
    fn default() -> Self {
        Self::Biblatex
    }
}

/// Main Config
#[derive(Debug, Clone)]
pub struct Config {
    pub csv_delimiter: String,
    pub csv_field_mapping: std::collections::HashMap<String, String>,
    /// Try to recover from as much errors as possible
    pub csv_lazy: bool,
    pub file_input: std::path::PathBuf,
    pub file_output: std::path::PathBuf,
    pub log_level: log::LevelFilter,
    pub output_type: OutputType,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            csv_delimiter: String::from(","),
            csv_field_mapping: std::collections::HashMap::new(),
            csv_lazy: false,
            file_input: std::path::PathBuf::new(),
            file_output: std::path::PathBuf::new(),
            log_level: log::LevelFilter::Info,
            output_type: OutputType::default(),
        }
    }
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
                    .long("delimiter")
                    .short("d")
                    .takes_value(true)
                    .value_name("DELIMITER"),
            )
            .group(
                clap::ArgGroup::with_name("output-type")
                    .args(&["bibtex", "biblatex"])
                    .multiple(false)
                    .required(false),
            )
            .arg(
                clap::Arg::with_name("bibtex")
                    .help("Print output in BibTeX mode (do not use with --biblatex)")
                    .long("bibtex")
                    .takes_value(false),
            )
            .arg(
                clap::Arg::with_name("biblatex")
                    .help("Print output in BibLaTeX mode (do not use with --bibtex)")
                    .long("biblatex")
                    .takes_value(false),
            )
            .arg(
                clap::Arg::with_name("lazy")
                    .help("Try to recover from as much errors as possible.")
                    .long("lazy")
                    .short("l")
                    .takes_value(false),
            )
            .arg(
                clap::Arg::with_name("field-csv-to-bib")
                    .help("Assignment of csv fields to bibtex fields")
                    .long("field-mapping")
                    .short("f")
                    .takes_value(true)
                    .multiple(true)
                    .number_of_values(1)
                    .value_name("FIELD"),
            )
            .get_matches();

        // get defaults
        let mut ret = Self::default();

        // input / output files
        ret.file_input = std::path::PathBuf::from(matches.value_of("input-file").unwrap());
        ret.file_output = std::path::PathBuf::from(matches.value_of("output-file").unwrap());

        // handle field assignments
        if let Some(x) = matches.values_of("field-csv-to-bib") {
            for field in x {
                let result: Vec<&str> = field.split('=').collect();
                ret.csv_field_mapping
                    .insert(String::from(result[0]), String::from(result[1]));
            }
        }

        // csv options
        if let Some(x) = matches.value_of("csv-delimiter") {
            ret.csv_delimiter = String::from(x)
        };

        // logging handling
        if let Some(x) = matches.value_of("log-level") {
            ret.log_level = match x.to_lowercase().as_str() {
                "debug" => log::LevelFilter::Debug,
                "info" => log::LevelFilter::Info,
                "warn" => log::LevelFilter::Warn,
                "error" => log::LevelFilter::Error,
                _ => return Err(anyhow!("Unknown log level given")),
            }
        };

        // output type
        if matches.is_present("bibtex") {
            ret.output_type = OutputType::Bibtex;
        } else if matches.is_present("biblatex") {
            ret.output_type = OutputType::Biblatex;
        }

        // lazy
        if matches.is_present("lazy") {
            ret.csv_lazy = true;
        }

        Ok(ret)
    }
}
