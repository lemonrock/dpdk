// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


// Used solely to support docopt
#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct SampleConfigurationAndProgramArguments
{
	flag_q: bool,
	flag_v: usize,
	flag_c: Option<String>,
	flag_d: Option<String>,
}

impl ProgramArguments for SampleConfigurationAndProgramArguments
{
	#[inline(always)]
	fn usage() -> String
	{
		format!
		("
{0}, {1}.
Written by {2}.
More information at `{3}`.

Usage:
  {0} -h | --help
  {0} -V | --version
  {0} [-q] [-v...] [options]

Options:
  -h, --help                              Show this screen and exit.
  -V, --version                           Show version and exit.
  -q                                      Be quiet.
  -v                                      Be verbose. Specify more than once for additional verbosity.
  -c <file>, --configuration-file <file>  Path to .json configuration file. Must exist.
  -d <file>, --dump-file <file>           Path to dump configuration as .json file. Will be overwritten if it exists.
", Self::Name.to_owned(), Self::Description.to_owned(), Self::authors(), Self::HomePage.to_owned())
	}
	
	#[inline(always)]
	fn isQuiet(&self) -> bool
	{
		self.flag_q
	}
	
	#[inline(always)]
	fn verbosity(&self) -> usize
	{
		self.flag_v
	}
}

impl ConfigurationAndProgramArguments for SampleConfigurationAndProgramArguments
{
	fn configurationAsModifiedByCommandLine(&self) -> Configuration
	{
		let configuration = self.configuration();
		
		fn dumpConfiguration(d: &str, configuration: &Configuration)
		{
			let mut file = match File::create(d)
			{
				Err(error) => panic!("--dump-file {} could not be created because '{:?}'", d, error),
				Ok(file) => file,
			};
			
			match ::serde_json::to_writer_pretty(&mut file, configuration)
			{
				Ok(_) => (),
				Err(message) => panic!("--dump-file {} could not dump JSON configuration created because '{:?}'", d, message),
			}
		}
		
		if let Some(ref d) = self.flag_d
		{
			dumpConfiguration(d, &configuration);
		}
		
		configuration
	}
}

impl SampleConfigurationAndProgramArguments
{
	fn configuration(&self) -> Configuration
	{
		if let Some(ref c) = self.flag_c
		{
			let configurationFile = PathBuf::from(&c);
			if !configurationFile.exists()
			{
				panic!("--configuration-file {} does not exist (or permissions deny discovery)", c);
			}
			if !configurationFile.is_file()
			{
				panic!("--configuration-file {} is not a file (or permissions deny discovery)", c);
			}
			
			let mut file = match File::open(configurationFile)
			{
				Ok(file) => file,
				Err(error) => panic!("--configuration-file {} could not be opened because '{:?}'", c, error),
			};
			
			match ::serde_json::from_reader(&mut file)
			{
				Ok(configuration) => configuration,
				Err(error) => panic!("--configuration-file {} could not be read as configuration JSON because '{:?}'", c, error),
			}
		}
		else
		{
			Configuration::default()
		}
	}
}
