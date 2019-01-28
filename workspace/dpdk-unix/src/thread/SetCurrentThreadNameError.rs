// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// An error occurred when setting the current thread name.
#[derive(Debug,)]
pub enum SetCurrentThreadNameError
{
	/// A thread name is empty.
	NameIsEmpty,

	/// A thread name is too long (it must be 15 characters or less).
	NameIsTooLong,

	/// A thread name contains an ASCII NUL.
	NameContainsNul(NulError),
}

impl Display for SetCurrentThreadNameError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		<SetCurrentThreadNameError as Debug>::fmt(self, f)
	}
}

impl error::Error for SetCurrentThreadNameError
{
	#[inline(always)]
	fn source(&self) ->  Option<&(error::Error + 'static)>
	{
		use self::SetCurrentThreadNameError::*;

		match self
		{
			&NameIsEmpty => None,

			&NameIsTooLong => None,

			&NameContainsNul(ref error) => Some(error),
		}
	}
}

impl From<NulError> for SetCurrentThreadNameError
{
	#[inline(always)]
	fn from(error: NulError) -> Self
	{
		SetCurrentThreadNameError::NameContainsNul(error)
	}
}
