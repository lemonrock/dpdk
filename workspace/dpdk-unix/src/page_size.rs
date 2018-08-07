// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A very slightly faster function to get page size than `sysconf(_SC_PAGESIZE)` on musl libc systems.
///
/// Result is normally constant, but is derived from data passed when an executable is first loaded.
#[inline(always)]
pub fn page_size() -> usize
{
	// `getpagesize()` is faster than `sysconf(_SC_PAGESIZE)` on musl libc systems.
	unsafe { ::libc_extra::unix::unistd::getpagesize() as usize }
}
