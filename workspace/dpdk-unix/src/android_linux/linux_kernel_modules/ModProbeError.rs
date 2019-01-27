// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Cause of error when modprobe fails.
#[derive(Debug)]
pub enum ModProbeError
{
	/// Could not open file list of Linux kernel linux_kernel_modules.
	InputOutputError(io::Error),

	/// A module name was empty.
	SignalTerminatedExitCode
	{
		/// The Linux kernel module name (not necessarily UTF-8).
		linux_kernel_module_name: Box<[u8]>,
	},

	/// A module name was duplicated.
	NonZeroExitCode
	{
		/// The Linux kernel module name (not necessarily UTF-8).
		linux_kernel_module_name: Box<[u8]>,

		/// Exit code.
		exit_code: i32,
	},
}

impl Display for ModProbeError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		<ModProbeError as Debug>::fmt(self, f)
	}
}

impl error::Error for ModProbeError
{
	#[inline(always)]
	fn source(&self) ->  Option<&(error::Error + 'static)>
	{
		use self::ModProbeError::*;

		match self
		{
			&InputOutputError(ref error) => Some(error),

			&SignalTerminatedExitCode { .. } => None,

			&NonZeroExitCode { .. } => None,
		}
	}
}

impl From<io::Error> for ModProbeError
{
	#[inline(always)]
	fn from(error: io::Error) -> Self
	{
		ModProbeError::InputOutputError(error)
	}
}
