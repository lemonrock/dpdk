// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_kni_device_info
{
	pub name: [c_char; 32usize],
	pub tx_phys: phys_addr_t,
	pub rx_phys: phys_addr_t,
	pub alloc_phys: phys_addr_t,
	pub free_phys: phys_addr_t,
	pub req_phys: phys_addr_t,
	pub resp_phys: phys_addr_t,
	pub sync_phys: phys_addr_t,
	pub sync_va: *mut c_void,
	pub mbuf_va: *mut c_void,
	pub mbuf_phys: phys_addr_t,
	pub vendor_id: u16,
	pub device_id: u16,
	pub bus: u8,
	pub devid: u8,
	pub function: u8,
	pub group_id: u16,
	pub core_id: u32,
	pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize], u8>,
	pub mbuf_size: c_uint,
	pub mtu: c_uint,
	pub mac_addr: [c_char; 6usize],
}

impl Default for rte_kni_device_info
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_kni_device_info
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(
			f,
			"rte_kni_device_info {{ name: [{}], sync_va: {:?}, mbuf_va: {:?}, force_bind : {:?}, mac_addr: {:?} }}",
			self.name
				.iter()
				.enumerate()
				.map(|(i, v)| format!(
					"{}{:?}",
					if i > 0
					{
						", "
					}
					else
					{
						""
					},
					v
				))
				.collect::<String>(),
			self.sync_va,
			self.mbuf_va,
			self.force_bind(),
			self.mac_addr
		)
	}
}

impl rte_kni_device_info
{
	
	#[inline(always)]
	pub fn force_bind(&self) -> u8
	{
		unsafe { transmute(self._bitfield_1.get(0usize, 1u8) as u8) }
	}
	
	#[inline(always)]
	pub fn set_force_bind(&mut self, val: u8)
	{
		unsafe {
			let val: u8 = transmute(val);
			self._bitfield_1.set(0usize, 1u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn new_bitfield_1(force_bind: u8) -> __BindgenBitfieldUnit<[u8; 1usize], u8>
	{
		let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize], u8> = Default::default();
		__bindgen_bitfield_unit.set(0usize, 1u8, {
			let force_bind: u8 = unsafe { transmute(force_bind) };
			force_bind as u64
		});
		__bindgen_bitfield_unit
	}
}
