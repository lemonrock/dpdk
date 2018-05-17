// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern crate docopt;
extern crate dpdk;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;


use ::dpdk::process::*;
use ::docopt::Docopt;
use ::docopt::Error as DocoptError;
use ::docopt::Error::*;
use ::serde_json::from_reader;
use ::std::error::Error;
use ::std::fs::File;
use ::std::path::PathBuf;
use ::std::process::exit;


include!("ALLOCATOR.rs");
include!("authors.rs");
include!("ParsedCommandLineArguments.rs");
include!("print_version.rs");


fn main()
{
	const Name: &'static str = env!("CARGO_PKG_NAME");
	
	const EXIT_SUCCESS: i32 = 0;
	
	const EX_USAGE: i32 = 64;
	
	const EX_SOFTWARE: i32 = 70;
	
	const EX_CONFIG: i32 = 78;
	
	#[inline(always)]
	fn should_be_impossible(reason: &str) -> i32
	{
		eprintln!(reason);
		EX_SOFTWARE
	}
	
	let exit_code = match ParsedCommandLineArguments::parse_command_line_arguments(Name)
	{
		Ok(parsed_command_line_arguments) => match parsed_command_line_arguments.execute()
		{
			Ok(exit_code) => exit_code,
			
			Err(failed_to_load_configuration) =>
			{
				eprintln!("Failed to read configuration ({})", failed_to_load_configuration);
				EX_CONFIG
			}
		},
		
		Err(Version(version)) =>
		{
			print_version(Name, version, "Copyright © 2018 The developers of dpdk.", "License AGPL3: GNU AGPL version 3 <https://www.gnu.org/licenses/agpl.html>.\nThis is free software: you are free to change and redistribute it.\n.There is NO WARRANTY, to the extent permitted by law.");
			EXIT_SUCCESS
		}
		
		Err(WithProgramUsage(error, usage)) =>
		{
			match error.as_ref()
			{
				&Argv(reason) =>
				{
					eprintln!("{}", reason);
					EX_USAGE
				}
				
				&NoMatch =>
				{
					eprintln!("No matching usage for arguments");
					EX_USAGE
				}
				
				&Help =>
				{
					println!("{}", usage);
					EXIT_SUCCESS
				}
				
				_ => should_be_impossible("WithProgramUsage nests something strange"),
			}
		}
		
		Err(Deserialize(reason)) =>
		{
			eprintln!("Could not use arguments because '{}'", reason);
			EX_USAGE
		}
		
		Err(Argv(_)) => should_be_impossible("Docopt Argv errror should always be wrapped in WithProgramUsage"),
		
		Err(NoMatch) => should_be_impossible("Docopt NoMatch errror should always be wrapped in WithProgramUsage"),
		
		Err(Help) => should_be_impossible("Docopt Help errror should always be wrapped in WithProgramUsage"),
		
		Err(Usage(_)) => should_be_impossible("Docopt Usage error should never occur as Docopt::new() was unwrap'd"),
	};
	
	exit(exit_code)
}
