// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Error caused when trying to open an ioctl socket.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OpenPciBusInformationError
{
	/// Permission denied.
	PermissionDenied,

	/// Unsupported.
	///
	/// Field contains details.
	Unsupported(&'static str),

	/// Out of memory or resources.
	///
	/// Field contains details.
	OutOfMemoryOrResources(&'static str),
}

impl Display for OpenPciBusInformationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		<OpenPciBusInformationError as Debug>::fmt(self, f)
	}
}

impl error::Error for OpenPciBusInformationError
{
	#[inline(always)]
	fn source(&self) ->  Option<&(error::Error + 'static)>
	{
		use self::OpenPciBusInformationError::*;

		match self
		{
			&PermissionDenied => None,

			&Unsupported(_) => None,

			&OutOfMemoryOrResources(_) => None,
		}
	}
}
