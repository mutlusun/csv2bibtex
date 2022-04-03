use csv2bibtex::args;
use log::error;

fn main() {
    // build config structure
    let config = args::Config::new().unwrap_or_else(|e| {
        eprintln!("Problem parsing arguments: {}.", e);
        std::process::exit(1);
    });

    // initialize logger
    simplelog::TermLogger::init(
        config.log_level,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    )
    .unwrap();

    // run main function
    if let Err(e) = csv2bibtex::run(&config) {
        error!("{:#}.", e);
        std::process::exit(1);
    }
}
