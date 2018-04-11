// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum rte_tm_stats_type
{
	RTE_TM_STATS_N_PKTS = 1,
	RTE_TM_STATS_N_BYTES = 2,
	RTE_TM_STATS_N_PKTS_GREEN_DROPPED = 4,
	RTE_TM_STATS_N_PKTS_YELLOW_DROPPED = 8,
	RTE_TM_STATS_N_PKTS_RED_DROPPED = 16,
	RTE_TM_STATS_N_BYTES_GREEN_DROPPED = 32,
	RTE_TM_STATS_N_BYTES_YELLOW_DROPPED = 64,
	RTE_TM_STATS_N_BYTES_RED_DROPPED = 128,
	RTE_TM_STATS_N_PKTS_QUEUED = 256,
	RTE_TM_STATS_N_BYTES_QUEUED = 512,
}
