// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_eth_rxtx_callback_1
{
	pub rx: BindgenUnionField<rte_rx_callback_fn>,
	pub tx: BindgenUnionField<rte_tx_callback_fn>,
	pub bindgen_union_field: u64,
}

impl Default for rte_eth_rxtx_callback_1
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_eth_rxtx_callback_1
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_eth_rxtx_callback_1 {{ union }}")
	}
}
