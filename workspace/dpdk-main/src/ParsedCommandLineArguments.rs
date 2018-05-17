// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug)]
#[derive(Deserialize)]
#[serde(default)]
struct ParsedCommandLineArguments
{
	flag_foreground: bool,
	flag_configuration: PathBuf,
}

impl Default for ParsedCommandLineArguments
{
	fn default() -> Self
	{
		Self
		{
			flag_foreground: false,
			flag_configuration: PathBuf::from(format!("/etc/{}.json", Name))
		}
	}
}

impl ParsedCommandLineArguments
{
	pub(crate) fn execute(self) -> Result<i32, Box<Error>>
	{
		let file = File::open(self.flag_configuration)?;
		
		let mut master_loop_configuration: MasterLoopConfiguration = from_reader(file)?;
		
		if self.flag_foreground
		{
			master_loop_configuration.daemonize = None;
		}
		
		let master_loop = MasterLoop::new();
		let exit_code = master_loop.execute(&master_loop_configuration, &ALLOCATOR);
		
		Ok(exit_code)
	}
	
	pub(crate) fn parse_command_line_arguments(name: &str) -> Result<Self, DocoptError>
	{
		Docopt::new
		(
			format!
			(
"
{0}.

Usage:
  {0} [--foreground] --configuration=</path/to/configuration.json>
  {0} (-h | --help)
  {0} --version

Options:
  --foreground  Run in the foreground, not as a daemon; overrides daemon setting in --configuration.
  --configuration=</path/to/configuration.json>  Path to configuration [default: /etc/{0}.json].
  -h --help     Show this help information.
  --version     Show version.
",
				name
			)
		).expect("Programming error: did not write a valid docopt usage string").options_first(true).help(true).version(Some(env!("CARGO_PKG_VERSION").to_owned())).deserialize::<Self>()
	}
}
