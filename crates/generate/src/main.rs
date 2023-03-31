//! Run code generation.

use anyhow::Result;

fn main() -> Result<()> {
	generate::generate_code("r4b")?;
	generate::generate_code("r5")?;

	Ok(())
}
