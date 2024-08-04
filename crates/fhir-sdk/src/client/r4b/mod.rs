//! FHIR R4B client implementation.

mod patch;
mod transaction;

use fhir_model::r4b::resources::{
	Bundle, Parameters, ParametersParameter, ParametersParameterValue, Patient, Resource,
	ResourceType,
};
use reqwest::header;

use self::{
	patch::{PatchViaFhir, PatchViaJson},
	transaction::BatchTransaction,
};
use super::{Client, Error, FhirR4B};
use crate::version::FhirVersion;

impl Client<FhirR4B> {
	/// Begin building a patch request for a FHIR resource on the server via the
	/// `FHIRPath Patch` method.
	pub fn patch_via_fhir<'a>(&self, resource_type: ResourceType, id: &'a str) -> PatchViaFhir<'a> {
		PatchViaFhir::new(self.clone(), resource_type, id)
	}

	/// Begin building a patch request for a FHIR resource on the server via the
	/// [`JSON Patch`](https://datatracker.ietf.org/doc/html/rfc6902) method.
	pub fn patch_via_json<'a>(&self, resource_type: ResourceType, id: &'a str) -> PatchViaJson<'a> {
		PatchViaJson::new(self.clone(), resource_type, id)
	}

	/// Start building a new batch request.
	pub fn batch(&self) -> BatchTransaction {
		BatchTransaction::new(self.clone(), false)
	}

	/// Start building a new transaction request.
	pub fn transaction(&self) -> BatchTransaction {
		BatchTransaction::new(self.clone(), true)
	}

	/// Operation `$everything` on `Encounter`, returning a Bundle with all
	/// resources for an `Encounter` record.
	pub async fn operation_encounter_everything(&self, id: &str) -> Result<Bundle, Error> {
		let url = self.url(&["Encounter", id, "$everything"]);
		let request = self.0.client.get(url).header(header::ACCEPT, FhirR4B::MIME_TYPE);

		let response = self.run_request(request).await?;
		if response.status().is_success() {
			let resource: Bundle = response.json().await?;
			Ok(resource)
		} else {
			Err(Error::from_response::<FhirR4B>(response).await)
		}
	}

	/// Operation `$everything` on `Patient`, returning a Bundle with all
	/// resources for an `Patient` record.
	pub async fn operation_patient_everything(&self, id: &str) -> Result<Bundle, Error> {
		let url = self.url(&["Patient", id, "$everything"]);
		let request = self.0.client.get(url).header(header::ACCEPT, FhirR4B::MIME_TYPE);

		let response = self.run_request(request).await?;
		if response.status().is_success() {
			let resource: Bundle = response.json().await?;
			Ok(resource)
		} else {
			Err(Error::from_response::<FhirR4B>(response).await)
		}
	}

	/// Operation `$match` on `Patient`, returning matches for Patient records
	/// based on a given incomplete Patient resource.
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
			.header(header::ACCEPT, FhirR4B::MIME_TYPE)
			.header(header::CONTENT_TYPE, FhirR4B::MIME_TYPE)
			.json(&parameters);

		let response = self.run_request(request).await?;
		if response.status().is_success() {
			let resource: Bundle = response.json().await?;
			Ok(resource)
		} else {
			Err(Error::from_response::<FhirR4B>(response).await)
		}
	}
}
