// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FlowDirectorConfiguration
{
	pub mode: rte_fdir_mode,
	pub allocationType: rte_fdir_pballoc_type,
	pub statusMode: rte_fdir_status_mode,
	pub dropQueue: u8,
	pub mask: rte_eth_fdir_masks,
}

impl Default for FlowDirectorConfiguration
{
	fn default() -> Self
	{
		FlowDirectorConfiguration
		{
			mode: rte_fdir_mode::RTE_FDIR_MODE_NONE,
			allocationType: rte_fdir_pballoc_type::RTE_FDIR_PBALLOC_64K,
			statusMode: rte_fdir_status_mode::RTE_FDIR_NO_REPORT_STATUS,
			dropQueue: 0,
			mask: rte_eth_fdir_masks::default(),
		}
	}
}

impl FlowDirectorConfiguration
{
	fn as_rte_fdir_conf(&self) -> rte_fdir_conf
	{
		rte_fdir_conf
		{
			mode: self.mode,
			pballoc: self.allocationType,
			status: self.statusMode,
			drop_queue: self.dropQueue,
			mask: self.mask,
			flex_conf: rte_eth_fdir_flex_conf::default(),
		}
	}
}
