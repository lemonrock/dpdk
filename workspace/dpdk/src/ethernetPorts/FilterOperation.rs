// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


// Based on the enum rte_filter_op but without the noise and invalid values
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FilterOperation
{
	Add,
	Update,
	Delete,
	Flush,
	Get,
	Set,
	Information,
	Statistics,
}

impl FilterOperation
{
	#[inline(always)]
	pub fn as_rte_filter_op(&self) -> rte_filter_op
	{
		match *self
		{
			FilterOperation::Add => rte_filter_op::RTE_ETH_FILTER_ADD,
			FilterOperation::Update => rte_filter_op::RTE_ETH_FILTER_UPDATE,
			FilterOperation::Delete => rte_filter_op::RTE_ETH_FILTER_DELETE,
			FilterOperation::Flush => rte_filter_op::RTE_ETH_FILTER_FLUSH,
			FilterOperation::Get => rte_filter_op::RTE_ETH_FILTER_GET,
			FilterOperation::Set => rte_filter_op::RTE_ETH_FILTER_SET,
			FilterOperation::Information => rte_filter_op::RTE_ETH_FILTER_INFO,
			FilterOperation::Statistics => rte_filter_op::RTE_ETH_FILTER_STATS,
		}
	}
}
