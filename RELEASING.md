# Releasing

1. Update `Cargo.toml` and `CHANGELOG.md`, then merge the release commit.
2. Create a `release` GitHub environment with no required reviewers and add a
   least-privilege `CARGO_REGISTRY_TOKEN` secret that can publish `inttegro`.
3. Push a `v<version>` tag that points at the release commit.

The workflow verifies the tag and package, runs formatting, Clippy, and tests,
attests the crate, records checksums and provenance, creates a draft GitHub
release, publishes to crates.io, and only then makes the release public.
