// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// How to find the PCI device.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub enum IndirectPciDeviceIdentifier
{
	/// Identified by PCI device address.
	ByPciDeviceAddress(PciDeviceAddress),
	
	/// Identified by network interface name, eg `eth0`.
	ByNetworkInterfaceName(NetworkInterfaceName),
}

impl IndirectPciDeviceIdentifier
{
	/// Converts to a PCI device.
	///
	/// Can panic.
	///
	/// `sys_path` is a path like `/sys`.
	#[inline(always)]
	pub fn to_pci_device(&self, sys_path: &Path) -> Result<PciDevice, String>
	{
		use self::IndirectPciDeviceIdentifier::*;
		
		let device_address = match *self
		{
			ByPciDeviceAddress(device_address) => device_address,
			
			ByNetworkInterfaceName(ref network_interface_name) =>
			{
				match network_interface_name.pci_device_address().expect("Could not parse PCI device address string")
				{
					Some(pci_device_address) => pci_device_address,
					None => return Err(format!("No valid PCI device for interface '{:?}'", network_interface_name))
				}
			}
		};
		
		PciDevice(device_address)
	}
}
