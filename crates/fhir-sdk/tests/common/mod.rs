//! Common test functions.

use tokio::sync::OnceCell;

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
