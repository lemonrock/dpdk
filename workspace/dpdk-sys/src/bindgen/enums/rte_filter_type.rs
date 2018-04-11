// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum rte_filter_type
{
	RTE_ETH_FILTER_NONE = 0,
	RTE_ETH_FILTER_MACVLAN = 1,
	RTE_ETH_FILTER_ETHERTYPE = 2,
	RTE_ETH_FILTER_FLEXIBLE = 3,
	RTE_ETH_FILTER_SYN = 4,
	RTE_ETH_FILTER_NTUPLE = 5,
	RTE_ETH_FILTER_TUNNEL = 6,
	RTE_ETH_FILTER_FDIR = 7,
	RTE_ETH_FILTER_HASH = 8,
	RTE_ETH_FILTER_L2_TUNNEL = 9,
	RTE_ETH_FILTER_GENERIC = 10,
	RTE_ETH_FILTER_MAX = 11,
}
