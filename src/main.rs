use std::error::Error;

use clap::{Parser, Subcommand};
use kdeets_lib::{CrateVersions, RustVersions, Setup};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(flatten)]
    logging: clap_verbosity_flag::Verbosity,
    /// No colour flag removes styling escapes from the output
    #[clap(long = "no-colour")]
    no_colour: bool,
    /// Force the calculation of the version number
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Query crates.io for information about a crate
    #[clap(name = "crate")]
    CrateVersions(CrateVersions),
    /// Query crates.io for maximum Rust version for a crate
    #[clap(name = "rust")]
    RustVersions(RustVersions),
    /// Setup local registry for a crate
    #[clap(name = "setup")]
    Setup(Setup),
}

fn main() {
    let args = Cli::parse();

    let mut builder = get_logging(args.logging.log_level_filter());
    builder.init();

    let result = match args.command {
        Commands::CrateVersions(crate_versions) => crate_versions.run(),
        Commands::RustVersions(rust_versions) => rust_versions.run(),
        Commands::Setup(setup) => setup.run(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_default_values() {
        let cli = Cli::try_parse_from(["kdeets", "crate", "some_crate"]).unwrap();
        assert!(!cli.no_colour);
        assert_eq!(cli.logging.log_level(), Some(log::Level::Error));
    }

    #[test]
    fn test_cli_no_colour_flag() {
        let cli = Cli::try_parse_from(["kdeets", "--no-colour", "crate", "some_crate"]).unwrap();
        assert!(cli.no_colour);
    }

    #[test]
    fn test_cli_no_colour_flag_rust() {
        let cli = Cli::try_parse_from(["kdeets", "--no-colour", "rust", "some_crate"]).unwrap();
        assert!(cli.no_colour);
    }

    #[test]
    fn test_cli_no_colour_flag_setup() {
        let cli = Cli::try_parse_from(["kdeets", "--no-colour", "setup", "some_crate"]).unwrap();
        assert!(cli.no_colour);
    }

    #[test]
    fn test_cli_verbosity_levels() {
        let quiet = Cli::try_parse_from(["kdeets", "-q", "crate", "some_crate"]).unwrap();
        assert_eq!(quiet.logging.log_level(), None);

        let verbose = Cli::try_parse_from(["kdeets", "-v", "crate", "some_crate"]).unwrap();
        assert_eq!(verbose.logging.log_level(), Some(log::Level::Warn));

        let debug = Cli::try_parse_from(["kdeets", "-vv", "crate", "some_crate"]).unwrap();
        assert_eq!(debug.logging.log_level(), Some(log::Level::Info));

        let debug = Cli::try_parse_from(["kdeets", "-vvv", "crate", "some_crate"]).unwrap();
        assert_eq!(debug.logging.log_level(), Some(log::Level::Debug));

        let debug = Cli::try_parse_from(["kdeets", "-vvvv", "crate", "some_crate"]).unwrap();
        assert_eq!(debug.logging.log_level(), Some(log::Level::Trace));
    }

    #[test]
    fn test_cli_invalid_args() {
        let result = Cli::try_parse_from(["kdeets", "--invalid-flag", "crate", "some_crate"]);
        assert!(result.is_err());
    }
}
