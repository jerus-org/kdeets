//! The HTTP transport for `tame_index`, which does no network I/O itself.

use reqwest::blocking::{Client, ClientBuilder};
use tame_index::external::http;
use tame_index::index::local::builder::ValidKrate;
use tame_index::index::{FileLock, IndexConfig};
use tame_index::{IndexKrate, IndexVersion, KrateName, SparseIndex};

use crate::Error;

/// A [`SparseIndex`] that fetches its entries over HTTP.
pub struct RemoteSparseIndex {
    index: SparseIndex,
    client: Client,
}

impl RemoteSparseIndex {
    pub(crate) fn new(index: SparseIndex, client: Client) -> Self {
        Self { index, client }
    }

    /// Fetches the index entry for the crate, optionally writing it to the local cache.
    pub(crate) fn krate(
        &self,
        name: KrateName<'_>,
        write_cache_entry: bool,
        lock: &FileLock,
    ) -> Result<Option<IndexKrate>, Error> {
        let request: http::Request<()> = self.index.make_remote_request(name, None, lock)?.into();
        let response = execute(&self.client, request)?;
        parse_response(&self.index, name, &response, write_cache_entry, lock)
    }

    /// Reads the index entry for the crate from the local cache only.
    pub(crate) fn cached_krate(
        &self,
        name: KrateName<'_>,
        lock: &FileLock,
    ) -> Result<Option<IndexKrate>, Error> {
        let entry = self.index.cached_krate(name, lock)?;
        Ok(entry.map(|entry| entry.deserialize()).transpose()?)
    }
}

fn execute(client: &Client, request: http::Request<()>) -> Result<http::Response<Vec<u8>>, Error> {
    let (parts, ()) = request.into_parts();
    let res = client
        .request(parts.method, parts.uri.to_string())
        .version(parts.version)
        .headers(parts.headers)
        .send()?;

    let mut builder = http::Response::builder()
        .status(res.status())
        .version(res.version());
    if let Some(headers) = builder.headers_mut() {
        headers.extend(res.headers().iter().map(|(k, v)| (k.clone(), v.clone())));
    }

    let body = res.bytes()?;
    Ok(builder.body(body.to_vec())?)
}

fn parse_response(
    index: &SparseIndex,
    name: KrateName<'_>,
    response: &http::Response<Vec<u8>>,
    write_cache_entry: bool,
    lock: &FileLock,
) -> Result<Option<IndexKrate>, Error> {
    let entry = index.parse_remote_response(name, response, write_cache_entry, lock)?;
    Ok(entry.map(|entry| entry.deserialize()).transpose()?)
}

/// Builds the client for downloading `.crate` files.
///
/// Compression is disabled so reqwest cannot decompress a `.crate` (already gzip data)
/// and change the bytes the checksum is computed over.
pub(crate) fn download_client(builder: ClientBuilder) -> Result<Client, Error> {
    Ok(builder.no_gzip().build()?)
}

pub(crate) fn download_krate<'iv>(
    client: &Client,
    config: &IndexConfig,
    version: &'iv IndexVersion,
) -> Result<ValidKrate<'iv>, Error> {
    let url = config.download_url(version.name.as_str().try_into()?, version.version.as_ref());
    let body = client.get(url).send()?.error_for_status()?.bytes()?;
    Ok(ValidKrate::validate(body, version)?)
}

#[cfg(test)]
mod tests {
    use super::{download_client, download_krate, execute, parse_response};
    use tame_index::external::http;
    use tame_index::index::{FileLock, IndexConfig};
    use tame_index::{IndexKrate, IndexLocation, IndexUrl, KrateName, SparseIndex};

    const FORESTRY_ENTRY: &[u8] = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/registry/index/fo/re/forestry"
    ));
    const FORESTRY_CRATE: &[u8] = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/registry/forestry-1.4.1.crate"
    ));

    /// Serves one canned `200 OK` response and returns the server's base URL.
    fn serve_once(extra_headers: &'static str, body: Vec<u8>) -> String {
        use std::io::{Read, Write};

        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut request = Vec::new();
            let mut chunk = [0u8; 1024];
            while !request.windows(4).any(|w| w == b"\r\n\r\n") {
                match stream.read(&mut chunk) {
                    Ok(0) | Err(_) => break,
                    Ok(n) => request.extend_from_slice(&chunk[..n]),
                }
            }
            let head = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n{extra_headers}\r\n",
                body.len()
            );
            let _ = stream.write_all(head.as_bytes());
            let _ = stream.write_all(&body);
        });
        format!("http://{addr}")
    }

    fn sparse_index(dir: &tempfile::TempDir) -> SparseIndex {
        let root = tame_index::PathBuf::from_path_buf(dir.path().to_path_buf()).unwrap();
        let location = IndexLocation::new(IndexUrl::CratesIoSparse).with_root(Some(root));
        SparseIndex::new(location).unwrap()
    }

    fn response(status: u16, body: &[u8]) -> http::Response<Vec<u8>> {
        http::Response::builder()
            .status(status)
            .body(body.to_vec())
            .unwrap()
    }

    fn forestry_config(base: &str) -> IndexConfig {
        IndexConfig {
            dl: format!("{base}/{{crate}}/{{version}}/download"),
            api: None,
            auth_required: false,
        }
    }

    #[test]
    fn parse_response_200_yields_the_krate() {
        let dir = tempfile::tempdir().unwrap();
        let index = sparse_index(&dir);
        let lock = FileLock::unlocked();

        let krate = parse_response(
            &index,
            KrateName::crates_io("forestry").unwrap(),
            &response(200, FORESTRY_ENTRY),
            false,
            &lock,
        )
        .unwrap()
        .expect("a 200 response carries the krate");

        assert_eq!(krate.name(), "forestry");
        assert!(krate.versions.iter().any(|v| v.version == "1.4.1"));
    }

    #[test]
    fn parse_response_404_yields_none() {
        let dir = tempfile::tempdir().unwrap();
        let index = sparse_index(&dir);
        let lock = FileLock::unlocked();

        let krate = parse_response(
            &index,
            KrateName::crates_io("forestry").unwrap(),
            &response(404, b""),
            false,
            &lock,
        )
        .unwrap();

        assert!(krate.is_none());
    }

    #[test]
    fn execute_returns_status_headers_and_body() {
        let base = serve_once("ETag: \"abc\"\r\n", b"hello".to_vec());
        let client = crate::get_client_builder().build().unwrap();
        let request = http::Request::builder().uri(&base).body(()).unwrap();

        let res = execute(&client, request).unwrap();

        assert_eq!(res.status(), 200);
        assert_eq!(res.headers()["etag"], "\"abc\"");
        assert_eq!(res.body().as_slice(), b"hello");
    }

    #[test]
    fn download_validates_the_checksum_of_the_raw_crate_bytes() {
        let krate = IndexKrate::from_slice(FORESTRY_ENTRY).unwrap();
        let version = &krate.versions[0];
        // A .crate is already gzip data. Servers that label it `Content-Encoding: gzip`
        // must not make the client decompress it, or the checksum no longer matches.
        let base = serve_once("Content-Encoding: gzip\r\n", FORESTRY_CRATE.to_vec());
        let client = download_client(crate::get_client_builder()).unwrap();

        let valid = download_krate(&client, &forestry_config(&base), version);

        assert!(valid.is_ok(), "{:?}", valid.err());
    }

    #[test]
    fn download_with_a_decompressing_client_fails_the_checksum() {
        let krate = IndexKrate::from_slice(FORESTRY_ENTRY).unwrap();
        let version = &krate.versions[0];
        let base = serve_once("Content-Encoding: gzip\r\n", FORESTRY_CRATE.to_vec());
        let client = crate::get_client_builder().build().unwrap();

        let valid = download_krate(&client, &forestry_config(&base), version);

        assert!(valid.is_err());
    }
}
