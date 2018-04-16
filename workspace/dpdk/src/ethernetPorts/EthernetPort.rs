// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EthernetPort
{
	portIdentifier: EthernetPortIdentifier,
}

impl Display for EthernetPort
{
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "{}", self.portIdentifier())
	}
}

impl EthernetPort
{
	pub const MaximumEthernetPorts: usize = RTE_MAX_ETHPORTS;
	pub const MaximumEthernetPortsU8: u8 = Self::MaximumEthernetPorts as u8;

	/// Returns None if not attached
	#[inline(always)]
	pub fn new(portIdentifier: EthernetPortIdentifier) -> Option<Self>
	{
		debug_assert!(portIdentifier < Self::MaximumEthernetPortsU8, "portIdentifier '{}' equals or exceeds MaximumEthernetPortsU8 '{}'", portIdentifier, Self::MaximumEthernetPortsU8);

		if Self::isAttachedPort(portIdentifier)
		{
			Some
			(
				EthernetPort
				{
					portIdentifier,
				}
			)
		}
		else
		{
			None
		}
	}

	#[inline(always)]
	pub fn portIdentifier(&self) -> EthernetPortIdentifier
	{
		self.portIdentifier
	}
}

// We do not implement wrappers for the RSS functions 'rte_eth_dev_rss_hash_update' and 'rte_eth_dev_rss_hash_conf_get'
include!("EthernetPort/Attachment.rs");
include!("EthernetPort/Bypass.rs");
include!("EthernetPort/Eeprom.rs");
include!("EthernetPort/Epoll.rs");
include!("EthernetPort/EventCallback.rs");
include!("EthernetPort/ExtendedStatistics.rs");
include!("EthernetPort/Filters.rs");
include!("EthernetPort/FlowControl.rs");
include!("EthernetPort/Led.rs");
include!("EthernetPort/LinkUpDown.rs");
include!("EthernetPort/MaximumTransmissionUnit.rs");
include!("EthernetPort/MediaAccessControl.rs");
include!("EthernetPort/MulticastReceive.rs");
include!("EthernetPort/Promiscuity.rs");
include!("EthernetPort/RetaIndirectionTable.rs");
include!("EthernetPort/StartStopClose.rs");
include!("EthernetPort/Statistics.rs");
include!("EthernetPort/Timestamping.rs");
include!("EthernetPort/TrafficMirroring.rs");
include!("EthernetPort/UdpTunnelOffloading.rs");
include!("EthernetPort/VirtualFunction.rs");
include!("EthernetPort/VirtualLan.rs");

// Device Information
impl EthernetPort
{
	#[inline(always)]
	pub fn parentNumaSocketId(&self) -> Option<NumaSocketId>
	{
		let result = unsafe { rte_eth_dev_socket_id(self.portIdentifier()) };
		if unlikely(result < 0)
		{
			match result
			{
				-1 => panic!("The port identifier '{}' is invalid", self.portIdentifier()),

				_ => panic!("Unexpected error code '{}' from rte_eth_dev_socket_id()", result),
			}
		}
		else
		{
			// The documentation of DPDK suggests 0 means indeterminate, but that can't be right as 0 is a valid NUMA socket id, and the code doesn't seem to support this
			NumaSocketId::from_i32(result)
		}
	}

	// Only call this AFTER the device and receive queue is started for accuracy
	#[inline(always)]
	pub fn allSupportedPacketTypes(&self) -> HashSet<PacketType>
	{
		let mut supportedPacketTypes = HashSet::with_capacity(64);

		let packetTypesSet = self.supportedPacketTypes(PacketTypeMask::All);

		for packetType in packetTypesSet.iter()
		{
			supportedPacketTypes.insert(*packetType);
		}

		supportedPacketTypes.shrink_to_fit();

		supportedPacketTypes
	}

	#[inline(always)]
	pub fn information(&self) -> EthernetPortInformation
	{
		let mut deviceInformation: rte_eth_dev_info = unsafe { uninitialized() };

		unsafe { rte_eth_dev_info_get(self.portIdentifier(), &mut deviceInformation) };

		let underlying_ethernet_device = self.underlying_ethernet_device();

		let device_name = self.getDeviceName().expect("Device name is not UTF-8 compliant");

		let parentNumaSocketId = self.parentNumaSocketId();

		let mut supportedFilterTypes = HashSet::with_capacity(FilterType::All.len());
		for filterType in FilterType::All.iter()
		{
			if self.supportsFilter(*filterType)
			{
				supportedFilterTypes.insert(*filterType);
			}
		}
		supportedFilterTypes.shrink_to_fit();

		let dataCentreBridgingInformation = self.getDataCentreBridgingInformation().ok();

		let eepromSize = self.getEepromSize().ok();

		let eepromInformation = self.getEepromInformation().ok();

		let deviceRegisters = self.getDeviceRegistersInformation().ok();

		EthernetPortInformation::new(*self, deviceInformation, underlying_ethernet_device, device_name, parentNumaSocketId, supportedFilterTypes, dataCentreBridgingInformation, eepromSize, eepromInformation, deviceRegisters)
	}

	/// Note: Statistics are not available until after a link comes up
	/// Can take up to 9s to return
	#[inline(always)]
	pub fn linkStatusWaitingUpToNineSeconds(&self) -> Result<LinkStatus, ()>
	{
		let mut linkDetails = unsafe { zeroed() };
		unsafe { rte_eth_link_get(self.portIdentifier(), &mut linkDetails) };
		match LinkStatus::from_rte_eth_link(&linkDetails)
		{
			Some(linkStatus) => Ok(linkStatus),
			None => Err(()),
		}
	}

	/// Note: Statistics are not available until after a link comes up
	/// None can mean either the link is Down or the link is not yet Up
	#[inline(always)]
	pub fn linkStatusWithoutWaitingUpToNineSeconds(&self) -> Option<LinkStatus>
	{
		let mut linkDetails = unsafe { zeroed() };
		unsafe { rte_eth_link_get_nowait(self.portIdentifier(), &mut linkDetails) };
		LinkStatus::from_rte_eth_link(&linkDetails)
	}

	#[inline(always)]
	fn getDataCentreBridgingInformation(&self) -> Result<rte_eth_dcb_info, UnsupportedByHardwareError>
	{
		let mut information = unsafe { uninitialized() };

		let result = unsafe { rte_eth_dev_get_dcb_info(self.portIdentifier(), &mut information) };
		if likely(result == 0)
		{
			Ok(information)
		}
		else
		{
			forget(information);

			match result
			{
				NegativeE::ENOTSUP => Err(UnsupportedByHardwareError::IsUnsupportedByTheHardware),

				NegativeE::ENODEV => panic!("The port identifier '{}' is invalid", self.portIdentifier()),

				_ => panic!("Unexpected error code '{}' from rte_eth_dev_get_dcb_info()", result),
			}
		}
	}

	#[inline(always)]
	fn supportedPacketTypes(&self, packetTypeMask: PacketTypeMask) -> HashSet<PacketType>
	{
		let mask = packetTypeMask.bits();

		let number = match unsafe { rte_eth_dev_get_supported_ptypes(self.portIdentifier(), mask, null_mut(), 0) }
		{
			number if number >= 0 => number,

			NegativeE::ENODEV => panic!("The port identifier '{}' is invalid", self.portIdentifier()),
			result @ _ => panic!("Unexpected error code '{}' from rte_eth_dev_get_supported_ptypes() when trying to find supported number", result),
		};

		let mut packetTypes = Vec::with_capacity(number as usize);

		match unsafe { rte_eth_dev_get_supported_ptypes(self.portIdentifier(), mask, packetTypes.as_mut_ptr(), number) }
		{
			asExpected if asExpected == number =>
			{
				let mut setOfPacketTypes = HashSet::with_capacity(number as usize);
				for packetType in packetTypes
				{
					let packetTypeParsed = PacketType::from_bits_truncate(packetType);
					let isOriginal = setOfPacketTypes.insert(packetTypeParsed);
					debug_assert!(isOriginal, "packetType '{:?}' is a duplicate", packetTypeParsed);
				}
				setOfPacketTypes
			},

			wrongNumber if wrongNumber >= 0 => panic!("rte_eth_dev_get_supported_ptypes() changed the returned number from '{}' to '{}'", number, wrongNumber),

			NegativeE::ENODEV => panic!("The port identifier '{}' is invalid", self.portIdentifier()),
			result @ _ => panic!("Unexpected error code '{}' from rte_eth_dev_get_supported_ptypes() when trying to find supported number", result),
		}
	}

	// Returns a tuple of width, length, version, register data
	#[inline(always)]
	fn getDeviceRegistersInformation(&self) -> Result<(u32, u32, u32, Vec<u8>), UnsupportedByHardwareError>
	{
		let mut findWidthAndLength = rte_dev_reg_info
		{
			data: null_mut(),
			offset: 0,
			length: 0,
			width: 0,
			version: 0,
		};

		let result = unsafe { rte_eth_dev_get_reg_info(self.portIdentifier(), &mut findWidthAndLength) };
		if likely(result == 0)
		{
			let width = findWidthAndLength.width;
			let length = findWidthAndLength.length;
			let bufferSize = width * length;
			let mut registers: Vec<u8> = Vec::with_capacity(bufferSize as usize);
			findWidthAndLength.data = registers.as_mut_ptr() as *mut c_void;

			let result = unsafe { rte_eth_dev_get_reg_info(self.portIdentifier(), &mut findWidthAndLength) };
			if likely(result == 0)
			{
				unsafe
				{
					registers.set_len(bufferSize as usize)
				}
				Ok((width, length, findWidthAndLength.version, registers))
			}
			else
			{
				match result
				{
					NegativeE::ENOTSUP => Err(UnsupportedByHardwareError::IsUnsupportedByTheHardware),

					NegativeE::ENODEV => panic!("The port identifier '{}' is invalid - how if it worked for the first call to rte_eth_dev_get_reg_info()?", self.portIdentifier()),

					negative if negative < 0 => Err(UnsupportedByHardwareError::IsUnsupportedByTheHardware),

					_ => panic!("Unexpected error code '{}' from second call to rte_eth_dev_get_reg_info()", result),
				}
			}
		}
		else
		{
			match result
			{
				NegativeE::ENOTSUP => Err(UnsupportedByHardwareError::IsUnsupportedByTheHardware),

				NegativeE::ENODEV => panic!("The port identifier '{}' is invalid", self.portIdentifier()),

				negative if negative < 0 => Err(UnsupportedByHardwareError::IsUnsupportedByTheHardware),

				_ => panic!("Unexpected error code '{}' from rte_eth_dev_get_reg_info()", result),
			}
		}
	}
}

// Device Details
impl EthernetPort
{
	#[inline(always)]
	pub fn parseDeviceName(mut device_name: Vec<u8>, pointerToDeviceName: *mut c_char) -> Result<String, FromUtf8Error>
	{
		let length = unsafe { strnlen(pointerToDeviceName, RTE_ETH_NAME_MAX_LEN) };
		unsafe { device_name.set_len(length) };
		device_name.shrink_to_fit();
		String::from_utf8(device_name)
	}

	#[inline(always)]
	fn initialiseDeviceNameBuffer() -> (Vec<u8>, *mut c_char)
	{
		let mut device_name: Vec<u8> = Vec::with_capacity(RTE_ETH_NAME_MAX_LEN);
		let pointerToDeviceName = device_name.as_mut_ptr() as *mut c_char;
		(device_name, pointerToDeviceName)
	}

	#[inline(always)]
	pub fn asBondedEthernetPort(&self) -> Option<BondedEthernetPort>
	{
		BondedEthernetPort::fromEthernetPort(*self)
	}

	#[inline(always)]
	pub fn underlying_ethernet_device(&self) -> rte_eth_dev
	{
		unsafe { rte_eth_devices[self.portIdentifier() as usize] }
	}

	#[inline(always)]
	pub fn underlying_dpdk_pci_device(&self) -> Option<DpdkPciDevice>
	{
		DpdkPciDevice::for_ethernet_port(self.portIdentifier())
	}

	#[inline(always)]
	pub fn getForDeviceName<D: DeviceName>(device_name: &D) -> Option<EthernetPort>
	{
		let device_name = device_name.to_string();

		let cDeviceName = CString::new(device_name).expect("device_name contained an interior ASCII NUL");
		let mut portIdentifier = unsafe { uninitialized() };
		let result = unsafe { rte_eth_dev_get_port_by_name(cDeviceName.as_ptr(), &mut portIdentifier) };
		if likely(result == 0)
		{
			// Should always return Some()
			Self::new(portIdentifier)
		}
		else
		{
			forget(portIdentifier);

			match result
			{
				NegativeE::ENODEV | NegativeE::EINVAL => None,

				_ => panic!("Unexpected error code '{}' from rte_eth_dev_get_port_by_name()", result),
			}
		}
	}

	#[inline(always)]
	pub fn getDeviceName(&self) -> Result<String, FromUtf8Error>
	{
		let (device_name, pointerToDeviceName) = Self::initialiseDeviceNameBuffer();
		let result = unsafe { rte_eth_dev_get_name_by_port(self.portIdentifier(), pointerToDeviceName) };

		if likely(result == 0)
		{
			Self::parseDeviceName(device_name, pointerToDeviceName)
		}
		else
		{
			match result
			{
				NegativeE::EINVAL => panic!("The port identifier '{}' is invalid", self.portIdentifier()),

				_ => panic!("Unexpected error code '{}' from rte_eth_dev_get_name_by_port()", result),
			}
		}
	}
}
