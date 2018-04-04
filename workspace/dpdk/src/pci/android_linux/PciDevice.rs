// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PciDevice(pub DeviceAddress);

impl PciDevice
{
	pub fn bindDevices(sysPath: &Path, networkPortIdentifiers: HashMap<NetworkPortIdentifier, PciDriver>) -> Vec<Unbind>
	{
		let allKnownPciDrivers = PciDriver::all(sysPath);
		let mut converted = Self::networkPortIdentifiersToPciDevices(sysPath, networkPortIdentifiers);
		let mut unbindList = Vec::with_capacity(converted.len());
		
		for (networkPortIdentifier, (pciDevice, dpdkPciDriverToBindTo)) in converted.drain()
		{
			let bindBackToOriginal = pciDevice.ensureBoundToDpdkDriver(sysPath, &allKnownPciDrivers, &dpdkPciDriverToBindTo);
			let unbind = Unbind
			{
				networkPortIdentifier: networkPortIdentifier,
				pciDevice: pciDevice,
				dpdkPciDriverToUnbindFrom: dpdkPciDriverToBindTo,
				bindBackToOriginal: bindBackToOriginal,
			};
			
			unbindList.push(unbind);
		}
		
		unbindList
	}
	
	pub fn all(sysPath: &Path) -> HashSet<PciDevice>
	{
		let mut results = HashSet::with_capacity(64);
		
		let devicesPath = Self::devicesPath(sysPath);
		
		if let Ok(iterator) = devicesPath.read_dir()
		{
			for entry in iterator
			{
				if let Ok(entry) = entry
				{
					if let Ok(fileName) = entry.file_name().into_string()
					{
						if let Ok(deviceAddress) = DeviceAddress::fromString(&fileName)
						{
							let value = PciDevice(deviceAddress);
							assert!(results.insert(value), "Duplicate in a read_dir() ? how ?");
						}
					}
				}
			}
		}
		
		results.shrink_to_fit();
		results
	}

	pub fn networkPortIdentifiersToPciDevices(sysPath: &Path, mut networkPortIdentifiers: HashMap<NetworkPortIdentifier, PciDriver>) -> HashMap<NetworkPortIdentifier, (PciDevice, PciDriver)>
	{
		let length = networkPortIdentifiers.len();
		let mut result = HashMap::with_capacity(length);
		let mut aliases = HashMap::with_capacity(length);
		for (networkPortIdentifier, dpdkPciDriverToBindTo) in networkPortIdentifiers.drain()
		{
			assert!(dpdkPciDriverToBindTo.isDpdkDriver(), "dpdkPciDriverToBindTo {:?} isn't a DPDK driver", dpdkPciDriverToBindTo);
			
			let pciDevice = networkPortIdentifier.toPciDevice(sysPath);
			if let Some(original) = aliases.get(&pciDevice)
			{
				panic!("Network port identifier '{:?}' is an alias of '{:?}' with the same PCI device ('{:?}') ", networkPortIdentifier, original, pciDevice);
			}
			aliases.insert(pciDevice.clone(), networkPortIdentifier.clone());
			result.insert(networkPortIdentifier, (pciDevice, dpdkPciDriverToBindTo));
		}
		
		result
	}
	
	// returns driver to bind back to at termination
	pub fn ensureBoundToDpdkDriver(&self, sysPath: &Path, allKnownPciDrivers: &HashSet<PciDriver>, dpdkPciDriverToBindTo: &PciDriver) -> Option<PciDriver>
	{
		assert!(dpdkPciDriverToBindTo.isDpdkDriver(), "dpdkPciDriverToBindTo {:?} isn't a DPDK driver", dpdkPciDriverToBindTo);
		assert!(self.isClassNetworkEthernet(sysPath), "We are not an ethernet network device");
		
		let vendorId = self.vendorId(sysPath);
		let deviceId = self.deviceId(sysPath);
		
		let bindBackToAtTermination = if let Some(existingPciDriver) = self.driver(sysPath, allKnownPciDrivers)
		{
			if existingPciDriver == dpdkPciDriverToBindTo
			{
				// Just in case it's bound but the id not added. Rare edge case.
				existingPciDriver.assignId(sysPath, &vendorId, &deviceId).expect("Could not assign device to PCI driver");
				return None;
			}
			existingPciDriver.unbindPciDevice(sysPath, &self.0).expect("Could not unbind");
			Some(existingPciDriver.clone())
		}
		else
		{
			None
		};
		
		dpdkPciDriverToBindTo.assignId(sysPath, &vendorId, &deviceId).expect("Could not assign device to PCI driver");
		dpdkPciDriverToBindTo.bindPciDevice(sysPath, &self.0).expect("Could not assign device to PCI driver");
		
		bindBackToAtTermination
	}
	
	pub fn driver<'a>(&self, sysPath: &Path, allKnownPciDrivers: &'a HashSet<PciDriver>) -> Option<&'a PciDriver>
	{
		// Could use /sys/bus/pci/devices/0000\:00\:05.0/driver, but this is a symlink and readlink() then path to pathbuf and get filename seems messy and likely to break
		for driver in allKnownPciDrivers.iter()
		{
			if driver.isPciDeviceBound(sysPath, &self.0)
			{
				return Some(driver);
			}
		}
		None
	}
	
	/// Useful if associatedNumaNode() returns None
	pub fn activeOnCpus(&self, sysPath: &Path) -> LogicalCoresActive
	{
		let filePath = self.fileOrFolderPath(sysPath, "local_cpulist");
		LogicalCoresActive::parseFromFilePath(&filePath).expect("Should exist for PCI device")
	}
	
	pub fn associatedNumaNode(&self, sysPath: &Path) -> Option<NumaSocketId>
	{
		let filePath = self.fileOrFolderPath(sysPath, "numa_node");
		NumaSocketId::fromI32(readValueFromFile(&filePath).expect("Could not parse numa_node"))
	}
	
	pub fn isClassNetworkEthernet(&self, sysPath: &Path) -> bool
	{
		let filePath = self.fileOrFolderPath(sysPath, "class");
		readHexadecimalValueWithPrefixFromFile(&filePath, 6, |rawString|
		{
			Ok
			(
				match rawString
				{
					"020000" => true,
					_ => false,
				
				}
			)
		}).expect("Could not parse class")
	}
	
	pub fn vendorId(&self, sysPath: &Path) -> VendorId
	{
		let filePath = self.fileOrFolderPath(sysPath, "vendor");
		VendorId::new(readHexadecimalValueWithPrefixFromFile_u16(&filePath).expect("Seems PCI device's vendor id does not properly exist")).expect("PCI vendor Id should not be 'Any'")
	}
	
	pub fn deviceId(&self, sysPath: &Path) -> DeviceId
	{
		let filePath = self.fileOrFolderPath(sysPath, "device");
		DeviceId::new(readHexadecimalValueWithPrefixFromFile_u16(&filePath).expect("Seems PCI device's device id does not properly exist")).expect("PCI device Id should not be 'Any'")
	}
	
	fn fileOrFolderPath(&self, sysPath: &Path, fileOrFolderName: &str) -> PathBuf
	{
		let mut path = Self::devicesPath(sysPath);
		path.push(self.0.to_string());
		path.push(fileOrFolderName);
		path
	}
	
	fn devicesPath(sysPath: &Path) -> PathBuf
	{
		let mut path = PathBuf::from(sysPath);
		path.push("bus/pci/devices");
		path
	}
}
