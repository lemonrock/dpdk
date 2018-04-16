// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// DPDK process type of process.
///
/// Defaults to `Auto`.
///
/// Obtain from `DpdkProcess::process_type()`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ProcessType
{
	Auto,
	Primary,
	Secondary,
}

impl Default for ProcessType
{
	#[inline(always)]
	fn default() -> Self
	{
		ProcessType::Auto
	}
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
	pub(crate) fn as_initialisation_argument(self) -> ConstCStr
	{
		use self::ProcessType::*;
		
		match self
		{
			Auto => Self::auto,
			Primary => Self::primary,
			Secondary => Self::secondary,
		}
	}
}
