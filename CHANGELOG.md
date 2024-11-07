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

### Security

- Dependencies: update dependency toolkit to v1.15.0(pr [#6])
- Dependencies: update rust crate thiserror to 1.0.67(pr [#18])
- Dependencies: update rust crate thiserror to 1.0.68(pr [#20])
- Dependencies: update rust crate thiserror to v2(pr [#22])
- Dependencies: update dependency toolkit to v1.16.0(pr [#23])

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
