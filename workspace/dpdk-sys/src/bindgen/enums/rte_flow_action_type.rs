// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum rte_flow_action_type
{
	RTE_FLOW_ACTION_TYPE_END = 0,
	RTE_FLOW_ACTION_TYPE_VOID = 1,
	RTE_FLOW_ACTION_TYPE_PASSTHRU = 2,
	RTE_FLOW_ACTION_TYPE_MARK = 3,
	RTE_FLOW_ACTION_TYPE_FLAG = 4,
	RTE_FLOW_ACTION_TYPE_QUEUE = 5,
	RTE_FLOW_ACTION_TYPE_DROP = 6,
	RTE_FLOW_ACTION_TYPE_COUNT = 7,
	RTE_FLOW_ACTION_TYPE_DUP = 8,
	RTE_FLOW_ACTION_TYPE_RSS = 9,
	RTE_FLOW_ACTION_TYPE_PF = 10,
	RTE_FLOW_ACTION_TYPE_VF = 11,
	RTE_FLOW_ACTION_TYPE_METER = 12,
	RTE_FLOW_ACTION_TYPE_SECURITY = 13,
}
