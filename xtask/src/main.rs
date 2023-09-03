//! XTask CLI tool.
#![allow(clippy::expect_used, clippy::print_stdout)]

use std::{env, ffi::OsStr, path::PathBuf, process::Command, str::FromStr};

use clap::Parser;
use color_eyre::{eyre::bail, Result};

/// XTask pattern helper CLI tool.
#[derive(Debug, Parser)]
#[command(about)]
enum Cli {
	/// Test command: run all tests with different feature selections.
	Test,
}

impl Cli {
	/// Run the CLI.
	pub fn run(self) -> Result<()> {
		match self {
			Self::Test => Self::run_test()?,
		}
		Ok(())
	}

	/// Run the test sub-command.
	fn run_test() -> Result<()> {
		let workspace_path = PathBuf::from_str(&env::var("CARGO_WORKSPACE_DIR")?)?;

		// Run all tests and doc-tests with default features.
		let mut command = Command::new("cargo");
		command.args(["test", "--workspace"]).current_dir(&workspace_path);
		run_command(command)?;

		// Run R4 tests.
		let mut command = Command::new("cargo");
		command
			.args([
				"test",
				"--workspace",
				"--no-default-features",
				"--features",
				"r4b,builders,client",
			])
			.current_dir(&workspace_path);
		run_command(command)?;

		// Make sure the models compile without builders in all FHIR versions.
		let mut command = Command::new("cargo");
		command.args(["check", "-p", "fhir-model", "--no-default-features", "--features", "r4b"]);
		run_command(command)?;
		let mut command = Command::new("cargo");
		command.args(["check", "-p", "fhir-model", "--no-default-features", "--features", "r5"]);
		run_command(command)?;

		Ok(())
	}
}

/// Print out the command being run and then run it, checking its exit code
/// afterwards.
fn run_command(mut command: Command) -> Result<()> {
	println!(
		"Running {} {}",
		command.get_program().to_string_lossy(),
		command.get_args().map(OsStr::to_string_lossy).collect::<Vec<_>>().join(" ")
	);

	let exit_code = command.spawn()?.wait()?;
	if !exit_code.success() {
		bail!("Command failed!");
	}

	Ok(())
}

fn main() -> Result<()> {
	color_eyre::install()?;
	let cli = Cli::parse();
	cli.run()?;
	Ok(())
}
