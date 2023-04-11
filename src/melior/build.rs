use anyhow::{bail, Context as _, Result};
use std::env;
use std::error::Error;
use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;
use std::str;

fn main() -> Result<()> {
	println!("cargo:rerun-if-changed=wrapper.h");
	println!("cargo:rustc-link-search={}", llvm_config("--libdir")?);

	for name in fs::read_dir(llvm_config("--libdir")?)?
		.map(|entry| {
			Ok(if let Some(name) = entry?.path().file_name() {
				name.to_str().map(String::from)
			} else {
				None
			})
		})
		.collect::<Result<Vec<_>, io::Error>>()?
		.into_iter()
		.flatten()
	{
		if name.starts_with("libMLIR")
			&& name.ends_with(".a")
			&& !name.contains("Main")
			&& name != "libMLIRSupportIndentedOstream.a"
		{
			if let Some(name) = trim_library_name(&name) {
				println!("cargo:rustc-link-lib=static={}", name);
			}
		}
	}

	for name in llvm_config("--libnames")?.trim().split(' ') {
		if let Some(name) = trim_library_name(name) {
			println!("cargo:rustc-link-lib={}", name);
		}
	}

	for flag in llvm_config("--system-libs")?.trim().split(' ') {
		let flag = flag.trim_start_matches("-l");

		if flag.starts_with('/') {
			// llvm-config returns absolute paths for dynamically linked libraries.
			let path = Path::new(flag);

			println!(
				"cargo:rustc-link-search={}",
				path.parent().context("invalid library path")?.display()
			);
			println!(
				"cargo:rustc-link-lib={}",
				path.file_name()
					.context("invalid library path")?
					.to_str()
					.context("invalid library path")?
					.split_once('.')
					.context("invalid library path")?
					.0
					.trim_start_matches("lib")
			);
		} else {
			println!("cargo:rustc-link-lib={}", flag);
		}
	}

	if let Some(name) = get_system_libcpp() {
		println!("cargo:rustc-link-lib={}", name);
	}

	bindgen::builder()
		.header("wrapper.h")
		.clang_arg(format!("-I{}", llvm_config("--includedir")?))
		.parse_callbacks(Box::new(bindgen::CargoCallbacks))
		.generate()
		.context("unable to generate bindings")?
		.write_to_file(Path::new(&env::var("OUT_DIR")?).join("bindings.rs"))?;

	Ok(())
}

fn get_system_libcpp() -> Option<&'static str> {
	if cfg!(target_env = "msvc") {
		None
	} else if cfg!(target_os = "macos") {
		Some("c++")
	} else {
		Some("stdc++")
	}
}

fn llvm_config(argument: &str) -> Result<String> {
	let prefix = env::var("MLIR_SYS_PREFIX")
		.map(|prefix| Path::new(&prefix).join("bin"))
		.unwrap_or_else(|_| Path::new("/usr/local/bin").to_path_buf());

	println!("Running {:?}", prefix.join("llvm-config"));

	let output = Command::new(prefix.join("llvm-config"))
		.args(&["--link-static", argument])
		.output()
		.context("unable to run llvm-config")?
		.stdout;

	Ok(str::from_utf8(&output)?.trim().to_string())
}

fn trim_library_name(name: &str) -> Option<&str> {
	if let Some(name) = name.strip_prefix("lib") {
		name.strip_suffix(".a")
	} else {
		None
	}
}
