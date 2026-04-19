# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Security

- Harden GitHub Actions workflows: pin third-party actions to commit SHAs,
  scope per-job permissions with least privilege, set persist-credentials: false,
  and guard caches against PR poisoning
- Add zizmor and poutine for workflow and CI/CD supply-chain static analysis,
  extracted into reusable workflows with a Tue/Fri scheduled caller
- Replace `ncipollo/release-action` with `gh` CLI in release workflow

### Changed

- Switch test runner from `cargo test` to `cargo-nextest` with CI profile
- Remove inline cargo-deny, cargo-outdated, and cargo-pants from essentials
  workflow (audits moved to reusable workflow; cargo-outdated replaced by
  Renovate)
- Tighten release tag filter from `v*` to semver pattern
- Add Renovate for automated dependency updates with SHA pinning and automerge
- Add CODEOWNERS (`@graelo`)

## [0.3.0] - 2025-12-16

### Added

- `lossy-conversions` feature: allow `IntoF64` for `u64`, `i64`, `usize`, and
  `isize` (these conversions may lose precision for values > 2^53)
- Compile-time diagnostic to advertise the `lossy-conversions` feature when
  attempting unsupported conversions

## [0.2.3] - 2024-08-16

### Fixed

- CI: remove nightly from test matrix (rounding errors)
- Obey clippy lints

## [0.2.2] - 2023-11-08

### Fixed

- Broken doc link
- Macro docs expansion

## [0.2.1] - 2022-08-20

### Changed

- Improve GitHub Actions CI configuration

## [0.2.0] - 2022-08-07

### Breaking changes

- Remove the groupings from `bytes()`
- Force all uses of the `scale_fn` macro to have a doc string

### Added

- `bytes2()` and `bibytes2()` helpers

### Fixed

- Issue #8

## [0.1.5] - 2022-06-26

### Fixed

- Macro hygiene bug in `scale_fn`

## [0.1.4] - 2022-02-17

### Fixed

- Formatting issue for the value 0
- General formatting issue

## [0.1.3] - 2021-10-21

### Changed

- Update README

## [0.1.2] - 2021-10-21

### Added

- `number_()` helper

## [0.1.1] - 2021-10-10

### Added

- `Constraint::UnitOnly` and `bytes_()` helper

## [0.1.0] - 2021-10-10

### Added

- Initial release
- `Value` type with SI prefix scaling
- `format_value!()` macro
- Allowed prefixes and exponent clamping
- Helpers module with `bytes()`, `bibytes()`, `number()` functions
