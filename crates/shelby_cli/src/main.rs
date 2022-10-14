//! Shelby

use figment::{
    providers::{Env, Format, Toml},
    Figment,
};

use crate::lib::cli::{build_cli, process_matches};

mod lib;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config_toml_path = dirs::config_dir().expect("Failed to get user config directory");
    config_toml_path.push("darbia");
    config_toml_path.push("shelby.toml");

    let mut config_builder = Figment::new()
        .merge(Toml::file(config_toml_path).nested())
        .merge(Env::prefixed("SHELBY_"));

    let matches = build_cli().get_matches();

    if let Some(passed_config_file) = matches.value_of("config-file") {
        config_builder = config_builder
            .clone()
            .merge(Toml::file(passed_config_file).nested());
    }

    let mut log_level = match matches.occurrences_of("verbose") {
        0 => tracing_subscriber::filter::LevelFilter::ERROR,
        1 => tracing_subscriber::filter::LevelFilter::WARN,
        2 => tracing_subscriber::filter::LevelFilter::INFO,
        3 => tracing_subscriber::filter::LevelFilter::DEBUG,
        _ => tracing_subscriber::filter::LevelFilter::TRACE,
    };

    if matches.is_present("quiet") {
        log_level = tracing_subscriber::filter::LevelFilter::OFF;
    }

    tracing_subscriber::fmt()
        .event_format(tracing_subscriber::fmt::format().pretty())
        .with_max_level(log_level)
        .init();

    process_matches(config_builder, matches);

    Ok(())
}
