#![cfg(all(feature = "r4b", feature = "builders", feature = "client"))]
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::print_stdout, clippy::indexing_slicing)]

mod common;

use std::str::FromStr;

use anyhow::Result;
use fhir_sdk::{
	Date,
	client::{Client, DateSearch, ResourceWrite, SearchParameters, TokenSearch},
	r4b::{
		codes::{
			AdministrativeGender, BundleType, EncounterStatus, IssueSeverity, SearchComparator,
		},
		resources::{
			BaseResource, Bundle, Encounter, OperationOutcome, Patient, Resource, ResourceType,
		},
		types::{Coding, HumanName, Reference},
	},
	version::FhirR4B,
};
use futures::TryStreamExt;
use reqwest::header::HeaderValue;
use serde_json::json;
use tokio::sync::OnceCell;

fn extract_json_field(json_body: serde_json::Value, field: &str) -> Option<String> {
	let code = json_body.as_object()?.get(field)?.as_str()?;
	Some(code.to_owned())
}

async fn medplum_auth(client: reqwest::Client) -> Result<HeaderValue> {
	println!("Getting new authorization token from Medplum");

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
		.ok_or_else(|| anyhow::format_err!("No code in login response"))?;

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
		.ok_or_else(|| anyhow::format_err!("No access_token in login response"))?;

	Ok(format!("Bearer {access_token}").parse()?)
}

/// Set up a client for testing with the (local) FHIR server.
async fn client() -> Result<Client<FhirR4B>> {
	static CLIENT: OnceCell<Client<FhirR4B>> = OnceCell::const_new();
	common::setup_logging().await;
	let client = CLIENT
		.get_or_try_init(|| async move {
			let client = Client::builder()
				.base_url(
					std::env::var("FHIR_SERVER")
						.unwrap_or("http://localhost:8080/fhir/R4".to_owned())
						.parse()?,
				)
				.auth_callback(medplum_auth)
				.build()?;
			anyhow::Ok(client)
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

#[test]
fn crud() -> Result<()> {
	common::RUNTIME.block_on(crud_inner())
}

async fn crud_inner() -> Result<()> {
	let client = client().await?;

	let mut patient = Patient::builder().active(false).build().unwrap();
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

#[test]
fn read_referenced() -> Result<()> {
	common::RUNTIME.block_on(read_referenced_inner())
}

async fn read_referenced_inner() -> Result<()> {
	let client = client().await?;

	let mut patient = Patient::builder().build().unwrap();
	patient.create(&client).await?;

	let reference = Reference::relative_to(&patient).expect("creating reference");
	let read = client.read_referenced(&reference).await?;
	assert_eq!(read.as_base_resource().id(), patient.id());

	Ok(())
}

#[test]
fn patch_via_json() -> Result<()> {
	common::RUNTIME.block_on(patch_via_json_inner())
}

async fn patch_via_json_inner() -> Result<()> {
	let client = client().await?;

	let mut patient = Patient::builder()
		.active(false)
		.gender(AdministrativeGender::Male)
		.name(vec![Some(HumanName::builder().family("Test".to_owned()).build().unwrap())])
		.build()
		.unwrap();
	patient.create(&client).await?;

	let date = Date::from_str("2021-02-01").expect("parse Date");
	client
		.patch_via_json(ResourceType::Patient, patient.id.as_ref().expect("Patient.id"))
		.add("/birthDate", &date)?
		.remove("/active")
		.replace("/gender", AdministrativeGender::Female)?
		.add("/name/0", HumanName::builder().family("Family".to_owned()).build().unwrap())?
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

#[test]
fn search() -> Result<()> {
	common::RUNTIME.block_on(search_inner())
}

async fn search_inner() -> Result<()> {
	let client = client().await?;

	let date_str = "5123-05-05";
	let date = Date::from_str(date_str).expect("parse Date");

	let mut patient = Patient::builder().active(false).birth_date(date.clone()).build().unwrap();
	let id = patient.create(&client).await?;

	let patients: Vec<Patient> = client
		.search(
			SearchParameters::empty()
				.and_raw("_id", id)
				.and(DateSearch::<FhirR4B> {
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
		.await?
		.all_matches()
		.try_collect()
		.await?;
	assert_eq!(patients.len(), 1);
	assert_eq!(patients[0].active, Some(false));
	assert_eq!(patients[0].birth_date, Some(date));

	patient.delete(&client).await?;
	Ok(())
}

#[test]
fn transaction() -> Result<()> {
	common::RUNTIME.block_on(transaction_inner())
}

async fn transaction_inner() -> Result<()> {
	let client = client().await?;

	let mut patient1 = Patient::builder().build().unwrap();
	patient1.create(&client).await?;
	let mut patient2 = Patient::builder().build().unwrap();
	patient2.create(&client).await?;
	let mut patient3 = Patient::builder().build().unwrap();
	patient3.create(&client).await?;

	let mut transaction = client.transaction();
	transaction.delete(ResourceType::Patient, patient1.id.as_ref().expect("Patient.id"));
	transaction.read(ResourceType::Patient, patient1.id.as_ref().expect("Patient.id"));
	transaction.update(patient3, true)?;
	let patient_ref = transaction.create(Patient::builder().build().unwrap());
	let _encounter_ref = transaction.create(
		Encounter::builder()
			.status(EncounterStatus::Planned)
			.class(
				Coding::builder()
					.system("test-system".to_owned())
					.code("test-code".to_owned())
					.build()
					.unwrap(),
			)
			.subject(Reference::builder().reference(patient_ref.clone()).build().unwrap())
			.build()
			.unwrap(),
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
		.read_referenced(&Reference::builder().reference(encounter_ref.clone()).build().unwrap())
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

#[test]
fn paging() -> Result<()> {
	common::RUNTIME.block_on(paging_inner())
}

async fn paging_inner() -> Result<()> {
	let client = client().await?;

	let date = "5123-05-10";
	let n = 99;

	println!("Preparing..");
	let patient = Patient::builder()
		.active(false)
		.birth_date(Date::from_str(date).expect("parse Date"))
		.build()
		.unwrap();
	let mut batch = client.batch();
	for _ in 0 .. n {
		batch.create(patient.clone());
	}
	ensure_batch_succeeded(batch.send().await?);

	println!("Starting search..");
	let patients: Vec<Patient> = client
		.search(SearchParameters::empty().and(DateSearch::<FhirR4B> {
			name: "birthdate",
			comparator: Some(SearchComparator::Eq),
			value: date,
		}))
		.await?
		.all_matches()
		.try_collect()
		.await?;
	let patients_len = patients.len();

	println!("Cleaning up..");
	let mut batch = client.batch();
	for patient in patients {
		batch.delete(ResourceType::Patient, patient.id.as_ref().expect("Patient.id"));
	}
	ensure_batch_succeeded(batch.send().await?);

	assert_eq!(patients_len, n);
	Ok(())
}

#[test]
fn history_with_id() -> Result<()> {
	common::RUNTIME.block_on(history_with_id_inner())
}

async fn history_with_id_inner() -> Result<()> {
	let client = client().await?;

	let mut patient = Patient::builder().language("DE".to_owned()).build().unwrap();
	patient.create(&client).await?;

	patient.language = Some("EN".to_owned());
	patient.update(false, &client).await?;

	patient.clone().delete(&client).await?;

	let bundle = client.history::<Patient>(patient.id.as_deref()).await?.into_inner();
	assert_eq!(bundle.r#type, BundleType::History);
	assert_eq!(bundle.entry.len(), 3);

	let Some(_response) = bundle.entry[0].as_ref().unwrap().request.as_ref() else {
		panic!("response should be BundleEntryRequest");
	};

	let Some(Resource::Patient(last_version)) = bundle.entry[1].as_ref().unwrap().resource.as_ref()
	else {
		panic!("Resource should be Patient");
	};
	assert_eq!(
		last_version.language,
		Some("EN".to_owned()),
		"Last version should have language 'EN'"
	);
	let Some(Resource::Patient(first_version)) =
		bundle.entry[2].as_ref().unwrap().resource.as_ref()
	else {
		panic!("Resource should be Patient");
	};
	assert_eq!(
		first_version.language,
		Some("DE".to_owned()),
		"Last version should have language 'DE'"
	);

	Ok(())
}
