//! Tests for the FHIR resources.
#![cfg(feature = "builders")]
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::print_stdout, reason = "Tests")]

mod json_compare;

use std::fs;

use fhir_model::{
	BuilderError, Date, DateTime, ParsedReference, WrongResourceType, for_all_versions,
};
use serde_json::Value;

use self::json_compare::assert_fhir_json_equal;

/// Test serialization and deserialization of resources for every FHIR version.
macro_rules! serialization_deserialization {
	($version:ident) => {
		#[test]
		fn serialization_deserialization() {
			for entry in fs::read_dir(format!(
				"{}/tests/{}-examples-json/",
				env!("CARGO_MANIFEST_DIR"),
				stringify!($version)
			))
			.expect("read dir")
			{
				let file = entry.expect("file entry").path();
				assert!(file.is_file());

				println!("Checking file `{}`..", file.display());

				let string = fs::read_to_string(file).expect("reading file");
				let json: Value = serde_json::from_str(&string).expect("deserialize to value");
				let deserialized: Resource =
					serde_json::from_value(json.clone()).expect("deserializing");
				let serialized = serde_json::to_value(&deserialized).expect("serializing");
				assert_fhir_json_equal(&serialized, &json);
			}
		}
	};
}

/// Make sure builder works for every FHIR version.
macro_rules! builder_works {
	(stu3) => {
		builder_works!(test @ RequestGroup);
	};
	(r4b) => {
		builder_works!(test @ RequestGroup);
	};
	(r5) => {
		builder_works!(test @ RequestOrchestration);
	};

	(test @ $which:ident) => {
		#[test]
		fn builder_works() -> Result<(), BuilderError> {
			builder_works!(@$which);

			let result = StructureDefinition::builder().build();
			assert!(result.is_err());

			Ok(())
		}
	};
	(@RequestGroup) => {
		let _request_group = RequestGroup::builder()
			.status(RequestStatus::Active)
			.intent(RequestIntent::Order)
			.action(vec![Some(
				RequestGroupAction::builder()
					.timing(RequestGroupActionTiming::DateTime(DateTime::Date(Date::Year(2023))))
					.code(vec![Some(
						CodeableConcept::builder()
							.coding(vec![Some(
								Coding::builder()
									.system("system".to_owned())
									.code("code".to_owned())
									.display("display".to_owned())
									.build()?,
							)])
							.build()?,
					)])
					.build()?,
			)])
			.action_ext(vec![Some(FieldExtension::builder().build()?)])
			.build()?;
	};
	(@RequestOrchestration) => {
		let _request_orchestration = RequestOrchestration::builder()
			.status(RequestStatus::Active)
			.intent(RequestIntent::Order)
			.action(vec![Some(
				RequestOrchestrationAction::builder()
					.timing(RequestOrchestrationActionTiming::DateTime(DateTime::Date(Date::Year(
						2023,
					))))
					.code(vec![Some(
						CodeableConcept::builder()
							.coding(vec![Some(
								Coding::builder()
									.system("system".to_owned())
									.code("code".to_owned())
									.display("display".to_owned())
									.build()?,
							)])
							.build()?,
					)])
					.build()?,
			)])
			.build()?;
	};
}

/// Test conversion of resources to generic Resource and back for every FHIR
/// version.
macro_rules! resource_conversion {
	($version:ident) => {
		#[test]
		fn resource_conversion() {
			let patient = Patient::builder().build().unwrap();
			let resource: Resource = patient.into();
			let patient: Patient = resource.try_into().expect("It is a Patient resource");
			let resource: Resource = patient.into();
			let _patient: &Patient = (&resource).try_into().expect("It is a Patient resource");
			let result: Result<Basic, WrongResourceType> = resource.try_into();
			assert!(result.is_err());
		}
	};
}

/// Test creation of codings and codeable concepts for every FHIR version.
macro_rules! coding_concepts {
	(r5) => {
		coding_concepts!(@extendable CatalogType);
	};
	(r4b) => {
		coding_concepts!(@extendable RiskProbability);
	};
	(stu3) => {
		coding_concepts!(@extendable ExtraActivityType);
	};

	(@extendable $which:ident) => {
		#[test]
		fn coding_concepts() {
			let code = $which::_Custom("Test".to_owned());
			let coding = Coding::from(code.clone());
			assert!(coding.code.is_some());
			assert!(coding.system.is_some());
			let concept = CodeableConcept::from(code);
			assert_eq!(concept.coding.len(), 1);
			assert!(concept.text.is_some());
		}
	};
}

/// Test traits on resources.
macro_rules! resource_traits {
	($version:ident) => {
		#[test]
		fn resource_traits() {
			let ty = Patient::TYPE;
			let mut patient: Resource =
				Patient::builder().id("1".to_owned()).build().unwrap().into();
			assert_eq!(patient.resource_type(), ty);

			assert!(patient.as_base_resource().id().is_some());
			assert!(patient.as_domain_resource().is_some());

			patient.as_base_resource_mut().set_id(None);
			assert!(patient.as_base_resource().id().is_none());
		}
	};
}

/// Test identifiable resource implementations.
macro_rules! identifiable_resource {
	($version:ident) => {
		#[test]
		fn identifiable_resource() {
			let patient: Resource = Patient::builder()
				.identifier(vec![Some(
					Identifier::builder()
						.system("system".to_owned())
						.value("bla".to_owned())
						.build()
						.unwrap(),
				)])
				.build()
				.unwrap()
				.into();
			assert!(patient.as_identifiable_resource().is_some());

			let identifier = patient
				.as_identifiable_resource()
				.expect("Patient has identifiers")
				.identifier()
				.first()
				.and_then(Option::as_ref)
				.expect("We set one identifier");
			assert_eq!(identifier.system.as_deref(), Some("system"));
		}
	};
}

/// Test identifier search.
macro_rules! identifier_search {
	($version:ident) => {
		#[test]
		fn identifier_search() {
			let patient = Patient::builder()
				.identifier(vec![
					Some(
						Identifier::builder()
							.system("system1".to_owned())
							.value("bla1".to_owned())
							.build()
							.unwrap(),
					),
					Some(
						Identifier::builder()
							.r#type(
								CodeableConcept::builder()
									.coding(vec![Some(
										Coding::builder()
											.system("system2".to_owned())
											.code("code2".to_owned())
											.build()
											.unwrap(),
									)])
									.build()
									.unwrap(),
							)
							.value("bla2".to_owned())
							.build()
							.unwrap(),
					),
				])
				.build()
				.unwrap();

			assert_eq!(patient.identifier_with_system("system1").map(String::as_str), Some("bla1"));
			assert_eq!(patient.identifier_with_type("system2", "code2").map(String::as_str), Some("bla2"));
		}
	};
}

/// Test reference parsing.
macro_rules! reference_parsing {
	(stu3) => {
		#[test]
		fn reference_parsing() {
			let reference = Reference::builder()
				.reference("https://server.test/fhir/Encounter/1".to_owned())
				.build()
				.unwrap();
			let parsed = reference.parse().expect("parsing reference");
			assert_eq!(
				parsed,
				ParsedReference::Absolute {
					url: "https://server.test/fhir/Encounter/1",
					resource_type: Some("Encounter"),
					id: Some("1")
				}
			);

			let reference = Reference::builder()
				.reference("https://server.test/fhir/Encounter/1/_history/1".to_owned())
				.build()
				.unwrap();
			let parsed = reference.parse().expect("parsing reference");
			assert_eq!(
				parsed,
				ParsedReference::Absolute {
					url: "https://server.test/fhir/Encounter/1/_history/1",
					resource_type: Some("Encounter"),
					id: Some("1")
				}
			);

			let reference =
				Reference::builder().reference("Encounter/1".to_owned()).build().unwrap();
			let parsed = reference.parse().expect("parsing reference");
			assert_eq!(
				parsed,
				ParsedReference::Relative { resource_type: "Encounter", id: "1", version_id: None }
			);

			let reference = Reference::builder()
				.reference("Encounter/1/_history/1".to_owned())
				.build()
				.unwrap();
			let parsed = reference.parse().expect("parsing reference");
			assert_eq!(
				parsed,
				ParsedReference::Relative {
					resource_type: "Encounter",
					id: "1",
					version_id: Some("1")
				}
			);

			let reference = Reference::builder().reference("#1".to_owned()).build().unwrap();
			let parsed = reference.parse().expect("parsing reference");
			assert_eq!(parsed, ParsedReference::Local { id: "1" });

			let reference = Reference::builder()
				.reference("http://not-fhir.test/1".to_owned())
				.build()
				.unwrap();
			let parsed = reference.parse().expect("parsing reference");
			assert_eq!(
				parsed,
				ParsedReference::Absolute {
					url: "http://not-fhir.test/1",
					resource_type: Some("not-fhir.test"), // irks
					id: Some("1")
				}
			);
		}
	};
	($version:ident) => {
		#[test]
		fn reference_parsing() {
			let reference = Reference::builder()
				.r#type("Encounter".to_owned())
				.reference("https://server.test/fhir/Encounter/1".to_owned())
				.build()
				.unwrap();
			let parsed = reference.parse().expect("parsing reference");
			assert_eq!(
				parsed,
				ParsedReference::Absolute {
					url: "https://server.test/fhir/Encounter/1",
					resource_type: Some("Encounter"),
					id: Some("1")
				}
			);

			let reference = Reference::builder()
				.r#type("Encounter".to_owned())
				.reference("https://server.test/fhir/Encounter/1/_history/1".to_owned())
				.build()
				.unwrap();
			let parsed = reference.parse().expect("parsing reference");
			assert_eq!(
				parsed,
				ParsedReference::Absolute {
					url: "https://server.test/fhir/Encounter/1/_history/1",
					resource_type: Some("Encounter"),
					id: Some("1")
				}
			);

			let reference = Reference::builder()
				.r#type("Encounter".to_owned())
				.reference("Encounter/1".to_owned())
				.build()
				.unwrap();
			let parsed = reference.parse().expect("parsing reference");
			assert_eq!(
				parsed,
				ParsedReference::Relative { resource_type: "Encounter", id: "1", version_id: None }
			);

			let reference = Reference::builder()
				.reference("Encounter/1/_history/1".to_owned())
				.build()
				.unwrap();
			let parsed = reference.parse().expect("parsing reference");
			assert_eq!(
				parsed,
				ParsedReference::Relative {
					resource_type: "Encounter",
					id: "1",
					version_id: Some("1")
				}
			);

			let reference = Reference::builder()
				.r#type("Encounter".to_owned())
				.reference("#1".to_owned())
				.build()
				.unwrap();
			let parsed = reference.parse().expect("parsing reference");
			assert_eq!(parsed, ParsedReference::Local { id: "1" });

			let reference = Reference::builder()
				.r#type("Task".to_owned())
				.reference("http://not-fhir.test/1".to_owned())
				.build()
				.unwrap();
			let parsed = reference.parse().expect("parsing reference");
			assert_eq!(
				parsed,
				ParsedReference::Absolute {
					url: "http://not-fhir.test/1",
					resource_type: Some("not-fhir.test"), // irks
					id: Some("1")
				}
			);
		}
	};
}

/// Test codeable concept usage.
macro_rules! codeable_concept {
	($version:ident) => {
		#[test]
		fn codeable_concept() {
			let concept = CodeableConcept::builder()
				.coding(vec![
					Some(
						Coding::builder()
							.system("system1".to_owned())
							.code("code1".to_owned())
							.build()
							.unwrap(),
					),
					Some(
						Coding::builder()
							.system("system2".to_owned())
							.code("code2".to_owned())
							.build()
							.unwrap(),
					),
					Some(
						Coding::builder()
							.system("system3".to_owned())
							.code("code3".to_owned())
							.build()
							.unwrap(),
					),
					Some(
						Coding::builder()
							.system("system1".to_owned())
							.code("code4".to_owned())
							.build()
							.unwrap(),
					),
				])
				.build()
				.unwrap();

			let mut codes1 = concept.codes_with_system("system1");
			assert_eq!(codes1.next(), Some("code1"));
			assert_eq!(codes1.next(), Some("code4"));
			assert_eq!(codes1.next(), None);
			let code3 = concept.code_with_system("system3");
			assert_eq!(code3, Some("code3"));
		}
	};
}

/// Example Bundle resource (search result) compatible with STU3, R4B and R5
/// (due to hacky duplicate information / additional keys).
const BUNDLE_SEARCH_RESULT_EXAMPLE: &str = r#"
	{
		"resourceType" : "Bundle",
		"id" : "bundle-example",
		"meta" : {
			"lastUpdated" : "2014-08-18T01:43:30Z"
		},
		"type" : "searchset",
		"total" : 3,
		"link" : [{
			"relation" : "self",
			"url" : "https://example.com/base/MedicationRequest?patient=347&_include=MedicationRequest.medication&_count=2"
		},
		{
			"relation" : "next",
			"url" : "https://example.com/base/MedicationRequest?patient=347&searchId=ff15fd40-ff71-4b48-b366-09c706bed9d0&page=2"
		}],
		"entry" : [{
			"fullUrl" : "https://example.com/base/MedicationRequest/3123",
			"resource" : {
				"resourceType" : "MedicationRequest",
				"id" : "3123",
				"status" : "unknown",
				"intent" : "order",
				"medication" : {
					"reference" : {
						"reference" : "Medication/example"
					}
				},
				"medicationReference": {
					"reference": "Medication/example"
				},
				"subject" : {
					"reference" : "Patient/347"
				}
			},
			"search" : {
				"mode" : "match",
				"score" : 1
			}
		},
		{
			"fullUrl" : "https://example.com/base/Medication/example",
			"resource" : {
				"resourceType" : "Medication",
				"id" : "example"
			},
			"search" : {
				"mode" : "include"
			}
		}]
	}
	"#;

/// Test for bundle functions.
macro_rules! bundle_fns {
	($version:ident) => {
		fn search_result() -> Bundle {
			serde_json::from_str(BUNDLE_SEARCH_RESULT_EXAMPLE).unwrap()
		}

		#[test]
		fn bundle_fns() {
			let bundle = search_result();

			let next_url = bundle.next_page_url().unwrap();
			assert_eq!(*next_url, "https://example.com/base/MedicationRequest?patient=347&searchId=ff15fd40-ff71-4b48-b366-09c706bed9d0&page=2");
		}
	};
}

/// Resource tests.
macro_rules! resource_tests {
	($version:ident) => {
		mod $version {
			use fhir_model::$version::{codes::*, resources::*, types::*};

			use super::*;

			serialization_deserialization!($version);
			builder_works!($version);
			resource_conversion!($version);
			coding_concepts!($version);
			resource_traits!($version);
			identifiable_resource!($version);
			identifier_search!($version);
			reference_parsing!($version);
			codeable_concept!($version);
			bundle_fns!($version);
		}
	};
}

for_all_versions!(resource_tests);
