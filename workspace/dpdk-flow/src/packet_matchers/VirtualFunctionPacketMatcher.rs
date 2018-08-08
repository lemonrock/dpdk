// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://any.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://any.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Matches traffic originating from (ingress) or going to (egress) a given virtual function (VF) of the current device.
///
/// If supported, should work even if the virtual function (VF) is not managed by the application and thus not associated with a DPDK port identifier (ID).
///
/// Note this pattern item does not match VF representors traffic which, as separate entities, should be addressed through their own DPDK port identifiers (IDs).
///
/// * Can be specified multiple times to match traffic addressed to several Virtual Function (VF) Identifiers (IDs).
/// * Can be combined with a PhysicalFunctionPacketMatcher to match both Physical Function (PF) and Virtual Function (VF) traffic.
#[derive(Debug)]
#[repr(transparent)]
pub struct VirtualFunctionPacketMatcher
{
	underlying: rte_flow_item_vf,
}

impl Clone for VirtualFunctionPacketMatcher
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		VirtualFunctionPacketMatcher
		{
			underlying: rte_flow_item_vf
			{
				id: self.underlying.id,
			}
		}
	}
}

impl PartialEq for VirtualFunctionPacketMatcher
{
	#[inline(always)]
	fn eq(&self, rhs: &Self) -> bool
	{
		self.underlying.id == rhs.underlying.id
	}
}

impl Eq for VirtualFunctionPacketMatcher
{
}

impl PartialOrd for VirtualFunctionPacketMatcher
{
	#[inline(always)]
	fn partial_cmp(&self, rhs: &Self) -> Option<Ordering>
	{
		Some(self.cmp(rhs))
	}
}

impl Ord for VirtualFunctionPacketMatcher
{
	#[inline(always)]
	fn cmp(&self, rhs: &Self) -> Ordering
	{
		self.underlying.id.cmp(&rhs.underlying.id)
	}
}

impl Hash for VirtualFunctionPacketMatcher
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, hasher: &mut H)
	{
		self.underlying.id.hash(hasher)
	}
}

impl PacketMatcher for VirtualFunctionPacketMatcher
{
	type DpdkType = rte_flow_item_vf;
	
	const Type: rte_flow_item_type = rte_flow_item_type::RTE_FLOW_ITEM_TYPE_VF;
	
	const IsMeta: bool = false;
	
	#[inline(always)]
	fn mask() -> &'static Self::DpdkType
	{
		unsafe { &rte_flow_item_vf_mask }
	}
}

impl VirtualFunctionPacketMatcher
{
	/// Create a new instance.
	///
	/// If `virtual_function_identifier` is zero then matches any virtual function.
	#[inline(always)]
	pub fn new(virtual_function_identifier: u32) -> Self
	{
		Self
		{
			underlying: rte_flow_item_vf
			{
				id: virtual_function_identifier,
			}
		}
	}
}
