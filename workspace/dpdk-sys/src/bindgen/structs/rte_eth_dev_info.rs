// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_eth_dev_info
{
	pub pci_dev: *mut rte_pci_device,
	pub driver_name: *const c_char,
	pub if_index: c_uint,
	pub min_rx_bufsize: u32,
	pub max_rx_pktlen: u32,
	pub max_rx_queues: u16,
	pub max_tx_queues: u16,
	pub max_mac_addrs: u32,
	pub max_hash_mac_addrs: u32,
	pub max_vfs: u16,
	pub max_vmdq_pools: u16,
	pub rx_offload_capa: u64,
	pub tx_offload_capa: u64,
	pub rx_queue_offload_capa: u64,
	pub tx_queue_offload_capa: u64,
	pub reta_size: u16,
	pub hash_key_size: u8,
	pub flow_type_rss_offloads: u64,
	pub default_rxconf: rte_eth_rxconf,
	pub default_txconf: rte_eth_txconf,
	pub vmdq_queue_base: u16,
	pub vmdq_queue_num: u16,
	pub vmdq_pool_base: u16,
	pub rx_desc_lim: rte_eth_desc_lim,
	pub tx_desc_lim: rte_eth_desc_lim,
	pub speed_capa: u32,
	pub nb_rx_queues: u16,
	pub nb_tx_queues: u16,
}

impl Default for rte_eth_dev_info
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_eth_dev_info
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_eth_dev_info {{ pci_dev: {:?}, driver_name: {:?}, default_rxconf: {:?}, default_txconf: {:?}, rx_desc_lim: {:?}, tx_desc_lim: {:?} }}", self.pci_dev, self.driver_name, self.default_rxconf, self.default_txconf, self.rx_desc_lim, self.tx_desc_lim)
	}
}
