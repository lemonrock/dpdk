// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Kernel modules, previously loaded, that should be unloaded at application exit to make execution idempotent.
#[derive(Debug)]
pub struct EssentialKernelModulesToUnload(Vec<&'static [u8]>);

impl EssentialKernelModulesToUnload
{
	/// Create a new instance.
	#[inline(always)]
	pub fn new() -> Self
	{
		EssentialKernelModulesToUnload(Vec::with_capacity(5))
	}
	
	#[inline(always)]
	pub(crate) fn add_to_list_of_those_to_unload(&mut self, essential_kernel_module: &EssentialKernelModule)
	{
		self.0.push(essential_kernel_module.module_name);
	}
	
	/// Unload.
	#[inline(always)]
	pub fn unload_kernel_modules(&self)
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
