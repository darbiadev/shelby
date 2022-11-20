use std::error::Error;

use clap::{Arg, ArgAction, ArgMatches, command, Command, value_parser};
use clap_complete::{generate, Generator, Shell};

pub(crate) struct Context {
    pub(crate) _quiet: bool,
}

impl Context {
    pub fn new(quiet: bool) -> Context {
        Context { _quiet: quiet }
    }
}

pub(crate) fn build_cli() -> Command {
    command!()
        .infer_long_args(true)
        .infer_subcommands(true)
        .arg_required_else_help(true)
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .conflicts_with("quiet")
                .action(ArgAction::Count)
                .help("Sets the level of verbosity"),
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .conflicts_with("verbose")
                .help("Suppresses all output"),
        )
        .arg(
            Arg::new("config-file")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file"),
        )
        .subcommand(
            Command::new("completions")
                .about("Generate completions")
                .long_about("Generate completions for FIDO")
                .arg(
                    Arg::new("generator")
                        .long("generate")
                        .help("The shell to generate completions for")
                        .value_parser(value_parser!(Shell)),
                ),
        )
        .subcommand(
            Command::new("process").about("process data").arg(
                Arg::new("input-file")
                    .short('i')
                    .long("input-file")
                    .value_name("FILE")
                    .help("The file to process"), // .takes_value(true),
            ),
        )
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut std::io::stdout());
}

pub(crate) async fn process_matches(
    _context: Context,
    _config_builder: figment::Figment,
    matches: ArgMatches,
) {
    if let Some(matches) = matches.subcommand_matches("completions") {
        if let Some(generator) = matches.get_one::<Shell>("generator") {
            let mut cmd = build_cli();
            print_completions(*generator, &mut cmd);
        }
    } else if let Some(matches) = matches.subcommand_matches("process") {
        if let Some(input_file) = matches.get_one::<String>("input-file") {
            read_csv(input_file).unwrap()
        }
    }
}


fn read_csv(input_file: &str) -> Result<(), Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_path(input_file)?;
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

#[test]
fn verify_cli() {
    build_cli().debug_assert();
}
