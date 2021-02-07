use anyhow::Context;
use log::info;
use std::io::Write;

pub mod args;
pub mod converter;
pub mod csvparser;
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
    let mut buf_output = std::io::BufWriter::new(file_output);

    // create new csvparser, converter
    let csvparser = csvparser::Parser::new(&file_input, &config.csv_delimiter);
    let converter =
        converter::FieldConverter::new(&mut config.csv_field_mapping, None).add_defaults();

    // main loop
    for entry in csvparser {
        let entry = converter.convert_fields(entry);
        let entry = entry::Entry::from_hashmap(entry);
        buf_output
            .write_fmt(format_args!("{}\n\n", entry.to_biblatex_string()))
            .context("Could not write entry to output file.")?;
    }

    // the end
    Ok(())
}
