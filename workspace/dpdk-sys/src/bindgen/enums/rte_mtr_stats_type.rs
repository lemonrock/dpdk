// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum rte_mtr_stats_type
{
	RTE_MTR_STATS_N_PKTS_GREEN = 1,
	RTE_MTR_STATS_N_PKTS_YELLOW = 2,
	RTE_MTR_STATS_N_PKTS_RED = 4,
	RTE_MTR_STATS_N_PKTS_DROPPED = 8,
	RTE_MTR_STATS_N_BYTES_GREEN = 16,
	RTE_MTR_STATS_N_BYTES_YELLOW = 32,
	RTE_MTR_STATS_N_BYTES_RED = 64,
	RTE_MTR_STATS_N_BYTES_DROPPED = 128,
}
