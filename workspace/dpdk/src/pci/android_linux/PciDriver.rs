// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub enum PciDriver
{
	IgbUio,
	UioPciGeneric,
	VfioPci,
	
	NonDpdk(String),
}

impl PciDriver
{
	pub fn new(value: String) -> PciDriver
	{
		match &value[..]
		{
			"igb_uio" => PciDriver::IgbUio,
			"vfio-pci" => PciDriver::VfioPci,
			"uio_pci_generic" => PciDriver::UioPciGeneric,
			
			_ => PciDriver::NonDpdk(value),
		}
	}
	
	pub fn all(sysPath: &Path) -> HashSet<PciDriver>
	{
		let mut results = HashSet::with_capacity(64);
		
		let driversPath = Self::driversPath(sysPath);
		
		if let Ok(iterator) = driversPath.read_dir()
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
	
	fn to_string<'a>(&'a self) -> &'a str
	{
		match *self
		{
			PciDriver::IgbUio => "igb_uio",
			PciDriver::VfioPci => "vfio-pci",
			PciDriver::UioPciGeneric => "uio_pci_generic",
			
			PciDriver::NonDpdk(ref value) => value,
		}
	}
	
	pub fn isDpdkDriver(&self) -> bool
	{
		match *self
		{
			PciDriver::NonDpdk(_) => false,
			_ => true,
		}
	}
	
	pub fn isPciDeviceBound(&self, sysPath: &Path, pciDeviceAddress: &DeviceAddress) -> bool
	{
		self.pathToFileOrFolder(sysPath, &pciDeviceAddress.to_string()).exists()
	}
	
	pub fn assignId(&self, sysPath: &Path, vendorId: &VendorId, deviceId: &DeviceId) -> io::Result<()>
	{
		assertEffectiveUserIsRoot(&format!("Assign vendor id '{:04x}', device id '{:04x}' to driver '{:?}'", vendorId.0, deviceId.0, self));
		
		let filePath = self.pathToFileOrFolder(sysPath, "new_id");
		let value = format!("{:04x} {:04x}", vendorId.0, deviceId.0);
		writeValueToFile(&filePath, value)
	}
	
	pub fn bindPciDevice(&self, sysPath: &Path, pciDeviceAddress: &DeviceAddress) -> io::Result<()>
	{
		assertEffectiveUserIsRoot(&format!("Bind device '{}' to driver '{:?}'", pciDeviceAddress.to_string(), self));

		let filePath = self.pathToFileOrFolder(sysPath, "bind");
		writeValueToFile(&filePath, pciDeviceAddress.to_string())
	}
	
	pub fn unbindPciDevice(&self, sysPath: &Path, pciDeviceAddress: &DeviceAddress) -> io::Result<()>
	{
		assertEffectiveUserIsRoot(&format!("Unbind device '{}' from driver '{:?}'", pciDeviceAddress.to_string(), self));

		let filePath = self.pathToFileOrFolder(sysPath, "unbind");
		writeValueToFile(&filePath, pciDeviceAddress.to_string())
	}
	
	pub fn unassignId(&self, sysPath: &Path, vendorId: &VendorId, deviceId: &DeviceId) -> io::Result<()>
	{
		assertEffectiveUserIsRoot(&format!("Unassign vendor id '{:04x}', device id '{:04x}' to driver '{:?}'", vendorId.0, deviceId.0, self));
		
		let filePath = self.pathToFileOrFolder(sysPath, "remove_id");
		let value = format!("{:04x} {:04x}", vendorId.0, deviceId.0);
		writeValueToFile(&filePath, value)
	}
	
	fn pathToFileOrFolder(&self, sysPath: &Path, fileOrFolderName: &str) -> PathBuf
	{
		let mut path = Self::driversPath(sysPath);
		path.push(self.to_string());
		path.push(fileOrFolderName);
		path
	}
	
	fn driversPath(sysPath: &Path) -> PathBuf
	{
		let mut path = PathBuf::from(sysPath);
		path.push("bus/pci/drivers");
		path
	}
}
