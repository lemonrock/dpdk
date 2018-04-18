// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A trait to dump all information to a file, a string, standard out or standard error.
pub trait PrintAllInformation
{
	/// Print information to standard out.
	#[inline(always)]
	fn print_information_to_standard_out()
	{
		let fake: Adaptor<Self> = Adaptor(PhantomData);
		fake.print_information_to_standard_out(file)
	}
	
	/// Print information to standard error.
	#[inline(always)]
	fn print_information_to_standard_error(&self)
	{
		let fake: Adaptor<Self> = Adaptor(PhantomData);
		fake.print_information_to_standard_error(file)
	}
	
	/// Print information to a file opened for writing.
	#[inline(always)]
	fn print_information_to_file(file: File) -> Result<(), io::Error>
	{
		let fake: Adaptor<Self> = Adaptor(PhantomData);
		fake.print_information_to_c_file(file)
	}
	
	/// Print information to a string.
	#[inline(always)]
	fn print_information_to_c_string() -> Result<CString, io::Error>
	{
		let fake: Adaptor<Self> = Adaptor(PhantomData);
		fake.print_information_to_c_string()
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn print_information_to_stream(stream: *mut FILE);
}
