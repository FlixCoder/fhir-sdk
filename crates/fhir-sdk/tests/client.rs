#![cfg(all(feature = "r5", feature = "builders", feature = "client"))]
#![allow(clippy::expect_used, clippy::print_stdout)]

use std::env;

use eyre::Result;
use fhir_sdk::{
	client::{Client, DateSearch, ResourceWrite, SearchParameters, TokenSearch},
	r5::{
		codes::SearchComparator,
		resources::{Patient, ResourceType},
	},
};
use futures::TryStreamExt;

fn client() -> Result<Client> {
	let base_url =
		env::var("FHIR_SERVER").unwrap_or("http://localhost:8090/fhir/".to_owned()).parse()?;
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
async fn search() -> Result<()> {
	let client = client()?;

	let date = "5123-05-05";

	let mut patient = Patient::builder().active(false).birth_date(date.to_owned()).build();
	let id = patient.create(&client).await?;

	let patients: Vec<Patient> = client
		.search(
			SearchParameters::empty()
				.and_raw("_id", id)
				.and(DateSearch {
					name: "birthdate",
					comparator: Some(SearchComparator::Eq),
					value: date,
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
	assert_eq!(patients[0].birth_date.as_deref(), Some(date));

	patient.delete(&client).await?;
	Ok(())
}

#[tokio::test]
async fn paging() -> Result<()> {
	let client = client()?;

	let date = "5123-05-10";
	let n = 99;

	println!("Preparing..");
	let mut ids = Vec::new();
	// TODO: Use batch/transaction instead.
	for _ in 0..n {
		let mut patient = Patient::builder().active(false).birth_date(date.to_owned()).build();
		let id = patient.create(&client).await?;
		ids.push(id);
	}

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
	// TODO: Use batch/transaction instead.
	for id in ids {
		client.delete(ResourceType::Patient, &id).await?;
	}
	Ok(())
}
