// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum rte_mtr_error_type
{
	RTE_MTR_ERROR_TYPE_NONE = 0,
	RTE_MTR_ERROR_TYPE_UNSPECIFIED = 1,
	RTE_MTR_ERROR_TYPE_METER_PROFILE_ID = 2,
	RTE_MTR_ERROR_TYPE_METER_PROFILE = 3,
	RTE_MTR_ERROR_TYPE_MTR_ID = 4,
	RTE_MTR_ERROR_TYPE_MTR_PARAMS = 5,
	RTE_MTR_ERROR_TYPE_POLICER_ACTION_GREEN = 6,
	RTE_MTR_ERROR_TYPE_POLICER_ACTION_YELLOW = 7,
	RTE_MTR_ERROR_TYPE_POLICER_ACTION_RED = 8,
	RTE_MTR_ERROR_TYPE_STATS_MASK = 9,
	RTE_MTR_ERROR_TYPE_STATS = 10,
	RTE_MTR_ERROR_TYPE_SHARED = 11,
}
