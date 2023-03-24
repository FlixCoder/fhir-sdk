//! Run code generation.

use anyhow::Result;

fn main() -> Result<()> {
	generate::generate_code("r4b")?;

	Ok(())
}
