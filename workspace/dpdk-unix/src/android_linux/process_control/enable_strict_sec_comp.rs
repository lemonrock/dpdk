// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Allows only the syscalls `read()`, `write()`, `_exit()` and `sigreturn()`.
#[inline(always)]
pub fn enable_strict_sec_comp()
{
	unsafe { prctl(PR_SET_SECCOMP, SECCOMP_MODE_STRICT) };
}
