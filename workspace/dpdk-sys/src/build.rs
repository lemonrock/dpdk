// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern crate rust_c;


use ::std::env;
use ::std::process::Command;


fn main()
{
	let cargo_manifest_folder_path = variable("CARGO_MANIFEST_DIR");
	let cargo_out_folder_path = variable("OUT_DIR");
	
	// We deliberately run as much as possible outside of cargo as it makes it far easier to debug a long, complex build which has little to do with Rust.
	// Of course, this script, being shell, won't run under Windows.
	println!("{}", tool(&cargo_manifest_folder_path, "bindgen-wrapper/build-under-cargo"));
	compile_embedded_c_code(&cargo_manifest_folder_path, &cargo_out_folder_path);
}

fn tool(cargo_manifest_folder_path: &str, program_name: &'static str) -> String
{
	let full_path = format!("{}/tools/{}", cargo_manifest_folder_path.to_owned(), program_name.to_owned());
	panic_if_process_not_successful(program_name, cargo_manifest_folder_path, Command::new(full_path))
}

fn panic_if_process_not_successful(program_name: &'static str, cargo_manifest_folder_path: &str, mut command: Command) -> String
{
	let output = command.output().unwrap_or_else(|error|
	{
		panic!("Failed to execute '{}' in '{}' error was '{}'", program_name, cargo_manifest_folder_path, error);
	});
	
	let code = output.status.code().unwrap_or_else(||
	{
		panic!("Failed to retrieve exit status from command - was it killed by a signal?");
	});

	let standard_out = String::from_utf8_lossy(&output.stdout);
	if code == 0
	{
		return standard_out.into_owned();
	}
	
	let standard_error = String::from_utf8_lossy(&output.stderr);
	panic!("Command '{}' failed with exit code '{}' (standard out was '{}'; standard error was '{}')", program_name, code, standard_out.into_owned(), standard_error.into_owned());
}

fn variable(environment_variable_name: &str) -> String
{
	env::var(environment_variable_name).unwrap()
}

fn compile_embedded_c_code(cargo_manifest_folder_path: &str, cargo_out_folder_path: &str)
{
	if env::var("CROSS_COMPILE").is_err()
	{
		panic!("Please specify CROSS_COMPILE=x86_64-linux-musl- cargo build --target=x86_64-unknown-linux-musl as the 'cc' (formerly 'gcc') crate incorrectly looks for musl-gcc")
	}
	
	let path = format!("{}/src/lib.rs", cargo_manifest_folder_path);
	rust_c::build(path, "dpdk_sys_c", |gcc_config|
	{
		gcc_config.flag("-Werror");
		gcc_config.define("_GNU_SOURCE", None);
		gcc_config.define("_BSD_SOURCE", None);
		gcc_config.flag(&format!("-isystem{}", format!("{}/root/extra-musl-headers", cargo_out_folder_path.to_owned()))); // can't use .include() as warnings then occur in system headers
		gcc_config.flag(&format!("-isystem{}", format!("{}/root/DESTDIR/usr/include/dpdk", cargo_out_folder_path.to_owned()))); // can't use .include() as warnings then occur in system headers
		gcc_config.flag("-msse4.2");
		gcc_config.opt_level(3); // DPDK code only compiles with optimisation enabled; we can't inherit OPT_LEVEL from the environment
	});
}
