// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct PciDeviceAddress(rte_pci_addr);

impl DeviceName for PciDeviceAddress
{
	#[inline]
	fn to_string(&self) -> String
	{
		format!("{:04x}:{:02x}:{:02x}.{:01x}", self.domain, self.bus, self.device_identifier, self.function)
	}
}

impl PciDeviceAddress
{
	pub(crate) fn from_str(value: &str) -> Result<PciDeviceAddress, PciDeviceAddressStringParseError>
	{
		let mut match_count = 0;

		let length = value.len();
		if length != PciBusInformation::NumberOfBytesInPciAddressString
		{
			return Err(PciDeviceAddressStringParseError::LengthIsWrong(length));
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
			None => return Err(PciDeviceAddressStringParseError::NoDomain),
			Some(value) =>
			{
				match u16::from_str_radix(value, 16)
				{
					Err(cause) => return Err(PciDeviceAddressStringParseError::CouldNotParseDomain(value.to_owned(), cause)),
					Ok(value) => value,
				}
			}
		};

		let bus = match split.next()
		{
			None => return Err(PciDeviceAddressStringParseError::NoBus),
			Some(value) =>
			{
				match u8::from_str_radix(value, 16)
				{
					Err(cause) => return Err(PciDeviceAddressStringParseError::CouldNotParseBus(value.to_owned(), cause)),
					Ok(value) => value,
				}
			}
		};

		let device_identifier = match split.next()
		{
			None => return Err(PciDeviceAddressStringParseError::NoDeviceIdentifier),
			Some(value) =>
			{
				match u8::from_str_radix(value, 16)
				{
					Err(cause) => return Err(PciDeviceAddressStringParseError::CouldNotParseDeviceIdentifier(value.to_owned(), cause)),
					Ok(value) => value,
				}
			}
		};

		let function = match split.next()
		{
			None => return Err(PciDeviceAddressStringParseError::NoFunction),
			Some(value) =>
			{
				match u8::from_str_radix(value, 16)
				{
					Err(cause) => return Err(PciDeviceAddressStringParseError::CouldNotParseFunction(value.to_owned(), cause)),
					Ok(value) => if value > 15
					{
						return Err(PciDeviceAddressStringParseError::FunctionExceeds4BitValue(value))
					}
					else
					{
						value
					},
				}
			}
		};

		Ok(PciDeviceAddress::fromDomainBusDeviceIdAndFunction(domain, bus, device_identifier, function))
	}

	#[inline(always)]
	pub(crate) fn new(domain: u16, bus: u8, device_identifier: u8, function: u4) -> Self
	{
		debug_assert!(function < 16, "function must be between 0 to 15 inclusive (it's 4-bit) and not be '{}'", function);

		PciDeviceAddress
		(
			rte_pci_addr
			{
				domain,
				bus,
				devid: device_identifier,
				function,
			}
		)
	}

	#[inline(always)]
	pub(crate) fn from_rte_pci_addr(value: rte_pci_addr) -> Self
	{
		PciDeviceAddress(value)
	}

	#[inline(always)]
	pub(crate) fn as_rte_pci_addr(&self) -> rte_pci_addr
	{
		rte_pci_addr
		{
			domain: self.0.domain,
			bus: self.0.bus,
			devid: self.0.devid,
			function: self.0.function,
		}
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
		let result = unsafe { rte_eal_pci_probe_one(&dpdk_address) };
		if likely(result == 0)
		{
			Ok(())
		}
		else
		{
			match result
			{
				negative if negative < 0 => Err(result),

				illegal @ _ => panic!("rte_eal_pci_probe_one() returned illegal result '{}'", illegal),
			}
		}
	}

	#[inline(always)]
	pub(crate) fn detach(&self) -> Result<(), i32>
	{
		let dpdk_address = self.as_rte_pci_addr();
		let result = unsafe { rte_eal_pci_detach(&dpdk_address) };
		if likely(result == 0)
		{
			Ok(())
		}
		else
		{
			match result
			{
				negative if negative < 0 => Err(result),

				illegal @ _ => panic!("rte_eal_pci_detach() returned illegal result '{}'", illegal),
			}
		}
	}
}
