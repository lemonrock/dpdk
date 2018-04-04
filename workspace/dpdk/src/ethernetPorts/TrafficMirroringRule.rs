// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TrafficMirroringRule
{
	pub ruleType: u8,
	pub destinationPool: u8,
	pub poolMask: u64,
	pub vlanMask: u64,
	pub vlanIds: Array64<u16>,
}

impl TrafficMirroringRule
{
	#[inline(always)]
	pub fn new(&self, ruleType: u8, destinationPool: u8, poolMask: u64, vlanMask: u64, vlanIds: [u16; 64]) -> Self
	{
		TrafficMirroringRule
		{
			ruleType: ruleType,
			destinationPool: destinationPool,
			poolMask: poolMask,
			vlanMask: vlanMask,
			vlanIds: Array64(vlanIds),
		}
	}
	
	#[inline(always)]
	pub fn as_rte_eth_mirror_conf(&self) -> rte_eth_mirror_conf
	{
		rte_eth_mirror_conf
		{
			rule_type: self.ruleType,
			dst_pool: self.destinationPool,
			pool_mask: self.poolMask,
			vlan: rte_eth_vlan_mirror
			{
				vlan_mask: self.vlanMask,
				vlan_id: self.vlanIds.0,
			}
		}
	}
}
