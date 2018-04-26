// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents an Ether type.
#[repr(C, packed)]
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct EtherType(pub NetworkByteOrderEndianU16);

impl EtherType
{
	/// Size of EtherType field.
	pub const SizeU32: u32 = 2;
	
	/// IEEE 802.3x-1997 frame size change over.
	#[cfg(target_endian = "big")] pub const Minimum: NetworkByteOrderEndianU16 = NetworkByteOrderEndianU16::from_network_byte_order_value(0x0600);
	
	/// IEEE 802.3x-1997 frame size change over.
	#[cfg(target_endian = "little")] pub const Minimum: NetworkByteOrderEndianU16 = NetworkByteOrderEndianU16::from_network_byte_order_value(0x0006);
	
	/// Slow protocols Link Aggregation Control Protocol (LACP) and Marker.
	///
	/// IEEE Std 802.3-2015, Annex 57A.
	#[cfg(target_endian = "big")] pub const Slow: Self = EtherType(NetworkByteOrderEndianU16::from_network_byte_order_value(0x8809));
	
	/// Slow protocols Link Aggregation Control Protocol (LACP) and Marker.
	///
	/// IEEE Std 802.3-2015, Annex 57A.
	#[cfg(target_endian = "little")] pub const Slow: Self = EtherType(NetworkByteOrderEndianU16::from_network_byte_order_value(0x0988));
	
	/// Internet protocol (IP) version 4 ether type.
	#[cfg(target_endian = "big")] pub const InternetProtocolVersion4: Self = EtherType(NetworkByteOrderEndianU16::from_network_byte_order_value(0x0800));
	
	/// Internet protocol (IP) version 4 ether type.
	#[cfg(target_endian = "little")] pub const InternetProtocolVersion4: Self = EtherType(NetworkByteOrderEndianU16::from_network_byte_order_value(0x0008));
	
	/// Internet protocol (IP) version 6 ether type.
	#[cfg(target_endian = "big")] pub const InternetProtocolVersion6: Self = EtherType(NetworkByteOrderEndianU16::from_network_byte_order_value(0x86DD));
	
	/// Internet protocol (IP) version 6 ether type.
	#[cfg(target_endian = "little")] pub const InternetProtocolVersion6: Self = EtherType(NetworkByteOrderEndianU16::from_network_byte_order_value(0x86DD));
	
	/// Address resolution protocol (ARP) ether type.
	#[cfg(target_endian = "big")] pub const AddressResolutionProtocol: Self = EtherType(NetworkByteOrderEndianU16::from_network_byte_order_value(0x0806));
	
	/// Address resolution protocol (ARP) ether type.
	#[cfg(target_endian = "little")] pub const AddressResolutionProtocol: Self = EtherType(NetworkByteOrderEndianU16::from_network_byte_order_value(0x0608));
	
	/// Reverse address resolution protocol (RARP) ether type.
	#[cfg(target_endian = "big")] pub const ReverseAddressResolutionProtocol: Self = EtherType(NetworkByteOrderEndianU16::from_network_byte_order_value(0x8035));
	
	/// Reverse address resolution protocol (RARP) ether type.
	#[cfg(target_endian = "little")] pub const ReverseAddressResolutionProtocol: Self = EtherType(NetworkByteOrderEndianU16::from_network_byte_order_value(0x3580));
	
	/// IEEE 802.1Q VLAN tagging.
	#[cfg(target_endian = "big")] pub const VlanTagging: Self = EtherType(NetworkByteOrderEndianU16::from_network_byte_order_value(0x8100));
	
	/// IEEE 802.1Q VLAN tagging.
	#[cfg(target_endian = "little")] pub const VlanTagging: Self = EtherType(NetworkByteOrderEndianU16::from_network_byte_order_value(0x0081));
	
	/// IEEE 802.1ad QinQ tagging.
	#[cfg(target_endian = "big")] pub const QinQVlanTagging: Self = EtherType(NetworkByteOrderEndianU16::from_network_byte_order_value(0x88A8));
	
	/// IEEE 802.1ad QinQ tagging.
	#[cfg(target_endian = "little")] pub const QinQVlanTagging: Self = EtherType(NetworkByteOrderEndianU16::from_network_byte_order_value(0xA888));
	
	/// IEEE1588 / 802.1AS Precise time protocol (PTP).
	#[cfg(target_endian = "big")] pub const PreciseTimeProtocol: Self = EtherType(NetworkByteOrderEndianU16::from_network_byte_order_value(0x88F7));
	
	/// IEEE1588 / 802.1AS Precise time protocol (PTP).
	#[cfg(target_endian = "little")] pub const PreciseTimeProtocol: Self = EtherType(NetworkByteOrderEndianU16::from_network_byte_order_value(0xF788));
	
	/// Transparent Ethernet Bridging.
	#[cfg(target_endian = "big")] pub const TransparentEthernetBridging: Self = EtherType(NetworkByteOrderEndianU16::from_network_byte_order_value(0x6558));
	
	/// Transparent Ethernet Bridging.
	#[cfg(target_endian = "little")] pub const TransparentEthernetBridging: Self = EtherType(NetworkByteOrderEndianU16::from_network_byte_order_value(0x5865));
	
	/// Link local discovery protocol (LLDP).
	#[cfg(target_endian = "big")] pub const LinkLocalDiscoveryProtocol: Self = EtherType(NetworkByteOrderEndianU16::from_network_byte_order_value(0x88CC));
	
	/// Link local discovery protocol (LLDP).
	#[cfg(target_endian = "little")] pub const LinkLocalDiscoveryProtocol: Self = EtherType(NetworkByteOrderEndianU16::from_network_byte_order_value(0xCC88));
	
	/// Is this a valid Ether Type (as opposed to a legacy ethernet frame size).
	#[inline(always)]
	pub fn is_valid_ether_type(self) -> bool
	{
		self.0.to_network_byte_order_value() >= Self::Minimum.to_native_byte_order_value()
	}
	
	/// Use this to eliminate unwanted ARP traffic.
	#[inline(always)]
	pub fn is_not_internet_protocol_version_4(self) -> bool
	{
		self.0 != Self::InternetProtocolVersion4.0
	}
}
