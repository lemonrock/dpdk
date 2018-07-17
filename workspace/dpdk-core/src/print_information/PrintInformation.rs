// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A trait to dump information to a file, a string, standard out or standard error.
pub trait PrintInformation
{
	/// Print information to standard out.
	#[inline(always)]
	fn print_information_to_standard_out(&self)
	{
		self.print_information_to_stream(unsafe { stdout } as *mut _)
	}
	
	/// Print information to standard error.
	#[inline(always)]
	fn print_information_to_standard_error(&self)
	{
		self.print_information_to_stream(unsafe { stderr } as *mut _)
	}
	
	/// Print information to a file opened for writing.
	#[inline(always)]
	fn print_information_to_file(&self, file: File) -> Result<(), io::Error>
	{
		let file_descriptor = file.into_raw_fd();
		
		let open_flags = const_cstr!("a");
		let stream = unsafe { fdopen(file_descriptor, open_flags.as_ptr()) };
		if stream.is_null()
		{
			return Err(io::Error::last_os_error());
		}
		
		self.print_information_to_stream(stream);
		
		match unsafe { fflush(stream) }
		{
			0 => (),
			
			EOF =>
			{
				unsafe { fclose(stream) };
				return Err(io::Error::last_os_error())
			}
			
			unexpected @ _ =>
			{
				unsafe { fclose(stream) };
				panic!("Unexpected result code '{}' from 'fflush'", unexpected)
			}
		}
		
		match unsafe { fclose(stream) }
		{
			0 => Ok(()),
			EOF => Err(io::Error::last_os_error()),
			unexpected @ _ => panic!("Unexpected result code '{}' from 'fclose'", unexpected),
		}
	}
	
	/// Print information to a string.
	#[inline(always)]
	fn print_information_to_c_string(&self) -> Result<CString, io::Error>
	{
		let mut buffer = unsafe { uninitialized() };
		let mut size = unsafe { uninitialized() };
		let stream = unsafe { open_memstream(&mut buffer, &mut size) };
		if stream.is_null()
		{
			return Err(io::Error::last_os_error());
		}
		
		self.print_information_to_stream(stream);
		
		match unsafe { fflush(stream) }
		{
			0 => (),
			
			EOF =>
			{
				unsafe { fclose(stream) };
				unsafe { free(buffer as *mut c_void) };
				return Err(io::Error::last_os_error())
			}
			
			unexpected @ _ =>
			{
				unsafe { fclose(stream) };
				unsafe { free(buffer as *mut c_void) };
				panic!("Unexpected result code '{}' from 'fflush'", unexpected)
			}
		}
		
		match unsafe { fclose(stream) }
		{
			0 => (),
			
			EOF =>
			{
				unsafe { free(buffer as *mut c_void) };
				return Err(io::Error::last_os_error())
			}
			
			unexpected @ _ =>
			{
				unsafe { free(buffer as *mut c_void) };
				panic!("Unexpected result code '{}' from 'fclose'", unexpected)
			}
		}
		
		let c_string = unsafe { CStr::from_ptr(buffer) }.into();
		
		unsafe { free(buffer as *mut c_void) };
		
		Ok(c_string)
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn print_information_to_stream(&self, stream: *mut FILE);
}
