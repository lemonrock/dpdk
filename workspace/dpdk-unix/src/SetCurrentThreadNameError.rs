// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


quick_error!
{
	/// An error occurred when setting the current thread name.
	#[derive(Debug)]
	pub enum SetCurrentThreadNameError
	{
		/// A thread name is empty.
		NameIsEmpty
		{
		}
		
		/// A thread name is too long (it must be 15 characters or less).
		NameIsTooLong
		{
			display("Name must be 15 characters or less")
		}
		
		/// A thread name contains an ASCII NUL.
		NameContainsNul(cause: NulError)
		{
			cause(cause)
			from()
		}
	}
}
