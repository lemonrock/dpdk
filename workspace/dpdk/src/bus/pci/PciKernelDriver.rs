// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents a PCI driver.
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub enum PciKernelDriver
{
	#[cfg(any(target_os = "android", target_os = "linux"))] IgbUio,
	#[cfg(any(target_os = "android", target_os = "linux"))] UioPciGeneric,
	#[cfg(any(target_os = "android", target_os = "linux"))] VfioPci,
	
	NonDpdk(String),
}

impl PciKernelDriver
{
	/// New.
	#[inline(always)]
	pub fn new(value: &str) -> PciKernelDriver
	{
		use self::PciKernelDriver::*;
		
		match value
		{
			#[cfg(any(target_os = "android", target_os = "linux"))] "igb_uio" => IgbUio,
			#[cfg(any(target_os = "android", target_os = "linux"))] "vfio-pci" => VfioPci,
			#[cfg(any(target_os = "android", target_os = "linux"))] "uio_pci_generic" => UioPciGeneric,
			
			_ => NonDpdk(value),
		}
	}
	
	#[inline(always)]
	pub(crate) fn all_known_pci_drivers(sys_path: &Path) -> HashSet<PciKernelDriver>
	{
		let mut results = HashSet::with_capacity(64);
		
		let drivers_path = Self::drivers_path(sys_path);
		
		if let Ok(iterator) = drivers_path.read_dir()
		{
			for entry in iterator
			{
				if let Ok(entry) = entry
				{
					if let Ok(fileName) = entry.file_name().into_string()
					{
						assert!(results.insert(Self::new(fileName)), "Duplicate? How?");
					}
				}
			}
		}
		
		results
	}
	
	#[inline(always)]
	fn to_string<'a>(&'a self) -> &'a str
	{
		use self::PciKernelDriver::*;
		
		match *self
		{
			#[cfg(any(target_os = "android", target_os = "linux"))] IgbUio => "igb_uio",
			#[cfg(any(target_os = "android", target_os = "linux"))] VfioPci => "vfio-pci",
			#[cfg(any(target_os = "android", target_os = "linux"))] UioPciGeneric => "uio_pci_generic",
			
			NonDpdk(ref value) => value,
		}
	}
	
	#[inline(always)]
	pub(crate) fn is_dpdk_driver(&self) -> bool
	{
		if let PciKernelDriver::NonDpdk(_) = *self
		{
			true
		}
		else
		{
			false
		}
	}
	
	#[inline(always)]
	pub(crate) fn is_pci_device_bound(&self, sys_path: &Path, pci_device_address: &DpdkPciDeviceAddress) -> bool
	{
		self.path_to_file_or_folder(sys_path, &pci_device_address.to_string()).exists()
	}
	
	#[inline(always)]
	pub(crate) fn assign_id(&self, sys_path: &Path, pci_vendor_identifier: &PciVendorIdentifier, pci_device_identifier: &PciDeviceIdentifier) -> io::Result<()>
	{
		assert_effective_user_id_is_root(&format!("Assign vendor id '{:04x}', device id '{:04x}' to driver '{:?}'", pci_vendor_identifier.0, pci_device_identifier.0, self));
		
		let file_path = self.path_to_file_or_folder(sys_path, "new_id");
		let value = format!("{:04x} {:04x}", pci_vendor_identifier.0, pci_device_identifier.0);
		file_path.write_value(value)
	}
	
	#[inline(always)]
	pub(crate) fn unassign_id(&self, sys_path: &Path, pci_vendor_identifier: &PciVendorIdentifier, pci_device_identifier: &PciDeviceIdentifier) -> io::Result<()>
	{
		assert_effective_user_id_is_root(&format!("Unassign vendor id '{:04x}', device id '{:04x}' to driver '{:?}'", pci_vendor_identifier.0, pci_device_identifier.0, self));
		
		let file_path = self.path_to_file_or_folder(sys_path, "remove_id");
		let value = format!("{:04x} {:04x}", pci_vendor_identifier.0, pci_device_identifier.0);
		file_path.write_value(value)
	}
	
	#[inline(always)]
	pub(crate) fn bind_pci_device(&self, sys_path: &Path, pci_device_address: &DpdkPciDeviceAddress) -> io::Result<()>
	{
		assert_effective_user_id_is_root(&format!("Bind device '{}' to driver '{:?}'", pci_device_address.to_string(), self));

		let file_path = self.path_to_file_or_folder(sys_path, "bind");
		file_path.write_value(pci_device_address.to_string())
	}
	
	#[inline(always)]
	pub(crate) fn unbind_pci_device(&self, sys_path: &Path, pci_device_address: &DpdkPciDeviceAddress) -> io::Result<()>
	{
		assert_effective_user_id_is_root(&format!("Unbind device '{}' from driver '{:?}'", pci_device_address.to_string(), self));

		let file_path = self.path_to_file_or_folder(sys_path, "unbind");
		file_path.write_value(pci_device_address.to_string())
	}
	
	#[inline(always)]
	fn path_to_file_or_folder(&self, sys_path: &Path, file_or_folder_name: &str) -> PathBuf
	{
		let mut path = Self::drivers_path(sys_path);
		path.push(self.to_string());
		path.push(file_or_folder_name);
		path
	}
	
	#[inline(always)]
	fn drivers_path(sys_path: &Path) -> PathBuf
	{
		let mut path = PathBuf::from(sys_path);
		path.push("bus/pci/drivers");
		path
	}
}
