// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_fdir_conf
{
	pub mode: rte_fdir_mode,
	pub pballoc: rte_fdir_pballoc_type,
	pub status: rte_fdir_status_mode,
	pub drop_queue: u8,
	pub mask: rte_eth_fdir_masks,
	pub flex_conf: rte_eth_fdir_flex_conf,
}

impl Default for rte_fdir_conf
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_fdir_conf
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_fdir_conf {{ mode: {:?}, pballoc: {:?}, status: {:?}, mask: {:?}, flex_conf: {:?} }}", self.mode, self.pballoc, self.status, self.mask, self.flex_conf)
	}
}
