// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[repr(align(64))]
pub struct rte_eth_dev_data
{
	pub name: [c_char; 64usize],
	pub rx_queues: *mut *mut c_void,
	pub tx_queues: *mut *mut c_void,
	pub nb_rx_queues: u16,
	pub nb_tx_queues: u16,
	pub sriov: rte_eth_dev_sriov,
	pub dev_private: *mut c_void,
	pub dev_link: rte_eth_link,
	pub dev_conf: rte_eth_conf,
	pub mtu: u16,
	pub min_rx_buf_size: u32,
	pub rx_mbuf_alloc_failed: u64,
	pub mac_addrs: *mut ether_addr,
	pub mac_pool_sel: [u64; 128usize],
	pub hash_mac_addrs: *mut ether_addr,
	pub port_id: u16,
	pub bitfield_1: BindgenBitfieldUnit<[u8; 1usize], u8>,
	pub rx_queue_state: [u8; 1024usize],
	pub tx_queue_state: [u8; 1024usize],
	pub dev_flags: u32,
	pub kdrv: rte_kernel_driver,
	pub numa_node: c_int,
	pub vlan_filter_conf: rte_vlan_filter_conf,
	pub owner: rte_eth_dev_owner,
	pub __bindgen_padding_0: [u64; 6usize],
	pub __bindgen_align: [u8; 0usize],
}

impl Default for rte_eth_dev_data
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_eth_dev_data
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(
			f,
			"rte_eth_dev_data {{ name: [{}], rx_queues: {:?}, tx_queues: {:?}, sriov: {:?}, dev_private: {:?}, dev_link: {:?}, dev_conf: {:?}, mac_addrs: {:?}, mac_pool_sel: [{}], hash_mac_addrs: {:?}, promiscuous : {:?}, scattered_rx : {:?}, all_multicast : {:?}, dev_started : {:?}, lro : {:?}, rx_queue_state: [{}], tx_queue_state: [{}], kdrv: {:?}, vlan_filter_conf: {:?}, owner: {:?} }}",
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
			self.rx_queues,
			self.tx_queues,
			self.sriov,
			self.dev_private,
			self.dev_link,
			self.dev_conf,
			self.mac_addrs,
			self.mac_pool_sel
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
			self.hash_mac_addrs,
			self.promiscuous(),
			self.scattered_rx(),
			self.all_multicast(),
			self.dev_started(),
			self.lro(),
			self.rx_queue_state
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
			self.tx_queue_state
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
			self.kdrv,
			self.vlan_filter_conf,
			self.owner
		)
	}
}

impl rte_eth_dev_data
{
	
	#[inline(always)]
	pub fn promiscuous(&self) -> u8
	{
		unsafe { transmute(self.bitfield_1.get(0usize, 1u8) as u8) }
	}
	
	#[inline(always)]
	pub fn set_promiscuous(&mut self, val: u8)
	{
		unsafe {
			let val: u8 = transmute(val);
			self.bitfield_1.set(0usize, 1u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn scattered_rx(&self) -> u8
	{
		unsafe { transmute(self.bitfield_1.get(1usize, 1u8) as u8) }
	}
	
	#[inline(always)]
	pub fn set_scattered_rx(&mut self, val: u8)
	{
		unsafe {
			let val: u8 = transmute(val);
			self.bitfield_1.set(1usize, 1u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn all_multicast(&self) -> u8
	{
		unsafe { transmute(self.bitfield_1.get(2usize, 1u8) as u8) }
	}
	
	#[inline(always)]
	pub fn set_all_multicast(&mut self, val: u8)
	{
		unsafe {
			let val: u8 = transmute(val);
			self.bitfield_1.set(2usize, 1u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn dev_started(&self) -> u8
	{
		unsafe { transmute(self.bitfield_1.get(3usize, 1u8) as u8) }
	}
	
	#[inline(always)]
	pub fn set_dev_started(&mut self, val: u8)
	{
		unsafe {
			let val: u8 = transmute(val);
			self.bitfield_1.set(3usize, 1u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn lro(&self) -> u8
	{
		unsafe { transmute(self.bitfield_1.get(4usize, 1u8) as u8) }
	}
	
	#[inline(always)]
	pub fn set_lro(&mut self, val: u8)
	{
		unsafe {
			let val: u8 = transmute(val);
			self.bitfield_1.set(4usize, 1u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn newbitfield_1(promiscuous: u8, scattered_rx: u8, all_multicast: u8, dev_started: u8, lro: u8) -> BindgenBitfieldUnit<[u8; 1usize], u8>
	{
		let mut __bindgen_bitfield_unit: BindgenBitfieldUnit<[u8; 1usize], u8> = Default::default();
		__bindgen_bitfield_unit.set(0usize, 1u8, {
			let promiscuous: u8 = unsafe { transmute(promiscuous) };
			promiscuous as u64
		});
		__bindgen_bitfield_unit.set(1usize, 1u8, {
			let scattered_rx: u8 = unsafe { transmute(scattered_rx) };
			scattered_rx as u64
		});
		__bindgen_bitfield_unit.set(2usize, 1u8, {
			let all_multicast: u8 = unsafe { transmute(all_multicast) };
			all_multicast as u64
		});
		__bindgen_bitfield_unit.set(3usize, 1u8, {
			let dev_started: u8 = unsafe { transmute(dev_started) };
			dev_started as u64
		});
		__bindgen_bitfield_unit.set(4usize, 1u8, {
			let lro: u8 = unsafe { transmute(lro) };
			lro as u64
		});
		__bindgen_bitfield_unit
	}
}
