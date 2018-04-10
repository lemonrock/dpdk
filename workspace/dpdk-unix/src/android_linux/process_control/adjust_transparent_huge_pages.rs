// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Enable or disable transparent huge pages.
#[inline(always)]
pub fn adjust_transparent_huge_pages(enable_transparent_huge_pages: bool)
{
	let value = if enable_transparent_huge_pages
	{
		1
	}
	else
	{
		0
	};
	unsafe { prctl(PR_SET_THP_DISABLE, value as c_ulong) };
}
