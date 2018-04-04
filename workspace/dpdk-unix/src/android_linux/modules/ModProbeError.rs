// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


quick_error!
{
	#[derive(Debug)]
	pub enum ModProbeError
	{
		IoError(cause: Error)
		{
			cause(cause)
			from()
		}
		
		SignalTerminatedExitCode(moduleName: String)
		{
			display("modprobe return a signal terminated exit for module '{}'", moduleName)
		}
		
		NonZeroExitCode(exitCode: i32, moduleName: String)
		{
			display("modprobe returned a non-zero exit code of '{}' for module '{}'", exitCode, moduleName)
		}
	}
}
