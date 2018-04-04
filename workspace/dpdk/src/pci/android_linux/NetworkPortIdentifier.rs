// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub enum NetworkPortIdentifier
{
	ByPciDeviceAddress(DeviceAddress),
	ByNetworkInterfaceName(NetworkInterfaceName),
}

impl NetworkPortIdentifier
{
	pub fn toPciDevice(&self, sysPath: &Path) -> PciDevice
	{
		let deviceAddress = match *self
		{
			NetworkPortIdentifier::ByPciDeviceAddress(deviceAddress) => deviceAddress.clone(),
			NetworkPortIdentifier::ByNetworkInterfaceName(ref networkInterfaceName) => networkInterfaceName.pciDeviceAddress().expect("Could not parse PCI device address string").expect(&format!("No valid PCI device for interface '{:?}'", networkInterfaceName))
		};
		
		let pciDevice = PciDevice(deviceAddress);
		
		assert!(pciDevice.isClassNetworkEthernet(sysPath), "PCI device '{:?}' for networkPortIdentifier '{:?}' is for not an Ethernet class PCI device (or does not exist at all)", pciDevice, self);
		
		pciDevice
	}
}
