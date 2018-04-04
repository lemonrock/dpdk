// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait ProgramArguments : Debug + Decodable
{
	const Name: &'static str = env!("CARGO_PKG_NAME");
	const Description: &'static str = env!("CARGO_PKG_DESCRIPTION");
	const Authors: &'static str = env!("CARGO_PKG_AUTHORS");
	const HomePage: &'static str = env!("CARGO_PKG_HOMEPAGE");
	const Version: &'static str = env!("CARGO_PKG_VERSION");
	
	#[inline(always)]
	fn isQuiet(&self) -> bool;
	
	#[inline(always)]
	fn verbosity(&self) -> usize;
	
	fn usage() -> String;
	
	fn authors() -> String
	{
		let mut splitter = Self::Authors.split(':').peekable();
		
		let mut authors = String::with_capacity(Self::Authors.len() * 2);
		
		authors.push_str(splitter.next().unwrap());
		loop
		{
			let author = match splitter.next()
			{
				None => break,
				Some(author) => author,
			};
			
			let isLast = splitter.peek().is_none();
			let push = if isLast
			{
				" and "
			}
			else
			{
				", "
			};
			authors.push_str(push);
			authors.push_str(author);
		}
		
		authors
	}

	fn parseThenDisplayHelpOrVersionAndExitIfSoRequestedThenConfigureLogging() -> Self
	{
		let programArguments = Self::parseThenDisplayHelpOrVersionAndExitIfSoRequested();
		programArguments.configureStandardErrorLogging();
		info!("Starting");
		programArguments
	}
	
	fn parseThenDisplayHelpOrVersionAndExitIfSoRequested() -> Self
	{
		match Docopt::new(Self::usage())
		{
			Err(error) => match error
			{
				Usage(value) => panic!("Incorrect usage string specifed to Docopt: '{}'", value),
				_ => panic!("Unexpected Docopt error '{}'", error),
			},
			Ok(docopt) =>
			{
				docopt.version(Some(Self::Version.to_owned()))
			}
		}
		.decode().unwrap_or_else(|error| error.exit())
	}
	
	fn configureStandardErrorLogging(&self)
	{
		StandardErrorAnsiLog::initialise(self.isQuiet(), self.verbosity()).expect("Could not configure logging to standard error");
	}
}
