// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct DeviceAddress
{
	domain: u16,
	bus: u8,
	deviceIdentifier: u8,
	function: u4,
}

impl DeviceName for DeviceAddress
{
	#[inline]
	fn to_string(&self) -> String
	{
		format!("{:04x}:{:02x}:{:02x}.{:01x}", self.domain, self.bus, self.deviceIdentifier, self.function)
	}
}

impl DeviceAddress
{
	const DefaultDomain: u16 = 0x0000;
	const DefaultBus: u8 = 0x00;
	
	pub fn fromString(value: &str) -> Result<DeviceAddress, DeviceAddressStringParseError>
	{
		let mut matchCount = 0;
		
		let length = value.len();
		if length != NumberOfBytesInPciAddressString
		{
			return Err(DeviceAddressStringParseError::LengthIsWrong(length));
		}
		
		let mut split = value.split(|character|
		{
			match matchCount
			{
				0 | 1 =>
				{
					matchCount += 1;
					character == ':'
				},
				
				2 =>
				{
					matchCount += 1;
					character == '.'
				},
				
				_ => false,
			}
		});
		
		let domain = match split.next()
		{
			None => return Err(DeviceAddressStringParseError::NoDomain),
			Some(value) =>
			{
				match u16::from_str_radix(value, 16)
				{
					Err(cause) => return Err(DeviceAddressStringParseError::CouldNotParseDomain(value.to_owned(), cause)),
					Ok(value) => value,
				}
			}
		};
		
		let bus = match split.next()
		{
			None => return Err(DeviceAddressStringParseError::NoBus),
			Some(value) =>
			{
				match u8::from_str_radix(value, 16)
				{
					Err(cause) => return Err(DeviceAddressStringParseError::CouldNotParseBus(value.to_owned(), cause)),
					Ok(value) => value,
				}
			}
		};
		
		let deviceIdentifier = match split.next()
		{
			None => return Err(DeviceAddressStringParseError::NoDeviceIdentifier),
			Some(value) =>
			{
				match u8::from_str_radix(value, 16)
				{
					Err(cause) => return Err(DeviceAddressStringParseError::CouldNotParseDeviceIdentifier(value.to_owned(), cause)),
					Ok(value) => value,
				}
			}
		};
		
		let function = match split.next()
		{
			None => return Err(DeviceAddressStringParseError::NoFunction),
			Some(value) =>
			{
				match u8::from_str_radix(value, 16)
				{
					Err(cause) => return Err(DeviceAddressStringParseError::CouldNotParseFunction(value.to_owned(), cause)),
					Ok(value) => if value > 15
					{
						return Err(DeviceAddressStringParseError::FunctionExceeds4BitValue(value))
					}
					else
					{
						value
					},
				}
			}
		};
		
		Ok(DeviceAddress::fromDomainBusDeviceIdAndFunction(domain, bus, deviceIdentifier, function))
	}
	
	// ls -ls /sys/devices/pci0000\:00  where 0000 before the colon is the domain 0x0000 and :00 is the bus 0x00
	// `bus` is normally 0x00
	#[inline(always)]
	pub fn fromBusDeviceIdAndFunctionAssumingDefaultDomainAndDefaultBus(deviceIdentifier: u8, function: u4) -> Self
	{
		Self::fromDomainBusDeviceIdAndFunction(Self::DefaultDomain, Self::DefaultBus, deviceIdentifier, function)
	}

	#[inline(always)]
	pub fn fromBusDeviceIdAndFunctionAssumingDefaultDomain(bus: u8, deviceIdentifier: u8, function: u4) -> Self
	{
		Self::fromDomainBusDeviceIdAndFunction(Self::DefaultDomain, bus, deviceIdentifier, function)
	}

	#[inline(always)]
	pub fn fromDomainBusDeviceIdAndFunction(domain: u16, bus: u8, deviceIdentifier: u8, function: u4) -> Self
	{
		debug_assert!(function < 16, "function must be between 0 to 15 inclusive (it's 4-bit) and not be '{}'", function);
		
		DeviceAddress
		{
			domain: domain,
			bus: bus,
			deviceIdentifier: deviceIdentifier,
			function: function,
		}
	}
	
	#[inline(always)]
	pub fn from_rte_pci_addr(value: &rte_pci_addr) -> Self
	{
		DeviceAddress
		{
			domain: value.domain,
			bus: value.bus,
			deviceIdentifier: value.devid,
			function: value.function,
		}
	}
	
	#[inline(always)]
	pub fn as_rte_pci_addr(&self) -> rte_pci_addr
	{
		rte_pci_addr
		{
			domain: self.domain,
			bus: self.bus,
			devid: self.deviceIdentifier,
			function: self.function,
		}
	}
	
	#[inline(always)]
	pub fn is_rte_pci_addr(&self, other: &rte_pci_addr) -> bool
	{
		self.domain == other.domain && self.bus == other.bus && self.deviceIdentifier == other.devid && self.function == other.function
	}
	
	#[inline(always)]
	pub fn underlyingEthernetPort(&self) -> Option<((EthernetPort, rte_eth_dev))>
	{
		let mut portIdentifier = 0;
		while portIdentifier < EthernetPort::MaximumEthernetPortsU8
		{
			if let Some(ethernetPort) = EthernetPort::new(portIdentifier)
			{
				let underlying_rte_eth_dev = ethernetPort.underlyingEthernetDevice();
				if !underlying_rte_eth_dev.device.is_null()
				{
					unsafe
					{
						let pciDevice = *(rust_RTE_DEV_TO_PCI(underlying_rte_eth_dev.device));
						if self.is_rte_pci_addr(&pciDevice.addr)
						{
							return Some((ethernetPort, underlying_rte_eth_dev));
						}
					}
				}
			}
			portIdentifier += 1;
		}
		None
	}
	
	#[inline]
	pub fn asCString(&self) -> CString
	{
		CString::new(self.to_string()).unwrap()
	}
	
	#[inline(always)]
	pub fn probe(&self) -> Result<(), i32>
	{
		let dpdkAddress = self.as_rte_pci_addr();
		let result = unsafe { ::dpdk_sys::rte_eal_pci_probe_one(&dpdkAddress) };
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
	pub fn detach(&self) -> Result<(), i32>
	{
		let dpdkAddress = self.as_rte_pci_addr();
		let result = unsafe { ::dpdk_sys::rte_eal_pci_detach(&dpdkAddress) };
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
