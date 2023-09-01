# FHIR SDK

[![crates.io page](https://img.shields.io/crates/v/fhir-sdk.svg)](https://crates.io/crates/fhir-sdk)
[![docs.rs page](https://docs.rs/fhir-sdk/badge.svg)](https://docs.rs/fhir-sdk/)
![license: MIT](https://img.shields.io/crates/l/fhir-sdk.svg)

This is a [FHIR](https://www.hl7.org/fhir/) library in its early stages. The models are generated from the FHIR StructureDefinitions (see [FHIR downloads](https://www.hl7.org/fhir/downloads.html)). It aims to be:

- fully compliant
- as safe as possibe
- as easy to use as possible
- fully featured

## Features

- [x] Generated FHIR codes, types and resources
- [x] Serialization and deserialization to and from JSON
- [x] Optional builders for types and resources
- [x] Implementation of base traits
  - [x] (Base)Resource for accessing common fields
  - [x] NamedResource for getting the resource type in const time
  - [x] DomainResource for accessing common fields
  - [x] IdentifiableResource for all resources with an identifier field
- [x] REST client implementation
  - [x] Create, Read, Update, Delete
  - [x] Search
  - [x] Paging
  - [ ] Capabilities
  - [ ] Batch operations / Transactions
  - [ ] Operations
  - [ ] Patch
- [ ] FHIRpath implementation
- [ ] Resource validation using FHIRpath and regular expressions
- [ ] GraphQL client

## Not Planned

- XML

## Example

```rust
#![cfg(all(feature = "r5", feature = "builders", feature = "client"))]
use fhir_sdk::r5::resources::Patient;
use fhir_sdk::client::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Set up the client using the local test server.
    let client = Client::new("http://localhost:8090/fhir/".parse().unwrap())?;

    // Create a Patient resource using a typed builder.
    let mut patient = Patient::builder().active(false).build();
    // Push it to the server.
    patient.create(&client).await?;
    // The id and versionId is updated automatically this way.
    assert!(patient.id.is_some());

    Ok(())
}
```

For more examples, see the [tests](https://github.com/FlixCoder/fhir-sdk/blob/main/crates/fhir-sdk/tests/client.rs).

## Testing

Simply set up the FHIR test server using `docker compose up -d` and run `cargo test --workspace` in the workspace root.

## Known Problems

- Due to the big number of big types, the compile time and its memory usage is really high. The auto-generated builders also take a long time during the build time. The builders can be disabled by disabling the `builders` feature to save some resources.
- The builder cannot use `setter(strip_option)`, because it disables dynamic setting of optional fields.
- `Vec<Option<T>>` is annoying, but sadly is required to allow `[null, {...}, null]` for extensions..

## Lints

This projects uses a bunch of clippy lints for higher code quality and style.

Install [`cargo-lints`](https://github.com/soramitsu/iroha2-cargo_lints) using `cargo install --git https://github.com/FlixCoder/cargo-lints`. The lints are defined in `lints.toml` and can be checked by running `cargo lints clippy --all-targets --workspace`.

## License

Licensed under the MIT license. All contributors agree to license under this license.
