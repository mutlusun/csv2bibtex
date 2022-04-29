use anyhow::{anyhow, Context};
use log::{error, info};

pub mod args;
pub mod bibwriter;
pub mod converter;
pub mod csvreader;

pub fn run(config: &args::Config) -> Result<(), anyhow::Error> {
    // open file for reading and writing
    let file_input = std::fs::File::open(&config.file_input)
        .with_context(|| format!("Could not open csv file: {}", &config.file_input.display()))?;
    let buf_output =
        std::io::BufWriter::new(std::fs::File::create(&config.file_output).with_context(|| {
            format!(
                "Could not open file for writing: {}",
                &config.file_output.display()
            )
        })?);
    info!(
        "Created file \"{}\" to write output.",
        &config.file_output.display()
    );

    // create new csvparser, converter, and writer
    let mut csv_field_mapping = config.csv_field_mapping.clone();
    let reader = csvreader::Reader::new(&file_input, &config.csv_delimiter, config.csv_lazy);
    let converter = {
        let mut ret = converter::FieldConverter::new(&mut csv_field_mapping);
        if config.mapping_defaults {
            ret = ret.add_defaults()
        }
        ret
    };
    let mut writer: Box<dyn bibwriter::BibWrite> = match config.output_type {
        args::OutputType::Bibtex => Box::new(bibwriter::BibtexWriter::new(buf_output)),
        args::OutputType::Biblatex => Box::new(bibwriter::BiblatexWriter::new(buf_output)),
    };

    // main loop
    let start = std::time::Instant::now();

    for (index, entry) in reader.enumerate() {
        let entry = match entry {
            Ok(x) => x,
            Err(e) => {
                if config.csv_lazy {
                    error!("Error converting item: {}. Skipping item.", e);
                    continue;
                } else {
                    return Err(anyhow!("{}. Option \"-l\" might help.", e));
                }
            }
        };
        let entry = converter.convert_fields(entry, format!("entry_{index}"));
        writer.write(&entry)?;
    }
    info!(
        "Wrote {} entries in {:?}.",
        writer.get_num_written_entries(),
        start.elapsed()
    );

    // the end
    Ok(())
}
