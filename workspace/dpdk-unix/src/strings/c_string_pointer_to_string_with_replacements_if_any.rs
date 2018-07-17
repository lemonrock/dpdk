// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[cfg(any(target_os = "android", target_os = "linux"))]
#[inline(always)]
pub(crate) fn c_string_pointer_to_string_with_replacements_if_any(nul_terminated: *mut c_char) -> Option<String>
{
	if unlikely!(nul_terminated.is_null())
	{
		return None;
	}
	
	let c_str = unsafe { CStr::from_ptr(nul_terminated) };
	Some(c_str.to_string_lossy().into_owned())
}
