# kdeets

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![Build Status][circleci-badge]][circleci-url]
[![Rust 1.89+][version-badge]][version-url]
[![Docs][docs-badge]][docs-url]
[![BuyMeaCoffee][bmac-badge]][bmac-url]
[![GitHubSponsors][ghub-badge]][ghub-url]

[crates-badge]: https://img.shields.io/crates/v/nextsv.svg
[crates-url]: https://crates.io/crates/nextsv
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/jerusdp/nextsv/blob/main/LICENSE
[circleci-badge]: https://dl.circleci.com/status-badge/img/gh/jerus-org/kdeets/tree/main.svg?style=svg
[circleci-url]: https://dl.circleci.com/status-badge/redirect/gh/jerus-org/kdeets/tree/main
[version-badge]: https://img.shields.io/badge/rust-1.89+-orange.svg
[version-url]: https://www.rust-lang.org
[docs-badge]:  https://docs.rs/kdeets/badge.svg
[docs-url]:  https://docs.rs/kdeets
[bmac-badge]: https://badgen.net/badge/icon/buymeacoffee?color=yellow&icon=buymeacoffee&label
[bmac-url]: https://buymeacoffee.com/jerusdp
[ghub-badge]: https://img.shields.io/badge/sponsor-30363D?logo=GitHub-Sponsors&logoColor=#white
[ghub-url]: https://github.com/sponsors/jerusdp

A utility to query crates.io for information about a crate.

## Feature set

- [x] Versions for a crate
- [x] Rust versions for dependencies
- [x] Setup limited clone for testing

## Installation

Install the CLI using cargo install.

```sh
cargo install kdeets

```

Check program is available and the version installed.

```console
$ kdeets --version
kdeets 0.1.29

```

## Usage

The available commands can be seen by running the command with the help flag.

```sh
$ kdeets --help
Query crates.io for information about a crate.

Usage: kdeets [OPTIONS] <COMMAND>

Commands:
  crate  Query crates.io for information about a crate
  rust   Query crates.io for maximum Rust version for a crate
  setup  Setup local registry for a crate
  help   Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose...  Increase logging verbosity
  -q, --quiet...    Decrease logging verbosity
  -h, --help        Print help
  -V, --version     Print version

```

### Versions for a crate  (cmd: crate)

Display the key versions for the crate.

```console
$ kdeets crate -h
Query crates.io for information about a crate

Usage: kdeets crate [OPTIONS] <CRATE>

Arguments:
  <CRATE>  The name of the crate

Options:
  -v, --verbose...  More output per occurrence
  -q, --quiet...    Less output per occurrence
  -b, --bare        Display bare version number without text for recent, highest normal, higest or earliest version
  -e, --earliest    First version ever published. May be yanked
  -n, --normal      Returns crate version with the highest version number according to semver, but excludes pre-release and yanked versions
  -t, --top         The highest version as per semantic versioning specification
  -r, --recent      The last release by date, even if it’s yanked or less than highest version
  -l, --list        List all versions of the crate
  -k, --key         List key values (equivalent to `-entr`)
  -a, --all         List all versions and key values (equivalent to `-entrl`)
  -h, --help        Print help
  -V, --version     Print version

```

This command queries crates.io for information about a crate and reports based on the options selected.

Crates.io tracks for each crate the following versions:

- `earliest`: The first version ever published. May be yanked.
- `normal`: Returns crate version with the highest version number according to semver, but excludes pre-release and yanked versions.
- `top`: The highest version as per semantic versioning specification.
- `recent`: The last release by date, even if it’s yanked or less than highest version.

The `key` option lists all of these versions (equivalent to `-entr`).

The command can create a table listing the yank status and version for all versions of the crate.

The `all` option lists all versions and key values (equivalent to `-entrl`).

```console
$ kdeets --no-colour crate -entrl some_crate

 Crate versions for some_crate.
 🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶
   Earliest version: 0.1.0
   Highest normal version: 0.2.1
   Highest version: 0.2.1
   Most recent version: 0.2.1
    Yanked  Version 
       No     0.1.0
       No     0.1.1
       No     0.1.3
       No     0.2.1


```

## Library API

`kdeets` also exposes a public Rust library (`kdeets_lib`) for use in other crates. The library provides two free functions that query the crates.io sparse index directly, without spawning a subprocess.

Add `kdeets` as a dependency in your `Cargo.toml`:

```toml
[dependencies]
kdeets = "0.1.29"
```

### Check whether a specific version exists

```rust
use kdeets_lib::{version_exists, Error};

fn main() -> Result<(), Error> {
    if version_exists("serde", "1.0.0")? {
        println!("serde 1.0.0 is published");
    }
    Ok(())
}
```

`version_exists` returns:
- `Ok(true)` — the crate and version exist in the index
- `Ok(false)` — the crate exists but that version does not
- `Err(Error::CrateNotFoundOnIndex)` — the crate is not in the index at all

### List all published versions

```rust
use kdeets_lib::{list_versions, Error};

fn main() -> Result<(), Error> {
    let versions = list_versions("serde")?;
    for v in &versions {
        println!("{v}");
    }
    Ok(())
}
```

`list_versions` returns all version strings in the order they appear in the index. It returns `Err(Error::CrateNotFoundOnIndex)` when the crate does not exist.

## License

Licensed under the MIT license (LICENSE-MIT or <http://opensource.org/licenses/MIT>).
