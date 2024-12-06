# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- add host rules for CircleCI in renovate configuration(pr [#4])
- add command-line interface with logging and verbosity options(pr [#5])
- add subcommand support for querying crate information(pr [#7])
- add colorful crate for enhanced console output(pr [#8])
- add key flag for listing key values and colorize output(pr [#9])
- add error handling with thiserror and create new library module(pr [#16])
- add RustVersions command to query maximum Rust version for a crate(pr [#30])
- add Setup struct with command-line parsing and crate setup functionality(pr [#33])
- add location option for local registry path and re-enable tests(pr [#41])
- add no_colour option to disable colored output(pr [#44])

### Changed

- chore-add sonar-project properties file for SonarQube integration(pr [#2])
- Configure Renovate(pr [#1])
- Prepare for first release(pr [#11])
- refactor(cli)-rename Krate to Crate_ for consistency with command name(pr [#13])
- docs-update README to remove outdated --krate flag reference(pr [#14])
- refactor-extract crate version handling logic into separate module(pr [#15])
- refactor(crate_versions)-change run function to return formatted string instead of printing directly(pr [#17])
- Output-string-from-crate-versions-run(pr [#19])
- ci(circleci)-add sonarcloud integration and security audit job(pr [#21])
- chore(circleci)-update toolkit orb to version 1.17.0 and add code coverage step(pr [#25])
- chore-update CircleCI toolkit orb to version 1.18.0(pr [#26])
- test(cli)-add CLI tests using trycmd for command cases(pr [#28])
- ci-add release-flag parameter to CircleCI config(pr [#35])
- ci-update CircleCI config to include pcu_push parameter(pr [#36])
- ci(circleci)-add make_release job with configurable parameters for release automation(pr [#37])
- ci(circleci)-add pcu_update_changelog flag to workflows configuration(pr [#38])
- ci-remove deprecated parameters from CircleCI config(pr [#39])
- test(setup)-add test for default directory creation in setup run(pr [#42])
- docs-update README with Rust versions for dependencies and usage instructions(pr [#43])
- Nextest-test(pr [#48])

### Security

- Dependencies: update dependency toolkit to v1.15.0(pr [#6])
- Dependencies: update rust crate thiserror to 1.0.67(pr [#18])
- Dependencies: update rust crate thiserror to 1.0.68(pr [#20])
- Dependencies: update rust crate thiserror to v2(pr [#22])
- Dependencies: update dependency toolkit to v1.16.0(pr [#23])
- Dependencies: update dependency toolkit to v1.19.0(pr [#29])
- Dependencies: update rust crate thiserror to 2.0.3(pr [#31])
- Dependencies: update rust crate clap to 4.5.21(pr [#40])
- Dependencies: update rust crate clap-verbosity-flag to 2.2.3(pr [#45])
- Dependencies: update rust crate clap-verbosity-flag to v3(pr [#46])
- Dependencies: bump rustls from 0.23.16 to 0.23.18 in the cargo group across 1 directory(pr [#47])
- Dependencies: update rust crate forestry to 1.6.0(pr [#51])
- Dependencies: update rust crate clap-verbosity-flag to 3.0.1(pr [#50])
- Dependencies: update rust crate tame-index to 0.16.0(pr [#52])
- Dependencies: update rust crate forestry to 1.6.1(pr [#54])
- Dependencies: update rust crate clap to 4.5.23(pr [#53])

[#2]: https://github.com/jerus-org/kdeets/pull/2
[#1]: https://github.com/jerus-org/kdeets/pull/1
[#4]: https://github.com/jerus-org/kdeets/pull/4
[#5]: https://github.com/jerus-org/kdeets/pull/5
[#6]: https://github.com/jerus-org/kdeets/pull/6
[#7]: https://github.com/jerus-org/kdeets/pull/7
[#8]: https://github.com/jerus-org/kdeets/pull/8
[#9]: https://github.com/jerus-org/kdeets/pull/9
[#11]: https://github.com/jerus-org/kdeets/pull/11
[#13]: https://github.com/jerus-org/kdeets/pull/13
[#14]: https://github.com/jerus-org/kdeets/pull/14
[#15]: https://github.com/jerus-org/kdeets/pull/15
[#16]: https://github.com/jerus-org/kdeets/pull/16
[#17]: https://github.com/jerus-org/kdeets/pull/17
[#19]: https://github.com/jerus-org/kdeets/pull/19
[#18]: https://github.com/jerus-org/kdeets/pull/18
[#20]: https://github.com/jerus-org/kdeets/pull/20
[#21]: https://github.com/jerus-org/kdeets/pull/21
[#22]: https://github.com/jerus-org/kdeets/pull/22
[#23]: https://github.com/jerus-org/kdeets/pull/23
[#25]: https://github.com/jerus-org/kdeets/pull/25
[#26]: https://github.com/jerus-org/kdeets/pull/26
[#28]: https://github.com/jerus-org/kdeets/pull/28
[#29]: https://github.com/jerus-org/kdeets/pull/29
[#30]: https://github.com/jerus-org/kdeets/pull/30
[#31]: https://github.com/jerus-org/kdeets/pull/31
[#33]: https://github.com/jerus-org/kdeets/pull/33
[#40]: https://github.com/jerus-org/kdeets/pull/40
[#41]: https://github.com/jerus-org/kdeets/pull/41
[#42]: https://github.com/jerus-org/kdeets/pull/42
[#43]: https://github.com/jerus-org/kdeets/pull/43
[#44]: https://github.com/jerus-org/kdeets/pull/44
[#45]: https://github.com/jerus-org/kdeets/pull/45
[#46]: https://github.com/jerus-org/kdeets/pull/46
[#47]: https://github.com/jerus-org/kdeets/pull/47
[#48]: https://github.com/jerus-org/kdeets/pull/48
[#51]: https://github.com/jerus-org/kdeets/pull/51
[#50]: https://github.com/jerus-org/kdeets/pull/50
[#52]: https://github.com/jerus-org/kdeets/pull/52
[#54]: https://github.com/jerus-org/kdeets/pull/54
[#53]: https://github.com/jerus-org/kdeets/pull/53
