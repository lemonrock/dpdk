// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


quick_error!
{
	/// Cause of error when trying to parse list of Linux linux_kernel_modules.
	#[derive(Debug)]
	pub enum LinuxKernelModulesListParseError
	{
		/// Could not open file list of Linux kernel linux_kernel_modules.
		CouldNotOpenFile(cause: Error)
		{
			cause(cause)
			from()
		}
		
		/// A module name was empty.
		CouldNotParseEmptyModuleName(zero_based_line_number: usize)
		{
			display("Zero-based line number '{}' could not be parsed as a module name because it is empty", zero_based_line_number)
		}
		
		/// A module name was duplicated.
		DuplicateModuleName(zero_based_line_number: usize, linux_kernel_module_name: String)
		{
			display("Zero-based line number '{}' module name '{:?}' was duplicated", zero_based_line_number, linux_kernel_module_name)
		}
	}
}
