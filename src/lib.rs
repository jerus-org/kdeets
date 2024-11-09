mod crate_versions;
mod error;
mod rust_versions;

pub use crate_versions::CrateVersions;
pub use error::Error;
pub use rust_versions::RustVersions;

use tame_index::{
    external::{
        http::{request::Parts, Response},
        reqwest::blocking::ClientBuilder,
    },
    index::FileLock,
    IndexKrate, IndexLocation, IndexUrl, KrateName, SparseIndex,
};

pub(crate) fn get_sparce_index() -> Result<SparseIndex, tame_index::error::Error> {
    let il = IndexLocation::new(IndexUrl::CratesIoSparse);
    SparseIndex::new(il)
}

pub(crate) fn get_index_crate(index: &SparseIndex, name: KrateName) -> Result<IndexKrate, Error> {
    let lock = FileLock::unlocked();
    let req = index.make_remote_request(name, None, &lock)?;
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

    let Some(index_crate) = index.parse_remote_response(name, response, false, &lock)? else {
        return Err(Error::CrateNotFoundonIndex);
    };

    Ok(index_crate)
}
