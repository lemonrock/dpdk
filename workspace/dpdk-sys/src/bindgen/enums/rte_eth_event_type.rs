// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum rte_eth_event_type
{
	RTE_ETH_EVENT_UNKNOWN = 0,
	RTE_ETH_EVENT_INTR_LSC = 1,
	RTE_ETH_EVENT_QUEUE_STATE = 2,
	RTE_ETH_EVENT_INTR_RESET = 3,
	RTE_ETH_EVENT_VF_MBOX = 4,
	RTE_ETH_EVENT_MACSEC = 5,
	RTE_ETH_EVENT_INTR_RMV = 6,
	RTE_ETH_EVENT_NEW = 7,
	RTE_ETH_EVENT_DESTROY = 8,
	RTE_ETH_EVENT_MAX = 9,
}
