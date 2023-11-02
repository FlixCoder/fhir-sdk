//! HTTP Request implementation.

use std::{future::Future, pin::Pin, sync::Arc, time::Duration};

use reqwest::{
	header::{HeaderMap, HeaderName, HeaderValue},
	StatusCode,
};
use tokio_retry::{
	strategy::{ExponentialBackoff, FixedInterval},
	RetryIf,
};

use super::error::Error;

/// Auth callback function type.
type AuthCallback = Arc<
	dyn Fn() -> Pin<
			Box<
				dyn Future<Output = Result<HeaderValue, Box<dyn std::error::Error + Send + Sync>>>
					+ Send,
			>,
		> + Send
		+ Sync,
>;

/// Settings for the HTTP Requests.
///
/// By default, 3 retries are done with a fixed retry_time of 1000 ms.
#[derive(Clone)]
pub struct RequestSettings {
	/// Number of retries to make.
	retries: usize,
	/// Duration to wait between the requests (either always for fixed or for
	/// the first time for exponential backoff).
	retry_time: Duration,
	/// Max retry duration between requests. Only relevant for exponential
	/// backoff.
	max_retry_time: Option<Duration>,
	/// Whether to use exponential backoff or not.
	exp_backoff: bool,
	/// Timeout for the requests.
	timeout: Option<Duration>,
	/// Additional headers to set for the requests.
	headers: HeaderMap,
	/// Authorization callback method, returning the authorization header value.
	auth_callback: Option<AuthCallback>,
}

impl Default for RequestSettings {
	fn default() -> Self {
		Self {
			retries: 3,
			retry_time: Duration::from_millis(1000),
			max_retry_time: None,
			exp_backoff: false,
			timeout: None,
			headers: HeaderMap::new(),
			auth_callback: None,
		}
	}
}

impl RequestSettings {
	/// Set the number of retries.
	#[must_use]
	pub fn retries(mut self, retries: usize) -> Self {
		self.retries = retries;
		self
	}

	/// Set to use fixed interval retrying.
	#[must_use]
	pub fn fixed_retry(mut self, interval: Duration) -> Self {
		self.exp_backoff = false;
		self.retry_time = interval;
		self.max_retry_time = None;
		self
	}

	/// Set to use exponential backoff retrying.
	#[must_use]
	pub fn exp_backoff(mut self, start_time: Duration, max_time: Option<Duration>) -> Self {
		self.exp_backoff = true;
		self.retry_time = start_time;
		self.max_retry_time = max_time;
		self
	}

	/// Set the request timeout.
	#[must_use]
	pub fn timeout(mut self, timeout: Option<Duration>) -> Self {
		self.timeout = timeout;
		self
	}

	/// Insert a header into the request headers to be set on each request.
	#[must_use]
	pub fn header(mut self, header: HeaderName, value: HeaderValue) -> Self {
		self.headers.insert(header, value);
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

	/// Make a HTTP request using the settings. In the process, the
	/// authorization callback may be called. Returns the response, as well as
	/// whether the request settings where modified.
	pub(crate) async fn make_request(
		&mut self,
		mut request: reqwest::RequestBuilder,
	) -> Result<(reqwest::Response, bool), Error> {
		if let Some(timeout) = self.timeout {
			request = request.timeout(timeout);
		}

		// Add or override default headers with request headers.
		let (client, request_result) = request.build_split();
		let mut request = request_result?;
		let mut headers = self.headers.clone();
		headers.extend(request.headers().clone());
		*request.headers_mut() = headers;

		// Run request.
		let response = self
			.make_retried_request(&client, request.try_clone().ok_or(Error::RequestNotClone)?)
			.await?;
		let Some(auth_callback) = self.auth_callback.as_ref() else {
			return Ok((response, false));
		};

		// On authorization failure, retry after resetting the authorization.
		if response.status() == StatusCode::UNAUTHORIZED {
			tracing::info!("Hit unauthorized response, calling auth_callback");
			let auth_value =
				(auth_callback)().await.map_err(|err| Error::AuthCallback(err.to_string()))?;
			self.headers.insert(reqwest::header::AUTHORIZATION, auth_value.clone());
			request.headers_mut().insert(reqwest::header::AUTHORIZATION, auth_value);
			let response = self.make_retried_request(&client, request).await?;
			return Ok((response, true));
		}

		Ok((response, false))
	}

	/// Make a retried request using the settings, except for the authorization
	/// handling.
	async fn make_retried_request(
		&self,
		client: &reqwest::Client,
		request: reqwest::Request,
	) -> Result<reqwest::Response, Error> {
		// Construct the dynamic retry strategy iterator.
		let strategy: Box<dyn Iterator<Item = Duration> + Send + Sync> = if self.exp_backoff {
			let mut exp_backoff =
				ExponentialBackoff::from_millis(self.retry_time.as_millis() as u64);
			if let Some(max_backoff) = self.max_retry_time {
				exp_backoff = exp_backoff.max_delay(max_backoff);
			}
			Box::new(exp_backoff)
		} else {
			Box::new(FixedInterval::from_millis(self.retry_time.as_millis() as u64))
		};

		// Send the request, but retry on specific failures.
		RetryIf::spawn(
			strategy.take(self.retries),
			|| async {
				tracing::debug!("Sending {} request to {}", request.method(), request.url());
				let request = request.try_clone().ok_or(Error::RequestNotClone)?;
				let response = client.execute(request).await?;
				tracing::debug!("Got response: {}", response.status());
				Ok(response)
			},
			Error::should_retry,
		)
		.await
	}
}

impl std::fmt::Debug for RequestSettings {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("RequestSettings")
			.field("retries", &self.retries)
			.field("retry_time", &self.retry_time)
			.field("max_retry_time", &self.max_retry_time)
			.field("exp_backoff", &self.exp_backoff)
			.field("timeout", &self.timeout)
			.field("headers", &self.headers)
			.field("auth_callback", &"<fn>")
			.finish()
	}
}
