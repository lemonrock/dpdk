// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Device(*mut rte_pci_device);

impl Device
{
	#[inline(always)]
	pub fn sysFsPath() -> PathBuf
	{
		let fromC = unsafe { ::dpdk_sys::pci_get_sysfs_path() };
		let slice = unsafe { from_raw_parts(fromC as *mut u8, strnlen(fromC, PATH_MAX as usize)) };
		let osStr = OsStr::from_bytes(slice);
		let mut path = PathBuf::new();
		path.push(osStr);
		path
	}
	
	#[inline(always)]
	pub fn scan() -> Vec<Device>
	{
		match unsafe { ::dpdk_sys::rte_eal_pci_scan() }
		{
			0 => (),
			negative if negative < 0 => panic!("Could not scan PCI bus, error code was '{}'", negative),
			
			illegal @ _ => panic!("Invalid result code '{}' from rte_eal_pci_scan()", illegal),
		};
		
		let pciDeviceList = unsafe { ::dpdk_sys::pci_device_list };
				
		let firstElement = pciDeviceList.tqh_first;
		let isEmpty = firstElement.is_null();
		let capacity = if isEmpty
		{
			0
		}
		else
		{
			256
		};
	
		let mut devices = Vec::with_capacity(capacity);
	
		let mut element = firstElement;
		while !element.is_null()
		{
			devices.push(Device(element));
			let elementValue = unsafe { (*element) };
			element = elementValue.next.tqe_next;
		}
		devices.shrink_to_fit();
		
		devices
	}
	
	#[inline(always)]
	pub fn dumpAllDevicesToStandardError()
	{
		unsafe { ::dpdk_sys::rte_eal_pci_dump(stderr as *mut FILE) }
	}
	
	#[inline(always)]
	fn deref(&self) -> rte_pci_device
	{
		unsafe { (*self.0) }
	}
	
    #[inline(always)]
	pub fn numaSocketId(&self) -> Option<NumaSocketId>
	{
		let socketId = self.deref().device.numa_node;
		NumaSocketId::fromI32(socketId)
	}
	
	#[inline(always)]
	pub fn address(&self) -> DeviceAddress
	{
		let address = self.deref().addr;
		DeviceAddress::from_rte_pci_addr(&address)
	}
	
	#[inline(always)]
	pub fn driver(&self) -> Option<Driver>
	{
		let driver = self.deref().driver;
		if unlikely(driver.is_null())
		{
			None
		}
		else
		{
			Some(Driver(driver))
		}
	}
	
	#[inline(always)]
	pub fn mapInputOutputPort(&mut self, baseAddressRegister: i32) -> Option<InputOutputPort>
	{
		let mut data = unsafe { uninitialized() };
		let result = unsafe { ::dpdk_sys::rte_eal_pci_ioport_map(self.0, baseAddressRegister, &mut data) };
		if likely(result == 0)
		{
			Some(InputOutputPort::new(data))
		}
		else
		{
			forget(data);
			
			match result
			{
				negative if negative < 0 => None,
				
				_ => panic!("rte_eal_pci_ioport_map() returned illegal result '{}'", result),
			}
		}
	}
	
	#[inline(always)]
	pub fn map(&mut self) -> Option<bool>
	{
		match unsafe { ::dpdk_sys::rte_eal_pci_map_device(self.0) }
		{
			0 => Some(true),
			negative if negative < 0 => Some(false),
			_ => None,
		}
	}
	
	/// Returns None if no driver found
	#[inline(always)]
	pub fn unmap(&mut self)
	{
		unsafe { ::dpdk_sys::rte_eal_pci_unmap_device(self.0) }
	}
	
	#[inline(always)]
	pub fn readConfigurationSpace(&self, readInto: &mut [u8], offsetIntoConfigurationSpace: isize) -> Result<(), i32>
	{
		let result = unsafe { ::dpdk_sys::rte_eal_pci_read_config(self.0, readInto.as_mut_ptr() as *mut c_void, readInto.len(), offsetIntoConfigurationSpace as off_t) };
		if likely(result == 0)
		{
			Ok(())
		}
		else
		{
			Err(result)
		}
	}
	
	#[inline(always)]
	pub fn writeConfigurationSpace(&self, writeFrom: &[u8], offsetIntoConfigurationSpace: isize) -> Result<(), i32>
	{
		let result = unsafe { ::dpdk_sys::rte_eal_pci_write_config(self.0, writeFrom.as_ptr() as *mut c_void, writeFrom.len(), offsetIntoConfigurationSpace as off_t) };
		if likely(result == 0)
		{
			Ok(())
		}
		else
		{
			Err(result)
		}
	}
}
