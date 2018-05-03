// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum rte_flow_action_type
{
	RTE_FLOW_ACTION_TYPE_END = 0,
	RTE_FLOW_ACTION_TYPE_VOID = 1,
	RTE_FLOW_ACTION_TYPE_PASSTHRU = 2,
	RTE_FLOW_ACTION_TYPE_JUMP = 3,
	RTE_FLOW_ACTION_TYPE_MARK = 4,
	RTE_FLOW_ACTION_TYPE_FLAG = 5,
	RTE_FLOW_ACTION_TYPE_QUEUE = 6,
	RTE_FLOW_ACTION_TYPE_DROP = 7,
	RTE_FLOW_ACTION_TYPE_COUNT = 8,
	RTE_FLOW_ACTION_TYPE_RSS = 9,
	RTE_FLOW_ACTION_TYPE_PF = 10,
	RTE_FLOW_ACTION_TYPE_VF = 11,
	RTE_FLOW_ACTION_TYPE_PHY_PORT = 12,
	RTE_FLOW_ACTION_TYPE_PORT_ID = 13,
	RTE_FLOW_ACTION_TYPE_METER = 14,
	RTE_FLOW_ACTION_TYPE_SECURITY = 15,
	RTE_FLOW_ACTION_TYPE_OF_SET_MPLS_TTL = 16,
	RTE_FLOW_ACTION_TYPE_OF_DEC_MPLS_TTL = 17,
	RTE_FLOW_ACTION_TYPE_OF_SET_NW_TTL = 18,
	RTE_FLOW_ACTION_TYPE_OF_DEC_NW_TTL = 19,
	RTE_FLOW_ACTION_TYPE_OF_COPY_TTL_OUT = 20,
	RTE_FLOW_ACTION_TYPE_OF_COPY_TTL_IN = 21,
	RTE_FLOW_ACTION_TYPE_OF_POP_VLAN = 22,
	RTE_FLOW_ACTION_TYPE_OF_PUSH_VLAN = 23,
	RTE_FLOW_ACTION_TYPE_OF_SET_VLAN_VID = 24,
	RTE_FLOW_ACTION_TYPE_OF_SET_VLAN_PCP = 25,
	RTE_FLOW_ACTION_TYPE_OF_POP_MPLS = 26,
	RTE_FLOW_ACTION_TYPE_OF_PUSH_MPLS = 27,
	RTE_FLOW_ACTION_TYPE_VXLAN_ENCAP = 28,
	RTE_FLOW_ACTION_TYPE_VXLAN_DECAP = 29,
	RTE_FLOW_ACTION_TYPE_NVGRE_ENCAP = 30,
	RTE_FLOW_ACTION_TYPE_NVGRE_DECAP = 31,
}
