use clap::Parser;
use clap_verbosity::Verbosity;
use colorful::Colorful;
use kdeets_lib::Error;
use tame_index::{
    external::{
        http::{request::Parts, Response},
        reqwest::blocking::ClientBuilder,
    },
    index::FileLock,
    IndexLocation, IndexUrl, KrateName, SparseIndex,
};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct CrateVersions {
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

pub fn run(args: CrateVersions) -> Result<(), Error> {
    log::info!("Getting details for crate: {}", args.crate_);

    let crate_name = KrateName::crates_io(&args.crate_)?;

    let il = IndexLocation::new(IndexUrl::CratesIoSparse);
    let index = SparseIndex::new(il)?;

    let lock = FileLock::unlocked();
    let req = index.make_remote_request(crate_name, None, &lock)?;

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
    let client = builder.build()?;

    let mut req = client.request(method, uri.to_string());
    req = req.version(version);
    req = req.headers(headers);
    log::info!("Remote request for reqwest: {:#?}!", req);

    let resp = client.execute(req.build()?)?;
    log::info!("Response: {:#?}!", resp);

    let mut builder = Response::builder()
        .status(resp.status())
        .version(resp.version());

    builder
        .headers_mut()
        .unwrap()
        .extend(resp.headers().iter().map(|(k, v)| (k.clone(), v.clone())));

    let body = resp.bytes().unwrap();
    let response = builder.body(body.to_vec())?;

    let Some(index_crate) = index.parse_remote_response(crate_name, response, false, &lock)? else {
        return Err(Error::CrateNotFoundonIndex);
    };

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

    Ok(())
}
