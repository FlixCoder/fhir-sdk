//! Patch request building.

use reqwest::{
	StatusCode,
	header::{self, HeaderValue},
};
use serde::Serialize;

use super::{Client, Error};
use crate::{
	extensions::{ParameterExt, ParameterValueExt, ParametersExt},
	version::FhirVersion,
};

/// Type alias for the `ParametersParameter` type for any version.
type ParametersParameter<V> = <<V as FhirVersion>::Parameters as ParametersExt>::Parameter;
/// Type alias for the `ParametersParameterValue` type for any version.
type ParametersParameterValue<V> =
	<<<V as FhirVersion>::Parameters as ParametersExt>::Parameter as ParameterExt>::Value;

/// Builder for a PATCH request via FHIRPath for a FHIR resource.
#[derive(Debug, Clone)]
#[must_use = "You probably want to send the PATCH request"]
pub struct PatchViaFhir<'a, V: FhirVersion> {
	/// FHIR client.
	client: Client<V>,
	/// Resource type to apply the patch to.
	resource_type: V::ResourceType,
	/// Resource ID to apply the path to.
	id: &'a str,
	/// Operations to apply.
	operations: Vec<Option<ParametersParameter<V>>>,
}

impl<'a, V: FhirVersion> PatchViaFhir<'a, V>
where
	(StatusCode, V::OperationOutcome): Into<Error>,
{
	/// Start building a new Patch request.
	pub(crate) const fn new(
		client: Client<V>,
		resource_type: V::ResourceType,
		id: &'a str,
	) -> Self {
		Self { client, resource_type, id, operations: Vec::new() }
	}

	/// Add an `add` operation to the list of operations. Note that the `path`
	/// and `name` need to be set according the FHIR defititions, e.g. path
	/// `Patient` and name `birthDate`. The value must have the `name` field set
	/// to `value` and then either set a `value[x]` or `part`.
	pub fn add<P, N>(mut self, path: P, name: N, value: ParametersParameter<V>) -> Self
	where
		P: Into<String>,
		N: Into<String>,
	{
		#[allow(clippy::unwrap_used, reason = "We know the builder succeeds")]
		let parameter = ParametersParameter::<V>::make(
			"operation".to_owned(),
			None,
			vec![
				Some(ParametersParameter::<V>::make(
					"type".to_owned(),
					Some(ParametersParameterValue::<V>::make_code("add".to_owned())),
					vec![],
				)),
				Some(ParametersParameter::<V>::make(
					"path".to_owned(),
					Some(ParametersParameterValue::<V>::make_string(path.into())),
					vec![],
				)),
				Some(ParametersParameter::<V>::make(
					"name".to_owned(),
					Some(ParametersParameterValue::<V>::make_string(name.into())),
					vec![],
				)),
				Some(value),
			],
		);

		self.operations.push(Some(parameter));
		self
	}

	/// Add an `insert` operation to the list of operations. Note that the
	/// `path` needs to be set according the FHIR defititions, e.g.
	/// `Patient.name`. The value must have the `name` field set to `value` and
	/// then either set a `value[x]` or `part`.
	pub fn insert<P: Into<String>>(
		mut self,
		path: P,
		value: ParametersParameter<V>,
		index: i32,
	) -> Self {
		#[allow(clippy::unwrap_used, reason = "We know the builder succeeds")]
		let parameter = ParametersParameter::<V>::make(
			"operation".to_owned(),
			None,
			vec![
				Some(ParametersParameter::<V>::make(
					"type".to_owned(),
					Some(ParametersParameterValue::<V>::make_code("insert".to_owned())),
					vec![],
				)),
				Some(ParametersParameter::<V>::make(
					"path".to_owned(),
					Some(ParametersParameterValue::<V>::make_string(path.into())),
					vec![],
				)),
				Some(ParametersParameter::<V>::make(
					"index".to_owned(),
					Some(ParametersParameterValue::<V>::make_integer(index)),
					vec![],
				)),
				Some(value),
			],
		);

		self.operations.push(Some(parameter));
		self
	}

	/// Add a `delete` operation to the list of operations. Note that the
	/// `path` needs to be set according the FHIR defititions, e.g.
	/// `Patient.active` to delete the `active` field on a `Patient` resource.
	pub fn delete<P: Into<String>>(mut self, path: P) -> Self {
		#[allow(clippy::unwrap_used, reason = "We know the builder succeeds")]
		let parameter = ParametersParameter::<V>::make(
			"operation".to_owned(),
			None,
			vec![
				Some(ParametersParameter::<V>::make(
					"type".to_owned(),
					Some(ParametersParameterValue::<V>::make_code("delete".to_owned())),
					vec![],
				)),
				Some(ParametersParameter::<V>::make(
					"path".to_owned(),
					Some(ParametersParameterValue::<V>::make_string(path.into())),
					vec![],
				)),
			],
		);

		self.operations.push(Some(parameter));
		self
	}

	/// Add a `replace` operation to the list of operations. Note that the
	/// `path` needs to be set according the FHIR defititions, e.g.
	/// `Patient.name`. The value must have the `name` field set to `value` and
	/// then either set a `value[x]` or `part`.
	pub fn replace<P: Into<String>>(mut self, path: P, value: ParametersParameter<V>) -> Self {
		#[allow(clippy::unwrap_used, reason = "We know the builder succeeds")]
		let parameter = ParametersParameter::<V>::make(
			"operation".to_owned(),
			None,
			vec![
				Some(ParametersParameter::<V>::make(
					"type".to_owned(),
					Some(ParametersParameterValue::<V>::make_code("replace".to_owned())),
					vec![],
				)),
				Some(ParametersParameter::<V>::make(
					"path".to_owned(),
					Some(ParametersParameterValue::<V>::make_string(path.into())),
					vec![],
				)),
				Some(value),
			],
		);

		self.operations.push(Some(parameter));
		self
	}

	/// Add a `move` operation to the list of operations. Note that the
	/// `path` needs to be set according the FHIR defititions, e.g.
	/// `Patient.name`. The value must have the `name` field set to `value` and
	/// then either set a `value[x]` or `part`.
	pub fn r#move<P: Into<String>>(mut self, path: P, source: i32, destination: i32) -> Self {
		#[allow(clippy::unwrap_used, reason = "We know the builder succeeds")]
		let parameter = ParametersParameter::<V>::make(
			"operation".to_owned(),
			None,
			vec![
				Some(ParametersParameter::<V>::make(
					"type".to_owned(),
					Some(ParametersParameterValue::<V>::make_code("move".to_owned())),
					vec![],
				)),
				Some(ParametersParameter::<V>::make(
					"path".to_owned(),
					Some(ParametersParameterValue::<V>::make_string(path.into())),
					vec![],
				)),
				Some(ParametersParameter::<V>::make(
					"source".to_owned(),
					Some(ParametersParameterValue::<V>::make_integer(source)),
					vec![],
				)),
				Some(ParametersParameter::<V>::make(
					"destination".to_owned(),
					Some(ParametersParameterValue::<V>::make_integer(destination)),
					vec![],
				)),
			],
		);

		self.operations.push(Some(parameter));
		self
	}

	/// Patch the resource on the FHIR server.
	pub async fn send(self) -> Result<(), Error> {
		let parameters = V::Parameters::make(self.operations);

		let url = self.client.url(&[self.resource_type.as_ref(), self.id]);
		let request = self
			.client
			.0
			.client
			.patch(url)
			.header(header::ACCEPT, V::MIME_TYPE)
			.header(header::CONTENT_TYPE, HeaderValue::from_static(V::MIME_TYPE))
			.json(&parameters);

		let response = self.client.run_request(request).await?;
		if response.status().is_success() {
			Ok(())
		} else {
			Err(Error::from_response::<V>(response).await)
		}
	}
}

/// Builder for a PATCH request via JSONPatch for a FHIR resource.
#[derive(Debug, Clone)]
#[must_use = "You probably want to send the PATCH request"]
pub struct PatchViaJson<'a, V: FhirVersion> {
	/// FHIR client.
	client: Client<V>,
	/// Resource type to apply the patch to.
	resource_type: V::ResourceType,
	/// Resource ID to apply the path to.
	id: &'a str,
	/// Operations to apply.
	operations: Vec<serde_json::Map<String, serde_json::Value>>,
}

impl<'a, V: FhirVersion> PatchViaJson<'a, V>
where
	(StatusCode, V::OperationOutcome): Into<Error>,
{
	/// Start building a new Patch request.
	pub(crate) const fn new(
		client: Client<V>,
		resource_type: V::ResourceType,
		id: &'a str,
	) -> Self {
		Self { client, resource_type, id, operations: Vec::new() }
	}

	/// Add an `add` operation to the list of operations. The `path` needs to be
	/// in the correct format, e.g. `/birthDate`. The value needs to serialize
	/// into the correct format for the respective FHIR datatype, this cannot be
	/// checked in the client.
	pub fn add<P, I>(mut self, path: P, value: I) -> Result<Self, Error>
	where
		P: Into<String>,
		I: Serialize,
	{
		let mut operation = serde_json::Map::new();

		operation.insert("op".to_owned(), "add".into());
		operation.insert("path".to_owned(), path.into().into());
		operation.insert("value".to_owned(), serde_json::to_value(value)?);

		self.operations.push(operation);
		Ok(self)
	}

	/// Add a `remove` operation to the list of operations. The `path` needs to
	/// be in the correct format, e.g. `/birthDate`.
	pub fn remove<P: Into<String>>(mut self, path: P) -> Self {
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
	pub fn test<P, I>(mut self, path: P, value: I) -> Result<Self, Error>
	where
		P: Into<String>,
		I: Serialize,
	{
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
	pub fn replace<P, I>(mut self, path: P, value: I) -> Result<Self, Error>
	where
		P: Into<String>,
		I: Serialize,
	{
		let mut operation = serde_json::Map::new();

		operation.insert("op".to_owned(), "replace".into());
		operation.insert("path".to_owned(), path.into().into());
		operation.insert("value".to_owned(), serde_json::to_value(value)?);

		self.operations.push(operation);
		Ok(self)
	}

	/// Add a `move` operation to the list of operations. The `path`s needs to
	/// be in the correct format, e.g. `/birthDate`.
	pub fn r#move<PF, PT>(mut self, from: PF, path: PT) -> Self
	where
		PF: Into<String>,
		PT: Into<String>,
	{
		let mut operation = serde_json::Map::new();

		operation.insert("op".to_owned(), "move".into());
		operation.insert("from".to_owned(), from.into().into());
		operation.insert("path".to_owned(), path.into().into());

		self.operations.push(operation);
		self
	}

	/// Add a `copy` operation to the list of operations. The `path`s needs to
	/// be in the correct format, e.g. `/birthDate`.
	pub fn copy<PF, PT>(mut self, from: PF, path: PT) -> Self
	where
		PF: Into<String>,
		PT: Into<String>,
	{
		let mut operation = serde_json::Map::new();

		operation.insert("op".to_owned(), "copy".into());
		operation.insert("from".to_owned(), from.into().into());
		operation.insert("path".to_owned(), path.into().into());

		self.operations.push(operation);
		self
	}

	/// Patch the resource on the FHIR server.
	pub async fn send(self) -> Result<(), Error> {
		let url = self.client.url(&[self.resource_type.as_ref(), self.id]);
		let request = self
			.client
			.0
			.client
			.patch(url)
			.header(header::ACCEPT, V::MIME_TYPE)
			.header(header::CONTENT_TYPE, HeaderValue::from_static("application/json-patch+json"))
			.json(&self.operations);

		let response = self.client.run_request(request).await?;
		if response.status().is_success() {
			Ok(())
		} else {
			Err(Error::from_response::<V>(response).await)
		}
	}
}
