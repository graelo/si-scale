# AGENTS.md

This file contains instructions for coding agents working in this repository.

- Repository: <https://github.com/graelo/si-scale>
- Prefer `gh` for GitHub operations.
- Do not mention an agent or assistant in issues, pull requests, comments, or
  commit messages.
- Do not expose private local information, including machine-specific paths.

## Project

`si-scale` is a library crate for formatting numerical values with SI prefixes,
including decimal (1000) and binary (1024) scales. It provides a low-level
`Value` API, the `format_value!` and `scale_fn!` macros, and helpers for common
units such as bytes and seconds.

Rust 1.78 or later is required. The crate uses edition 2018.

## Architecture

- `src/value.rs`: `Value`, numerical conversion traits, scaling, and display.
- `src/prefix.rs`: SI prefixes and scale constraints.
- `src/base.rs`: decimal and binary scale bases.
- `src/format.rs`: `format_value!` and mantissa grouping.
- `src/helpers.rs`: `scale_fn!` and predefined formatting helpers.
- `src/lib.rs`: crate exports, error/result types, prelude, and README-backed
  crate documentation. Keep long-form documentation and examples in `README.md`.

## Verification

The `Makefile` is the canonical definition of local verification tasks. **Read
it before choosing or running verification commands**; do not duplicate its
command implementations here. `make help` lists every target.

The primary targets are:

- `make check`: pre-push gate (formatting, linting, and tests).
- `make check-all`: pre-PR gate (adds dependency, commit-message, Markdown,
  and GitHub Actions security checks).
- `make fix`: formats code and applies Clippy fixes.
- `make md`: lints Markdown against `rumdl.toml`. Run this after editing
  Markdown files.
- `make ci-security`: runs the Poutine and Zizmor GitHub Actions scans.

The check targets mirror the GitHub workflows and assume their external tools
(for example `cargo-nextest`, `cargo-deny`, `cargo-pants`, `convco`, `poutine`,
`zizmor`, `rumdl`, and `cargo-llvm-cov`) are already installed locally.

For focused Rust tests, use `cargo nextest run <test_name>` or
`cargo nextest run <module::tests::name>`. The complete CI test sequence is
implemented in `ci/test_full.sh`; its Nextest CI profile is configured in
`.config/nextest.toml`.

## Documentation and releases

Keep user-facing documentation in sync with behavior:

- Update `README.md` when changing the public API, helper behavior, features,
  supported Rust version, or verification workflow. It is the canonical
  long-form documentation and must not regain cargo-sync-readme markers.
- Keep public-item rustdocs in `src/` accurate; `src/lib.rs` includes the
  README directly so its examples remain crate doctests without duplication.
- For a release version bump, update `Cargo.toml`, the versioned section and
  comparison links in `CHANGELOG.md`, and the README dependency examples.
  Create a `vX.Y.Z` tag; the release workflow derives the GitHub Release
  version from it.
- Commit messages must follow `.convco` Conventional Commit rules. Use
  `make commits` to check them.

`Cargo.toml`, `deny.toml`, and the GitHub workflows define the release and
supply-chain constraints.
