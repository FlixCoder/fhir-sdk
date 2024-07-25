//! Common test functions.

use std::sync::LazyLock;

use tokio::{runtime::Runtime, sync::OnceCell};

/// Test-global runtime to be stable across multiple tests using the same client
/// and runtime.
#[allow(clippy::incompatible_msrv)] // Just the test, don't want to block/wwarn production..
pub static RUNTIME: LazyLock<Runtime> = LazyLock::new(|| {
	tokio::runtime::Builder::new_multi_thread().enable_all().build().expect("creating runtime")
});

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
