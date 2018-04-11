// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_eth_conf__bindgen_ty_1
{
	pub rss_conf: rte_eth_rss_conf,
	pub vmdq_dcb_conf: rte_eth_vmdq_dcb_conf,
	pub dcb_rx_conf: rte_eth_dcb_rx_conf,
	pub vmdq_rx_conf: rte_eth_vmdq_rx_conf,
}

impl Default for rte_eth_conf__bindgen_ty_1
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_eth_conf__bindgen_ty_1
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_eth_conf__bindgen_ty_1 {{ rss_conf: {:?}, vmdq_dcb_conf: {:?}, dcb_rx_conf: {:?}, vmdq_rx_conf: {:?} }}", self.rss_conf, self.vmdq_dcb_conf, self.dcb_rx_conf, self.vmdq_rx_conf)
	}
}
