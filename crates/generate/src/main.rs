//! Run code generation.
#![allow(clippy::print_stdout)]

use anyhow::Result;

fn main() -> Result<()> {
	println!("Generating R4B models..");
	generate::generate_code("r4b")?;
	println!("Generating R5 models..");
	generate::generate_code("r5")?;

	println!("Done.");
	Ok(())
}
