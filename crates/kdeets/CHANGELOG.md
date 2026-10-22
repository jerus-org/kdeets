<!-- LTex: Enabled=false -->
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.34] - 2026-10-22

Summary: Changed[1], Fixed[5]

### Fixed

 - fix(deps): update rust crate rstest to 0.28.0
 - fix(deps): update rust crate webpki-roots to 1.0.10
 - fix(deps): update rust crate smol_str to 0.3.7
 - fix(deps): port to tame-index 0.27 sans-IO API
 - fix(deps): update rust crate tame-index to 0.27.0

### Changed

 - refactor: remove unused cached_krate wrappers

## [0.1.33] - 2026-10-16

Summary: Chore[2], Fixed[13]

### Fixed

 - fix: add binstall signing scaffold to kdeets
 - fix(deps): update rust crate rustls to 0.23.46
 - fix(deps): update rust crate rustls to v0.23.46 [security]
 - fix(deps): update rust crate rstest to 0.27.0
 - fix(deps): update rust crate thiserror to 2.0.21
 - fix(deps): update rust crate rustls to 0.23.45
 - fix(deps): update rust crate reqwest to 0.13.5
 - fix(deps): update rust crate clap to 4.6.7
 - fix(deps): update rust crate rustls to v0.23.45 [security]
 - fix(deps): update rust crate thiserror to 2.0.20
 - fix(deps): update rust crate log to 0.4.34
 - fix(deps): update rust crate clap to 4.6.6
 - fix(deps): update rust crate rustls to 0.23.43

## [0.1.32] - 2026-07-24

Summary: Chore[1], Fixed[5]

### Fixed

 - fix(deps): update rust crate webpki-roots to 1.0.9
 - fix(deps): update rust crate trycmd to 1.2.1
 - fix(deps): update rust crate thiserror to 2.0.19
 - fix(deps): update rust crate rustls to 0.23.42
 - fix(deps): update rust crate clap to 4.6.4

## [0.1.31] - 2026-06-29

Summary: Chore[1], Fixed[14]

### Fixed

 - fix(deps): update rust crate env_logger to 0.11.11
 - fix(deps): update rust crate webpki-roots to 1.0.8
 - fix(deps): update rust crate rustls to 0.23.41
 - fix(deps): update rust crate log to 0.4.33
 - fix(deps): update rust crate log to 0.4.32
 - fix(deps): update rust crate tame-index to 0.26.3
 - fix(deps): update rust crate reqwest to 0.13.4
 - fix(deps): update rust crate rustls to 0.23.40
 - fix(deps): update rust crate webpki-roots to 1.0.7
 - fix(deps): update rust crate tame-index to 0.26.2
 - fix(deps): update rust crate semver to 1.0.28
 - fix(deps): update rust crate rustls to 0.23.39
 - fix(deps): update rust crate clap to 4.6.1
 - fix(docs): correct crates badge URLs in README

## [0.1.30] - 2026-03-27

Summary: Added[1], Chore[1], Documentation[1], Fixed[6], Testing[2]

### Added

 - feat: expose version_exists and list_versions as public library API

### Fixed

 - fix(deps): update rust crate trycmd to 1.2.0
 - fix(deps): update rust crate tempfile to 3.27.0
 - fix(deps): update rust crate clap to 4.6.0
 - fix(deps): update rust crate tame-index to 0.26.1
 - fix(deps): update rust crate env_logger to 0.11.10
 - fix: absolute TEST_REGISTRY path and network tests for public API

## [0.1.29] - 2026-03-04

Summary: Chore[1], Fixed[8]

### Fixed

 - fix(deps): update rust crate smol_str to 0.3.6
 - fix(deps): update rust crate trycmd to 1.0.1
 - fix(deps): update rust crate rustls to 0.23.37
 - fix(deps): update rust crate trycmd to v1
 - fix(deps): update rust crate tempfile to 3.26.0
 - fix(deps): update rust crate simple_logger to 5.2.0
 - fix(deps): update rust crate env_logger to 0.11.9
 - fix(deps): update rust crate clap to 4.5.60

## [0.1.28] - 2026-02-13

Summary: Added[2], Build[1], Changed[2], Chore[1], Documentation[2], Fixed[25], Security[54], Testing[1]

### Added

 - ✨ feat(help): add bare version display option
 - ✨ feat(crate_versions): add bare version output option

### Fixed

 - fix(deps): update rust crate tame-index to 0.26.0
 - fix(deps): update rust crate thiserror to 2.0.18
 - fix(deps): update rust crate smol_str to 0.3.5
 - fix(deps): update rust crate clap to 4.5.54
 - fix(deps): update rust crate tempfile to 3.24.0
 - fix(deps): update rust crate log to 0.4.29
 - fix(deps): update rust crate tame-index to 0.25.0
 - fix(deps): update rust crate trycmd to 0.15.11
 - fix(deps): update rust crate clap to 4.5.53
 - fix(deps): update rust crate tame-index to 0.24.1
 - fix(deps): update rust crate simple_logger to 5.1.0
 - fix(deps): update rust crate thiserror to 2.0.17
 - fix(deps): update rust crate smol_str to 0.3.4
 - fix(deps): update rust crate clap to 4.5.50
 - fix(deps): update rust crate tempfile to 3.23.0
 - fix(deps): update rust crate semver to 1.0.27
 - fix(deps): update rust crate log to 0.4.28
 - fix(deps): update rust crate clap to 4.5.48
 - fix(deps): update rust crate tame-index to 0.23.0
 - fix(deps): update rust crate tempfile to 3.21.0
 - fix(deps): update rust crate rstest to 0.26.1
 - fix(deps): update rust crate trycmd to 0.15.10
 - fix(deps): update rust crate thiserror to 2.0.16
 - fix(deps): update rust crate clap-verbosity-flag to 3.0.4
 - fix(deps): update rust crate clap to 4.5.45

### Changed

 - ♻️ refactor(crate_versions): adjust output format for consistency
 - refactor: convert to workspace structure

### Security

 - chore(deps): update rust crate tempfile to 3.20.0
 - chore(deps): update rust crate tame-index to 0.22.0
 - chore(deps): update rust crate clap-verbosity-flag to 3.0.3
 - chore(deps): update rust crate clap to 4.5.40
 - chore(deps): update rust crate clap to 4.5.37
 - chore(deps): update rust crate clap to 4.5.36
 - chore(deps): update rust crate env_logger to 0.11.8
 - chore(deps): update rust crate log to 0.4.27
 - chore(deps): update rust crate clap to 4.5.34
 - chore(deps): update rust crate tempfile to 3.19.1
 - chore(deps): update rust crate tempfile to 3.19.0
 - chore(deps): update rust crate clap to 4.5.32
 - chore(deps): update rust crate env_logger to 0.11.7
 - chore(deps): update rust crate tempfile to 3.18.0
 - chore(deps): update rust crate rstest to 0.25.0
 - chore(deps): update rust crate thiserror to 2.0.12
 - chore(deps): update rust crate semver to 1.0.26
 - chore(deps): update rust crate tame-index to 0.18.1
 - chore(deps): update rust crate clap to 4.5.31
 - chore(deps): update rust crate tempfile to 3.17.1
 - chore(deps): update rust crate log to 0.4.26
 - chore(deps): update rust crate tame-index to 0.18.0
 - chore(deps): update rust crate log to 0.4.26
 - chore(deps): update rust crate clap to 4.5.30
 - chore(deps): update rust crate clap to 4.5.29
 - chore(deps): update rust crate trycmd to 0.15.9
 - chore(deps): update rust crate clap to 4.5.28
 - chore(deps): update rust crate tempfile to 3.16.0
 - chore(deps): update rust crate tame-index to 0.17.0
 - chore(deps): update rust crate semver to 1.0.25
 - chore(deps): update rust crate clap to 4.5.27
 - chore(deps): update rust crate thiserror to 2.0.11
 - chore(deps): update rust crate log to 0.4.25
 - chore(deps): update rust crate thiserror to 2.0.10
 - chore(deps): update rust crate clap to 4.5.26
 - chore(deps): update rust crate tempfile to 3.15.0
 - chore(deps): update rust crate rstest to 0.24.0
 - chore(deps): update rust crate env_logger to 0.11.6
 - chore(deps): update rust crate thiserror to 2.0.9
 - chore(deps): update rust crate clap-verbosity-flag to 3.0.2
 - chore(deps): update rust crate thiserror to 2.0.8
 - chore(deps): update rust crate thiserror to 2.0.6
 - chore(deps): update rust crate semver to 1.0.24
 - chore(deps): update rust crate thiserror to 2.0.4
 - chore(deps): update rust crate clap to 4.5.23
 - chore(deps): update rust crate tame-index to 0.16.0
 - chore(deps): update rust crate clap-verbosity-flag to 3.0.1
 - chore(deps): update rust crate clap-verbosity-flag to v3
 - chore(deps): update rust crate clap-verbosity-flag to 2.2.3
 - chore(deps): update rust crate clap to 4.5.21
 - chore(deps): update rust crate thiserror to 2.0.3
 - chore(deps): update rust crate thiserror to v2
 - chore(deps): update rust crate thiserror to 1.0.68
 - chore(deps): update rust crate thiserror to 1.0.67

[Unreleased]: https://github.com/jerus-org/kdeets/compare/v0.1.33...HEAD
[0.1.33]: https://github.com/jerus-org/kdeets/compare/v0.1.32...v0.1.33
[0.1.32]: https://github.com/jerus-org/kdeets/compare/v0.1.31...v0.1.32
[0.1.31]: https://github.com/jerus-org/kdeets/compare/v0.1.30...v0.1.31
[0.1.30]: https://github.com/jerus-org/kdeets/compare/v0.1.29...v0.1.30
[0.1.29]: https://github.com/jerus-org/kdeets/compare/v0.1.28...v0.1.29
[0.1.28]: https://github.com/jerus-org/kdeets/releases/tag/v0.1.28

