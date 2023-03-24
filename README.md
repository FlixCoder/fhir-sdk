# FHIR SDK

[![crates.io page](https://img.shields.io/crates/v/fhir-sdk.svg)](https://crates.io/crates/fhir-sdk)
[![docs.rs page](https://docs.rs/fhir-sdk/badge.svg)](https://docs.rs/fhir-sdk/)
![license: MIT](https://img.shields.io/crates/l/fhir-sdk.svg)

This is a [FHIR](https://www.hl7.org/fhir/) library in its early stages. The models are generated from the FHIR StructureDefinitions (see <https://www.hl7.org/fhir/downloads.html>). It aims to be:

- fully compliant
- as safe as possibe
- as easy to use as possible
- fully featured

## Features

- [x] Generation of FHIR codes, types and resources
- [x] Serialization and deserialization to and from JSON
- [x] Linked code-fields to respective enums
- [x] Builders for types and resources
- [ ] Allow to convert code enums to Coding and CodeableConcept
- [ ] Implementation of base traits
  - [ ] Resource, including FHIR version, resource type (const)
  - [ ] DomainResource
  - [ ] IdentifiableResource
- [ ] FHIR client implmentation
- [ ] FHIRpath implementation
- [ ] Resource validation using FHIRpath
- [ ] Search parameters
- [ ] XML

## Known Problems

- Due to the big number of big types, the auto-generated builders take a long time during the build time.
- The builder cannot use `setter(strip_option)`, because it disables dynamic setting of optional fields.
- `Vec<Option<T>>` is annoying, but sadly is required to allow `[null, {...}, null]` for extensions..

## Lints

This projects uses a bunch of clippy lints for higher code quality and style.

Install [`cargo-lints`](https://github.com/soramitsu/iroha2-cargo_lints) using `cargo install --git https://github.com/FlixCoder/cargo-lints`. The lints are defined in `lints.toml` and can be checked by running `cargo lints clippy --all-targets --workspace`.

## License

Licensed under the MIT license. All contributors agree to license under this license.
