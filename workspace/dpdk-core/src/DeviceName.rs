// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A device name.
pub trait DeviceName: Hash + PartialEq + Eq + Sized
{
	/// Name as a string.
	///
	/// Only works after configuration of the DPDK environment.
	#[inline(always)]
	fn to_string(&self) -> String;
	
	/// DPDK device name.
	///
	/// Only works after configuration of the DPDK environment.
	#[inline(always)]
	fn to_device_name(&self) -> CString
	{
		CString::new(self.to_string().as_str()).unwrap()
	}
}
