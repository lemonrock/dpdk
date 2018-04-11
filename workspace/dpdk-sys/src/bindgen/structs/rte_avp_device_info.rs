// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_avp_device_info
{
	pub magic: u32,
	pub version: u32,
	pub ifname: [c_char; 32usize],
	pub tx_phys: rte_iova_t,
	pub rx_phys: rte_iova_t,
	pub alloc_phys: rte_iova_t,
	pub free_phys: rte_iova_t,
	pub features: u32,
	pub min_rx_queues: u8,
	pub num_rx_queues: u8,
	pub max_rx_queues: u8,
	pub min_tx_queues: u8,
	pub num_tx_queues: u8,
	pub max_tx_queues: u8,
	pub tx_size: u32,
	pub rx_size: u32,
	pub alloc_size: u32,
	pub free_size: u32,
	pub req_phys: rte_iova_t,
	pub resp_phys: rte_iova_t,
	pub sync_phys: rte_iova_t,
	pub sync_va: *mut c_void,
	pub mbuf_va: *mut c_void,
	pub mbuf_phys: rte_iova_t,
	pub pool: [rte_avp_mempool_info; 8usize],
	pub ethaddr: [c_char; 6usize],
	pub mode: u8,
	pub mbuf_size: c_uint,
	pub device_id: u64,
	pub max_rx_pkt_len: u32,
}

impl Default for rte_avp_device_info
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_avp_device_info
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(
			f,
			"rte_avp_device_info {{ ifname: [{}], sync_va: {:?}, mbuf_va: {:?}, pool: {:?}, ethaddr: {:?} }}",
			self.ifname
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
			self.pool,
			self.ethaddr
		)
	}
}
