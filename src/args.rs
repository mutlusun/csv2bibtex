use anyhow::anyhow;
use clap::crate_authors;
use clap::crate_description;
use clap::crate_name;
use clap::crate_version;

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
    pub mapping_defaults: bool,
    pub verbatim_fields: Vec<String>,
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
            mapping_defaults: true,
            verbatim_fields: std::vec::Vec::new(),
        }
    }
}

impl Config {
    pub fn new() -> Result<Self, anyhow::Error> {
        let matches = clap::Command::new(crate_name!())
            .version(crate_version!())
            .about(crate_description!())
            .author(crate_authors!())
            .arg(
                clap::Arg::new("input-file")
                    .help("Input file to use")
                    .takes_value(true)
                    .value_name("INPUT")
                    .required(true)
                    .index(1),
            )
            .arg(
                clap::Arg::new("output-file")
                    .help("Output file to use")
                    .takes_value(true)
                    .value_name("OUTPUT")
                    .required(true)
                    .index(2),
            )
            .arg(
                clap::Arg::new("log-level")
                    .help("Verbosity level, either DEBUG, INFO, WARN, or ERROR")
                    .long("verbosity")
                    .short('v')
                    .takes_value(true)
                    .value_name("LEVEL"),
            )
            .arg(
                clap::Arg::new("csv-delimiter")
                    .help("Delimiter between cells in CSV file")
                    .long("delimiter")
                    .short('d')
                    .takes_value(true)
                    .value_name("DELIMITER"),
            )
            .group(
                clap::ArgGroup::new("output-type")
                    .args(&["bibtex", "biblatex"])
                    .multiple(false)
                    .required(false),
            )
            .arg(
                clap::Arg::new("bibtex")
                    .help("Print output in BibTeX mode")
                    .long("bibtex")
                    .takes_value(false),
            )
            .arg(
                clap::Arg::new("biblatex")
                    .help("Print output in BibLaTeX mode (default)")
                    .long("biblatex")
                    .takes_value(false),
            )
            .arg(
                clap::Arg::new("lazy")
                    .help("Try to recover from as much errors as possible.")
                    .long("lazy")
                    .short('l')
                    .takes_value(false),
            )
            .arg(
                clap::Arg::new("no-defaults")
                    .help("Don't add default field mappings and verbatim fields.")
                    .long("no-defaults")
                    .takes_value(false),
            )
            .arg(
                clap::Arg::new("field-csv-to-bib")
                    .help("Assignment of csv fields to bibtex fields")
                    .long("field-mapping")
                    .short('f')
                    .takes_value(true)
                    .multiple_occurrences(true)
                    .number_of_values(1)
                    .value_name("FIELD"),
            )
            .arg(
                clap::Arg::new("verbatim-field")
                    .help("Bib(La)TeX verbatim fields, like url, file or doi")
                    .long("verbatim-field")
                    .takes_value(true)
                    .multiple_occurrences(true)
                    .number_of_values(1)
                    .value_name("FIELD"),
            )
            .get_matches();

        // get defaults
        let mut ret = Self {
            // input / output files
            file_input: std::path::PathBuf::from(matches.value_of("input-file").unwrap()),
            file_output: std::path::PathBuf::from(matches.value_of("output-file").unwrap()),

            // Lazy switch (recover from errors)
            csv_lazy: matches.is_present("lazy"),

            // prevent the use of defaults?
            mapping_defaults: !matches.is_present("no-defaults"),

            // Set other fields to default values
            ..Default::default()
        };

        // handle field assignments
        if let Some(x) = matches.values_of("field-csv-to-bib") {
            for field in x {
                let result: Vec<&str> = field.split('=').collect();
                ret.csv_field_mapping
                    .insert(String::from(result[0]), String::from(result[1]));
            }
        }

        if let Some(x) = matches.values_of("verbatim-field") {
            for field in x {
                ret.verbatim_fields.push(field.to_string());
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

        // Output type. Biblatex is the default ...
        if matches.is_present("bibtex") {
            ret.output_type = OutputType::Bibtex;
        }

        Ok(ret)
    }
}
