// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[repr(align(64))]
pub struct rte_eventdev_data
{
	pub socket_id: c_int,
	pub dev_id: u8,
	pub nb_queues: u8,
	pub nb_ports: u8,
	pub ports: *mut *mut c_void,
	pub ports_cfg: *mut rte_event_port_conf,
	pub queues_cfg: *mut rte_event_queue_conf,
	pub links_map: *mut u16,
	pub dev_private: *mut c_void,
	pub event_dev_cap: u32,
	pub dev_conf: rte_event_dev_config,
	pub service_inited: u8,
	pub service_id: u32,
	pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize], u8>,
	pub name: [c_char; 64usize],
	pub __bindgen_padding_0: [u8; 39usize],
	pub __bindgen_align: [u8; 0usize],
}

impl Default for rte_eventdev_data
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_eventdev_data
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(
			f,
			"rte_eventdev_data {{ ports: {:?}, ports_cfg: {:?}, queues_cfg: {:?}, links_map: {:?}, dev_private: {:?}, dev_conf: {:?}, dev_started : {:?}, name: [{}] }}",
			self.ports,
			self.ports_cfg,
			self.queues_cfg,
			self.links_map,
			self.dev_private,
			self.dev_conf,
			self.dev_started(),
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
				.collect::<String>()
		)
	}
}

impl rte_eventdev_data
{
	
	#[inline(always)]
	pub fn dev_started(&self) -> u8
	{
		unsafe { transmute(self._bitfield_1.get(0usize, 1u8) as u8) }
	}
	
	#[inline(always)]
	pub fn set_dev_started(&mut self, val: u8)
	{
		unsafe {
			let val: u8 = transmute(val);
			self._bitfield_1.set(0usize, 1u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn new_bitfield_1(dev_started: u8) -> __BindgenBitfieldUnit<[u8; 1usize], u8>
	{
		let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize], u8> = Default::default();
		__bindgen_bitfield_unit.set(0usize, 1u8, {
			let dev_started: u8 = unsafe { transmute(dev_started) };
			dev_started as u64
		});
		__bindgen_bitfield_unit
	}
}
