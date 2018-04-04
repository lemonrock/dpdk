// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct NetworkInterfacesConfiguration
{
	pciDevices: HashMap<NetworkPortIdentifier, DeviceConfiguration<PciDriver>>,
	
	afPacketNetVirtualDevices: Vec<DeviceConfiguration<AfPacketNetVirtualDevice>>,
	packetCaptureNetVirtualDevices: Vec<DeviceConfiguration<PacketCaptureNetVirtualDevice>>,
	virtIoForContainersNetVirtualDevices: Vec<DeviceConfiguration<VirtIoForContainersNetVirtualDevice>>,
	virtualHostNetVirtualDevices: Vec<DeviceConfiguration<VirtualHostNetVirtualDevice>>,
	xenNetVirtualDevices: Vec<DeviceConfiguration<XenNetVirtualDevice>>,
	bondingNetVirtualDevices: Vec<DeviceConfiguration<BondingNetVirtualDevice>>,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct DeviceConfiguration<D>
{
	pub device: D,
	pub settings: EthernetPortConfiguration,
}

impl Default for NetworkInterfacesConfiguration
{
	fn default() -> Self
	{
		NetworkInterfacesConfiguration
		{
			pciDevices: HashMap::new(),
			
			afPacketNetVirtualDevices: Vec::new(),
			packetCaptureNetVirtualDevices: Vec::new(),
			virtIoForContainersNetVirtualDevices: Vec::new(),
			virtualHostNetVirtualDevices: Vec::new(),
			xenNetVirtualDevices: Vec::new(),
			
			bondingNetVirtualDevices: Vec::new(),
		}
	}
}

impl NetworkInterfacesConfiguration
{
	pub fn usesPciDriver(&self, usePciDriver: PciDriver) -> bool
	{
		for ref deviceConfiguration in self.pciDevices.values()
		{
			if &deviceConfiguration.device == &usePciDriver
			{
				return true;
			}
		}
		false
	}
	
	// aka 'rte_kni'; not implemented used a PMD
	pub fn hasKernelNativeInterfaceDevices(&self) -> bool
	{
		false
	}
	
	pub fn hasXenVirtualDevices(&self) -> bool
	{
		if self.xenNetVirtualDevices.is_empty()
		{
			false
		}
		else
		{
			warn!("Xen drivers are not supported at this time using static linking. Be warned.");
			true
		}
	}
	
	pub fn addTo(&self, dpdkRteInitData: &mut DpdkRteInitData, sysPath: &Path) -> (Vec<Unbind>, EthernetPortConfigurations)
	{
		let mut configurations = HashMap::new();
		
		let expected = self.pciDevices.iter().map(|(key, value)| { (key.clone(), value.device.clone())}).collect();
		let unbinds = PciDevice::bindDevices(sysPath, expected);
		for unbind in &unbinds
		{
			dpdkRteInitData.addPciDevice(unbind.pciDevice.0);
			
			let settings = self.pciDevices.get(&unbind.networkPortIdentifier).unwrap().settings.clone();
			configurations.insert(unbind.pciDevice.0.to_string(), settings);
		}
		
		for ref deviceConfiguration in &self.afPacketNetVirtualDevices
		{
			dpdkRteInitData.addAfPacketNetVirtualDevice(deviceConfiguration.device.clone());
			configurations.insert(deviceConfiguration.device.name().to_string(), deviceConfiguration.settings.clone());
		}
		
		for ref deviceConfiguration in &self.packetCaptureNetVirtualDevices
		{
			dpdkRteInitData.addPacketCaptureNetVirtualDevice(deviceConfiguration.device.clone());
			configurations.insert(deviceConfiguration.device.name().to_string(), deviceConfiguration.settings.clone());
		}
		
		for ref deviceConfiguration in &self.virtIoForContainersNetVirtualDevices
		{
			dpdkRteInitData.addVirtIoForContainersNetVirtualDevice(deviceConfiguration.device.clone());
			configurations.insert(deviceConfiguration.device.name().to_string(), deviceConfiguration.settings.clone());
		}
		
		for ref deviceConfiguration in &self.virtualHostNetVirtualDevices
		{
			dpdkRteInitData.addVirtualHostNetVirtualDevice(deviceConfiguration.device.clone());
			configurations.insert(deviceConfiguration.device.name().to_string(), deviceConfiguration.settings.clone());
		}
		
		for ref deviceConfiguration in &self.xenNetVirtualDevices
		{
			dpdkRteInitData.addXenNetVirtualDevice(deviceConfiguration.device.clone());
			configurations.insert(deviceConfiguration.device.name().to_string(), deviceConfiguration.settings.clone());
		}
		
		for ref deviceConfiguration in &self.bondingNetVirtualDevices
		{
			dpdkRteInitData.addBondingNetVirtualDevice(deviceConfiguration.device.clone());
			configurations.insert(deviceConfiguration.device.name().to_string(), deviceConfiguration.settings.clone());
		}
		
		(unbinds, EthernetPortConfigurations(configurations))
	}
}
