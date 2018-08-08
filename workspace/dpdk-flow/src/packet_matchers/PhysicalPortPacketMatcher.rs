// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://any.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://any.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Matches traffic originating from (ingress) or going to (egress) a physical port of the underlying device.
///
/// The first PhysicalPortPacketMatcher overrides the physical port normally associated with the specified DPDK input port (`port_id`).
/// This item can be provided several times to match additional physical ports.
///
/// Note that physical ports are not necessarily tied to DPDK input ports (`port_id`) when those are not under DPDK control.
/// Possible values are specific to each device, they are not necessarily indexed from zero and may not be contiguous.
///
/// As a device property, the list of allowed values as well as the value associated with a `port_id` should be retrieved by other means.
#[derive(Debug)]
#[repr(transparent)]
pub struct PhysicalPortPacketMatcher
{
	underlying: rte_flow_item_phy_port,
}

impl Clone for PhysicalPortPacketMatcher
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		PhysicalPortPacketMatcher
		{
			underlying: rte_flow_item_phy_port
			{
				index: self.underlying.index,
			}
		}
	}
}

impl PartialEq for PhysicalPortPacketMatcher
{
	#[inline(always)]
	fn eq(&self, rhs: &Self) -> bool
	{
		self.underlying.index == rhs.underlying.index
	}
}

impl Eq for PhysicalPortPacketMatcher
{
}

impl PartialOrd for PhysicalPortPacketMatcher
{
	#[inline(always)]
	fn partial_cmp(&self, rhs: &Self) -> Option<Ordering>
	{
		Some(self.cmp(rhs))
	}
}

impl Ord for PhysicalPortPacketMatcher
{
	#[inline(always)]
	fn cmp(&self, rhs: &Self) -> Ordering
	{
		self.underlying.index.cmp(&rhs.underlying.index)
	}
}

impl Hash for PhysicalPortPacketMatcher
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, hasher: &mut H)
	{
		self.underlying.index.hash(hasher)
	}
}

impl PacketMatcher for PhysicalPortPacketMatcher
{
	type DpdkType = rte_flow_item_phy_port;
	
	const Type: rte_flow_item_type = rte_flow_item_type::RTE_FLOW_ITEM_TYPE_PHY_PORT;
	
	const IsMeta: bool = false;
	
	#[inline(always)]
	fn mask() -> &'static Self::DpdkType
	{
		unsafe { &rte_flow_item_phy_port_mask }
	}
}

impl PhysicalPortPacketMatcher
{
	/// Create a new instance.
	///
	/// If `physical_port_identifier` is zero then matches any physical port.
	#[inline(always)]
	pub fn new(physical_port_identifier: u32) -> Self
	{
		let this = Self
		{
			underlying: rte_flow_item_phy_port
			{
				index: physical_port_identifier,
			}
		};
		
		this
	}
}
