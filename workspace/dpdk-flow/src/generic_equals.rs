// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// This method exists because bindgen does not alway generate Copy or Clone implementations when asked to do so.
#[inline(always)]
pub(crate) fn generic_clone<T>(original: &T) -> T
{
	let mut clone: T = unsafe { uninitialized() };
	unsafe { copy_nonoverlapping(original as *const T as *const u8, (&mut clone) as *mut T as *mut u8, size_of::<T>()) };
	clone
}
