// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum rte_tm_dynamic_update_type
{
	RTE_TM_UPDATE_NODE_PARENT_KEEP_LEVEL = 1,
	RTE_TM_UPDATE_NODE_PARENT_CHANGE_LEVEL = 2,
	RTE_TM_UPDATE_NODE_ADD_DELETE = 4,
	RTE_TM_UPDATE_NODE_SUSPEND_RESUME = 8,
	RTE_TM_UPDATE_NODE_WFQ_WEIGHT_MODE = 16,
	RTE_TM_UPDATE_NODE_N_SP_PRIORITIES = 32,
	RTE_TM_UPDATE_NODE_CMAN = 64,
	RTE_TM_UPDATE_NODE_STATS = 128,
}
