# Inttegro Rust SDK

The official typed, asynchronous Rust client for server-side Inttegro integrations.

## Install

```toml
[dependencies]
inttegro = "0.1"
```

```rust,no_run
use inttegro::order::LookupOrderRequest;
use inttegro::Client;

# async fn example() -> inttegro::Result<()> {
let client = Client::new(std::env::var("INTTEGRO_API_KEY").unwrap())?;
let order = client.orders().lookup(&LookupOrderRequest {
    order_id: "order_...".into(),
}).await?;
println!("{}", order.id);
# Ok(())
# }
```

All resource methods return typed domain values. HTTP envelopes are private and
no API key, request body, or resource identifier is emitted through telemetry or
error reporting.

Models and API clients are organized in public resource modules. For example,
`inttegro::order::Order` and `inttegro::order::Orders` live in the `order` module.
Generated resource types are imported from their owning modules, not the crate
root. Each generated model has a focused source file under
`src/generated/<resource>/`.

## Observability and error reporting

Pass an implementation of `Telemetry` or `ErrorReporter` through `Client::builder`.
Telemetry emits `inttegro.request.prepared`, `inttegro.response.received`,
`inttegro.response.decoded`, and `inttegro.request.failed`. Error reports are
constructed only when a reporter is configured; the default `Unexpected` policy
reports transport, decoding, unknown, and server failures while leaving routine
client errors to the caller.

See the [API reference](https://docs.rs/inttegro/latest/inttegro/) and
[Inttegro Studio](https://studio.inttegro.com/sdks/rust).
