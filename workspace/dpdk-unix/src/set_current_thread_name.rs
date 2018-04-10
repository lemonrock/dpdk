// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Sets the current thread name.
pub fn set_current_thread_name(name: &str) -> Result<(), SetCurrentThreadNameError>
{
	match name.len()
	{
		0 => Err(SetCurrentThreadNameError::NameIsEmpty),
		
		length if length > 15 => Err(SetCurrentThreadNameError::NameIsTooLong),
		
		_ =>
		{
			let c_string = CString::new(name.to_owned())?;
			let pointer = c_string.as_ptr();
			
			#[cfg(any(target_os = "android", target_os = "linux"))] unsafe { ::libc::prctl(::libc::PR_SET_NAME, pointer) }; // `pthread_getname_np` in glibc and modern musl but not Android.
			#[cfg(target_os = "netbsd")] unsafe { ::libc::pthread_setname_np(::libc::pthread_self(), pointer) };
			#[cfg(any(target_os = "ios", target_os = "macos"))] unsafe { ::libc::pthread_setname_np(pointer) };
			
			// pthread_set_name_np for FreeBSD / OpenBSD, but not implemented by Rust libc yet.
			// See https://stackoverflow.com/questions/2369738/can-i-set-the-name-of-a-thread-in-pthreads-linux.
			
			Ok(())
		}
	}
}
