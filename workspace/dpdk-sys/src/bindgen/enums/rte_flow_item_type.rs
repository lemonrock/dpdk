// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum rte_flow_item_type
{
	RTE_FLOW_ITEM_TYPE_END = 0,
	RTE_FLOW_ITEM_TYPE_VOID = 1,
	RTE_FLOW_ITEM_TYPE_INVERT = 2,
	RTE_FLOW_ITEM_TYPE_ANY = 3,
	RTE_FLOW_ITEM_TYPE_PF = 4,
	RTE_FLOW_ITEM_TYPE_VF = 5,
	RTE_FLOW_ITEM_TYPE_PORT = 6,
	RTE_FLOW_ITEM_TYPE_RAW = 7,
	RTE_FLOW_ITEM_TYPE_ETH = 8,
	RTE_FLOW_ITEM_TYPE_VLAN = 9,
	RTE_FLOW_ITEM_TYPE_IPV4 = 10,
	RTE_FLOW_ITEM_TYPE_IPV6 = 11,
	RTE_FLOW_ITEM_TYPE_ICMP = 12,
	RTE_FLOW_ITEM_TYPE_UDP = 13,
	RTE_FLOW_ITEM_TYPE_TCP = 14,
	RTE_FLOW_ITEM_TYPE_SCTP = 15,
	RTE_FLOW_ITEM_TYPE_VXLAN = 16,
	RTE_FLOW_ITEM_TYPE_E_TAG = 17,
	RTE_FLOW_ITEM_TYPE_NVGRE = 18,
	RTE_FLOW_ITEM_TYPE_MPLS = 19,
	RTE_FLOW_ITEM_TYPE_GRE = 20,
	RTE_FLOW_ITEM_TYPE_FUZZY = 21,
	RTE_FLOW_ITEM_TYPE_GTP = 22,
	RTE_FLOW_ITEM_TYPE_GTPC = 23,
	RTE_FLOW_ITEM_TYPE_GTPU = 24,
	RTE_FLOW_ITEM_TYPE_ESP = 25,
	RTE_FLOW_ITEM_TYPE_GENEVE = 26,
}
