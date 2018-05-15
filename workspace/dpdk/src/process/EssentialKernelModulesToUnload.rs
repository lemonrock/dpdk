// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct EssentialKernelModulesToUnload(Vec<&'static str>);

impl EssentialKernelModulesToUnload
{
	#[inline(always)]
	pub(crate) fn new() -> Self
	{
		EssentialKernelModulesToUnload(Vec::with_capacity(5))
	}
	
	#[inline(always)]
	pub(crate) fn add_to_list_of_those_to_unload(&mut self, essential_kernel_module: &EssentialKernelModule)
	{
		self.0.insert(essential_kernel_module.module_name);
	}
	
	#[inline(always)]
	pub(crate) fn unload_kernel_modules(&self)
	{
		for module_name in self.0.iter().rev()
		{
			#[cfg(target_os = "linux")]
			{
				if LinuxKernelModulesList::unload_linux_kernel_module(module_name).is_err()
				{
					continue
				}
			}
		}
	}
}
