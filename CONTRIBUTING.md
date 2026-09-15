# Contributing

Open an issue before proposing a public API change. Pull requests must preserve
typed domain returns, keep HTTP envelopes private, include tests, and pass every
required workflow. Generated contract files are reviewed as source; regeneration
tools are maintained outside this public repository.

Models and API clients live under `src/generated/<resource>/`. Every resource
exports its own public module (for example `inttegro::payment_method`), with one
model per file and `client_<resource>.rs` for its operations. Generated resource
types have no crate-root compatibility re-exports; SDK-wide types such as
`Client` and `RequestOptions` remain at the root.

After the external contract generator writes its flat intermediate files, split
them before reviewing or committing generated code:

```sh
node scripts/split-generated-models.mjs
node scripts/split-resource-operations.mjs
cargo fmt
node scripts/split-generated-models.mjs --check
node scripts/split-resource-operations.mjs --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
```

The splitters refuse to overwrite dirty generated files. Review any contract
change at its resource path, including the model and its operation client.
