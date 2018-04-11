// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum rte_pmd_i40e_package_info
{
	RTE_PMD_I40E_PKG_INFO_UNDEFINED = 0,
	RTE_PMD_I40E_PKG_INFO_GLOBAL_HEADER = 1,
	RTE_PMD_I40E_PKG_INFO_GLOBAL_NOTES_SIZE = 2,
	RTE_PMD_I40E_PKG_INFO_GLOBAL_NOTES = 3,
	RTE_PMD_I40E_PKG_INFO_GLOBAL_MAX = 1024,
	RTE_PMD_I40E_PKG_INFO_HEADER = 1025,
	RTE_PMD_I40E_PKG_INFO_DEVID_NUM = 1026,
	RTE_PMD_I40E_PKG_INFO_DEVID_LIST = 1027,
	RTE_PMD_I40E_PKG_INFO_PROTOCOL_NUM = 1028,
	RTE_PMD_I40E_PKG_INFO_PROTOCOL_LIST = 1029,
	RTE_PMD_I40E_PKG_INFO_PCTYPE_NUM = 1030,
	RTE_PMD_I40E_PKG_INFO_PCTYPE_LIST = 1031,
	RTE_PMD_I40E_PKG_INFO_PTYPE_NUM = 1032,
	RTE_PMD_I40E_PKG_INFO_PTYPE_LIST = 1033,
	RTE_PMD_I40E_PKG_INFO_MAX = -1,
}
