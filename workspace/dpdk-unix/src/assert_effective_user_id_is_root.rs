// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Asserts that the effective user id (`uid`) is root.
///
/// Takes a necessity to explain why the user must be root.
#[inline(always)]
pub fn assert_effective_user_id_is_root(necessity: &str)
{
	let effective_user_id = unsafe { geteuid() };
	assert_eq!(effective_user_id, 0, "Effective User Id (euid) is not root (0). Necessity: {}", necessity);
}
