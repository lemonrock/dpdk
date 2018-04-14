// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DpdkPciDevice(NonNull<rte_pci_device>);

impl DpdkPciDevice
{
	/// `/sys/fs` path used by DPDK.
	#[inline(always)]
	pub fn sys_fs_path() -> PathBuf
	{
		let fromC = unsafe { pci_get_sysfs_path() };
		let slice = unsafe { from_raw_parts(fromC as *mut u8, strnlen(fromC, PATH_MAX as usize)) };
		let osStr = OsStr::from_bytes(slice);
		let mut path = PathBuf::new();
		path.push(osStr);
		path
	}

	/// Scan.
	#[inline(always)]
	pub fn scan() -> Vec<Self>
	{
		match unsafe { rte_eal_pci_scan() }
		{
			0 => (),
			negative if negative < 0 => panic!("Could not scan PCI bus, error code was '{}'", negative),

			illegal @ _ => panic!("Invalid result code '{}' from rte_eal_pci_scan()", illegal),
		};

		let pci_device_list = unsafe { pci_device_list };

		let first_element = pci_device_list.tqh_first;
		let is_empty = first_element.is_null();
		let capacity = if is_empty
		{
			0
		}
		else
		{
			256
		};

		let mut devices = Vec::with_capacity(capacity);

		let mut element = first_element;
		while !element.is_null()
		{
			devices.push(DpdkPciDevice(unsafe { NonNull::new_unchecked(element) }));
			let element_value = unsafe { (*element) };
			element = element_value.next.tqe_next;
		}
		devices.shrink_to_fit();

		devices
	}

	#[inline(always)]
	pub fn dumpAllDevicesToStandardError()
	{
		unsafe { rte_eal_pci_dump(stderr as *mut FILE) }
	}

	#[inline(always)]
	fn deref(&self) -> rte_pci_device
	{
		unsafe { (*self.handle()) }
	}
	
	/// NUMA socket id, if any.
    #[inline(always)]
	pub fn numa_socket_id(&self) -> Option<NumaSocketId>
	{
		let socketId = self.deref().device.numa_node;
		NumaSocketId::from_i32(socketId)
	}
	
	/// PCI device address.
	#[inline(always)]
	pub fn pci_device_address(&self) -> PciDeviceAddress
	{
		let address = self.deref().addr;
		PciDeviceAddress::from_rte_pci_addr(address)
	}
	
	/// DPDK driver.
	#[inline(always)]
	pub fn driver(&self) -> Option<DpdkPciDriver>
	{
		let driver = self.deref().driver;
		if unlikely(driver.is_null())
		{
			None
		}
		else
		{
			Some(DpdkPciDriver(driver))
		}
	}
	
	/// Map IO port.
	#[inline(always)]
	pub fn map_input_output_port(&mut self, base_address_register: i32) -> Option<DpdkPciInputOutputPort>
	{
		let mut data = unsafe { uninitialized() };
		let result = unsafe { rte_eal_pci_ioport_map(self.handle(), base_address_register, &mut data) };
		if likely(result == 0)
		{
			Some(DpdkPciInputOutputPort::new(UnsafeCell::new(data)))
		}
		else
		{
			match result
			{
				negative if negative < 0 => None,

				_ => panic!("rte_eal_pci_ioport_map() returned illegal result '{}'", result),
			}
		}
	}
	
	/// Map.
	#[inline(always)]
	pub fn map(&mut self) -> Option<bool>
	{
		match unsafe { rte_eal_pci_map_device(self.handle()) }
		{
			0 => Some(true),
			negative if negative < 0 => Some(false),
			_ => None,
		}
	}

	/// Returns None if no driver found.
	#[inline(always)]
	pub fn unmap(&mut self)
	{
		unsafe { rte_eal_pci_unmap_device(self.handle()) }
	}
	
	/// Read configuration space.
	#[inline(always)]
	pub fn read_configuration_space(&self, read_into: &mut [u8], offset_into_configuration_space: isize) -> Result<(), i32>
	{
		let result = unsafe { rte_eal_pci_read_config(self.handle(), read_into.as_mut_ptr() as *mut c_void, read_into.len(), offset_into_configuration_space as off_t) };
		if likely(result == 0)
		{
			Ok(())
		}
		else
		{
			Err(result)
		}
	}
	
	/// Write configuration space.
	#[inline(always)]
	pub fn write_configuration_space(&self, write_from: &[u8], offset_into_configuration_space: isize) -> Result<(), i32>
	{
		let result = unsafe { rte_eal_pci_write_config(self.handle(), write_from.as_ptr() as *mut c_void, write_from.len(), offset_into_configuration_space as off_t) };
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
	fn handle(&self) -> *mut rte_pci_device
	{
		self.0.as_ptr()
	}
}
