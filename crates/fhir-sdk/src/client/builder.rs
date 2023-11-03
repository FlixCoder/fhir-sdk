//! Builder implementation for the client.

use std::{future::Future, sync::Arc};

use reqwest::{header::HeaderValue, Url};

use super::{AuthCallback, Client, Error, RequestSettings};

/// Builder for the [Client]
#[derive(Clone, Default)]
pub struct ClientBuilder {
	/// The FHIR server's base URL.
	base_url: Option<Url>,
	/// Request settings.
	request_settings: Option<RequestSettings>,
	/// Auth callback.
	auth_callback: Option<AuthCallback>,
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

	/// Set an authorization callback to be called every time the server returns
	/// unauthorized. Should return the header value for the `Authorization`
	/// header.
	#[must_use]
	pub fn auth_callback<F, O, E>(mut self, auth: F) -> Self
	where
		F: Fn() -> O + Send + Sync + Copy + 'static,
		O: Future<Output = Result<HeaderValue, E>> + Send,
		E: Into<Box<dyn std::error::Error + Send + Sync>>,
	{
		self.auth_callback =
			Some(Arc::new(move || Box::pin(async move { (auth)().await.map_err(Into::into) })));
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

		#[allow(clippy::expect_used)] // only happens on panics, so we can panic again.
		let mut auth_callback = client.0.auth_callback.lock().expect("mutex poisened");
		*auth_callback = self.auth_callback;
		drop(auth_callback);

		Ok(client)
	}
}

impl std::fmt::Debug for ClientBuilder {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ClientBuilder")
			.field("base_url", &self.base_url)
			.field("request_settings", &self.request_settings)
			.field("auth_callback", &self.auth_callback.as_ref().map(|_| "<fn>"))
			.finish()
	}
}
