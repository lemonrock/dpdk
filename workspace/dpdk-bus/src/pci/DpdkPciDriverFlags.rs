// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


bitflags!
{
	/// DPDK PCI driver flags.
	pub struct DpdkPciDriverFlags: u32
	{
		/// Needs base address register mapping.
		const NeedsBaseAddressRegisterMapping = RTE_PCI_DRV_NEED_MAPPING;
		
		/// Supports link status interrupt.
		const SupportsLinkStatusInterrupt = RTE_PCI_DRV_INTR_LSC;
		
		/// Supports device removal interrupt.
		const SupportsDeviceRemovalInterrupt = RTE_PCI_DRV_INTR_RMV;
		
		/// ?
		const DeviceDriverNeedsToKeepMappedResourcesIfUnsupportedDeviceDetected = RTE_PCI_DRV_KEEP_MAPPED_RES;
		
		/// IOVA as virtually addressed ('VA').
		///
		/// DPDK constant is `RTE_PCI_DRV_IOVA_AS_VA`.
		const SupportsIOVirtualAddressAsVirtualAddress = 0x0040;
	}
}

impl Default for DpdkPciDriverFlags
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::empty()
	}
}
