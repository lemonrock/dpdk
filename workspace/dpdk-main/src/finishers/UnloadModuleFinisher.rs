// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnloadModuleFinisher
{
	pub moduleName: &'static str,
}

impl Finisher for UnloadModuleFinisher
{
	#[allow(unused_variables)]
	fn finish(&self, sys_path: &Path)
	{
		if let Err(error) = LinuxKernelModulesList::unload_linux_kernel_module(self.moduleName)
		{
			warn!("Could not unload module we loaded called '{}' because '{:?}'", self.moduleName, error);
		}
	}
}

impl UnloadModuleFinisher
{
	pub fn ifWasLoaded(wasLoaded: bool, finishers: &mut Finishers, moduleName: &'static str)
	{
		if wasLoaded
		{
			finishers.push(Box::new(UnloadModuleFinisher
			{
				moduleName,
			}))
		}
	}
}
