// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2019 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Disable Transparent Huge Pages (THP) error.
#[derive(Debug)]
pub enum DisableTransparentHugePagesError
{
	/// Could not disable defragmentation.
	Defragmentation(io::Error),

	/// Could not disable usage.
	Usage(io::Error),
}

impl Display for DisableTransparentHugePagesError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		<DisableTransparentHugePagesError as Debug>::fmt(self, f)
	}
}

impl error::Error for DisableTransparentHugePagesError
{
	#[inline(always)]
	fn source(&self) ->  Option<&(error::Error + 'static)>
	{
		use self::DisableTransparentHugePagesError::*;

		match self
		{
			&Defragmentation(ref error) => Some(error),

			&Usage(ref error) => Some(error),
		}
	}
}
