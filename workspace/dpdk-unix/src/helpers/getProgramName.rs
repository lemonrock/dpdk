// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[cfg(not(any(target_os="linux", target_os="android", target_os="windows")))]
pub fn getProgramName() -> String
{
	unsafe { CStr::from_ptr(::libc::getprogname()).to_string_lossy().into_owned() }
}

#[cfg(target_os="linux")]
pub fn getProgramName() -> String
{
	unsafe { CStr::from_ptr(::libc_extra::errno::program_invocation_short_name).to_string_lossy().into_owned() }
}

#[cfg(target_os="android")]
pub fn getProgramName() -> String
{
	let pathToExeNameOrUnknownString = unsafe { CStr::from_ptr(::libc::__progname).to_string_lossy().into_owned() };
	match pathToExeNameOrUnknownString.rfind('/')
	{
		None => pathToExeNameOrUnknownString,
		Some(index) => String::from(&pathToExeNameOrUnknownString[(index + 1)..])
	}
}

#[cfg(target_os="windows")]
pub fn getProgramName() -> String
{
	match env::current_exe()
	{
		Err(_) => "(unknown)".to_string(),
		Ok(pathBuffer) =>
		{
			match pathBuffer.file_name()
			{
				None => "(unspecified)".to_string(),
				Some(ref osFileNameString) =>
				{
					let lossy = osFileNameString.to_string_lossy().to_string();
				}
			}
		}
	};
}
