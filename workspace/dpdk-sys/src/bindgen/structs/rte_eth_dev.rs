// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[repr(align(64))]
pub struct rte_eth_dev
{
	pub rx_pkt_burst: eth_rx_burst_t,
	pub tx_pkt_burst: eth_tx_burst_t,
	pub tx_pkt_prepare: eth_tx_prep_t,
	pub data: *mut rte_eth_dev_data,
	pub dev_ops: *const eth_dev_ops,
	pub device: *mut rte_device,
	pub intr_handle: *mut rte_intr_handle,
	pub link_intr_cbs: rte_eth_dev_cb_list,
	pub post_rx_burst_cbs: [*mut rte_eth_rxtx_callback; 1024usize],
	pub pre_tx_burst_cbs: [*mut rte_eth_rxtx_callback; 1024usize],
	pub state: rte_eth_dev_state,
	pub security_ctx: *mut c_void,
	pub __bindgen_padding_0: [u64; 5usize],
	pub __bindgen_align: [u8; 0usize],
}

impl Default for rte_eth_dev
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_eth_dev
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(
			f,
			"rte_eth_dev {{ data: {:?}, dev_ops: {:?}, device: {:?}, intr_handle: {:?}, link_intr_cbs: {:?}, post_rx_burst_cbs: [{}], pre_tx_burst_cbs: [{}], state: {:?}, security_ctx: {:?} }}",
			self.data,
			self.dev_ops,
			self.device,
			self.intr_handle,
			self.link_intr_cbs,
			self.post_rx_burst_cbs
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
			self.pre_tx_burst_cbs
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
			self.state,
			self.security_ctx
		)
	}
}
