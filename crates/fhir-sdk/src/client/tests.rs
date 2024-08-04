#![allow(clippy::unwrap_used, clippy::indexing_slicing)]

use std::{
	sync::atomic::{AtomicUsize, Ordering},
	time::Duration,
};

use header::HeaderValue;
use reqwest::Method;
use wiremock::{matchers, Mock, MockServer, ResponseTemplate};

use super::*;

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
		.build()?;

	let url = format!("{}/my/custom/path", mocks.uri());
	let response = client.send_custom_request(|http| http.get(url)).await?;

	assert_eq!(response.status(), StatusCode::OK);
	assert_eq!(counter.load(Ordering::SeqCst), 1);

	mocks.verify().await;
	Ok(())
}
