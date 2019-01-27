// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Executes Linux modprobe.
#[inline(always)]
pub fn modprobe(linux_kernel_module_name: &[u8]) -> Result<(), ModProbeError>
{
	#[inline(always)]
	fn new_command_in_clean_environment(command: &str) -> Command
	{
		let mut command = Command::new(command);
		command.stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null()).env_clear();
		if let Some(path) = var_os("PATH")
		{
			command.env("PATH", path);
		}
		command
	}
	
	assert!(!linux_kernel_module_name.starts_with(b"-"), "linux_kernel_module_name '{:?}' starts with a hyphen. This confuses some modprobe implementations (and some don't support -- parsing it seems)", linux_kernel_module_name);
	assert_effective_user_id_is_root(&format!("modprobe of '{:?}'", linux_kernel_module_name));

	let exit_code = new_command_in_clean_environment("modprobe").arg("-s").arg("-b").arg(OsStr::from_bytes(linux_kernel_module_name)).status()?;
	match exit_code.code()
	{
		None => Err(ModProbeError::SignalTerminatedExitCode { linux_kernel_module_name: linux_kernel_module_name.to_vec().into_boxed_slice() }),
		Some(exit_code) => match exit_code
		{
			0 => Ok(()),
			_ => Err(ModProbeError::NonZeroExitCode { linux_kernel_module_name: linux_kernel_module_name.to_vec().into_boxed_slice(), exit_code }),
		}
	}
}
