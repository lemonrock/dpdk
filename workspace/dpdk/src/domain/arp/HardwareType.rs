// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


//noinspection SpellCheckingInspection
/// ARP Hardware type.
///
/// Representation as an u16 is in native endian order.
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
	/// Convert ARP hardware type to network byte order.
	#[inline(always)]
	pub fn to_network_byte_order(self) -> NetworkByteOrderEndianU16
	{
		NetworkByteOrderEndianU16::from_native_byte_order_value(self as u16)
	}
	
	/// Is not Ethernet 2?
	#[inline(always)]
	pub fn is_not_ethernet_2(hardware_type: NetworkByteOrderEndianU16) -> bool
	{
		hardware_type != HardwareType::Ethernet2.to_network_byte_order()
	}
	
	/// Parse ARP operation from network byte order; special cased for Ethernet2.
	#[inline(always)]
	pub fn parse_from_network_byte_order_is_ethernet_2(big_endian_value: NetworkByteOrderEndianU16) -> bool
	{
		// This operation occurs so frequently we avoid an if statement and object construction
		big_endian_value == HardwareType::Ethernet2.to_network_byte_order()
	}
	
	/// Parse ARP operation from network byte order.
	#[inline(always)]
	pub fn parse_from_network_byte_order(hardware_type: NetworkByteOrderEndianU16) -> Result<HardwareType, ParseError>
	{
		use self::ParseError::*;
		use self::HardwareType::*;
		
		match hardware_type.to_native_byte_order_value()
		{
			0 => Err(Reserved(0)),
			1 => Ok(Ethernet2),
			2 => Ok(Experimental_Ethernet),
			3 => Ok(Amateur_Radio_AX_25),
			4 => Ok(Proteon_ProNET_Token_Ring),
			5 => Ok(Chaos),
			6 => Ok(IEEE_802_Networks),
			7 => Ok(ARCNET),
			8 => Ok(Hyperchannel),
			9 => Ok(Lanstar),
			10 => Ok(Autonet_Short_Address),
			11 => Ok(LocalTalk),
			12 => Ok(LocalNet),
			13 => Ok(Ultra_link),
			14 => Ok(SMDS),
			15 => Ok(Frame_Relay),
			16 => Ok(Asynchronous_Transmission_Mode1),
			17 => Ok(HDLC),
			18 => Ok(Fibre_Channel),
			19 => Ok(Asynchronous_Transmission_Mode2),
			20 => Ok(Serial_Line),
			21 => Ok(Asynchronous_Transmission_Mode3),
			22 => Ok(MIL_STD_188_220),
			23 => Ok(Metricom),
			24 => Ok(IEEE_1394_1995),
			25 => Ok(MAPOS),
			26 => Ok(Twinaxial),
			27 => Ok(EUI_64),
			28 => Ok(HIPARP),
			29 => Ok(IP_and_ARP_over_ISO_7816_3),
			30 => Ok(ARPSec),
			31 => Ok(IPsec_Tunnel),
			32 => Ok(InfiniBand),
			33 => Ok(TIA_102_Project_25_Common_Air_Interface),
			34 => Ok(Wiegand_Interface),
			35 => Ok(Pure_IP),
			36 => Ok(HW_EXP1),
			37 => Ok(HFI),
			unassigned @ 38 ... 255 => Err(Unassigned(unassigned)),
			256 => Ok(HW_EXP2),
			257 => Ok(AEthernet),
			unassigned @ 258 ... 65534 => Err(Unassigned(unassigned)),
			65535 => Err(Reserved(65535)),
			unassigned @ _ => Err(Unassigned(unassigned)),
		}
	}
	
}
