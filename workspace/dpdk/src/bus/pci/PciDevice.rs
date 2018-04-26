// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Models a PCI device.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PciDevice(DpdkPciDeviceAddress);

impl PciDevice
{
	/// Bind all PCI devices.
	///
	/// `sys_path` is a path like `/sys`.
	pub fn bind_all_devices(sys_path: &Path, indirect_pci_device_identifiers: HashMap<IndirectPciDeviceIdentifier, PciKernelDriver>) -> Vec<Unbind>
	{
		let all_known_pci_drivers = PciKernelDriver::all_known_pci_drivers(sys_path);
		let mut converted = Self::indirect_pci_device_identifiers_to_pci_devices(sys_path, indirect_pci_device_identifiers);
		let mut unbind_list = Vec::with_capacity(converted.len());

		for (indirect_pci_device_identifier, (pci_device, dpdk_pci_driver_to_bind_to)) in converted.drain()
		{
			let bind_back_to_original = pci_device.ensure_bound_to_dpdk_driver(sys_path, &all_known_pci_drivers, &dpdk_pci_driver_to_bind_to);
			let unbind = Unbind
			{
				indirect_pci_device_identifier,
				pci_device,
				dpdk_pci_driver_to_unbind_from: dpdk_pci_driver_to_bind_to,
				bind_back_to_original,
			};

			unbind_list.push(unbind);
		}

		unbind_list
	}

	/// All PCI devices.
	///
	/// `sys_path` is a path like `/sys`.
	pub(crate) fn all_pci_devices(sys_path: &Path) -> HashSet<PciDevice>
	{
		let mut results = HashSet::with_capacity(64);

		let devices_path = Self::devices_path(sys_path);

		if let Ok(iterator) = devices_path.read_dir()
		{
			for entry in iterator
			{
				if let Ok(entry) = entry
				{
					if let Ok(fileName) = entry.file_name().into_string()
					{
						if let Ok(device_address) = DeviceAddress::from_str(&fileName)
						{
							let value = PciDevice(device_address);
							assert!(results.insert(value), "Duplicate in a read_dir() ? how ?");
						}
					}
				}
			}
		}

		results.shrink_to_fit();
		results
	}
	
	fn indirect_pci_device_identifiers_to_pci_devices(sys_path: &Path, mut indirect_pci_device_identifiers: HashMap<IndirectPciDeviceIdentifier, PciKernelDriver>) -> HashMap<IndirectPciDeviceIdentifier, (PciDevice, PciKernelDriver)>
	{
		let length = indirect_pci_device_identifiers.len();
		let mut result = HashMap::with_capacity(length);
		let mut aliases = HashMap::with_capacity(length);
		for (indirect_pci_device_identifier, dpdk_pci_driver_to_bind_to) in indirect_pci_device_identifiers.drain()
		{
			assert!(dpdk_pci_driver_to_bind_to.is_dpdk_driver(), "dpdk_pci_driver_to_bind_to {:?} isn't a DPDK driver", dpdk_pci_driver_to_bind_to);

			let pci_device = indirect_pci_device_identifier.to_pci_device(sys_path).unwrap();
			assert!(pci_device.is_class_network_ethernet(sys_path), "PCI device '{:?}' for indirect_pci_device_identifier '{:?}' is for not an Ethernet class PCI device (or does not exist at all)", pci_device, indirect_pci_device_identifier);
			
			if let Some(original) = aliases.get(&pci_device)
			{
				panic!("IndirectPciDeviceIdentifier '{:?}' is an alias of '{:?}' with the same PCI device ('{:?}') ", indirect_pci_device_identifier, original, pci_device);
			}
			aliases.insert(pci_device.clone(), indirect_pci_device_identifier.clone());
			result.insert(indirect_pci_device_identifier, (pci_device, dpdk_pci_driver_to_bind_to));
		}

		result
	}

	// returns driver to bind back to at termination.
	fn ensure_bound_to_dpdk_driver(&self, sys_path: &Path, pci_drivers: &HashSet<PciKernelDriver>, dpdk_pci_driver_to_bind_to: &PciKernelDriver) -> Option<PciKernelDriver>
	{
		assert!(dpdk_pci_driver_to_bind_to.is_dpdk_driver(), "dpdk_pci_driver_to_bind_to {:?} isn't a DPDK driver", dpdk_pci_driver_to_bind_to);
		assert!(self.is_class_network_ethernet(sys_path), "We are not an ethernet network device");

		let pci_vendor_identifier = self.pci_vendor_identifier(sys_path);
		let pci_device_identifier = self.pci_device_identifier(sys_path);

		let bind_back_to_at_termination = if let Some(existing_pci_driver) = self.pci_driver(sys_path, pci_drivers)
		{
			if existing_pci_driver == dpdk_pci_driver_to_bind_to
			{
				// Just in case it's bound but the id not added. Rare edge case.
				existing_pci_driver.assign_id(sys_path, &pci_vendor_identifier, &pci_device_identifier).expect("Could not assign device to PCI driver");
				return None;
			}
			existing_pci_driver.unbind_pci_device(sys_path, &self.0).expect("Could not unbind");
			Some(existing_pci_driver.clone())
		}
		else
		{
			None
		};
		
		dpdk_pci_driver_to_bind_to.assign_id(sys_path, &pci_vendor_identifier, &pci_device_identifier).expect("Could not assign device to PCI driver");
		dpdk_pci_driver_to_bind_to.bind_pci_device(sys_path, &self.0).expect("Could not assign device to PCI driver");

		bind_back_to_at_termination
	}

	#[inline(always)]
	fn pci_driver<'a>(&self, sys_path: &Path, pci_drivers: &'a HashSet<PciKernelDriver>) -> Option<&'a PciKernelDriver>
	{
		// Could use /sys/bus/pci/devices/0000\:00\:05.0/pci_driver, but this is a symlink and readlink() then path to pathbuf and get filename seems messy and likely to break
		for pci_driver in pci_drivers.iter()
		{
			if pci_driver.is_pci_device_bound(sys_path, &self.0)
			{
				return Some(pci_driver);
			}
		}
		None
	}

	/// Useful if associated_numa_node() returns None.
	#[inline(always)]
	pub(crate) fn active_on_cpus(&self, sys_path: &Path) -> LogicalCoresActive
	{
		let file_path = self.file_or_folder_path(sys_path, "local_cpulist");
		LogicalCoresActive::parse_from_file_path(&file_path).expect("Should exist for PCI device")
	}
	
	#[inline(always)]
	pub(crate) fn associated_numa_node(&self, sys_path: &Path) -> NumaNodeChoice
	{
		let file_path = self.file_or_folder_path(sys_path, "numa_node");
		NumaNodeChoice::from_i32(file_path.read_value().expect("Could not parse numa_node"))
	}
	
	#[inline(always)]
	pub(crate) fn is_class_network_ethernet(&self, sys_path: &Path) -> bool
	{
		// See: https://pci-ids.ucw.cz/read/PD/
		const Network: u8 = 0x02;
		const EthernetNetwork: u8 = 0x00;
		
		match self.pci_class_identifier(sys_path)
		{
			(Network, EthernetNetwork, _) => true,
			_ => false,
		}
	}
	
	#[inline(always)]
	pub(crate) fn pci_vendor_identifier(&self, sys_path: &Path) -> PciVendorIdentifier
	{
		let file_path = self.file_or_folder_path(sys_path, "vendor");
		PciVendorIdentifier::new(file_path.read_hexadecimal_value_with_prefix_u16().expect("Seems PCI device's vendor id does not properly exist")).expect("PCI vendor Id should not be 'Any'")
	}
	
	#[inline(always)]
	pub(crate) fn pci_device_identifier(&self, sys_path: &Path) -> PciDeviceIdentifier
	{
		let file_path = self.file_or_folder_path(sys_path, "device");
		PciDeviceIdentifier::new(file_path.read_hexadecimal_value_with_prefix_u16().expect("Seems PCI device's device id does not properly exist")).expect("PCI device Id should not be 'Any'")
	}
	
	#[inline(always)]
	pub(crate) fn pci_class_identifier(&self, sys_path: &Path) -> (u8, u8, u8)
	{
		let file_path = self.file_or_folder_path(sys_path, "class");
		let value = file_path.read_hexadecimal_value_with_prefix(6, |raw_string| u32::from_str_radix(raw_string, 16)).expect("Could not parse class");
		(((value & 0xFF0000) >> 16) as u8, ((value & 0x00FF00) >> 8) as u8, (value & 0x0000FF) as u8)
	}
	
	#[inline(always)]
	fn file_or_folder_path(&self, sys_path: &Path, file_or_folder_name: &str) -> PathBuf
	{
		let mut path = Self::devices_path(sys_path);
		path.push(self.0.to_string());
		path.push(file_or_folder_name);
		path
	}
	
	#[inline(always)]
	fn devices_path(sys_path: &Path) -> PathBuf
	{
		let mut path = PathBuf::from(sys_path);
		path.push("bus/pci/devices");
		path
	}
}
