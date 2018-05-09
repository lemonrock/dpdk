// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// PCI network devices configuration.
#[derive(Debug)]
#[derive(Deserialize)]
#[serde(default)]
pub struct PciNetDevicesConfiguration
{
	/// PCI network devices (ethernet cards).
	pub pci_net_devices: HashMap<IndirectPciDeviceIdentifier, PciKernelDriver>,
}

impl PciNetDeviceConfiguration
{
	#[inline(always)]
	pub(crate) fn add_essential_kernel_modules(&self, essential_kernel_modules: &mut HashSet<EssentialKernelModule>)
	{
		for pci_kernel_driver in self.pci_net_devices.values()
		{
			essential_kernel_modules.insert(*pci_kernel_driver);
		}
	}
	
	#[inline(always)]
	pub(crate) fn take_for_use_with_dpdk(&self, sys_path: &SysPath) -> HashMap<PciDevice, Option<String>>
	{
		let mut aliases = HashMap::with_capacity(self.pci_net_devices.len());
		let mut ethernet_pci_devices = HashSet::with_capacity(self.pci_net_devices.len());
		for (indirect_pci_device_identifier, pci_kernel_driver) in self.pci_net_devices.iter()
		{
			let ethernet_pci_device =
			{
				let pci_device = indirect_pci_device_identifier.to_pci_device();
				assert!(pci_device.is_class_network_ethernet(sys_path), "PCI device '{:?}' for indirect_pci_device_identifier '{:?}' is for not an Ethernet class PCI device", pci_device, indirect_pci_device_identifier);
				
				if let Some((_pci_kernel_driver, alias)) = aliases.insert(pci_device.clone(), (*pci_kernel_driver, indirect_pci_device_identifier.clone()))
				{
					panic!("ethernet_pci_device '{}' is an alias of '{}'", ethernet_pci_device, alias);
				}
				pci_device
			};
		}
		
		let mut originals_to_restore = HashMap::with_capacity(aliases.len());
		for (ethernet_pci_device, (pci_kernel_driver, _alias)) in aliases.drain()
		{
			let original_driver_name = ethernet_pci_device.take_for_use_with_dpdk(sys_path, pci_kernel_driver);
			originals_to_restore.insert(ethernet_pci_device, original_driver_name);
		}
		
		originals_to_restore
	}
	
	#[inline(always)]
	pub(crate) fn release_all_from_use_with_dpdk(sys_path: &SysPath, pci_devices_and_original_driver_names: HashMap<Self, Option<String>>)
	{
		for (pci_device, original_drive_name) in pci_devices_and_original_driver_names.drain()
		{
			pci_device.release_from_use_with_dpdk(sys_path, original_drive_name)
		}
	}
}
