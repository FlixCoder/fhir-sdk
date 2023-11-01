//! Patch request building.

use model::resources::{Parameters, ParametersParameter, ParametersParameterValue, ResourceType};
use reqwest::header::{self, HeaderValue};

use super::{model, Client, Error, MIME_TYPE};

/// Builder for a PATCH request for a FHIR resource.
#[derive(Debug, Clone)]
#[must_use = "You probably want to send the PATCH request"]
pub struct Patch<'a> {
	/// FHIR client.
	client: Client,
	/// Resource type to apply the patch to.
	resource_type: ResourceType,
	/// Resource ID to apply the path to.
	id: &'a str,
	/// Operations to apply.
	operations: Vec<Option<ParametersParameter>>,
}

impl<'a> Patch<'a> {
	/// Start building a new Patch request.
	pub fn new(client: Client, resource_type: ResourceType, id: &'a str) -> Self {
		Self { client, resource_type, id, operations: Vec::new() }
	}

	/// Add an `add` operation to the list of operations. The value must have
	/// the `name` field set to `value` and then either set a `value[x]` or
	/// `part`.
	pub fn add(
		mut self,
		path: impl Into<String>,
		name: impl Into<String>,
		value: ParametersParameter,
	) -> Self {
		let parameter = ParametersParameter::builder()
			.name("operation".to_owned())
			.part(vec![
				Some(
					ParametersParameter::builder()
						.name("type".to_owned())
						.value(ParametersParameterValue::Code("add".to_owned()))
						.build(),
				),
				Some(
					ParametersParameter::builder()
						.name("path".to_owned())
						.value(ParametersParameterValue::String(path.into()))
						.build(),
				),
				Some(
					ParametersParameter::builder()
						.name("name".to_owned())
						.value(ParametersParameterValue::String(name.into()))
						.build(),
				),
				Some(value),
			])
			.build();

		self.operations.push(Some(parameter));
		self
	}

	/// Add an `insert` operation to the list of operations. The value must have
	/// the `name` field set to `value` and then either set a `value[x]` or
	/// `part`.
	pub fn insert(
		mut self,
		path: impl Into<String>,
		value: ParametersParameter,
		index: i32,
	) -> Self {
		let parameter = ParametersParameter::builder()
			.name("operation".to_owned())
			.part(vec![
				Some(
					ParametersParameter::builder()
						.name("type".to_owned())
						.value(ParametersParameterValue::Code("insert".to_owned()))
						.build(),
				),
				Some(
					ParametersParameter::builder()
						.name("path".to_owned())
						.value(ParametersParameterValue::String(path.into()))
						.build(),
				),
				Some(
					ParametersParameter::builder()
						.name("index".to_owned())
						.value(ParametersParameterValue::Integer(index))
						.build(),
				),
				Some(value),
			])
			.build();

		self.operations.push(Some(parameter));
		self
	}

	/// Add an `delete` operation to the list of operations.
	pub fn delete(mut self, path: impl Into<String>) -> Self {
		let parameter = ParametersParameter::builder()
			.name("operation".to_owned())
			.part(vec![
				Some(
					ParametersParameter::builder()
						.name("type".to_owned())
						.value(ParametersParameterValue::Code("delete".to_owned()))
						.build(),
				),
				Some(
					ParametersParameter::builder()
						.name("path".to_owned())
						.value(ParametersParameterValue::String(path.into()))
						.build(),
				),
			])
			.build();

		self.operations.push(Some(parameter));
		self
	}

	/// Add an `replace` operation to the list of operations. The value must
	/// have the `name` field set to `value` and then either set a `value[x]` or
	/// `part`.
	pub fn replace(mut self, path: impl Into<String>, value: ParametersParameter) -> Self {
		let parameter = ParametersParameter::builder()
			.name("operation".to_owned())
			.part(vec![
				Some(
					ParametersParameter::builder()
						.name("type".to_owned())
						.value(ParametersParameterValue::Code("replace".to_owned()))
						.build(),
				),
				Some(
					ParametersParameter::builder()
						.name("path".to_owned())
						.value(ParametersParameterValue::String(path.into()))
						.build(),
				),
				Some(value),
			])
			.build();

		self.operations.push(Some(parameter));
		self
	}

	/// Add an `move` operation to the list of operations. The value must
	/// have the `name` field set to `value` and then either set a `value[x]` or
	/// `part`.
	pub fn r#move(mut self, path: impl Into<String>, source: i32, destination: i32) -> Self {
		let parameter = ParametersParameter::builder()
			.name("operation".to_owned())
			.part(vec![
				Some(
					ParametersParameter::builder()
						.name("type".to_owned())
						.value(ParametersParameterValue::Code("move".to_owned()))
						.build(),
				),
				Some(
					ParametersParameter::builder()
						.name("path".to_owned())
						.value(ParametersParameterValue::String(path.into()))
						.build(),
				),
				Some(
					ParametersParameter::builder()
						.name("source".to_owned())
						.value(ParametersParameterValue::Integer(source))
						.build(),
				),
				Some(
					ParametersParameter::builder()
						.name("destination".to_owned())
						.value(ParametersParameterValue::Integer(destination))
						.build(),
				),
			])
			.build();

		self.operations.push(Some(parameter));
		self
	}

	/// Patch the resource on the FHIR server.
	pub async fn send(self) -> Result<(), Error> {
		let parameters = Parameters::builder().parameter(self.operations).build();

		let url = self.client.url(&[self.resource_type.as_str(), self.id]);
		let request = self
			.client
			.0
			.client
			.patch(url)
			.header(header::CONTENT_TYPE, HeaderValue::from_static(MIME_TYPE))
			.json(&parameters);

		let response = self.client.run_request(request).await?;
		if response.status().is_success() {
			Ok(())
		} else {
			Err(Error::from_response(response).await)
		}
	}
}
