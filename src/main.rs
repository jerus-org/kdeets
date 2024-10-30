use clap::Parser;
use clap_verbosity::Verbosity;
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
    logging: Verbosity,

    /// The name of the crate
    #[clap(short = 'k', long = "krate")]
    krate: String,
}

fn main() {
    let args = Cli::parse();

    let mut builder = get_logging(args.logging.log_level_filter());
    builder.init();

    let krate = args.krate;
    log::info!("Hello, {}!", krate);

    let krate_name = KrateName::crates_io(&krate).unwrap();
    log::info!("Hello, {:?}!", krate_name);

    let il = IndexLocation::new(IndexUrl::CratesIoSparse);

    let index = SparseIndex::new(il).unwrap();

    let lock = FileLock::unlocked();

    let req = index.make_remote_request(krate_name, None, &lock).unwrap();

    log::info!("Request: {:?}!", req);

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
    let client = builder.build().unwrap();
    let mut req = client.request(method, uri.to_string());
    req = req.version(version);
    req = req.headers(headers);
    log::info!("Request: {:#?}!", req);

    let resp = client.execute(req.build().unwrap()).unwrap();
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

    let index_krate = index
        .parse_remote_response(krate_name, response, false, &lock)
        .unwrap()
        .unwrap();

    log::info!(
        "Index krate - earliest version: {:#?}!",
        index_krate.earliest_version().version
    );
    log::info!(
        "Index krate - highest normal version: {:#?}!",
        index_krate.highest_normal_version().unwrap().version
    );
    log::info!(
        "Index krate - highest version: {:#?}!",
        index_krate.highest_version().version
    );
    log::info!(
        "Index krate - most recent version: {:#?}!",
        index_krate.most_recent_version().version
    );

    println!("Highest version: {:#?}!", index_krate.highest_version());

    println!(
        "Versions of crate {}: {}",
        index_krate.name(),
        index_krate
            .versions
            .iter()
            .map(|c| c.version.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    );
    println!("Hello, world!");
}

fn get_logging(level: log::LevelFilter) -> env_logger::Builder {
    let mut builder = env_logger::Builder::new();

    builder.filter(None, level);

    builder.format_timestamp_secs().format_module_path(false);

    builder
}
