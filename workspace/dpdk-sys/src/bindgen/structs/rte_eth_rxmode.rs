// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_eth_rxmode
{
	pub mq_mode: rte_eth_rx_mq_mode,
	pub max_rx_pkt_len: u32,
	pub split_hdr_size: u16,
	pub offloads: u64,
	pub bitfield_1: BindgenBitfieldUnit<[u8; 2usize], u8>,
	pub __bindgen_padding_0: [u16; 3usize],
}

impl Default for rte_eth_rxmode
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_eth_rxmode
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_eth_rxmode {{ mq_mode: {:?}, header_split : {:?}, hw_ip_checksum : {:?}, hw_vlan_filter : {:?}, hw_vlan_strip : {:?}, hw_vlan_extend : {:?}, jumbo_frame : {:?}, hw_strip_crc : {:?}, enable_scatter : {:?}, enable_lro : {:?}, hw_timestamp : {:?}, security : {:?}, ignore_offload_bitfield : {:?} }}", self.mq_mode, self.header_split(), self.hw_ip_checksum(), self.hw_vlan_filter(), self.hw_vlan_strip(), self.hw_vlan_extend(), self.jumbo_frame(), self.hw_strip_crc(), self.enable_scatter(), self.enable_lro(), self.hw_timestamp(), self.security(), self.ignore_offload_bitfield())
	}
}

impl rte_eth_rxmode
{
	
	#[inline(always)]
	pub fn header_split(&self) -> u16
	{
		unsafe { transmute(self.bitfield_1.get(0usize, 1u8) as u16) }
	}
	
	#[inline(always)]
	pub fn set_header_split(&mut self, val: u16)
	{
		unsafe {
			let val: u16 = transmute(val);
			self.bitfield_1.set(0usize, 1u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn hw_ip_checksum(&self) -> u16
	{
		unsafe { transmute(self.bitfield_1.get(1usize, 1u8) as u16) }
	}
	
	#[inline(always)]
	pub fn set_hw_ip_checksum(&mut self, val: u16)
	{
		unsafe {
			let val: u16 = transmute(val);
			self.bitfield_1.set(1usize, 1u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn hw_vlan_filter(&self) -> u16
	{
		unsafe { transmute(self.bitfield_1.get(2usize, 1u8) as u16) }
	}
	
	#[inline(always)]
	pub fn set_hw_vlan_filter(&mut self, val: u16)
	{
		unsafe {
			let val: u16 = transmute(val);
			self.bitfield_1.set(2usize, 1u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn hw_vlan_strip(&self) -> u16
	{
		unsafe { transmute(self.bitfield_1.get(3usize, 1u8) as u16) }
	}
	
	#[inline(always)]
	pub fn set_hw_vlan_strip(&mut self, val: u16)
	{
		unsafe {
			let val: u16 = transmute(val);
			self.bitfield_1.set(3usize, 1u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn hw_vlan_extend(&self) -> u16
	{
		unsafe { transmute(self.bitfield_1.get(4usize, 1u8) as u16) }
	}
	
	#[inline(always)]
	pub fn set_hw_vlan_extend(&mut self, val: u16)
	{
		unsafe {
			let val: u16 = transmute(val);
			self.bitfield_1.set(4usize, 1u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn jumbo_frame(&self) -> u16
	{
		unsafe { transmute(self.bitfield_1.get(5usize, 1u8) as u16) }
	}
	
	#[inline(always)]
	pub fn set_jumbo_frame(&mut self, val: u16)
	{
		unsafe {
			let val: u16 = transmute(val);
			self.bitfield_1.set(5usize, 1u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn hw_strip_crc(&self) -> u16
	{
		unsafe { transmute(self.bitfield_1.get(6usize, 1u8) as u16) }
	}
	
	#[inline(always)]
	pub fn set_hw_strip_crc(&mut self, val: u16)
	{
		unsafe {
			let val: u16 = transmute(val);
			self.bitfield_1.set(6usize, 1u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn enable_scatter(&self) -> u16
	{
		unsafe { transmute(self.bitfield_1.get(7usize, 1u8) as u16) }
	}
	
	#[inline(always)]
	pub fn set_enable_scatter(&mut self, val: u16)
	{
		unsafe {
			let val: u16 = transmute(val);
			self.bitfield_1.set(7usize, 1u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn enable_lro(&self) -> u16
	{
		unsafe { transmute(self.bitfield_1.get(8usize, 1u8) as u16) }
	}
	
	#[inline(always)]
	pub fn set_enable_lro(&mut self, val: u16)
	{
		unsafe {
			let val: u16 = transmute(val);
			self.bitfield_1.set(8usize, 1u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn hw_timestamp(&self) -> u16
	{
		unsafe { transmute(self.bitfield_1.get(9usize, 1u8) as u16) }
	}
	
	#[inline(always)]
	pub fn set_hw_timestamp(&mut self, val: u16)
	{
		unsafe {
			let val: u16 = transmute(val);
			self.bitfield_1.set(9usize, 1u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn security(&self) -> u16
	{
		unsafe { transmute(self.bitfield_1.get(10usize, 1u8) as u16) }
	}
	
	#[inline(always)]
	pub fn set_security(&mut self, val: u16)
	{
		unsafe {
			let val: u16 = transmute(val);
			self.bitfield_1.set(10usize, 1u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn ignore_offload_bitfield(&self) -> u16
	{
		unsafe { transmute(self.bitfield_1.get(11usize, 1u8) as u16) }
	}
	
	#[inline(always)]
	pub fn set_ignore_offload_bitfield(&mut self, val: u16)
	{
		unsafe {
			let val: u16 = transmute(val);
			self.bitfield_1.set(11usize, 1u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn newbitfield_1(header_split: u16, hw_ip_checksum: u16, hw_vlan_filter: u16, hw_vlan_strip: u16, hw_vlan_extend: u16, jumbo_frame: u16, hw_strip_crc: u16, enable_scatter: u16, enable_lro: u16, hw_timestamp: u16, security: u16, ignore_offload_bitfield: u16) -> BindgenBitfieldUnit<[u8; 2usize], u8>
	{
		let mut __bindgen_bitfield_unit: BindgenBitfieldUnit<[u8; 2usize], u8> = Default::default();
		__bindgen_bitfield_unit.set(0usize, 1u8, {
			let header_split: u16 = unsafe { transmute(header_split) };
			header_split as u64
		});
		__bindgen_bitfield_unit.set(1usize, 1u8, {
			let hw_ip_checksum: u16 = unsafe { transmute(hw_ip_checksum) };
			hw_ip_checksum as u64
		});
		__bindgen_bitfield_unit.set(2usize, 1u8, {
			let hw_vlan_filter: u16 = unsafe { transmute(hw_vlan_filter) };
			hw_vlan_filter as u64
		});
		__bindgen_bitfield_unit.set(3usize, 1u8, {
			let hw_vlan_strip: u16 = unsafe { transmute(hw_vlan_strip) };
			hw_vlan_strip as u64
		});
		__bindgen_bitfield_unit.set(4usize, 1u8, {
			let hw_vlan_extend: u16 = unsafe { transmute(hw_vlan_extend) };
			hw_vlan_extend as u64
		});
		__bindgen_bitfield_unit.set(5usize, 1u8, {
			let jumbo_frame: u16 = unsafe { transmute(jumbo_frame) };
			jumbo_frame as u64
		});
		__bindgen_bitfield_unit.set(6usize, 1u8, {
			let hw_strip_crc: u16 = unsafe { transmute(hw_strip_crc) };
			hw_strip_crc as u64
		});
		__bindgen_bitfield_unit.set(7usize, 1u8, {
			let enable_scatter: u16 = unsafe { transmute(enable_scatter) };
			enable_scatter as u64
		});
		__bindgen_bitfield_unit.set(8usize, 1u8, {
			let enable_lro: u16 = unsafe { transmute(enable_lro) };
			enable_lro as u64
		});
		__bindgen_bitfield_unit.set(9usize, 1u8, {
			let hw_timestamp: u16 = unsafe { transmute(hw_timestamp) };
			hw_timestamp as u64
		});
		__bindgen_bitfield_unit.set(10usize, 1u8, {
			let security: u16 = unsafe { transmute(security) };
			security as u64
		});
		__bindgen_bitfield_unit.set(11usize, 1u8, {
			let ignore_offload_bitfield: u16 = unsafe { transmute(ignore_offload_bitfield) };
			ignore_offload_bitfield as u64
		});
		__bindgen_bitfield_unit
	}
}
