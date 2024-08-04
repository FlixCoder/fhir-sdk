//! Authentication helpers & implementation.

use std::future::Future;

use async_trait::async_trait;
use reqwest::header::HeaderValue;

/// Error type of the auth callback.
type AuthCallbackError = Box<dyn std::error::Error + Send + Sync>;

/// Wrapper around a dynamic [`LoginManager`] that converts the error type.
pub struct AuthCallback {
	/// Dynamic [LoginManager].
	inner: Box<dyn LoginManagerConverted>,
}

impl AuthCallback {
	/// Create new auth callback.
	#[must_use]
	pub fn new<ACB>(auth_callback: ACB) -> Self
	where
		ACB: LoginManager + 'static,
	{
		Self { inner: Box::new(auth_callback) }
	}

	/// Authenticate, converting the error.
	pub async fn authenticate(
		&mut self,
		client: reqwest::Client,
	) -> Result<HeaderValue, AuthCallbackError> {
		self.inner.authenticate(client).await
	}
}

/// Trait to be implemented for the
/// [`ClientBuilder::auth_callback`](super::builder::ClientBuilder::auth_callback).
/// You can implement the functions as `async fn`, no need for `impl Future`.
///
/// It is automatically implemented for async functions and closures:
/// - Async functions `async fn my_auth_callback(client: reqwest::Client) ->
///   Result<HeaderValue, MyError>`
/// - Async closures `|client: reqwest::Client| async move { ... }`
pub trait LoginManager: Send + Sync {
	/// Error this login manager returns. Must be convertible to a `Box<dyn
	/// Error + Send + Sync>`.
	type Error: Into<AuthCallbackError>;

	/// This method is called whenever the FHIR client encounters an
	/// UNAUTHORIZED response. Do whatever you need to login and then return the
	/// appropriate header value for the `Authorization`-header, e.g. `Bearer
	/// <my-token>`.
	///
	/// While this method is running, all further UNAUTHORIZED responses trigger
	/// waiting for this instance to finish.
	///
	/// There is no need to implement this as `impl Future`, you can simply
	/// implement it as `async fn` with return type `Result<HeaderValue,
	/// Self::Error>`.
	fn authenticate(
		&mut self,
		client: reqwest::Client,
	) -> impl Future<Output = Result<HeaderValue, Self::Error>> + Send;
}

impl LoginManager for () {
	type Error = AuthCallbackError;

	async fn authenticate(&mut self, _client: reqwest::Client) -> Result<HeaderValue, Self::Error> {
		unreachable!("Somehow called auth_callback of unit type `()`")
	}
}

impl<F, Fut, E> LoginManager for F
where
	F: FnMut(reqwest::Client) -> Fut + Send + Sync,
	Fut: Future<Output = Result<HeaderValue, E>> + Send,
	E: Into<AuthCallbackError>,
{
	type Error = E;

	async fn authenticate(&mut self, client: reqwest::Client) -> Result<HeaderValue, Self::Error> {
		(self)(client).await
	}
}

/// Wrapper trait for [LoginManager] to get rid of the generic `Error` and
/// convert to [AuthCallbackError]. Also has to be object safe (`dyn Trait`).
#[async_trait]
trait LoginManagerConverted: Send + Sync {
	/// Same as [LoginManager::authenticate], but converted error.
	async fn authenticate(
		&mut self,
		client: reqwest::Client,
	) -> Result<HeaderValue, AuthCallbackError>;
}

#[async_trait]
impl<T> LoginManagerConverted for T
where
	T: LoginManager,
{
	async fn authenticate(
		&mut self,
		client: reqwest::Client,
	) -> Result<HeaderValue, AuthCallbackError> {
		LoginManager::authenticate(self, client).await.map_err(Into::into)
	}
}

#[cfg(test)]
mod tests {
	#![allow(dead_code)] // Just check whether it compiles.

	use super::*;

	async fn auth_test_fn(_client: reqwest::Client) -> anyhow::Result<HeaderValue> {
		anyhow::bail!("Test");
	}

	fn construct_test_fn() -> AuthCallback {
		AuthCallback::new(auth_test_fn)
	}

	struct MyLoginManager {
		fail_after: usize,
	}

	impl LoginManager for MyLoginManager {
		type Error = anyhow::Error;

		async fn authenticate(
			&mut self,
			_client: reqwest::Client,
		) -> Result<HeaderValue, Self::Error> {
			if self.fail_after == 0 {
				anyhow::bail!("test");
			} else {
				self.fail_after -= 1;
				Ok(HeaderValue::from_static("whatever"))
			}
		}
	}

	fn construct_test_login_manager() -> AuthCallback {
		AuthCallback::new(MyLoginManager { fail_after: 5 })
	}

	fn construct_test_closure() -> AuthCallback {
		AuthCallback::new(|client: reqwest::Client| async move {
			let _response = client.get("invalid URL").send().await?;
			anyhow::Ok(HeaderValue::from_static("ignored"))
		})
	}
}
