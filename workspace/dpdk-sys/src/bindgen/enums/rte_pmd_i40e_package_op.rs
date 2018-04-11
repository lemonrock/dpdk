// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum rte_pmd_i40e_package_op
{
	RTE_PMD_I40E_PKG_OP_UNDEFINED = 0,
	RTE_PMD_I40E_PKG_OP_WR_ADD = 1,
	RTE_PMD_I40E_PKG_OP_WR_DEL = 2,
	RTE_PMD_I40E_PKG_OP_WR_ONLY = 3,
	RTE_PMD_I40E_PKG_OP_MAX = 32,
}
