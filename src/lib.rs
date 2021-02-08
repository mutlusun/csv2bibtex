use anyhow::Context;
use log::info;

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
    let csvparser = csvreader::Reader::new(&file_input, &config.csv_delimiter);
    let converter =
        converter::FieldConverter::new(&mut config.csv_field_mapping, None).add_defaults();
    let mut writer = bibwriter::Writer::new(buf_output);

    // main loop
    let start = std::time::Instant::now();

    for entry in csvparser {
        let entry = converter.convert_fields(entry);
        let entry = entry::Entry::from_hashmap(entry);
        writer.write(&entry.to_biblatex_string())?;
    }
    info!(
        "Wrote {} entries in {:?}.",
        writer.get_num_written_entries(),
        start.elapsed()
    );

    // the end
    Ok(())
}
