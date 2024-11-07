# kdeets

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![Build Status][circleci-badge]][circleci-url]
[![Rust 1.74+][version-badge]][version-url]
[![Docs][docs-badge]][docs-url]
[![BuyMeaCoffee][bmac-badge]][bmac-url]
[![GitHubSponsors][ghub-badge]][ghub-url]

[crates-badge]: https://img.shields.io/crates/v/nextsv.svg
[crates-url]: https://crates.io/crates/nextsv
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/jerusdp/nextsv/blob/main/LICENSE
[circleci-badge]: https://dl.circleci.com/status-badge/img/gh/jerus-org/kdeets/tree/main.svg?style=svg
[circleci-url]: https://dl.circleci.com/status-badge/redirect/gh/jerus-org/kdeets/tree/main
[version-badge]: https://img.shields.io/badge/rust-1.74+-orange.svg
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
- [ ] Rust versions for dependencies

## CLI Usage

Install the CLI using cargo install.

```sh

cargo install kdeets

```

Run in your project directory and check the version

```console
$ kdeets --version
kdeets 0.1.0

```

### Versions for a crate

Display the key versions for the crate.

```sh

$ kdeets crate -k nextsv
Earliest version: 0.1.0!
Highest normal version: 0.9.2!
Highest version: 9.0.2!
Most recent version: 0.9.2!

```

## License

Licensed under the MIT license (LICENSE-MIT or <http://opensource.org/licenses/MIT>).
