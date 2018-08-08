// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://any.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://any.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Matches traffic originating from (ingress) or going to (egress) a given DPDK port identifier (also known as `port_id` and 'port ID').
///
/// Normally only supported if the port identifier in question is known by the underlying PMD and related to the device the flow rule is created against.
///
/// A port identifier is the application-side way of referring to 'ethernet' connections and getting reference to `eth_dev` structures.
#[derive(Debug)]
#[repr(transparent)]
pub struct PortIdentifierMaskedPacketMatcher
{
	underlying: rte_flow_item_port_id,
}

impl Clone for PortIdentifierMaskedPacketMatcher
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		PortIdentifierMaskedPacketMatcher
		{
			underlying: rte_flow_item_port_id
			{
				id: self.underlying.id,
			}
		}
	}
}

impl PartialEq for PortIdentifierMaskedPacketMatcher
{
	#[inline(always)]
	fn eq(&self, rhs: &Self) -> bool
	{
		self.underlying.id == rhs.underlying.id
	}
}

impl Eq for PortIdentifierMaskedPacketMatcher
{
}

impl PartialOrd for PortIdentifierMaskedPacketMatcher
{
	#[inline(always)]
	fn partial_cmp(&self, rhs: &Self) -> Option<Ordering>
	{
		Some(self.cmp(rhs))
	}
}

impl Ord for PortIdentifierMaskedPacketMatcher
{
	#[inline(always)]
	fn cmp(&self, rhs: &Self) -> Ordering
	{
		self.underlying.id.cmp(&rhs.underlying.id)
	}
}

impl Hash for PortIdentifierMaskedPacketMatcher
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, hasher: &mut H)
	{
		self.underlying.id.hash(hasher)
	}
}

impl PacketMatcher for PortIdentifierMaskedPacketMatcher
{
	const Type: rte_flow_item_type = rte_flow_item_type::RTE_FLOW_ITEM_TYPE_PORT_ID;
	
	const IsMeta: bool = false;
}

impl MaskedPacketMatcher for PortIdentifierMaskedPacketMatcher
{
	type DpdkType = rte_flow_item_port_id;
	
	#[inline(always)]
	fn mask() -> &'static Self::DpdkType
	{
		unsafe { &rte_flow_item_port_id_mask }
	}
}

impl PortIdentifierMaskedPacketMatcher
{
	/// Creates a new instance.
	#[inline(always)]
	pub fn new(port_identifier: u32) -> Self
	{
		Self
		{
			underlying: rte_flow_item_port_id
			{
				id: port_identifier,
			}
		}
	}
}
