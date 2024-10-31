use clap::{Parser, Subcommand};
use crate_versions::CrateVersions;

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

mod crate_versions;

fn main() {
    let args = Cli::parse();

    let mut builder = get_logging(args.logging.log_level_filter());
    builder.init();

    match args.command {
        Commands::CrateVersions(args) => crate_versions::run(args),
    };
}

fn get_logging(level: log::LevelFilter) -> env_logger::Builder {
    let mut builder = env_logger::Builder::new();

    builder.filter(None, level);

    builder.format_timestamp_secs().format_module_path(false);

    builder
}
