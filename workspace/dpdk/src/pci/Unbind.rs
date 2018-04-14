// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Assists with unbinding a PCI device on process termination.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Unbind
{
	/// PCI device identifier.
	pub indirect_pci_device_identifier: IndirectPciDeviceIdentifier,
	
	/// PCI device.
	pub pci_device: PciDevice,
	
	dpdk_pci_driver_to_unbind_from: PciDriver,
	bind_back_to_original: Option<PciDriver>,
}

impl Unbind
{
	/// Unbind on termination a pci device using `sys_path` such as `/sys`.
	#[inline(always)]
	pub fn unbind_on_termination(&self, sys_path: &Path)
	{
		if let Some(ref bind_back_to_original) = self.bind_back_to_original
		{
			if bind_back_to_original != &self.dpdk_pci_driver_to_unbind_from
			{
				if let Err(error) = self.dpdk_pci_driver_to_unbind_from.unbind_pci_device(sys_path, &self.pci_device.0)
				{
					warn!("Could not unbind {:?} from DPDK because {:?}", self.pci_device, error);
				}
				else
				{
					if let Err(error) = bind_back_to_original.bind_pci_device(sys_path, &self.pci_device.0)
					{
						warn!("Could not rebind to original {:?} from DPDK because {:?}", self.pci_device, error);
					}
				}
			}
		}
	}
}
