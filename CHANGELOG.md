# Changelog

All notable changes to this project will be documented in this file.

## [0.15.1] - 2026-01-03

- Re-enabled reqwest forms.

## [0.15.0] - 2026-01-03

- Updated reqwest to 0.13.

## [0.14.3] - 2025-11-05

- Updated dependencies

## [0.14.2] - 2025-07-26

### Bug Fixes

- Remove duplicate `resourceType` field in serialized generic `Resource` by @SHA888.

### Miscellaneous

- Allow using RusTLS instead of OpenSSL by @drakon64.
- Make all `reqwest` features configurable.
- Switch to Rust edition 2024.

## [0.14.1] - 2024-08-25

### Bug Fixes

- Do not resolve fullUrl if it is a DELETE request

### Features

- Set X-Correlation-Id and X-Request-Id headers and log them
- Re-use the same correlation ID in paging

## [0.14.0] - 2024-08-18

### Bug Fixes

- Fix feature specific build errors
- Add source impls to errors

### Features

- [**breaking**] By default, clients reject different major FHIR versions and different URL origins. This can be allowed through flags in the `ClientBuilder`.
- Allow sending custom HTTP requests
- Add and implement AnyResource trait for all resources
- Implement custom search request

### Refactor

- [**breaking**] Remove code duplication across FHIR versions
  - This requires the FHIR version to be implement an internal trait.
  - There are now generic search parameters across versions. Though there is aliases for the version specific parameters, so that hopefully nothing breaks.
  - Some more generic requirements, should not be visible to the user mostly.
- [**breaking**] Remodel the way paging works
  - History and search requests return a `Page` now. It offers methods to either manually go through the pages and get the entries, or automatically page and return a `Stream` of all matches. It is possible to access the inner `Bundle` though, so `history` can be compatible to previous usage.
  - Different parameters and generics on the bespoken methods.

### Testing

- Add first CI and switch to cargo-make

## [0.13.0] - 2024-07-28

### Bug Fixes

- Atomically patch request settings to not overwrite unrelated changes.

### Features

- Add derive Hash to codes
- **breaking**: Revamp auth_callback
  - Introduce the `LoginManager` trait for more flexible authentication management.
  - Allow mutable access in auth_callback.
  - Improve internal request logic to not run multiple auth callbacks at the same time.

## [0.12.0] - 2024-05-26

### Features

- Allow Clone callbacks in ClientBuilder::auth_callback
- Allow Custom Reqwest Client

## [0.11.0] - 2024-04-14

### Documentation

- Add example for JSON deserialization

### Features

- Add and implement AnyResourceWrite for Resource
- Add REST call `history`

### Miscellaneous Tasks

- Bump dependencies
- Bump reqwest to 0.12, going to hyper 1.0

## [0.10.0] - 2024-02-29

### Bug Fixes

- Expose BuilderError

### Features

- Impl FromStr for ResourceType and revamp WrongResourceType error

### Miscellaneous Tasks

- Switch generator back to crates io fhir models

### Testing

- Fix docker setup for new versions

## [0.9.0] - 2024-01-25

### Features

- Add FHIR STU3 -- thanks @jarimayenburg

## [0.8.0] - 2024-01-14

### Features

- Implement JSON Patch alongside FHIRPath Patch

### Miscellaneous Tasks

- Improve format implementation generation
- Add link to fhirbolt
- Add missing derives on MissingSearch

### Refactor

- Parse FHIR versions separately
- [**breaking**] Switch back to derive_builder
- [**breaking**] Change the whole client structure to allow multiple FHIR versions at once

## [0.7.0] - 2023-11-03

### Bug Fixes

- R4B was not compiling with new Subscription operations
- Make ETag parsing work with non-weak tags

### Features

- Allow spawning different FHIR servers
- Add Subscription operations to client
- Implement authorization callback
- Add tracing

### Miscellaneous Tasks

- Improve error messages
- Improve README

### Refactor

- Move docker setup to xtask command
- Make auth_callback part of Client instead of RequestSettings

### Testing

- Make client tests for R4B too
- Add Medplum as alternative FHIR server to docker setup
- Test client with medplum server
- Make all servers run at once to be able to test them at once
- Fix dispatch dropped errors

## [0.6.0] - 2023-09-16

### Bug Fixes

- Do not filter out codes like HTTPVerb
- Go deeper into the codes, there are more layers

### Features

- Implement batch/transaction requests
- Implement Encounter $everything operation
- Implement Patient $everything and $match
- Implement PATCH

### Refactor

- Change the way references are parsed

### Testing

- Refactor to use batches

## [0.5.0] - 2023-09-08

### Bug Fixes

- Have the doc-test only at the right features
- Impl Send for returned Stream
- Base64 needs to use padding and strip whitespaces

### Documentation

- Add search to example

### Features

- Implement creation of References
- Add Reference parsing
- Implement reading a referenced resource
- Add getters for code by system for CodeableConcept
- Implement Rust types for primitive FHIR types
- Generate models using new primitive types

### Miscellaneous Tasks

- Update generator fhir-model

### Performance

- Use optimized proc macro compilation

### Refactor

- Move misc functions to module
- Move and rename reference creation methods

### Testing

- Create xtask test command
- Reference search
- Fetch referenced resource
- Adjust to new primitive date types
- Adjust JSON comparison to FHIR to fix tests

## [0.4.1] - 2023-09-01

### Bug Fixes

- Make everything compile with different features

### Features

- Add back search for all resources
- Allow to disable builders
- Implement getting the server's capabilities

### Miscellaneous Tasks

- Improve README

### Testing

- Use local test server

## [0.4.0] - 2023-08-31

### Bug Fixes

- Improve code generation using cleaner URL usage
- Do not override headers with default headers
- Adjust wrong LinkRelationTypes definitions
- Implement Copy for ResourceType
- Implement Display for ResourceType

### Documentation

- Improve generated doc comments

### Features

- Implement basic REST client
- Implement paging
- Implement search with typed helpers

### Miscellaneous Tasks

- Fix new clippy lints
- Add expansions for R4B and remove unneeded definitions
- Update typed-builder and fhir-sdk dependencies
- Update README

## [0.3.0] - 2023-04-01

### Bug Fixes

- Integer64 is actually a JSON String
- Add values for RelatedArtifactTypeExpanded
- [**breaking**] Choice field extensions include the value type in the field name
- Specimen.combined actually links to SpecimenCombined, not PublicationStatus

### Features

- Add FHIR R5 files
- Generate R5 models

### Refactor

- Use own FHIR models for code generation

## [0.2.0] - 2023-03-26

### Features

- Implement code conversion to Coding and CodeableConcept
- Implement base traits for all resources
- Implement IdentifiableResource
- Extend the IdentifiableResource trait

### Miscellaneous Tasks

- Add resolver = 2

## [0.1.0] - 2023-03-24

### Features

- Initial implementation

<!-- generated by git-cliff -->
