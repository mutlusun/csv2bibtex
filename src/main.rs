mod args;
mod converter;
mod csvparser;
mod entry;

fn main() {
    // build config structure
    let config = args::Config::new().unwrap_or_else(|e| {
        eprintln!("Problem parsing arguments: {}.", e);
        std::process::exit(1);
    });
}
