// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum rte_bbdev_event_type
{
	RTE_BBDEV_EVENT_UNKNOWN = 0,
	RTE_BBDEV_EVENT_ERROR = 1,
	RTE_BBDEV_EVENT_DEQUEUE = 2,
	RTE_BBDEV_EVENT_MAX = 3,
}
