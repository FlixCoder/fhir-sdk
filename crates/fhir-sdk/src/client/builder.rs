//! Builder implementation for the client.

use reqwest::Url;

use super::{Client, Error, RequestSettings};

/// Builder for the [Client]
#[derive(Debug, Clone, Default)]
pub struct ClientBuilder {
	/// The FHIR server's base URL.
	base_url: Option<Url>,
	/// Request settings.
	request_settings: Option<RequestSettings>,
}

impl ClientBuilder {
	/// The FHIR server's base URL.
	#[must_use]
	pub fn base_url(mut self, base_url: Url) -> Self {
		self.base_url = Some(base_url);
		self
	}

	/// Request settings.
	#[must_use]
	pub fn request_settings(mut self, settings: RequestSettings) -> Self {
		self.request_settings = Some(settings);
		self
	}

	/// Finalize building the client.
	pub fn build(self) -> Result<Client, Error> {
		let Some(base_url) = self.base_url else {
			return Err(Error::BuilderMissingField("base_url"));
		};
		let client = Client::new(base_url)?;

		if let Some(request_settings) = self.request_settings {
			client.set_request_settings(request_settings);
		}

		Ok(client)
	}
}
