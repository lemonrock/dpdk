// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Gets the program name using the best available technique for the Operating System.
#[cfg(any(target_os = "solaris", target_os = "macos", target_os = "ios", target_os = "freebsd", target_os = "dragonfly", target_os = "openbsd", target_os = "netbsd", target_os = "bitrig"))]
#[inline(always)]
pub fn get_program_name() -> String
{
	unsafe { CStr::from_ptr(::libc::getprogname()).to_string_lossy().into_owned() }
}

/// Gets the program name using the best available technique for the Operating System.
#[cfg(target_os = "linux")]
#[inline(always)]
pub fn get_program_name() -> String
{
	unsafe { CStr::from_ptr(program_invocation_short_name).to_string_lossy().into_owned() }
}

/// Gets the program name using the best available technique for the Operating System.
#[cfg(target_os = "android")]
#[inline(always)]
pub fn get_program_name() -> String
{
	let path_to_exe_name_or_unknown_string = unsafe { CStr::from_ptr(::libc::__progname).to_string_lossy().into_owned() };
	match path_to_exe_name_or_unknown_string.rfind('/')
	{
		None => path_to_exe_name_or_unknown_string,
		Some(index) => String::from(&path_to_exe_name_or_unknown_string[(index + 1)..])
	}
}

/// Gets the program name using the best available technique for the Operating System.
#[cfg(target_os = "windows")]
#[inline(always)]
pub fn get_program_name() -> String
{
	match env::current_exe()
	{
		Err(_) => "(unknown)".to_string(),
		Ok(path_buffer) =>
		{
			match path_buffer.file_name()
			{
				None => "(unspecified)".to_string(),
				Some(ref os_file_name_string) =>
				{
					let lossy = os_file_name_string.to_string_lossy().to_string();
				}
			}
		}
	}
}
