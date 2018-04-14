// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_eth_conf
{
	pub link_speeds: u32,
	pub rxmode: rte_eth_rxmode,
	pub txmode: rte_eth_txmode,
	pub lpbk_mode: u32,
	pub rx_adv_conf: rte_eth_conf_1,
	pub tx_adv_conf: rte_eth_conf_2,
	pub dcb_capability_en: u32,
	pub fdir_conf: rte_fdir_conf,
	pub intr_conf: rte_intr_conf,
}

impl Default for rte_eth_conf
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_eth_conf
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_eth_conf {{ rxmode: {:?}, txmode: {:?}, rx_adv_conf: {:?}, tx_adv_conf: {:?}, fdir_conf: {:?}, intr_conf: {:?} }}", self.rxmode, self.txmode, self.rx_adv_conf, self.tx_adv_conf, self.fdir_conf, self.intr_conf)
	}
}
