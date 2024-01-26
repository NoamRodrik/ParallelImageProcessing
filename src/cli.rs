mod cli;

pub fn configuration() -> &'static cli::Configuration {
    return &cli::CONFIG;
}