// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// This method exists because bindgen does not alway generate PartialEq and Eq implementations when asked to do so.
#[inline(always)]
pub(crate) fn generic_equals<T>(left: &T, right: &T) -> bool
{
	(unsafe { memcmp(left, right, size_of::<T>()) }) == 0
}
