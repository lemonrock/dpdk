// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents a PCI driver.
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub enum PciKernelDriver
{
	#[cfg(target_os = "linux")] IgbUio,
	#[cfg(target_os = "linux")] UioPciGeneric,
	#[cfg(target_os = "linux")] VfioPci,
}

impl PciKernelDriver
{
	/// Essential kernel module.
	#[inline(always)]
	pub fn essential_kernel_module(&self) -> EssentialKernelModule
	{
		use self::PciKernelDriver::*;
		
		match *self
		{
			#[cfg(target_os = "linux")] IgbUio => EssentialKernelModule::IgbUio,
			#[cfg(target_os = "linux")] UioPciGeneric => EssentialKernelModule::UioPciGeneric,
			#[cfg(target_os = "linux")] VfioPci => EssentialKernelModule::VfioPci,
		}
	}
	
	/// Kernel module name.
	#[inline(always)]
	pub fn driver_name(&self) -> &'static str
	{
		self.essential_kernel_module().module_name()
	}
}
