// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


//noinspection SpellCheckingInspection
#[allow(non_camel_case_types)]
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HardwareType
{
	Ethernet2 = 1,
	Experimental_Ethernet = 2,
	Amateur_Radio_AX_25 = 3,
	Proteon_ProNET_Token_Ring = 4,
	Chaos = 5,
	IEEE_802_Networks = 6,
	ARCNET = 7,
	Hyperchannel = 8,
	Lanstar = 9,
	Autonet_Short_Address = 10,
	LocalTalk = 11,
	LocalNet = 12,
	Ultra_link = 13,
	SMDS = 14,
	Frame_Relay = 15,
	Asynchronous_Transmission_Mode1 = 16,
	HDLC = 17,
	Fibre_Channel = 18,
	Asynchronous_Transmission_Mode2 = 19,
	Serial_Line = 20,
	Asynchronous_Transmission_Mode3 = 21,
	MIL_STD_188_220 = 22,
	Metricom = 23,
	IEEE_1394_1995 = 24,
	MAPOS = 25,
	Twinaxial = 26,
	EUI_64 = 27,
	HIPARP = 28,
	IP_and_ARP_over_ISO_7816_3 = 29,
	ARPSec = 30,
	IPsec_Tunnel = 31,
	InfiniBand = 32,
	TIA_102_Project_25_Common_Air_Interface = 33,
	Wiegand_Interface = 34,
	Pure_IP = 35,
	HW_EXP1 = 36,
	HFI = 37,
	HW_EXP2 = 256,
	AEthernet = 257,
}

impl HardwareType
{
	#[inline(always)]
	pub fn isNotEthernet2(hardwareTypeBigEndianValue: u16) -> bool
	{
		hardwareTypeBigEndianValue != HardwareType::Ethernet2.toNetworkByteOrder()
	}
	
	/// AKA Big Endian
	#[inline(always)]
	pub fn toNetworkByteOrder(self) -> u16
	{
		(self as u16).to_be()
	}
	
	#[inline(always)]
	pub fn parseFromNetworkByteOrderIsEthernet2(bigEndianValue: u16) -> bool
	{
		// This operation occurs so frequently we avoid an if statement and object construction
		bigEndianValue == HardwareType::Ethernet2.toNetworkByteOrder()
	}
	
	#[inline(always)]
	pub fn parseFromNetworkByteOrder(bigEndianValue: u16) -> Result<HardwareType, ParseError>
	{
		match u16::from_be(bigEndianValue)
		{
			0 => Err(ParseError::Reserved(0)),
			1 => Ok(HardwareType::Ethernet2),
			2 => Ok(HardwareType::Experimental_Ethernet),
			3 => Ok(HardwareType::Amateur_Radio_AX_25),
			4 => Ok(HardwareType::Proteon_ProNET_Token_Ring),
			5 => Ok(HardwareType::Chaos),
			6 => Ok(HardwareType::IEEE_802_Networks),
			7 => Ok(HardwareType::ARCNET),
			8 => Ok(HardwareType::Hyperchannel),
			9 => Ok(HardwareType::Lanstar),
			10 => Ok(HardwareType::Autonet_Short_Address),
			11 => Ok(HardwareType::LocalTalk),
			12 => Ok(HardwareType::LocalNet),
			13 => Ok(HardwareType::Ultra_link),
			14 => Ok(HardwareType::SMDS),
			15 => Ok(HardwareType::Frame_Relay),
			16 => Ok(HardwareType::Asynchronous_Transmission_Mode1),
			17 => Ok(HardwareType::HDLC),
			18 => Ok(HardwareType::Fibre_Channel),
			19 => Ok(HardwareType::Asynchronous_Transmission_Mode2),
			20 => Ok(HardwareType::Serial_Line),
			21 => Ok(HardwareType::Asynchronous_Transmission_Mode3),
			22 => Ok(HardwareType::MIL_STD_188_220),
			23 => Ok(HardwareType::Metricom),
			24 => Ok(HardwareType::IEEE_1394_1995),
			25 => Ok(HardwareType::MAPOS),
			26 => Ok(HardwareType::Twinaxial),
			27 => Ok(HardwareType::EUI_64),
			28 => Ok(HardwareType::HIPARP),
			29 => Ok(HardwareType::IP_and_ARP_over_ISO_7816_3),
			30 => Ok(HardwareType::ARPSec),
			31 => Ok(HardwareType::IPsec_Tunnel),
			32 => Ok(HardwareType::InfiniBand),
			33 => Ok(HardwareType::TIA_102_Project_25_Common_Air_Interface),
			34 => Ok(HardwareType::Wiegand_Interface),
			35 => Ok(HardwareType::Pure_IP),
			36 => Ok(HardwareType::HW_EXP1),
			37 => Ok(HardwareType::HFI),
			unassigned @ 38 ... 255 => Err(ParseError::Unassigned(unassigned)),
			256 => Ok(HardwareType::HW_EXP2),
			257 => Ok(HardwareType::AEthernet),
			unassigned @ 258 ... 65534 => Err(ParseError::Unassigned(unassigned)),
			65535 => Err(ParseError::Reserved(65535)),
			unassigned @ _ => Err(ParseError::Unassigned(unassigned)),
		}
	}
	
}
