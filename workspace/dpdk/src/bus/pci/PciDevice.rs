// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Models a PCI device.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PciDevice(DpdkPciDeviceAddress);

impl PciDevice
{
	#[inline(always)]
	pub(crate) fn to_address_c_string(&self) -> CString
	{
		CString::from(self.0.to_string()).unwrap()
	}
	
	#[inline(always)]
	pub(crate) fn associated_numa_node(&self, sys_path: &Path) -> NumaNodeChoice
	{
		let file_path = self.device_file_or_folder_path(sys_path, "numa_node");
		if !file_path.exists()
		{
			return NumaNodeChoice::Any
		}
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
		let file_path = self.device_file_or_folder_path(sys_path, "vendor");
		PciVendorIdentifier::new(file_path.read_hexadecimal_value_with_prefix_u16().expect("Seems PCI device's vendor id does not properly exist")).expect("PCI vendor Id should not be 'Any'")
	}
	
	#[inline(always)]
	pub(crate) fn pci_device_identifier(&self, sys_path: &Path) -> PciDeviceIdentifier
	{
		let file_path = self.device_file_or_folder_path(sys_path, "device");
		PciDeviceIdentifier::new(file_path.read_hexadecimal_value_with_prefix_u16().expect("Seems PCI device's device id does not properly exist")).expect("PCI device Id should not be 'Any'")
	}
	
	#[inline(always)]
	pub(crate) fn pci_class_identifier(&self, sys_path: &Path) -> (u8, u8, u8)
	{
		let file_path = self.device_file_or_folder_path(sys_path, "class");
		let value = file_path.read_hexadecimal_value_with_prefix(6, |raw_string| u32::from_str_radix(raw_string, 16)).expect("Could not parse class");
		(((value & 0xFF0000) >> 16) as u8, ((value & 0x00FF00) >> 8) as u8, (value & 0x0000FF) as u8)
	}
	
	#[inline(always)]
	pub(crate) fn take_for_use_with_dpdk(&self, sys_path: &Path, pci_kernel_driver: PciKernelDriver) -> Option<String>
	{
		assert_effective_user_id_is_root(&format!("Changing override of PCI driver for PCI device '{}'", self.to_string()));
		
		let original_driver_name = self.unbind_from_driver_if_necessary(sys_path);
		self.add_override_of_pci_kernel_driver(sys_path, pci_kernel_driver);
		self.bind_to_new_driver(sys_path);
		original_driver_name
	}
	
	#[inline(always)]
	pub(crate) fn release_from_use_with_dpdk(&self, sys_path: &Path, original_driver_name: Option<String>)
	{
		assert_effective_user_id_is_root(&format!("Changing override of PCI driver for PCI device '{}'", self.to_string()));
		
		self.remove_override_of_pci_kernel_driver(sys_path);
		self.unbind_from_driver_if_necessary(sys_path);
		self.bind_to_original_driver_if_necessary(sys_path, original_driver_name)
	}
	
	#[inline(always)]
	fn unbind_from_driver_if_necessary(&self, sys_path: &Path) -> Option<String>
	{
		let unbind_file_path = self.driver_file_or_folder_path(sys_path, "unbind");
		let is_not_bound = !unbind_file_path.exists();
		if is_not_bound
		{
			return None
		}
		
		let original_driver_name = unbind_file_path.canonicalize().unwrap().parent().unwrap().file_name().unwrap().to_str().unwrap().to_owned();
		
		unbind_file_path.write_value(self.to_string()).unwrap();
		
		Some(original_driver_name)
	}
	
	#[inline(always)]
	fn add_override_of_pci_kernel_driver(&self, sys_path: &Path, pci_kernel_driver: PciKernelDriver)
	{
		self.write_to_driver_override_file(sys_path, pci_kernel_driver.driver_name.to_string())
	}
	
	#[inline(always)]
	fn bind_to_new_driver(&self, sys_path: &Path)
	{
		let file_path = self.driver_file_or_folder_path(sys_path, "bind");
		file_path.write_value(pci_device_address.to_string()).unwrap()
	}
	
	#[inline(always)]
	fn remove_override_of_pci_kernel_driver(&self, sys_path: &Path)
	{
		self.write_to_driver_override_file(sys_path, "\0".to_string())
	}
	
	#[inline(always)]
	fn bind_to_original_driver_if_necessary(&self, sys_path: &Path, original_driver_name: Option<String>)
	{
		if let Some(original_driver_name) = original_driver_name
		{
			let bind_file_path = self.driver_file_or_folder_path(sys_path, "bind");
			bind_file_path.write_value(self.to_string()).unwrap();
		}
	}
	
	#[inline(always)]
	fn write_to_driver_override_file(&self, sys_path: &Path, value: String)
	{
		let file_path = self.device_file_or_folder_path(sys_path, "driver_override");
		file_path.write_value(value).unwrap()
	}
	
	#[inline(always)]
	fn driver_file_or_folder_path(&self, sys_path: &Path, file_or_folder_name: &str) -> PathBuf
	{
		let mut path = self.device_file_or_folder_path(sys_path, "driver");
		path.push(file_or_folder_name);
		path
	}
	
	#[inline(always)]
	fn device_file_or_folder_path(&self, sys_path: &Path, file_or_folder_name: &str) -> PathBuf
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
