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
- [x] Client implementation
  - [x] Create, Read, Update, Delete
  - [x] Search + Paging
  - [x] Batch operations / Transactions
  - [x] Authentication callback
  - [x] Operations
  - [x] Patch
  - [ ] GraphQL
- [ ] FHIRpath implementation
- [ ] Resource validation using FHIRpath and regular expressions

## Not Planned

- XML ([fhirbolt](https://github.com/lschmierer/fhirbolt) implements it though)

## Example

```rust
use fhir_sdk::r5::resources::Patient;
use fhir_sdk::client::{*, r5::*};
use fhir_sdk::TryStreamExt;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Set up the client using the local test server.
    let settings = RequestSettings::default()
        .header(header::AUTHORIZATION, "Bearer <token>".parse().unwrap());
    let client = Client::builder()
        .base_url("http://localhost:8100/fhir/".parse().unwrap())
        .request_settings(settings)
        .build()?;

    // Create a Patient resource using a builder.
    let mut patient = Patient::builder().active(false).build().unwrap();
    // Push it to the server.
    patient.create(&client).await?;
    // The id and versionId is updated automatically this way.
    assert!(patient.id.is_some());
    
    // Search for all patient with `active` = false, including pagination.
    let patients: Vec<Patient> = client
        .search(SearchParameters::empty().and(TokenSearch::Standard {
            name: "active",
            system: None,
            code: Some("false"),
            not: false,
        }))
        .try_collect()
        .await?;

    Ok(())
}
```

For more examples, see the [tests](https://github.com/FlixCoder/fhir-sdk/blob/main/crates/fhir-sdk/tests/client-r5.rs) or below.

## Testing

Simply set up the FHIR test environment using `cargo xtask docker -- up -d` and then run `cargo xtask test`.
If you need sudo to run docker, use the `--sudo` or just `-s` flag on `cargo xtask docker`.

## Known Problems

- The compile time and its memory usage are really high. This is due to the big serde derives being highly generic. It might be possible to shave some off by manually implementing Deserialize and Serialize, but that is complex.
- `Vec<Option<T>>` is annoying, but sadly is required to allow `[null, {...}, null]` for using FHIR resources with extensions..
- It is not supported to replace required fields by an extension.

## More examples

### Authentication callback

```rust
use fhir_sdk::r5::resources::Patient;
use fhir_sdk::client::*;

async fn my_auth_callback() -> Result<HeaderValue, eyre::Report> {
    Ok(HeaderValue::from_static("Bearer <token>"))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Set up the client using the local test server.
    let client = Client::builder()
        .base_url("http://localhost:8100/fhir/".parse().unwrap())
        .auth_callback(my_auth_callback)
        .build()?;

    // Create a Patient resource using a builder.
    let mut patient = Patient::builder().active(false).build().unwrap();
    // Push it to the server. On unauthorized failures, the client will call our
    // auth_callback method to refresh the authorization.
    patient.create(&client).await?;

    Ok(())
}
```

### Resource identifier access

```rust
use fhir_sdk::r5::{
    codes::AdministrativeGender,
    resources::{IdentifiableResource, Patient},
    types::{Identifier, HumanName},
};

#[tokio::main]
async fn main() {
    // Create a Patient resource using a builder.
    let mut patient = Patient::builder()
        .active(false)
        .identifier(vec![Some(
            Identifier::builder()
                .system("MySystem".to_owned())
                .value("ID".to_owned())
                .build()
                .unwrap()
        )])
        .gender(AdministrativeGender::Male)
        .name(vec![Some(HumanName::builder().family("Test".to_owned()).build().unwrap())])
        .build()
        .unwrap();

    // Check the identifier value.
    assert_eq!(patient.identifier_with_system("MySystem").map(String::as_str), Some("ID"));
}
```

## License

Licensed under the MIT license. All contributors agree to license under this license.
