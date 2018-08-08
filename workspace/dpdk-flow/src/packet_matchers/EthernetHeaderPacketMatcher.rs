// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A matcher that matches an ethernet header.
///
/// When followed by a 'layer 2.5' matcher such as VirtualLanHeaderPacketMatcher, the Ether Type is a tag protocol identifier (TPID).
/// In this case, the ether type refers to the outer header, with the VirtualLanHeaderPacketMatcher's ether type referring to the inner Ether Type or tag protocol identifier (TPID).
#[derive(Debug)]
#[repr(transparent)]
pub struct EthernetHeaderPacketMatcher
{
	underlying: rte_flow_item_eth,
}

impl Clone for EthernetHeaderPacketMatcher
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		generic_clone(&self)
	}
}

impl PartialEq for EthernetHeaderPacketMatcher
{
	#[inline(always)]
	fn eq(&self, rhs: &Self) -> bool
	{
		generic_equals(&self, &rhs)
	}
}

impl Eq for EthernetHeaderPacketMatcher
{
}

impl PartialOrd for EthernetHeaderPacketMatcher
{
	#[inline(always)]
	fn partial_cmp(&self, rhs: &Self) -> Option<Ordering>
	{
		Some(self.cmp(rhs))
	}
}

impl Ord for EthernetHeaderPacketMatcher
{
	#[inline(always)]
	fn cmp(&self, rhs: &Self) -> Ordering
	{
		generic_compare(&self, &rhs)
	}
}

impl Hash for EthernetHeaderPacketMatcher
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, hasher: &mut H)
	{
		generic_hash::<H, _>(self, hasher)
	}
}

impl PacketMatcher for EthernetHeaderPacketMatcher
{
	type DpdkType = rte_flow_item_eth;
	
	const Type: rte_flow_item_type = rte_flow_item_type::RTE_FLOW_ITEM_TYPE_ETH;
	
	const IsMeta: bool = false;
	
	#[inline(always)]
	fn mask() -> &'static Self::DpdkType
	{
		unsafe { &rte_flow_item_eth_mask }
	}
}

impl EthernetHeaderPacketMatcher
{
	/// A `source` of 0xFFFFFF matches all Ethernet source addresses.
	/// A `destination` of 0xFFFFFF matches all Ethernet destination addresses.
	/// A `ether_type_or_tag_protocol_identifier` of 0x0000 matches all EtherTypes and tag protocol identifiers (TPID)s.
	#[inline(always)]
	pub fn new(source: MediaAccessControlAddress, destination: MediaAccessControlAddress, ether_type_or_tag_protocol_identifier: EtherType) -> Self
	{
		Self
		{
			underlying: rte_flow_item_eth
			{
				dst: destination.to_ether_addr(),
				src: source.to_ether_addr(),
				type_:
				{
					let into: NetworkEndianU16 = ether_type_or_tag_protocol_identifier.into();
					into.to_network_endian()
				},
			}
		}
	}
}
