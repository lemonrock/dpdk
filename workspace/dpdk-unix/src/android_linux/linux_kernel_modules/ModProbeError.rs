// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


quick_error!
{
	/// Cause of error when modprobe fails.
	#[derive(Debug)]
	pub enum ModProbeError
	{
		/// Input / Output error.
		InputOutputError(cause: Error)
		{
			cause(cause)
			from()
		}
		
		/// Signal-terminated modprobe.
		SignalTerminatedExitCode(linux_kernel_module_name: String)
		{
			display("modprobe return a signal terminated exit for module '{}'", linux_kernel_module_name)
		}
		
		/// Non-zero exit code.
		NonZeroExitCode(exit_code: i32, linux_kernel_module_name: String)
		{
			display("modprobe returned a non-zero exit code of '{}' for module '{}'", exit_code, linux_kernel_module_name)
		}
	}
}
