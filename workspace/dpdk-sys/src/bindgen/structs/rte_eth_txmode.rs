// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_eth_txmode
{
	pub mq_mode: rte_eth_tx_mq_mode,
	pub offloads: u64,
	pub pvid: u16,
	pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize], u8>,
	pub __bindgen_padding_0: [u8; 5usize],
}

impl Default for rte_eth_txmode
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_eth_txmode
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_eth_txmode {{ mq_mode: {:?}, hw_vlan_reject_tagged : {:?}, hw_vlan_reject_untagged : {:?}, hw_vlan_insert_pvid : {:?} }}", self.mq_mode, self.hw_vlan_reject_tagged(), self.hw_vlan_reject_untagged(), self.hw_vlan_insert_pvid())
	}
}

impl rte_eth_txmode
{
	
	#[inline(always)]
	pub fn hw_vlan_reject_tagged(&self) -> u8
	{
		unsafe { transmute(self._bitfield_1.get(0usize, 1u8) as u8) }
	}
	
	#[inline(always)]
	pub fn set_hw_vlan_reject_tagged(&mut self, val: u8)
	{
		unsafe {
			let val: u8 = transmute(val);
			self._bitfield_1.set(0usize, 1u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn hw_vlan_reject_untagged(&self) -> u8
	{
		unsafe { transmute(self._bitfield_1.get(1usize, 1u8) as u8) }
	}
	
	#[inline(always)]
	pub fn set_hw_vlan_reject_untagged(&mut self, val: u8)
	{
		unsafe {
			let val: u8 = transmute(val);
			self._bitfield_1.set(1usize, 1u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn hw_vlan_insert_pvid(&self) -> u8
	{
		unsafe { transmute(self._bitfield_1.get(2usize, 1u8) as u8) }
	}
	
	#[inline(always)]
	pub fn set_hw_vlan_insert_pvid(&mut self, val: u8)
	{
		unsafe {
			let val: u8 = transmute(val);
			self._bitfield_1.set(2usize, 1u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn new_bitfield_1(hw_vlan_reject_tagged: u8, hw_vlan_reject_untagged: u8, hw_vlan_insert_pvid: u8) -> __BindgenBitfieldUnit<[u8; 1usize], u8>
	{
		let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize], u8> = Default::default();
		__bindgen_bitfield_unit.set(0usize, 1u8, {
			let hw_vlan_reject_tagged: u8 = unsafe { transmute(hw_vlan_reject_tagged) };
			hw_vlan_reject_tagged as u64
		});
		__bindgen_bitfield_unit.set(1usize, 1u8, {
			let hw_vlan_reject_untagged: u8 = unsafe { transmute(hw_vlan_reject_untagged) };
			hw_vlan_reject_untagged as u64
		});
		__bindgen_bitfield_unit.set(2usize, 1u8, {
			let hw_vlan_insert_pvid: u8 = unsafe { transmute(hw_vlan_insert_pvid) };
			hw_vlan_insert_pvid as u64
		});
		__bindgen_bitfield_unit
	}
}
