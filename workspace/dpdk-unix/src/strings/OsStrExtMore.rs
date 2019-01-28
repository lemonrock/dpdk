// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// An extension trait for `OsStr`.
pub trait OsStrExtMore
{
	/// Converts as `OsStr` to a `CString`.
	#[inline(always)]
	fn os_str_to_c_string(&self) -> CString;
}

impl OsStrExtMore for OsStr
{
	#[inline(always)]
	fn os_str_to_c_string(&self) -> CString
	{
		CString::new(self.as_bytes()).expect("os_str should not contain interior ASCII NULs")
	}
}
