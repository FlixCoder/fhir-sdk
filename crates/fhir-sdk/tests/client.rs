#![cfg(feature = "client")]
#![allow(clippy::expect_used)]

use std::env;

use eyre::Result;
use fhir_sdk::{
	client::{Client, ResourceWrite},
	r5::resources::{Patient, Resource},
};

fn client() -> Result<Client> {
	dotenvy::dotenv()?;
	let base_url = env::var("BASE_URL").expect("BASE_URL missing").parse()?;
	Ok(Client::new(base_url)?)
}

#[tokio::test]
async fn crud() -> Result<()> {
	let client = client()?;

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
async fn search_all_raw() -> Result<()> {
	let client = client()?;

	let mut patient = Patient::builder().active(false).build();
	let id = patient.create(&client).await?;

	let bundle = client.search_all_raw(&[("_type", "Patient"), ("_id", &id)]).await?;
	let patient = bundle
		.0
		.entry
		.into_iter()
		.flatten()
		.filter_map(|entry| entry.resource)
		.find_map(|resource| match resource {
			Resource::Patient(patient) => Some(patient),
			_ => None,
		})
		.expect("There should be a patient found");
	assert_eq!(patient.active, Some(false));

	patient.delete(&client).await?;
	Ok(())
}

#[tokio::test]
async fn search_raw() -> Result<()> {
	let client = client()?;

	let date = "5123-05-01";

	let mut patient = Patient::builder().active(false).birth_date(date.to_owned()).build();
	let id = patient.create(&client).await?;

	let patients = client.search_raw::<Patient>(&[("_id", &id)]).await?;
	assert_eq!(patients.len(), 1);
	assert_eq!(patients[0].active, Some(false));
	assert_eq!(patients[0].birth_date.as_deref(), Some(date));

	patient.delete(&client).await?;
	Ok(())
}
