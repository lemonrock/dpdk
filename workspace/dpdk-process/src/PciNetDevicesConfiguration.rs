// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// PCI network devices configuration.
#[derive(Default, Debug)]
#[derive(Deserialize)]
#[serde(default)]
pub struct PciNetDevicesConfiguration
{
	/// PCI network devices (ethernet cards) to PCI kernel drivers and NUMA node fix (if necessary).
	pub pci_net_devices: HashMap<IndirectPciDeviceIdentifier, (PciKernelDriver, Option<u8>)>,
}

impl PciNetDevicesConfiguration
{
	#[inline(always)]
	pub(crate) fn uses_ugb_uio_or_pci_vfio(&self) -> (bool, bool)
	{
		use self::PciKernelDriver::*;
		
		let mut uses_ugb_uio = false;
		let mut uses_pci_vfio = false;
		for pci_kernel_driver in self.pci_kernel_drivers()
		{
			match pci_kernel_driver
			{
				#[cfg(target_os = "linux")] IgbUio =>
				{
					uses_ugb_uio = true
				}
				
				#[cfg(target_os = "linux")] VfioPci =>
				{
					uses_pci_vfio = true
				}
				
				_ => (),
			}
		}
		
		(uses_ugb_uio, uses_pci_vfio)
	}
	
	#[inline(always)]
	pub(crate) fn add_essential_kernel_modules(&self, essential_kernel_modules: &mut HashSet<EssentialKernelModule>)
	{
		for pci_kernel_driver in self.pci_kernel_drivers()
		{
			match pci_kernel_driver
			{
				#[cfg(target_os = "linux")] IgbUio =>
				{
					essential_kernel_modules.insert(EssentialKernelModule::IgbUio);
				}
				
				#[cfg(target_os = "linux")] UioPciGeneric =>
				{
					essential_kernel_modules.insert(EssentialKernelModule::UioPciGeneric);
				}
				
				#[cfg(target_os = "linux")] VfioPci =>
				{
					essential_kernel_modules.insert(EssentialKernelModule::VfioPci);
				}
				
				_ => (),
			}
		}
	}
	
	#[inline(always)]
	pub(crate) fn take_for_use_with_dpdk(&self, sys_path: &SysPath) -> HashMap<PciDevice, Option<String>>
	{
		let is_a_numa_machine = sys_path.is_a_numa_machine();
		
		let mut aliases = HashMap::with_capacity(self.pci_net_devices.len());
		let mut ethernet_pci_devices = HashSet::with_capacity(self.pci_net_devices.len());
		for (indirect_pci_device_identifier, (pci_kernel_driver, numa_node_fix)) in self.pci_net_devices.iter()
		{
			let ethernet_pci_device =
			{
				let pci_device = indirect_pci_device_identifier.to_pci_device();
				assert!(pci_device.is_class_network_ethernet(sys_path), "PCI device '{:?}' for indirect_pci_device_identifier '{:?}' is for not an Ethernet class PCI device", pci_device, indirect_pci_device_identifier);
				
				if let Some((_pci_kernel_driver, alias)) = aliases.insert(pci_device.clone(), (*pci_kernel_driver, indirect_pci_device_identifier.clone()))
				{
					panic!("pci_device '{:?}' is an alias of '{:?}'", pci_device, alias);
				}
				
				if is_a_numa_machine
				{
					if let Some(numa_node_fix) = numa_node_fix
					{
						pci_device.set_numa_node_swallowing_errors_as_this_is_brittle(sys_path, *numa_node_fix);
					}
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
	pub(crate) fn release_all_from_use_with_dpdk(sys_path: &SysPath, pci_devices_and_original_driver_names: HashMap<PciDevice, Option<String>>)
	{
		for (pci_device, original_drive_name) in pci_devices_and_original_driver_names.drain()
		{
			pci_device.release_from_use_with_dpdk(sys_path, original_drive_name)
		}
	}
	
	#[inline(always)]
	fn pci_kernel_drivers(&self) -> impl Iterator<Item=&PciKernelDriver>
	{
		self.pci_net_devices.values().map(|(ref pci_kernel_driver, ref _numa_node_fix)| pci_kernel_driver)
	}
}
