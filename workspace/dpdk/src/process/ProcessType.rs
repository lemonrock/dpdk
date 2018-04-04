// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ProcessType
{
	Auto,
	Primary,
	Secondary,
}

impl ProcessType
{
	const_cstr!
	{
		auto = "auto";
		primary = "primary";
		secondary = "secondary";
	}
	
	#[inline(always)]
	pub fn current() -> Option<ProcessType>
	{
		match unsafe { ::dpdk_sys::rte_eal_process_type() }
		{
			rte_proc_type_t::RTE_PROC_AUTO => Some(ProcessType::Auto),
			rte_proc_type_t::RTE_PROC_PRIMARY => Some(ProcessType::Primary),
			rte_proc_type_t::RTE_PROC_SECONDARY => Some(ProcessType::Secondary),
			rte_proc_type_t::RTE_PROC_INVALID => None,
		}
	}
	
	pub fn asInitialisationArgument(self) -> ConstCStr
	{
		match self
		{
			ProcessType::Auto => Self::auto,
			ProcessType::Primary => Self::primary,
			ProcessType::Secondary => Self::secondary,
		}
	}
}
