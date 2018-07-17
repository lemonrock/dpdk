// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents the unique address of a PCI device in a system, such as an individual ethernet port (connector).
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct DpdkPciDeviceAddress(rte_pci_addr);

impl Clone for DpdkPciDeviceAddress
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		DpdkPciDeviceAddress
		(
			rte_pci_addr
			{
				domain: self.0.domain,
				bus: self.0.bus,
				devid: self.0.devid,
				function: self.0.function,
			}
		)
	}
}

impl PartialOrd for DpdkPciDevice
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		Some(self.cmp(other))
	}
}

impl Ord for DpdkPciDevice
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		use self::Ordering::*;
		
		match unsafe { rte_pci_addr_cmp(self.0.as_ptr() as *const _, other.0.as_ptr() as *const _) }
		{
			0 => Equal,
			negative if negative < 0 => Less,
			_ => Greater,
		}
	}
}

impl DeviceName for DpdkPciDeviceAddress
{
	#[inline]
	fn to_string(&self) -> String
	{
		format!("{:04x}:{:02x}:{:02x}.{:01x}", self.domain, self.bus, self.device_identifier, self.function)
	}
}

impl DpdkPciDeviceAddress
{
	/// Parses a PCI device address string.
	///
	/// Prefer the function `Self::from_str`.
	#[inline(always)]
	pub fn parse_pci_device_address_string(pci_device_address_string: &CStr) -> Result<Self, ()>
	{
		let mut device = unsafe { uninitialized() };
		match unsafe { rte_pci_addr_parse(pci_device_address_string.as_ptr(), &mut device) }
		{
			0 => Ok(Self::from_rte_pci_addr(device)),
			negative if negative < 0 => Err(()),
			illegal @ _ => panic!("Illegal value '{}' from rte_pci_addr_parse()", illegal),
		}
	}
	
	/// Creates a PCI device address string.
	///
	/// Also known as PCI device name.
	#[inline(always)]
	pub fn to_pci_device_address_string(&self) -> CString
	{
		// size of "XXXXXXXX:XX:XX.X" + 1 for trailing ASCII NUL.
		const PCI_PRI_STR_SIZE: usize = 17;
		
		let mut output = Vec::with_capacity(PCI_PRI_STR_SIZE);
		let buffer = output.as_mut_ptr() as *mut c_char;
		unsafe { rte_pci_device_name(&self.0 as *const _, buffer, PCI_PRI_STR_SIZE) };
		
		unsafe { Vec::set_len(PCI_PRI_STR_SIZE) };
		output.shrink_to_fit();
		
		unsafe { CString::from_vec_unchecked(output) }
	}
	
	pub(crate) fn from_str(value: &str) -> Result<DpdkPciDeviceAddress, DpdkPciDeviceAddressStringParseError>
	{
		let mut match_count = 0;

		let length = value.len();
		if length != PciBusInformation::NumberOfBytesInPciAddressString
		{
			return Err(DpdkPciDeviceAddressStringParseError::LengthIsWrong(length));
		}

		let mut split = value.split(|character|
		{
			match match_count
			{
				0 | 1 =>
				{
					match_count += 1;
					character == ':'
				},

				2 =>
				{
					match_count += 1;
					character == '.'
				},

				_ => false,
			}
		});

		let domain = match split.next()
		{
			None => return Err(DpdkPciDeviceAddressStringParseError::NoDomain),
			Some(value) =>
			{
				match u16::from_str_radix(value, 16)
				{
					Err(cause) => return Err(DpdkPciDeviceAddressStringParseError::CouldNotParseDomain(value.to_owned(), cause)),
					Ok(value) => value,
				}
			}
		};

		let bus = match split.next()
		{
			None => return Err(DpdkPciDeviceAddressStringParseError::NoBus),
			Some(value) =>
			{
				match u8::from_str_radix(value, 16)
				{
					Err(cause) => return Err(DpdkPciDeviceAddressStringParseError::CouldNotParseBus(value.to_owned(), cause)),
					Ok(value) => value,
				}
			}
		};

		let devid = match split.next()
		{
			None => return Err(DpdkPciDeviceAddressStringParseError::NoDeviceIdentifier),
			Some(value) =>
			{
				match u8::from_str_radix(value, 16)
				{
					Err(cause) => return Err(DpdkPciDeviceAddressStringParseError::CouldNotParseDeviceIdentifier(value.to_owned(), cause)),
					Ok(value) => if value > 31
					{
						return Err(DpdkPciDeviceAddressStringParseError::DeviceNumberExceeds5BitValue(value))
					}
					else
					{
						value
					},
				}
			}
		};

		let function = match split.next()
		{
			None => return Err(DpdkPciDeviceAddressStringParseError::NoFunction),
			Some(value) =>
			{
				match u8::from_str_radix(value, 16)
				{
					Err(cause) => return Err(DpdkPciDeviceAddressStringParseError::CouldNotParseFunction(value.to_owned(), cause)),
					Ok(value) => if value > 15
					{
						return Err(DpdkPciDeviceAddressStringParseError::FunctionExceeds4BitValue(value))
					}
					else
					{
						value
					},
				}
			}
		};

		Ok(DpdkPciDeviceAddress::fromDomainBusDeviceIdAndFunction(domain, bus, devid, function))
	}

	/// Creates a new instance.
	///
	/// More information is [here|https://static.lwn.net/images/pdf/LDD3/ch12.pdf].
	///
	/// Domain is normally `0` and bus is often `0`.
	///
	/// A device number is effectively a slot number. Zero based, 0 - 31 ('u5').
	///
	/// A function is usually one less than the number of ethernet ports, connectors, etc on a device. Zero based, 0 -15 ('u4').
	///
	/// Domain is a Linux-centric concept.
	#[inline(always)]
	pub fn new(domain: u16, bus: u8, device_number: u8, function: u8) -> Self
	{
		debug_assert!(device_number < 32, "device_number must be between 0 to 32 inclusive (it's 5-bit) and not be '{}'", device_number);
		debug_assert!(function < 16, "function must be between 0 to 15 inclusive (it's 4-bit) and not be '{}'", function);

		DpdkPciDeviceAddress
		(
			rte_pci_addr
			{
				domain: domain as u32,
				bus,
				devid: device_number,
				function,
			}
		)
	}
	
	/// Creates a new instance.
	#[inline(always)]
	pub fn from_rte_pci_addr(value: rte_pci_addr) -> Self
	{
		debug_assert!(value.domain <= ::std::u16::MAX as u32, "domain exceeds maximum");
		debug_assert!(value.devid < 32, "devid exceeds maximum");
		debug_assert!(value.function < 16, "function exceeds maximum");
		
		DpdkPciDeviceAddress(value)
	}
	
	/// Creates a DPDK instance.
	#[inline(always)]
	pub fn as_rte_pci_addr(&self) -> rte_pci_addr
	{
		rte_pci_addr
		{
			domain: self.0.domain,
			bus: self.0.bus,
			devid: self.0.devid,
			function: self.0.function,
		}
	}
	
	/// Domain of this address.
	#[inline(always)]
	pub fn domain(&self) -> u16
	{
		debug_assert!(self.0.domain <= ::std::u16::MAX as u32, "domain exceeds maximum");
		self.0.domain as u16
	}
	
	/// Bus of this address.
	#[inline(always)]
	pub fn bus(&self) -> u8
	{
		self.0.bus
	}
	
	/// In PCIe, effectively a slot number within the bus.
	#[inline(always)]
	pub fn device_number(&self) -> u8
	{
		self.0.devid
	}
	
	/// Function of this address.
	///
	/// An example of a function might be a port on an ethernet adaptor; an ethernet adaptor with 4 ports might have 4 functions, each of which would have a `DpdkPciDeviceAddress`.
	#[inline(always)]
	pub fn function(&self) -> u8
	{
		self.0.function
	}

	#[inline(always)]
	pub(crate) fn is_rte_pci_addr(&self, other: &rte_pci_addr) -> bool
	{
		self.domain == other.domain && self.bus == other.bus && self.device_identifier == other.devid && self.function == other.function
	}

	#[inline(always)]
	pub(crate) fn underlying_ethernet_port(&self) -> Option<((EthernetPort, rte_eth_dev))>
	{
		let mut port_identifier = 0;
		while port_identifier < EthernetPort::MaximumEthernetPortsU8
		{
			if let Some(ethernet_port) = EthernetPort::new(port_identifier)
			{
				let underlying_rte_eth_dev = ethernet_port.underlying_ethernet_device();
				if !underlying_rte_eth_dev.device.is_null()
				{
					unsafe
					{
						let pci_device = *(rust_RTE_DEV_TO_PCI(underlying_rte_eth_dev.device));
						if self.is_rte_pci_addr(&pci_device.addr)
						{
							return Some((ethernet_port, underlying_rte_eth_dev));
						}
					}
				}
			}
			port_identifier += 1;
		}
		None
	}

	#[inline(always)]
	pub(crate) fn as_c_string(&self) -> CString
	{
		CString::new(self.to_string()).unwrap()
	}

	#[inline(always)]
	pub(crate) fn probe(&self) -> Result<(), i32>
	{
		let dpdk_address = self.as_rte_pci_addr();
		let result = unsafe { rte_pci_probe_one(&dpdk_address) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else
		{
			match result
			{
				negative if negative < 0 => Err(result),

				illegal @ _ => panic!("rte_pci_probe_one() returned illegal result '{}'", illegal),
			}
		}
	}

	#[inline(always)]
	pub(crate) fn detach(&self) -> Result<(), i32>
	{
		let dpdk_address = self.as_rte_pci_addr();
		let result = unsafe { rte_pci_detach(&dpdk_address) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else
		{
			match result
			{
				negative if negative < 0 => Err(result),

				illegal @ _ => panic!("rte_pci_detach() returned illegal result '{}'", illegal),
			}
		}
	}
}
