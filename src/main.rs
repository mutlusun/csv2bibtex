mod args;
mod converter;
mod csvparser;
mod entry;

use anyhow::Context;
use log::{debug, error, info};
use simplelog;
use std::io::Write;

fn run(config: &mut args::Config) -> Result<(), anyhow::Error> {
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
        "Created file {} to write output.",
        &config.file_output.display()
    );
    let mut buf_output = std::io::BufWriter::new(file_output);

    // create new csvparser, converter
    let csvparser = csvparser::Parser::new(&file_input);
    let converter =
        converter::FieldConverter::new(&mut config.csv_field_mapping, None).add_defaults();

    for entry in csvparser {
        let entry = converter.convert_fields(entry);
        let entry = entry::Entry::from_hashmap(entry);
        buf_output
            .write_fmt(format_args!("{}\n\n", entry.to_biblatex_string()))
            .context("Could not write entry to output file.")?;
    }

    Ok(())
}

fn main() {
    // build config structure
    let mut config = args::Config::new().unwrap_or_else(|e| {
        eprintln!("Problem parsing arguments: {}.", e);
        std::process::exit(1);
    });

    // initialize logger
    simplelog::TermLogger::init(
        config.log_level,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
    )
    .unwrap();

    // run main function
    if let Err(e) = run(&mut config) {
        error!("{:#}.", e);
        std::process::exit(1);
    }
}
