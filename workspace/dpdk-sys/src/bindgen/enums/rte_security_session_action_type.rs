// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum rte_security_session_action_type
{
	RTE_SECURITY_ACTION_TYPE_NONE = 0,
	RTE_SECURITY_ACTION_TYPE_INLINE_CRYPTO = 1,
	RTE_SECURITY_ACTION_TYPE_INLINE_PROTOCOL = 2,
	RTE_SECURITY_ACTION_TYPE_LOOKASIDE_PROTOCOL = 3,
}
