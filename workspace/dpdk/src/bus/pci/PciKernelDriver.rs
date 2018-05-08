// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents a PCI driver.
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub enum PciKernelDriver
{
	#[cfg(any(target_os = "android", target_os = "linux"))] IgbUio,
	#[cfg(any(target_os = "android", target_os = "linux"))] UioPciGeneric,
	#[cfg(any(target_os = "android", target_os = "linux"))] VfioPci,
}

impl PciKernelDriver
{
	#[inline(always)]
	pub(crate) fn all_known_pci_drivers(sys_path: &Path) -> HashSet<Self>
	{
		use self::PciKernelDriver::*;
		
		#[cfg(any(target_os = "android", target_os = "linux"))]
		{
			hashset!
			(
				IgbUio,
				UioPciGeneric,
				UioPciGeneric,
			)
		}
	}
	
	#[inline(always)]
	pub(crate) fn essential_kernel_module(&self) -> Option<EssentialKernelModule>
	{
		use self::PciKernelDriver::*;
		
		match *self
		{
			#[cfg(any(target_os = "android", target_os = "linux"))] IgbUio => EssentialKernelModule::IgbUio,
			#[cfg(any(target_os = "android", target_os = "linux"))] UioPciGeneric => EssentialKernelModule::UioPciGeneric,
			#[cfg(any(target_os = "android", target_os = "linux"))] UioPciGeneric => EssentialKernelModule::VfioPci,
		}
	}
	
	#[inline(always)]
	fn driver_name(&self) -> &'static str
	{
		self.essential_kernel_module().module_name
	}
}
