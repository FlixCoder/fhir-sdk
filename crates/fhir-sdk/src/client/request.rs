//! HTTP Request implementation.

use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use tokio_retry::{
	strategy::{ExponentialBackoff, FixedInterval},
	RetryIf,
};

use super::{error::Error, misc::make_uuid_header_value};

/// Settings for the HTTP Requests.
///
/// By default, 3 retries are done with a fixed retry_time of 1000 ms.
#[derive(Debug, Clone)]
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
		}
	}
}

impl RequestSettings {
	/// Set the number of retries.
	#[must_use]
	pub const fn retries(mut self, retries: usize) -> Self {
		self.retries = retries;
		self
	}

	/// Set to use fixed interval retrying.
	#[must_use]
	pub const fn fixed_retry(mut self, interval: Duration) -> Self {
		self.exp_backoff = false;
		self.retry_time = interval;
		self.max_retry_time = None;
		self
	}

	/// Set to use exponential backoff retrying.
	#[must_use]
	pub const fn exp_backoff(mut self, start_time: Duration, max_time: Option<Duration>) -> Self {
		self.exp_backoff = true;
		self.retry_time = start_time;
		self.max_retry_time = max_time;
		self
	}

	/// Set the request timeout.
	#[must_use]
	pub const fn timeout(mut self, timeout: Option<Duration>) -> Self {
		self.timeout = timeout;
		self
	}

	/// Insert a header into the request headers to be set on each request.
	#[must_use]
	pub fn header(mut self, header: HeaderName, value: HeaderValue) -> Self {
		self.headers.insert(header, value);
		self
	}

	/// Make a HTTP request using the settings. Returns the response.
	///
	/// It is recommended to set the `X-Correlation-Id` header outside, for a whole transaction.
	#[tracing::instrument(level = "debug", skip_all, fields(x_correlation_id, x_request_id))]
	pub(crate) async fn make_request(
		&self,
		mut request: reqwest::RequestBuilder,
	) -> Result<reqwest::Response, Error> {
		if let Some(timeout) = self.timeout {
			request = request.timeout(timeout);
		}

		// Add or override default headers with request headers.
		let (client, request_result) = request.build_split();
		let mut request = request_result?;
		let mut headers = self.headers.clone();
		headers.extend(request.headers().clone());
		*request.headers_mut() = headers;

		// Add `X-Request-Id` and `X-Correlation-Id` header if not already set.
		let id_value = make_uuid_header_value();
		request.headers_mut().entry("X-Correlation-Id").or_insert_with(|| id_value.clone());
		request.headers_mut().entry("X-Request-Id").or_insert(id_value);

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

		// Fill in tracing spans to log the informational/correlational headers.
		let x_correlation_id =
			request.headers().get("X-Correlation-Id").and_then(|v| v.to_str().ok());
		let x_request_id = request.headers().get("X-Request-Id").and_then(|v| v.to_str().ok());
		tracing::Span::current()
			.record("x_correlation_id", x_correlation_id)
			.record("x_request_id", x_request_id);

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
