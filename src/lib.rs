use anyhow::{anyhow, Context};
use log::{error, info};

pub mod args;
pub mod bibwriter;
pub mod converter;
pub mod csvreader;
pub mod entry;

pub fn run(config: &mut args::Config) -> Result<(), anyhow::Error> {
    // open file for reading and writing
    let file_input = std::fs::File::open(&config.file_input)
        .with_context(|| format!("Could not open csv file: {}", &config.file_input.display()))?;
    let file_output = std::fs::File::create(&config.file_output).with_context(|| {
        format!(
            "Could not open file for writing: {}",
            &config.file_output.display()
        )
    })?;
    info!(
        "Created file \"{}\" to write output.",
        &config.file_output.display()
    );
    let buf_output = std::io::BufWriter::new(file_output);

    // create new csvparser, converter, and writer
    let reader = csvreader::Reader::new(&file_input, &config.csv_delimiter, config.csv_lazy);
    let converter =
        converter::FieldConverter::new(&mut config.csv_field_mapping, None).add_defaults();
    let mut writer = bibwriter::Writer::new(buf_output);

    // main loop
    let start = std::time::Instant::now();

    for entry in reader {
        let entry = match entry {
            Ok(x) => x,
            Err(e) => {
                if config.csv_lazy {
                    error!("{}", e);
                    continue;
                } else {
                    return Err(anyhow!("{}. Option \"-l\" might help.", e));
                }
            }
        };
        let entry = converter.convert_fields(entry);
        let entry = entry::Entry::from_hashmap(entry);

        // TODO maybe it's a bit faster to use a closure here that is constructed either to write
        // bibtex or biblatex
        // Maybe the compiler does this optimisation already for us
        match config.output_type {
            args::OutputType::Bibtex => writer.write(&entry.to_bibtex_string())?,
            args::OutputType::Biblatex => writer.write(&entry.to_biblatex_string())?,
        };
    }
    info!(
        "Wrote {} entries in {:?}.",
        writer.get_num_written_entries(),
        start.elapsed()
    );

    // the end
    Ok(())
}
