// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub fn cStringPointerToPathBuf(nulTerminated: *mut c_char) -> Result<Option<PathBuf>, CStringPointerConversionError>
{
	if unlikely(nulTerminated.is_null())
	{
		return Ok(None);
	}
	
	let cStr = unsafe { CStr::from_ptr(nulTerminated) };
	
	let bytes = cStr.to_bytes();
	if bytes.len() == 0
	{
		Err(CStringPointerConversionError::Empty)
	}
	else
	{
		let osStr: &OsStr = OsStrExt::from_bytes(bytes);
		Ok(Some(PathBuf::from(osStr)))
	}
}
