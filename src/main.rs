use std::process::exit;

use clap::{Parser, Subcommand};
use clap_verbosity::Verbosity;
use colorful::Colorful;
use tame_index::{
    external::{
        http::{request::Parts, Response},
        reqwest::blocking::ClientBuilder,
    },
    index::FileLock,
    IndexLocation, IndexUrl, KrateName, SparseIndex,
};

// use tame_index::external::

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
    Crate_(Crate_),
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Crate_ {
    #[clap(flatten)]
    logging: Verbosity,

    /// The name of the crate
    crate_: String,

    /// First version ever published. May be yanked.
    #[clap(short = 'e', long = "earliest")]
    earliest: bool,
    /// Returns crate version with the highest version number according to semver, but excludes pre-release and yanked versions.
    #[clap(short = 'n', long = "normal")]
    normal: bool,
    /// The highest version as per semantic versioning specification
    #[clap(short = 't', long = "top")]
    highest: bool,
    /// The last release by date, even if it’s yanked or less than highest version.
    #[clap(short = 'r', long = "recent")]
    recent: bool,
    /// List all versions of the crate
    #[clap(short = 'l', long = "list")]
    list: bool,
    /// List key values (equivalent to -entr)
    #[clap(short = 'k', long = "key")]
    key: bool,
    /// List all versions and key values (equivalent to -entrl)
    #[clap(short = 'a', long = "all")]
    all: bool,
}

fn main() {
    let args = Cli::parse();

    let mut builder = get_logging(args.logging.log_level_filter());
    builder.init();

    match args.command {
        Commands::Crate_(args) => {
            log::info!("Getting details for crate: {}", args.crate_);

            let crate_name = KrateName::crates_io(&args.crate_)
                .map_err(|_| {
                    log::error!("Invalid crate name: {}", args.crate_);
                    exit(101)
                })
                .unwrap();

            let il = IndexLocation::new(IndexUrl::CratesIoSparse);
            let index = SparseIndex::new(il)
                .map_err(|_| {
                    log::error!("Failed to create sparse index");
                    exit(102)
                })
                .unwrap();

            let lock = FileLock::unlocked();
            let req = index
                .make_remote_request(crate_name, None, &lock)
                .map_err(|_| {
                    log::error!("Failed to make remote request");
                    exit(103)
                })
                .unwrap();

            log::debug!("Constructed remote request: {:?}!", req);

            let (
                Parts {
                    method,
                    uri,
                    version,
                    headers,
                    ..
                },
                _,
            ) = req.into_parts();

            let builder = ClientBuilder::new();
            let builder = builder.tls_built_in_root_certs(true);
            let client = builder
                .build()
                .map_err(|_| {
                    log::error!("Failed to build client");
                    exit(104)
                })
                .unwrap();

            let mut req = client.request(method, uri.to_string());
            req = req.version(version);
            req = req.headers(headers);
            log::info!("Remote request for reqwest: {:#?}!", req);

            let resp = client
                .execute(
                    req.build()
                        .map_err(|_| {
                            log::error!("Failed to build request");
                            exit(107)
                        })
                        .unwrap(),
                )
                .map_err(|_| {
                    log::error!("Failed to execute request");
                    exit(108)
                })
                .unwrap();
            log::info!("Response: {:#?}!", resp);

            let mut builder = Response::builder()
                .status(resp.status())
                .version(resp.version());

            builder
                .headers_mut()
                .unwrap()
                .extend(resp.headers().iter().map(|(k, v)| (k.clone(), v.clone())));

            let body = resp.bytes().unwrap();
            let response = builder
                .body(body.to_vec())
                .map_err(|e| tame_index::Error::from(tame_index::error::HttpError::from(e)))
                .unwrap();

            let index_crate = index
                .parse_remote_response(crate_name, response, false, &lock)
                .map_err(|_| {
                    log::error!("Failed to parse remote response");
                    exit(109)
                })
                .unwrap()
                .unwrap();

            if args.earliest | args.all | args.key {
                println!(
                    "Earliest version: {}!",
                    index_crate.earliest_version().version
                );
            };

            if args.normal | args.all | args.key {
                println!(
                    "{}",
                    format!(
                        "Highest normal version: {}!",
                        index_crate.highest_normal_version().unwrap().version
                    )
                    .blue()
                );
            };

            if args.highest | args.all | args.key {
                println!(
                    "{}",
                    format!(
                        "Highest version: {}!",
                        index_crate.highest_version().version
                    )
                    .green()
                );
            };

            if args.recent | args.all | args.key {
                println!(
                    "{}",
                    format!(
                        "Most recent version: {}!",
                        index_crate.most_recent_version().version
                    )
                    .yellow()
                );
            };

            if args.list | args.all {
                println!("Versions of crate {}\n", index_crate.name(),);

                println!("Yanked  Version");
                for v in index_crate.versions.iter() {
                    let version = v.version.clone();
                    let yanked = if v.yanked {
                        " Yes".red()
                    } else {
                        "  No".green()
                    };

                    println!("{}     {}", yanked, version);
                }
            }
        }
    };
}

fn get_logging(level: log::LevelFilter) -> env_logger::Builder {
    let mut builder = env_logger::Builder::new();

    builder.filter(None, level);

    builder.format_timestamp_secs().format_module_path(false);

    builder
}
