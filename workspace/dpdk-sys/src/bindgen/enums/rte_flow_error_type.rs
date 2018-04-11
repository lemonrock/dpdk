// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum rte_flow_error_type
{
	RTE_FLOW_ERROR_TYPE_NONE = 0,
	RTE_FLOW_ERROR_TYPE_UNSPECIFIED = 1,
	RTE_FLOW_ERROR_TYPE_HANDLE = 2,
	RTE_FLOW_ERROR_TYPE_ATTR_GROUP = 3,
	RTE_FLOW_ERROR_TYPE_ATTR_PRIORITY = 4,
	RTE_FLOW_ERROR_TYPE_ATTR_INGRESS = 5,
	RTE_FLOW_ERROR_TYPE_ATTR_EGRESS = 6,
	RTE_FLOW_ERROR_TYPE_ATTR = 7,
	RTE_FLOW_ERROR_TYPE_ITEM_NUM = 8,
	RTE_FLOW_ERROR_TYPE_ITEM = 9,
	RTE_FLOW_ERROR_TYPE_ACTION_NUM = 10,
	RTE_FLOW_ERROR_TYPE_ACTION = 11,
}
