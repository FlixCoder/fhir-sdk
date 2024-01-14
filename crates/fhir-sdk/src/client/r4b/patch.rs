//! Patch request building.

use fhir_model::r4b::resources::{
	Parameters, ParametersParameter, ParametersParameterValue, ResourceType,
};
use reqwest::header::{self, HeaderValue};
use serde::Serialize;

use super::{Client, Error, FhirR4B, MIME_TYPE};

/// Builder for a PATCH request via FHIRPath for a FHIR resource.
#[derive(Debug, Clone)]
#[must_use = "You probably want to send the PATCH request"]
pub struct PatchViaFhir<'a> {
	/// FHIR client.
	client: Client<FhirR4B>,
	/// Resource type to apply the patch to.
	resource_type: ResourceType,
	/// Resource ID to apply the path to.
	id: &'a str,
	/// Operations to apply.
	operations: Vec<Option<ParametersParameter>>,
}

impl<'a> PatchViaFhir<'a> {
	/// Start building a new Patch request.
	pub fn new(client: Client<FhirR4B>, resource_type: ResourceType, id: &'a str) -> Self {
		Self { client, resource_type, id, operations: Vec::new() }
	}

	/// Add an `add` operation to the list of operations. Note that the `path`
	/// and `name` need to be set according the FHIR defititions, e.g. path
	/// `Patient` and name `birthDate`. The value must have the `name` field set
	/// to `value` and then either set a `value[x]` or `part`.
	pub fn add(
		mut self,
		path: impl Into<String>,
		name: impl Into<String>,
		value: ParametersParameter,
	) -> Self {
		#[allow(clippy::unwrap_used)] // Will always succeed.
		let parameter = ParametersParameter::builder()
			.name("operation".to_owned())
			.part(vec![
				Some(
					ParametersParameter::builder()
						.name("type".to_owned())
						.value(ParametersParameterValue::Code("add".to_owned()))
						.build()
						.unwrap(),
				),
				Some(
					ParametersParameter::builder()
						.name("path".to_owned())
						.value(ParametersParameterValue::String(path.into()))
						.build()
						.unwrap(),
				),
				Some(
					ParametersParameter::builder()
						.name("name".to_owned())
						.value(ParametersParameterValue::String(name.into()))
						.build()
						.unwrap(),
				),
				Some(value),
			])
			.build()
			.unwrap();

		self.operations.push(Some(parameter));
		self
	}

	/// Add an `insert` operation to the list of operations. Note that the
	/// `path` needs to be set according the FHIR defititions, e.g.
	/// `Patient.name`. The value must have the `name` field set to `value` and
	/// then either set a `value[x]` or `part`.
	pub fn insert(
		mut self,
		path: impl Into<String>,
		value: ParametersParameter,
		index: i32,
	) -> Self {
		#[allow(clippy::unwrap_used)] // Will always succeed.
		let parameter = ParametersParameter::builder()
			.name("operation".to_owned())
			.part(vec![
				Some(
					ParametersParameter::builder()
						.name("type".to_owned())
						.value(ParametersParameterValue::Code("insert".to_owned()))
						.build()
						.unwrap(),
				),
				Some(
					ParametersParameter::builder()
						.name("path".to_owned())
						.value(ParametersParameterValue::String(path.into()))
						.build()
						.unwrap(),
				),
				Some(
					ParametersParameter::builder()
						.name("index".to_owned())
						.value(ParametersParameterValue::Integer(index))
						.build()
						.unwrap(),
				),
				Some(value),
			])
			.build()
			.unwrap();

		self.operations.push(Some(parameter));
		self
	}

	/// Add a `delete` operation to the list of operations. Note that the
	/// `path` needs to be set according the FHIR defititions, e.g.
	/// `Patient.active` to delete the `active` field on a `Patient` resource.
	pub fn delete(mut self, path: impl Into<String>) -> Self {
		#[allow(clippy::unwrap_used)] // Will always succeed.
		let parameter = ParametersParameter::builder()
			.name("operation".to_owned())
			.part(vec![
				Some(
					ParametersParameter::builder()
						.name("type".to_owned())
						.value(ParametersParameterValue::Code("delete".to_owned()))
						.build()
						.unwrap(),
				),
				Some(
					ParametersParameter::builder()
						.name("path".to_owned())
						.value(ParametersParameterValue::String(path.into()))
						.build()
						.unwrap(),
				),
			])
			.build()
			.unwrap();

		self.operations.push(Some(parameter));
		self
	}

	/// Add a `replace` operation to the list of operations. Note that the
	/// `path` needs to be set according the FHIR defititions, e.g.
	/// `Patient.name`. The value must have the `name` field set to `value` and
	/// then either set a `value[x]` or `part`.
	pub fn replace(mut self, path: impl Into<String>, value: ParametersParameter) -> Self {
		#[allow(clippy::unwrap_used)] // Will always succeed.
		let parameter = ParametersParameter::builder()
			.name("operation".to_owned())
			.part(vec![
				Some(
					ParametersParameter::builder()
						.name("type".to_owned())
						.value(ParametersParameterValue::Code("replace".to_owned()))
						.build()
						.unwrap(),
				),
				Some(
					ParametersParameter::builder()
						.name("path".to_owned())
						.value(ParametersParameterValue::String(path.into()))
						.build()
						.unwrap(),
				),
				Some(value),
			])
			.build()
			.unwrap();

		self.operations.push(Some(parameter));
		self
	}

	/// Add a `move` operation to the list of operations. Note that the
	/// `path` needs to be set according the FHIR defititions, e.g.
	/// `Patient.name`. The value must have the `name` field set to `value` and
	/// then either set a `value[x]` or `part`.
	pub fn r#move(mut self, path: impl Into<String>, source: i32, destination: i32) -> Self {
		#[allow(clippy::unwrap_used)] // Will always succeed.
		let parameter = ParametersParameter::builder()
			.name("operation".to_owned())
			.part(vec![
				Some(
					ParametersParameter::builder()
						.name("type".to_owned())
						.value(ParametersParameterValue::Code("move".to_owned()))
						.build()
						.unwrap(),
				),
				Some(
					ParametersParameter::builder()
						.name("path".to_owned())
						.value(ParametersParameterValue::String(path.into()))
						.build()
						.unwrap(),
				),
				Some(
					ParametersParameter::builder()
						.name("source".to_owned())
						.value(ParametersParameterValue::Integer(source))
						.build()
						.unwrap(),
				),
				Some(
					ParametersParameter::builder()
						.name("destination".to_owned())
						.value(ParametersParameterValue::Integer(destination))
						.build()
						.unwrap(),
				),
			])
			.build()
			.unwrap();

		self.operations.push(Some(parameter));
		self
	}

	/// Patch the resource on the FHIR server.
	pub async fn send(self) -> Result<(), Error> {
		#[allow(clippy::unwrap_used)] // Will always succeed.
		let parameters = Parameters::builder().parameter(self.operations).build().unwrap();

		let url = self.client.url(&[self.resource_type.as_str(), self.id]);
		let request = self
			.client
			.0
			.client
			.patch(url)
			.header(header::ACCEPT, MIME_TYPE)
			.header(header::CONTENT_TYPE, HeaderValue::from_static(MIME_TYPE))
			.json(&parameters);

		let response = self.client.run_request(request).await?;
		if response.status().is_success() {
			Ok(())
		} else {
			Err(Error::from_response_r4b(response).await)
		}
	}
}

/// Builder for a PATCH request via JSONPatch for a FHIR resource.
#[derive(Debug, Clone)]
#[must_use = "You probably want to send the PATCH request"]
pub struct PatchViaJson<'a> {
	/// FHIR client.
	client: Client<FhirR4B>,
	/// Resource type to apply the patch to.
	resource_type: ResourceType,
	/// Resource ID to apply the path to.
	id: &'a str,
	/// Operations to apply.
	operations: Vec<serde_json::Map<String, serde_json::Value>>,
}

impl<'a> PatchViaJson<'a> {
	/// Start building a new Patch request.
	pub fn new(client: Client<FhirR4B>, resource_type: ResourceType, id: &'a str) -> Self {
		Self { client, resource_type, id, operations: Vec::new() }
	}

	/// Add an `add` operation to the list of operations. The `path` needs to be
	/// in the correct format, e.g. `/birthDate`. The value needs to serialize
	/// into the correct format for the respective FHIR datatype, this cannot be
	/// checked in the client.
	pub fn add(mut self, path: impl Into<String>, value: impl Serialize) -> Result<Self, Error> {
		let mut operation = serde_json::Map::new();

		operation.insert("op".to_owned(), "add".into());
		operation.insert("path".to_owned(), path.into().into());
		operation.insert("value".to_owned(), serde_json::to_value(value)?);

		self.operations.push(operation);
		Ok(self)
	}

	/// Add a `remove` operation to the list of operations. The `path` needs to
	/// be in the correct format, e.g. `/birthDate`.
	pub fn remove(mut self, path: impl Into<String>) -> Self {
		let mut operation = serde_json::Map::new();

		operation.insert("op".to_owned(), "remove".into());
		operation.insert("path".to_owned(), path.into().into());

		self.operations.push(operation);
		self
	}

	/// Add a `test` operation to the list of operations. The `path` needs to be
	/// in the correct format, e.g. `/birthDate`. The value needs to serialize
	/// into the correct format for the respective FHIR datatype, this cannot be
	/// checked in the client.
	pub fn test(mut self, path: impl Into<String>, value: impl Serialize) -> Result<Self, Error> {
		let mut operation = serde_json::Map::new();

		operation.insert("op".to_owned(), "test".into());
		operation.insert("path".to_owned(), path.into().into());
		operation.insert("value".to_owned(), serde_json::to_value(value)?);

		self.operations.push(operation);
		Ok(self)
	}

	/// Add a `replace` operation to the list of operations. The `path` needs to
	/// be in the correct format, e.g. `/birthDate`. The value needs to
	/// serialize into the correct format for the respective FHIR datatype, this
	/// cannot be checked in the client.
	pub fn replace(
		mut self,
		path: impl Into<String>,
		value: impl Serialize,
	) -> Result<Self, Error> {
		let mut operation = serde_json::Map::new();

		operation.insert("op".to_owned(), "replace".into());
		operation.insert("path".to_owned(), path.into().into());
		operation.insert("value".to_owned(), serde_json::to_value(value)?);

		self.operations.push(operation);
		Ok(self)
	}

	/// Add a `move` operation to the list of operations. The `path`s needs to
	/// be in the correct format, e.g. `/birthDate`.
	pub fn r#move(mut self, from: impl Into<String>, path: impl Into<String>) -> Self {
		let mut operation = serde_json::Map::new();

		operation.insert("op".to_owned(), "move".into());
		operation.insert("from".to_owned(), from.into().into());
		operation.insert("path".to_owned(), path.into().into());

		self.operations.push(operation);
		self
	}

	/// Add a `copy` operation to the list of operations. The `path`s needs to
	/// be in the correct format, e.g. `/birthDate`.
	pub fn copy(mut self, from: impl Into<String>, path: impl Into<String>) -> Self {
		let mut operation = serde_json::Map::new();

		operation.insert("op".to_owned(), "copy".into());
		operation.insert("from".to_owned(), from.into().into());
		operation.insert("path".to_owned(), path.into().into());

		self.operations.push(operation);
		self
	}

	/// Patch the resource on the FHIR server.
	pub async fn send(self) -> Result<(), Error> {
		let url = self.client.url(&[self.resource_type.as_str(), self.id]);
		let request = self
			.client
			.0
			.client
			.patch(url)
			.header(header::ACCEPT, MIME_TYPE)
			.header(header::CONTENT_TYPE, HeaderValue::from_static("application/json-patch+json"))
			.json(&self.operations);

		let response = self.client.run_request(request).await?;
		if response.status().is_success() {
			Ok(())
		} else {
			Err(Error::from_response_r4b(response).await)
		}
	}
}
