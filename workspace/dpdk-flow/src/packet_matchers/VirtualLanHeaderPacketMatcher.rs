// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A matcher that matches either an IEEE 802.1Q Virtual LAN header or an IEEE 802.1ad QinQ Virtual LAN header.
///
/// If precedeeded by an EthernetHeaderPacketMatcher, then matches on an IEEE 802.1ad QinQ Virtual LAN header's inner Tag Control Information (TCI).
#[derive(Debug)]
#[repr(transparent)]
pub struct VirtualLanHeaderPacketMatcher
{
	underlying: rte_flow_item_vlan,
}

impl Clone for VirtualLanHeaderPacketMatcher
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		generic_clone(&self)
	}
}

impl PartialEq for VirtualLanHeaderPacketMatcher
{
	#[inline(always)]
	fn eq(&self, rhs: &Self) -> bool
	{
		generic_equals(&self, &rhs)
	}
}

impl Eq for VirtualLanHeaderPacketMatcher
{
}

impl PartialOrd for VirtualLanHeaderPacketMatcher
{
	#[inline(always)]
	fn partial_cmp(&self, rhs: &Self) -> Option<Ordering>
	{
		Some(self.cmp(rhs))
	}
}

impl Ord for VirtualLanHeaderPacketMatcher
{
	#[inline(always)]
	fn cmp(&self, rhs: &Self) -> Ordering
	{
		generic_compare(&self, &rhs)
	}
}

impl Hash for VirtualLanHeaderPacketMatcher
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, hasher: &mut H)
	{
		generic_hash::<H, _>(self, hasher)
	}
}

impl PacketMatcher for VirtualLanHeaderPacketMatcher
{
	type DpdkType = rte_flow_item_vlan;
	
	const Type: rte_flow_item_type = rte_flow_item_type::RTE_FLOW_ITEM_TYPE_VLAN;
	
	const IsMeta: bool = false;
	
	#[inline(always)]
	fn mask() -> &'static Self::DpdkType
	{
		unsafe { &rte_flow_item_vlan_mask }
	}
}

impl VirtualLanHeaderPacketMatcher
{
	/// A `tag_control_information` of 0x0FFF matches all values; the top 4-bit nibble of `tag_control_information` is ignored.
	/// A `inner_ether_type_or_tag_protocol_identifier` of 0x0000 matches all EtherTypes and tag protocol identifiers (TPID)s. If it is EtherType::QinQVlanTagging (`0x88A8`), then this will match a IEEE 802.1ad QinQ Virtual LAN.
	#[inline(always)]
	pub fn new(tag_control_information: TagControlInformation, inner_ether_type_or_tag_protocol_identifier: EtherType) -> Self
	{
		Self
		{
			underlying: rte_flow_item_vlan
			{
				tci:
				{
					let into: NetworkEndianU16 = tag_control_information.into();
					into.to_network_endian()
				},
				inner_type:
				{
					let into: NetworkEndianU16 = inner_ether_type_or_tag_protocol_identifier.into();
					into.to_network_endian()
				},
			}
		}
	}
}
