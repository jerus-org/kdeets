use std::error::Error;

use clap::{Parser, Subcommand};
use kdeets_lib::CrateVersions;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(flatten)]
    logging: clap_verbosity_flag::Verbosity,
    /// Force the calculation of the version number
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Query crates.io for information about a crate
    #[clap(name = "crate")]
    CrateVersions(CrateVersions),
}

fn main() {
    let args = Cli::parse();

    let mut builder = get_logging(args.logging.log_level_filter());
    builder.init();

    let result = match args.command {
        Commands::CrateVersions(crate_versions) => crate_versions.run(),
    };

    match result {
        Ok(output) => {
            println!("{}", output);
        }
        Err(e) => {
            if let Some(src) = e.source() {
                log::error!("{}: {}", e, src);
                eprintln!("{}: {}", e, src);
            } else {
                log::error!("{}", e);
                eprintln!("{}", e);
            }
            std::process::exit(1);
        }
    };
}

fn get_logging(level: log::LevelFilter) -> env_logger::Builder {
    let mut builder = env_logger::Builder::new();

    builder.filter(None, level);

    builder.format_timestamp_secs().format_module_path(false);

    builder
}
