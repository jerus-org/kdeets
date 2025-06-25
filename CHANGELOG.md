# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Security

- Dependencies: update rust crate clap to 4.5.40(pr [#120])
- Dependencies: update rust crate clap-verbosity-flag to 3.0.3(pr [#121])

## [0.1.21] - 2025-05-28

### Changed

- 🔧 chore(config)-update renovate schedule configuration(pr [#119])

## [0.1.20] - 2025-05-03

### Security

- Dependencies: update dependency toolkit to v2.10.7(pr [#118])

## [0.1.19] - 2025-04-26

### Security

- Dependencies: update rust crate clap to 4.5.37(pr [#117])

## [0.1.18] - 2025-04-22

### Changed

- 👷 ci(circleci)-update circleci-toolkit orb version(pr [#116])

### Security

- Dependencies: update dependency toolkit to v2.8.1(pr [#115])

## [0.1.17] - 2025-04-12

### Security

- Dependencies: update rust crate clap to 4.5.36(pr [#111])

## [0.1.16] - 2025-04-05

### Security

- Dependencies: update rust crate env_logger to 0.11.8(pr [#112])

## [0.1.15] - 2025-03-29

### Security

- Dependencies: update rust crate clap to 4.5.34(pr [#109])
- Dependencies: update rust crate log to 0.4.27(pr [#110])

## [0.1.14] - 2025-03-22

### Security

- Dependencies: update dependency toolkit to v2.5.1(pr [#108])
- Dependencies: update rust crate tempfile to 3.19.1(pr [#107])

## [0.1.13] - 2025-03-15

### Security

- Dependencies: update rust crate env_logger to 0.11.7(pr [#105])
- Dependencies: update rust crate clap to 4.5.32(pr [#104])
- Dependencies: update rust crate tempfile to 3.19.0(pr [#106])

## [0.1.12] - 2025-03-08

### Security

- Dependencies: update rust crate semver to 1.0.26(pr [#99])
- Dependencies: update rust crate thiserror to 2.0.12(pr [#100])
- Dependencies: update dependency toolkit to v2.1.0(pr [#101])
- Dependencies: update rust crate rstest to 0.25.0(pr [#102])
- Dependencies: update rust crate tempfile to 3.18.0(pr [#103])

## [0.1.11] - 2025-03-01

### Security

- Dependencies: update dependency toolkit to v2.0.13(pr [#96])
- Dependencies: update rust crate clap to 4.5.31(pr [#97])
- Dependencies: update rust crate tame-index to 0.18.1(pr [#98])

## [0.1.10] - 2025-02-22

### Security

- Dependencies: update rust crate clap to 4.5.30(pr [#92])
- Dependencies: update rust crate tame-index to 0.18.0(pr [#93])
- Dependencies: update rust crate log to 0.4.26(pr [#94])
- Dependencies: update rust crate tempfile to 3.17.1(pr [#95])

## [0.1.9] - 2025-02-15

### Security

- Dependencies: update rust crate trycmd to 0.15.9(pr [#91])
- Dependencies: update rust crate clap to 4.5.29(pr [#90])

## [0.1.8] - 2025-02-08

### Security

- Dependencies: update rust crate clap to 4.5.28(pr [#89])

## [0.1.7] - 2025-02-01

### Security

- Dependencies: update rust crate tempfile to 3.16.0(pr [#88])

## [0.1.6] - 2025-01-25

### Changed

- chore(config)-migrate renovate config(pr [#86])

### Security

- Dependencies: update rust crate clap to 4.5.27(pr [#84])
- Dependencies: update rust crate semver to 1.0.25(pr [#85])
- Dependencies: update rust crate tame-index to 0.17.0(pr [#87])

## [0.1.5] - 2025-01-20

### Changed

- 🔧 chore(circleci): modify CircleCI configuration(pr [#75])
- 👷 ci(circleci): update circleci toolkit orb version(pr [#76])
- 👷 ci(circleci): update release workflow dependencies(pr [#77])
- ci(circleci)-update circleci-toolkit orb version(pr [#78])
- ci(circleci)-update release workflow with workspace and versioning(pr [#79])
- ci(circleci)-update CircleCI toolkit orb version(pr [#81])
- ci(circleci)-persist semver in bash environment(pr [#82])
- 🔧 chore(ci): update circleci toolkit orb version(pr [#83])

### Fixed

- ci: reformat indentation in CircleCI config(pr [#80])

### Security

- Dependencies: update rust crate clap to 4.5.26(pr [#69])
- Dependencies: update rust crate thiserror to 2.0.10(pr [#70])
- Dependencies: update rust crate forestry to 1.9.3(pr [#71])
- Dependencies: update dependency toolkit to v2(pr [#72])
- Dependencies: update rust crate log to 0.4.25(pr [#73])
- Dependencies: update rust crate thiserror to 2.0.11(pr [#74])

## [0.1.4] - 2025-01-04

### Security

- Dependencies: update rust crate forestry to 1.7.0(pr [#66])
- Dependencies: update rust crate rstest to 0.24.0(pr [#67])
- Dependencies: update rust crate tempfile to 3.15.0(pr [#68])

## [0.1.3] - 2024-12-28

### Changed

- chore-add release configuration file with pre-release settings(pr [#65])

## [0.1.2] - 2024-12-28

### Security

- Dependencies: update rust crate thiserror to 2.0.8(pr [#61])
- Dependencies: update rust crate thiserror to 2.0.9(pr [#63])
- Dependencies: update rust crate env_logger to 0.11.6(pr [#62])
- Dependencies: update dependency toolkit to v1.23.0(pr [#64])

## [0.1.1] - 2024-12-22

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
- Dependencies: update rust crate thiserror to 2.0.4(pr [#55])
- Dependencies: update rust crate semver to 1.0.24(pr [#57])
- Dependencies: update rust crate forestry to 1.6.2(pr [#56])
- Dependencies: update rust crate thiserror to 2.0.6(pr [#58])
- Dependencies: update dependency toolkit to v1.20.2(pr [#59])
- Dependencies: update rust crate clap-verbosity-flag to 3.0.2(pr [#60])

## [0.1.0] - 2024-10-27

### Changed

- Configure Renovate(pr [#1])
- chore-add sonar-project properties file for SonarQube integration(pr [#2])

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
[#55]: https://github.com/jerus-org/kdeets/pull/55
[#57]: https://github.com/jerus-org/kdeets/pull/57
[#56]: https://github.com/jerus-org/kdeets/pull/56
[#58]: https://github.com/jerus-org/kdeets/pull/58
[#59]: https://github.com/jerus-org/kdeets/pull/59
[#61]: https://github.com/jerus-org/kdeets/pull/61
[#60]: https://github.com/jerus-org/kdeets/pull/60
[#63]: https://github.com/jerus-org/kdeets/pull/63
[#62]: https://github.com/jerus-org/kdeets/pull/62
[#64]: https://github.com/jerus-org/kdeets/pull/64
[#65]: https://github.com/jerus-org/kdeets/pull/65
[#66]: https://github.com/jerus-org/kdeets/pull/66
[#67]: https://github.com/jerus-org/kdeets/pull/67
[#68]: https://github.com/jerus-org/kdeets/pull/68
[#69]: https://github.com/jerus-org/kdeets/pull/69
[#70]: https://github.com/jerus-org/kdeets/pull/70
[#71]: https://github.com/jerus-org/kdeets/pull/71
[#72]: https://github.com/jerus-org/kdeets/pull/72
[#73]: https://github.com/jerus-org/kdeets/pull/73
[#74]: https://github.com/jerus-org/kdeets/pull/74
[#75]: https://github.com/jerus-org/kdeets/pull/75
[#76]: https://github.com/jerus-org/kdeets/pull/76
[#77]: https://github.com/jerus-org/kdeets/pull/77
[#78]: https://github.com/jerus-org/kdeets/pull/78
[#79]: https://github.com/jerus-org/kdeets/pull/79
[#80]: https://github.com/jerus-org/kdeets/pull/80
[#81]: https://github.com/jerus-org/kdeets/pull/81
[#82]: https://github.com/jerus-org/kdeets/pull/82
[#83]: https://github.com/jerus-org/kdeets/pull/83
[#86]: https://github.com/jerus-org/kdeets/pull/86
[#84]: https://github.com/jerus-org/kdeets/pull/84
[#85]: https://github.com/jerus-org/kdeets/pull/85
[#87]: https://github.com/jerus-org/kdeets/pull/87
[#88]: https://github.com/jerus-org/kdeets/pull/88
[#89]: https://github.com/jerus-org/kdeets/pull/89
[#91]: https://github.com/jerus-org/kdeets/pull/91
[#90]: https://github.com/jerus-org/kdeets/pull/90
[#92]: https://github.com/jerus-org/kdeets/pull/92
[#93]: https://github.com/jerus-org/kdeets/pull/93
[#94]: https://github.com/jerus-org/kdeets/pull/94
[#95]: https://github.com/jerus-org/kdeets/pull/95
[#96]: https://github.com/jerus-org/kdeets/pull/96
[#97]: https://github.com/jerus-org/kdeets/pull/97
[#98]: https://github.com/jerus-org/kdeets/pull/98
[#99]: https://github.com/jerus-org/kdeets/pull/99
[#100]: https://github.com/jerus-org/kdeets/pull/100
[#101]: https://github.com/jerus-org/kdeets/pull/101
[#102]: https://github.com/jerus-org/kdeets/pull/102
[#103]: https://github.com/jerus-org/kdeets/pull/103
[#105]: https://github.com/jerus-org/kdeets/pull/105
[#104]: https://github.com/jerus-org/kdeets/pull/104
[#106]: https://github.com/jerus-org/kdeets/pull/106
[#108]: https://github.com/jerus-org/kdeets/pull/108
[#107]: https://github.com/jerus-org/kdeets/pull/107
[#109]: https://github.com/jerus-org/kdeets/pull/109
[#110]: https://github.com/jerus-org/kdeets/pull/110
[#112]: https://github.com/jerus-org/kdeets/pull/112
[#111]: https://github.com/jerus-org/kdeets/pull/111
[#115]: https://github.com/jerus-org/kdeets/pull/115
[#116]: https://github.com/jerus-org/kdeets/pull/116
[#117]: https://github.com/jerus-org/kdeets/pull/117
[#118]: https://github.com/jerus-org/kdeets/pull/118
[#119]: https://github.com/jerus-org/kdeets/pull/119
[#120]: https://github.com/jerus-org/kdeets/pull/120
[#121]: https://github.com/jerus-org/kdeets/pull/121
[Unreleased]: https://github.com/jerus-org/kdeets/compare/v0.1.21...HEAD
[0.1.21]: https://github.com/jerus-org/kdeets/compare/v0.1.20...v0.1.21
[0.1.20]: https://github.com/jerus-org/kdeets/compare/v0.1.19...v0.1.20
[0.1.19]: https://github.com/jerus-org/kdeets/compare/v0.1.18...v0.1.19
[0.1.18]: https://github.com/jerus-org/kdeets/compare/v0.1.17...v0.1.18
[0.1.17]: https://github.com/jerus-org/kdeets/compare/v0.1.16...v0.1.17
[0.1.16]: https://github.com/jerus-org/kdeets/compare/v0.1.15...v0.1.16
[0.1.15]: https://github.com/jerus-org/kdeets/compare/v0.1.14...v0.1.15
[0.1.14]: https://github.com/jerus-org/kdeets/compare/v0.1.13...v0.1.14
[0.1.13]: https://github.com/jerus-org/kdeets/compare/v0.1.12...v0.1.13
[0.1.12]: https://github.com/jerus-org/kdeets/compare/v0.1.11...v0.1.12
[0.1.11]: https://github.com/jerus-org/kdeets/compare/v0.1.10...v0.1.11
[0.1.10]: https://github.com/jerus-org/kdeets/compare/v0.1.9...v0.1.10
[0.1.9]: https://github.com/jerus-org/kdeets/compare/v0.1.8...v0.1.9
[0.1.8]: https://github.com/jerus-org/kdeets/compare/v0.1.7...v0.1.8
[0.1.7]: https://github.com/jerus-org/kdeets/compare/v0.1.6...v0.1.7
[0.1.6]: https://github.com/jerus-org/kdeets/compare/v0.1.5...v0.1.6
[0.1.5]: https://github.com/jerus-org/kdeets/compare/v0.1.4...v0.1.5
[0.1.4]: https://github.com/jerus-org/kdeets/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/jerus-org/kdeets/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/jerus-org/kdeets/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/jerus-org/kdeets/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/jerus-org/kdeets/releases/tag/v0.1.0
