// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum rte_keepalive_state
{
	RTE_KA_STATE_UNUSED = 0,
	RTE_KA_STATE_ALIVE = 1,
	RTE_KA_STATE_MISSING = 4,
	RTE_KA_STATE_DEAD = 2,
	RTE_KA_STATE_GONE = 3,
	RTE_KA_STATE_DOZING = 5,
	RTE_KA_STATE_SLEEP = 6,
}
