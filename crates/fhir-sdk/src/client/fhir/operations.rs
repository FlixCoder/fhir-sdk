//! Implementation of certain FHIR operations.

use fhir_model::for_all_versions;
use reqwest::header;

use super::{Client, Error};
use crate::version::{FhirVersion, fhir_version};

/// Implement the operation "Encounter/<id>/$everything" for the appropriate
/// versions.
macro_rules! impl_operation_encounter_everything {
	($version:ident) => {
		mod $version {
			use fhir_model::$version::resources::Bundle;

			use super::*;

			/// Selected FHIR version.
			type Version = fhir_version!($version);

			impl Client<Version> {
				/// Operation `$everything` on `Encounter`, returning a Bundle with all
				/// resources for an `Encounter` record.
				pub async fn operation_encounter_everything(
					&self,
					id: &str,
				) -> Result<Bundle, Error> {
					let url = self.url(&["Encounter", id, "$everything"]);
					let request = self.0.client.get(url).header(header::ACCEPT, Version::MIME_TYPE);

					let response = self.run_request(request).await?;
					if response.status().is_success() {
						let resource: Bundle = response.json().await?;
						Ok(resource)
					} else {
						Err(Error::from_response::<Version>(response).await)
					}
				}
			}
		}
	};
}
mod operation_1 {
	//! Module for avoidance of conflicts.
	use super::*;
	for_all_versions!(impl_operation_encounter_everything);
}

/// Implement the operation "Patient/<id>/$everything" for the appropriate
/// versions.
macro_rules! impl_operation_patient_everything {
	($version:ident) => {
		mod $version {
			use fhir_model::$version::resources::Bundle;

			use super::*;

			/// Selected FHIR version.
			type Version = fhir_version!($version);

			impl Client<Version> {
				/// Operation `$everything` on `Patient`, returning a Bundle with all
				/// resources for an `Patient` record.
				pub async fn operation_patient_everything(
					&self,
					id: &str,
				) -> Result<Bundle, Error> {
					let url = self.url(&["Patient", id, "$everything"]);
					let request = self.0.client.get(url).header(header::ACCEPT, Version::MIME_TYPE);

					let response = self.run_request(request).await?;
					if response.status().is_success() {
						let resource: Bundle = response.json().await?;
						Ok(resource)
					} else {
						Err(Error::from_response::<Version>(response).await)
					}
				}
			}
		}
	};
}
mod operation_2 {
	//! Module for avoidance of conflicts.
	use super::*;
	for_all_versions!(impl_operation_patient_everything);
}

/// Implement the operation "Patient/$match" for the appropriate versions.
macro_rules! impl_operation_patient_match {
	($version:ident) => {
		mod $version {
			use fhir_model::$version::resources::{
				Bundle, Parameters, ParametersParameter, ParametersParameterValue, Patient,
				Resource,
			};

			use super::*;

			/// Selected FHIR version.
			type Version = fhir_version!($version);

			impl Client<Version> {
				/// Operation `$match` on `Patient`, returning matches for Patient
				/// records based on a given incomplete Patient resource.
				pub async fn operation_patient_match(
					&self,
					patient: Patient,
					only_certain: bool,
					count: i32,
				) -> Result<Bundle, Error> {
					#[allow(clippy::unwrap_used)] // Will always succeed.
					let parameters = Parameters::builder()
						.parameter(vec![
							Some(
								ParametersParameter::builder()
									.name("resource".to_owned())
									.resource(Resource::from(patient))
									.build()
									.unwrap(),
							),
							Some(
								ParametersParameter::builder()
									.name("onlyCertainMatches".to_owned())
									.value(ParametersParameterValue::Boolean(only_certain))
									.build()
									.unwrap(),
							),
							Some(
								ParametersParameter::builder()
									.name("count".to_owned())
									.value(ParametersParameterValue::Integer(count))
									.build()
									.unwrap(),
							),
						])
						.build()
						.unwrap();

					let url = self.url(&["Patient", "$match"]);
					let request = self
						.0
						.client
						.post(url)
						.header(header::ACCEPT, Version::MIME_TYPE)
						.header(header::CONTENT_TYPE, Version::MIME_TYPE)
						.json(&parameters);

					let response = self.run_request(request).await?;
					if response.status().is_success() {
						let resource: Bundle = response.json().await?;
						Ok(resource)
					} else {
						Err(Error::from_response::<Version>(response).await)
					}
				}
			}
		}
	};
}
mod operation_3 {
	//! Module for avoidance of conflicts.
	use super::*;
	for_all_versions!(impl_operation_patient_match);
}

/// Implement the operation "Subscription/<id>/$status" for the appropriate
/// versions.
macro_rules! impl_operation_subscription_status {
	// These versions do not have that operation.
	(stu3) => {};
	(r4b) => {};
	// Implement it for all others.
	($version:ident) => {
		mod $version {
			use fhir_model::$version::resources::{Bundle, SubscriptionStatus};

			use super::*;

			/// Selected FHIR version.
			type Version = fhir_version!($version);

			impl Client<Version> {
				/// Operation `$status` on `Subscription`, returning the
				/// `SubcriptionStatus`.
				pub async fn operation_subscription_status(
					&self,
					id: &str,
				) -> Result<SubscriptionStatus, Error> {
					let url = self.url(&["Subscription", id, "$status"]);
					let request =
						self.0.client.get(url.clone()).header(header::ACCEPT, Version::MIME_TYPE);

					let response = self.run_request(request).await?;
					if response.status().is_success() {
						let bundle: Bundle = response.json().await?;
						let resource = bundle
							.0
							.entry
							.into_iter()
							.flatten()
							.filter_map(|entry| entry.resource)
							.find_map(|res| SubscriptionStatus::try_from(res).ok())
							.ok_or_else(|| Error::ResourceNotFound(url.to_string()))?;
						Ok(resource)
					} else {
						Err(Error::from_response::<Version>(response).await)
					}
				}
			}
		}
	};
}
mod operation_4 {
	//! Module for avoidance of conflicts.
	use super::*;
	for_all_versions!(impl_operation_subscription_status);
}

/// Implement the operation "Subscription/<id>/$events" for the appropriate
/// versions.
macro_rules! impl_operation_subscription_events {
	// These versions do not have that operation.
	(stu3) => {};
	(r4b) => {};
	// Implement it for all others.
	($version:ident) => {
		mod $version {
			use fhir_model::$version::{codes::SubscriptionPayloadContent, resources::Bundle};

			use super::*;

			/// Selected FHIR version.
			type Version = fhir_version!($version);

			impl Client<Version> {
				/// Operation `$events` on `Subscription`, returning the previous
				/// notifications that were triggered by a topic.
				pub async fn operation_subscription_events(
					&self,
					id: &str,
					events_since: Option<i64>,
					events_until: Option<i64>,
					content: Option<SubscriptionPayloadContent>,
				) -> Result<Bundle, Error> {
					let mut queries = Vec::new();
					if let Some(events_since) = events_since {
						queries.push(("eventsSinceNumber", events_since.to_string()));
					}
					if let Some(events_until) = events_until {
						queries.push(("eventsUntilNumber", events_until.to_string()));
					}
					if let Some(content) = content {
						queries.push(("content", content.to_string()));
					}

					let url = self.url(&["Subscription", id, "$events"]);
					let request = self
						.0
						.client
						.get(url)
						.query(&queries)
						.header(header::ACCEPT, Version::MIME_TYPE);

					let response = self.run_request(request).await?;
					if response.status().is_success() {
						let bundle: Bundle = response.json().await?;
						Ok(bundle)
					} else {
						Err(Error::from_response::<Version>(response).await)
					}
				}
			}
		}
	};
}
mod operation_5 {
	//! Module for avoidance of conflicts.
	use super::*;
	for_all_versions!(impl_operation_subscription_events);
}
