#![allow(clippy::unwrap_used, clippy::indexing_slicing)]

use std::{
	sync::atomic::{AtomicUsize, Ordering},
	time::Duration,
};

use ::tokio::sync::OnceCell;
use header::HeaderValue;
use reqwest::Method;
use serde_json::json;
use wiremock::{Mock, MockServer, ResponseTemplate, matchers};

use super::*;

/// Set up logging for the tests.
pub async fn setup_logging() {
	static ONCE: OnceCell<()> = OnceCell::const_new();
	ONCE.get_or_init(|| async {
		tracing_subscriber::fmt::fmt()
			.with_test_writer()
			.with_env_filter("DEBUG,hyper=error,reqwest=info")
			.init();
	})
	.await;
}

async fn mock_custom_request() -> MockServer {
	let server = MockServer::start().await;

	Mock::given(matchers::method(Method::GET))
		.and(matchers::path("/my/custom/path"))
		.respond_with(ResponseTemplate::new(StatusCode::UNAUTHORIZED))
		.with_priority(1)
		.named("Custom request unauthorized")
		.expect(1)
		.up_to_n_times(1)
		.mount(&server)
		.await;

	Mock::given(matchers::method(Method::GET))
		.and(matchers::path("/my/custom/path"))
		.respond_with(
			ResponseTemplate::new(StatusCode::INTERNAL_SERVER_ERROR)
				.set_delay(Duration::from_secs(5)),
		)
		.with_priority(2)
		.named("Custom request timeout")
		.expect(1)
		.up_to_n_times(1)
		.mount(&server)
		.await;

	Mock::given(matchers::method(Method::GET))
		.and(matchers::path("/my/custom/path"))
		.respond_with(ResponseTemplate::new(StatusCode::OK))
		.with_priority(5)
		.named("Custom request success")
		.expect(1)
		.mount(&server)
		.await;

	server
}

#[tokio::test]
async fn send_custom_request_features() -> anyhow::Result<()> {
	setup_logging().await;
	let mocks = mock_custom_request().await;

	let counter = Arc::new(AtomicUsize::new(0));
	let counter_for_closure = counter.clone();

	let settings = RequestSettings::default()
		.timeout(Some(Duration::from_millis(500)))
		.retries(1)
		.fixed_retry(Duration::from_millis(100));
	let client = <Client>::builder()
		.base_url(Url::parse("http://localhost/")?)
		.request_settings(settings)
		.auth_callback(move |_http: reqwest::Client| {
			let counter = counter_for_closure.clone();
			async move {
				counter.fetch_add(1, Ordering::Relaxed);
				anyhow::Ok(HeaderValue::from_static("Bearer <token>"))
			}
		})
		.allow_origin_mismatch()
		.allow_version_mismatch()
		.build()?;

	let url = format!("{}/my/custom/path", mocks.uri());
	let response = client.send_custom_request(|http| http.get(url)).await?;

	assert_eq!(response.status(), StatusCode::OK);
	assert_eq!(counter.load(Ordering::SeqCst), 1);

	mocks.verify().await;
	Ok(())
}


async fn mock_version_mismatch() -> MockServer {
	let server = MockServer::start().await;

	Mock::given(matchers::method(Method::GET))
		.and(matchers::path("/Patient/1"))
		.respond_with(
			ResponseTemplate::new(StatusCode::OK).set_body_raw(
				serde_json::to_vec(&json!({
					"resourceType": "Patient",
					"id": "1",
					"active": true
				}))
				.unwrap(),
				"application/fhir+json; fhirVersion=1.0",
			),
		)
		.named("FHIR version 1 response")
		.mount(&server)
		.await;

	server
}

#[tokio::test]
async fn check_major_fhir_version() -> anyhow::Result<()> {
	setup_logging().await;
	let mocks = mock_version_mismatch().await;

	let client = <Client>::builder().base_url(Url::parse(&mocks.uri())?).build()?;

	// Avoid FHIR version specific types.. So sending custom request.
	let url = format!("{}/Patient/1", mocks.uri());
	let result = client.send_custom_request(|http| http.get(url)).await;
	assert!(matches!(result, Err(Error::DifferentFhirVersion(_))));

	// Allow different versions this time.
	let client =
		<Client>::builder().base_url(Url::parse(&mocks.uri())?).allow_version_mismatch().build()?;

	// Avoid FHIR version specific types.. So sending custom request.
	let url = format!("{}/Patient/1", mocks.uri());
	let response = client.send_custom_request(|http| http.get(url)).await?;
	assert_eq!(response.status(), StatusCode::OK);

	Ok(())
}

#[tokio::test]
async fn check_url_origin() -> anyhow::Result<()> {
	setup_logging().await;
	let mocks = mock_version_mismatch().await;

	let client = <Client>::builder().base_url("http://localhost/".parse()?).build()?;

	let result = client.send_custom_request(|http| http.get("http://localhost:5555/")).await;
	assert!(matches!(result, Err(Error::DifferentOrigin(_))));

	// Allow different origins this time.
	let client = <Client>::builder()
		.base_url("http://localhost/".parse()?)
		.allow_version_mismatch()
		.allow_origin_mismatch()
		.build()?;

	let url = format!("{}/Patient/1", mocks.uri());
	let response = client.send_custom_request(|http| http.get(url)).await?;
	assert_eq!(response.status(), StatusCode::OK);

	Ok(())
}


async fn mock_x_correlation_id() -> MockServer {
	let server = MockServer::start().await;

	Mock::given(matchers::method(Method::GET))
		.and(matchers::header("X-Correlation-Id", "custom-id"))
		.respond_with(ResponseTemplate::new(StatusCode::UNAUTHORIZED))
		.with_priority(1)
		.named("Custom correlation ID failure retry")
		.expect(1)
		.up_to_n_times(1)
		.mount(&server)
		.await;

	Mock::given(matchers::method(Method::GET))
		.and(matchers::header("X-Correlation-Id", "custom-id"))
		.respond_with(ResponseTemplate::new(StatusCode::OK))
		.with_priority(2)
		.named("Custom correlation ID success")
		.expect(1)
		.up_to_n_times(1)
		.mount(&server)
		.await;

	Mock::given(matchers::method(Method::GET))
		.and(matchers::header_exists("X-Correlation-Id"))
		.respond_with(ResponseTemplate::new(StatusCode::OK))
		.with_priority(100)
		.named("Any correlation ID success")
		.expect(1)
		.up_to_n_times(1)
		.mount(&server)
		.await;

	server
}

#[tokio::test]
async fn use_correlation_id() -> anyhow::Result<()> {
	setup_logging().await;
	let mocks = mock_x_correlation_id().await;

	let client = <Client>::builder()
		.base_url(Url::parse(&mocks.uri())?)
		.auth_callback(move |_http: reqwest::Client| {
			std::future::ready(anyhow::Ok(HeaderValue::from_static("Bearer <mock>")))
		})
		.build()?;

	let url = client.base_url();

	let response = client
		.send_custom_request(|http| http.get(url.clone()).header("X-Correlation-Id", "custom-id"))
		.await?;
	assert_eq!(response.status(), StatusCode::OK);

	let response = client.send_custom_request(|http| http.get(url.clone())).await?;
	assert_eq!(response.status(), StatusCode::OK);

	mocks.verify().await;
	Ok(())
}
