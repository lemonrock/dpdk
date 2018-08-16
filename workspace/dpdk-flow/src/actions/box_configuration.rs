// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[inline(always)]
pub(crate) fn box_configuration<T: 'static>(drop_prevention: &mut Vec<Box<Any>>, configuration: T) -> *const c_void
{
	let boxed = Box::new(configuration);
	let pointer = boxed.as_ref() as *const T as *const c_void;
	drop_prevention.push(boxed);
	pointer
}
