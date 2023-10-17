#![cfg(all(feature = "r4b", feature = "builders", feature = "client"))]
#![allow(clippy::expect_used, clippy::print_stdout)]

use std::str::FromStr;

use eyre::Result;
use fhir_sdk::{
	client::{Client, DateSearch, ResourceWrite, SearchParameters, TokenSearch},
	r4b::{
		codes::{AdministrativeGender, EncounterStatus, IssueSeverity, SearchComparator},
		reference_to,
		resources::{
			BaseResource, Bundle, Encounter, OperationOutcome, ParametersParameter,
			ParametersParameterValue, Patient, Resource, ResourceType,
		},
		types::{Coding, HumanName, Identifier, Reference},
	},
	Date,
};
use futures::TryStreamExt;
use serde_json::json;
use tokio::sync::OnceCell;

fn extract_json_field(json_body: serde_json::Value, field: &str) -> Option<String> {
	let code = json_body.as_object()?.get(field)?.as_str()?;
	Some(code.to_owned())
}

/// Set up a client for testing with the (local) FHIR server.
async fn client() -> Result<Client> {
	static CLIENT: OnceCell<Client> = OnceCell::const_new();
	let client = CLIENT
		.get_or_try_init(|| async move {
			let client = reqwest::Client::builder().user_agent("fhir-sdk tests").build()?;
			let my_challenge = "my_challenge";

			let auth_url = "http://localhost:8080/auth/login";
			let response = client
				.post(auth_url)
				.json(&json!({
					"email": "admin@example.com",
					"password": "medplum_admin",
					"codeChallengeMethod": "plain",
					"codeChallenge": my_challenge
				}))
				.send()
				.await?
				.error_for_status()?;
			let login_code = extract_json_field(response.json().await?, "code")
				.ok_or_else(|| eyre::eyre!("No code in login response"))?;

			let token_url = "http://localhost:8080/oauth2/token";
			let response = client
				.post(token_url)
				.form(&[
					("grant_type", "authorization_code"),
					("code_verifier", my_challenge),
					("code", &login_code),
				])
				.send()
				.await?
				.error_for_status()?;
			let access_token = extract_json_field(response.json().await?, "access_token")
				.ok_or_else(|| eyre::eyre!("No access_token in login response"))?;

			let base_url = "http://localhost:8080/fhir/R4".parse()?;
			let client = Client::new(base_url)?;
			client.set_request_settings(
				client
					.request_settings()
					.header("Authorization".parse()?, format!("Bearer {access_token}").parse()?),
			);

			Ok::<_, eyre::Report>(client)
		})
		.await?;
	Ok(client.clone())
}

/// Go through all entries of the bundle, extracting the outcomes and search for
/// errors inside. Fail if there is any of severity error or fatal.
fn ensure_batch_succeeded(bundle: Bundle) {
	let batch_errors = bundle
		.entry
		.iter()
		.flatten()
		.filter_map(|entry| entry.response.as_ref())
		.filter_map(|response| response.outcome.as_ref())
		.filter_map(|resource| <&OperationOutcome>::try_from(resource).ok())
		.flat_map(|outcome| outcome.issue.iter().flatten())
		.any(|issue| matches!(issue.severity, IssueSeverity::Error | IssueSeverity::Fatal));
	assert!(!batch_errors);
}

#[tokio::test]
#[ignore = "This can only be executed with the Medplum R4 server"]
async fn crud() -> Result<()> {
	let client = client().await?;

	let mut patient = Patient::builder().active(false).build();
	let id = patient.create(&client).await?;
	let resource = client.read::<Patient>(&id).await?.expect("should find resource");
	assert_eq!(resource.active, patient.active);

	patient.active = Some(true);
	patient.update(false, &client).await?;
	patient.active = None;
	patient.update(true, &client).await?;
	let version_id =
		patient.meta.as_ref().and_then(|meta| meta.version_id.as_ref()).expect("get version ID");
	let resource =
		client.read_version::<Patient>(&id, version_id).await?.expect("should find resource");
	assert_eq!(resource.active, patient.active);

	patient.delete(&client).await?;
	let resource = client.read::<Patient>(&id).await?;
	assert_eq!(resource, None);

	Ok(())
}

#[tokio::test]
#[ignore = "This can only be executed with the Medplum R4 server"]
async fn read_referenced() -> Result<()> {
	let client = client().await?;

	let mut patient = Patient::builder().build();
	patient.create(&client).await?;

	let reference = reference_to(&patient).expect("creating reference");
	let read = client.read_referenced(&reference).await?;
	assert_eq!(read.as_base_resource().id(), patient.id());

	Ok(())
}

#[tokio::test]
#[ignore = "This can only be executed with the Medplum R4 server"]
async fn patch() -> Result<()> {
	let client = client().await?;

	let mut patient = Patient::builder()
		.active(false)
		.gender(AdministrativeGender::Male)
		.name(vec![Some(HumanName::builder().family("Test".to_owned()).build())])
		.build();
	patient.create(&client).await?;

	let date = Date::from_str("2021-02-01").expect("parse Date");
	client
		.patch(ResourceType::Patient, patient.id.as_ref().expect("Patient.id"))
		.add(
			"Patient",
			"birthDate",
			ParametersParameter::builder()
				.name("value".to_owned())
				.value(ParametersParameterValue::Date(date.clone()))
				.build(),
		)
		.delete("Patient.active")
		.replace(
			"Patient.gender",
			ParametersParameter::builder()
				.name("value".to_owned())
				.value(ParametersParameterValue::Code("female".to_owned()))
				.build(),
		)
		.insert(
			"Patient.name",
			ParametersParameter::builder()
				.name("value".to_owned())
				.value(ParametersParameterValue::HumanName(
					HumanName::builder().family("Family".to_owned()).build(),
				))
				.build(),
			0,
		)
		.send()
		.await?;

	let patient: Patient =
		client.read(patient.id.as_ref().expect("Patient.id")).await?.expect("Patient should exist");
	assert_eq!(patient.birth_date, Some(date));
	assert_eq!(patient.active, None);
	assert_eq!(patient.gender, Some(AdministrativeGender::Female));
	assert_eq!(patient.name.len(), 2);

	Ok(())
}

#[tokio::test]
#[ignore = "This can only be executed with the Medplum R4 server"]
async fn search() -> Result<()> {
	let client = client().await?;

	let date_str = "5123-05-05";
	let date = Date::from_str(date_str).expect("parse Date");

	let mut patient = Patient::builder().active(false).birth_date(date.clone()).build();
	let id = patient.create(&client).await?;

	let patients: Vec<Patient> = client
		.search(
			SearchParameters::empty()
				.and_raw("_id", id)
				.and(DateSearch {
					name: "birthdate",
					comparator: Some(SearchComparator::Eq),
					value: date_str,
				})
				.and(TokenSearch::Standard {
					name: "active",
					system: None,
					code: Some("false"),
					not: false,
				}),
		)
		.try_collect()
		.await?;
	assert_eq!(patients.len(), 1);
	assert_eq!(patients[0].active, Some(false));
	assert_eq!(patients[0].birth_date, Some(date));

	patient.delete(&client).await?;
	Ok(())
}

#[tokio::test]
#[ignore = "This can only be executed with the Medplum R4 server"]
async fn transaction() -> Result<()> {
	let client = client().await?;

	let mut patient1 = Patient::builder().build();
	patient1.create(&client).await?;
	let mut patient2 = Patient::builder().build();
	patient2.create(&client).await?;
	let mut patient3 = Patient::builder().build();
	patient3.create(&client).await?;

	let mut transaction = client.transaction();
	transaction.delete(ResourceType::Patient, patient1.id.as_ref().expect("Patient.id"));
	transaction.read(ResourceType::Patient, patient1.id.as_ref().expect("Patient.id"));
	transaction.update(patient3, true)?;
	let patient_ref = transaction.create(Patient::builder().build());
	let _encounter_ref = transaction.create(
		Encounter::builder()
			.status(EncounterStatus::Planned)
			.class(
				Coding::builder()
					.system("test-system".to_owned())
					.code("test-code".to_owned())
					.build(),
			)
			.subject(Reference::builder().reference(patient_ref.clone()).build())
			.build(),
	);

	let mut entries = transaction.send().await?.0.entry.into_iter().flatten();
	let _delete = entries.next().expect("DELETE response");
	let _read = entries.next().expect("GET response");
	let _update = entries.next().expect("PUT response");
	let _create_patient = entries.next().expect("POST Patient response");
	let create_encounter = entries.next().expect("POST Encounter response");
	assert!(entries.next().is_none());

	let encounter_ref = create_encounter
		.full_url
		.as_ref()
		.or(create_encounter.response.as_ref().and_then(|response| response.location.as_ref()))
		.expect("Encounter ID in response");
	let Resource::Encounter(encounter) = client
		.read_referenced(&Reference::builder().reference(encounter_ref.clone()).build())
		.await?
	else {
		panic!("Resource should be Encounter");
	};
	let subject_ref = encounter
		.subject
		.as_ref()
		.expect("Encounter.subject")
		.reference
		.as_ref()
		.expect("Encounter.subject.reference");
	println!("Subject reference is: {subject_ref}");
	assert_ne!(subject_ref, &patient_ref);

	Ok(())
}

#[tokio::test]
#[ignore = "This can only be executed with the Medplum R4 server"]
async fn paging() -> Result<()> {
	let client = client().await?;

	let date = "5123-05-10";
	let n = 99;

	println!("Preparing..");
	let patient = Patient::builder()
		.active(false)
		.birth_date(Date::from_str(date).expect("parse Date"))
		.build();
	let mut batch = client.batch();
	for _ in 0..n {
		batch.create(patient.clone());
	}
	ensure_batch_succeeded(batch.send().await?);

	println!("Starting search..");
	let patients: Vec<Patient> = client
		.search(SearchParameters::empty().and(DateSearch {
			name: "birthdate",
			comparator: Some(SearchComparator::Eq),
			value: date,
		}))
		.try_collect()
		.await?;
	assert_eq!(patients.len(), n);

	println!("Cleaning up..");
	let mut batch = client.batch();
	for patient in patients {
		batch.delete(ResourceType::Patient, patient.id.as_ref().expect("Patient.id"));
	}
	ensure_batch_succeeded(batch.send().await?);
	Ok(())
}

#[tokio::test]
#[ignore = "This can only be executed with the Medplum R4 server"]
async fn operation_encounter_everything() -> Result<()> {
	let client = client().await?;

	let mut patient = Patient::builder().build();
	patient.create(&client).await?;
	let mut encounter = Encounter::builder()
		.status(EncounterStatus::InProgress)
		.class(
			Coding::builder().system("test-system".to_owned()).code("test-code".to_owned()).build(),
		)
		.subject(reference_to(&patient).expect("Patient reference"))
		.build();
	encounter.create(&client).await?;

	let bundle =
		client.operation_encounter_everything(encounter.id.as_ref().expect("Encounter.id")).await?;
	let contains_patient = bundle
		.entry
		.iter()
		.flatten()
		.filter_map(|entry| entry.resource.as_ref())
		.filter_map(|resource| resource.as_base_resource().id().as_ref())
		.any(|id| Some(id) == patient.id.as_ref());
	assert!(contains_patient);

	Ok(())
}

#[tokio::test]
#[ignore = "This can only be executed with the Medplum R4 server"]
async fn operation_patient_match() -> Result<()> {
	let client = client().await?;

	let mut patient = Patient::builder()
		.identifier(vec![Some(Identifier::builder().value("Test".to_owned()).build())])
		.build();
	patient.create(&client).await?;

	let bundle = client.operation_patient_match(patient.clone(), true, 1).await?;
	let contains_patient = bundle
		.entry
		.iter()
		.flatten()
		.filter_map(|entry| entry.resource.as_ref())
		.filter_map(|resource| resource.as_base_resource().id().as_ref())
		.any(|id| Some(id) == patient.id.as_ref());
	assert!(contains_patient);

	Ok(())
}
