// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2019 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Process niceness error.
#[derive(Debug)]
pub enum ProcessNicenessAdjustmentError
{
	/// Could not set current real effective user priority niceness (permission was denied in some way).
	CouldNotSetCurrentRealEffectiveUserPriorityNiceness,

	/// Could not set current process group user priority niceness (permission was denied in some way).
	CouldNotSetCurrentProcessGroupPriorityNiceness,

	/// Could not set current process user priority niceness (permission was denied in some way).
	CouldNotSetCurrentProcessPriorityNiceness,

	/// Could not set current process user autogroup priority niceness.
	CouldNotSetCurrentProcessAutogroupPriorityNiceness(io::Error),
}

impl Display for ProcessNicenessAdjustmentError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		<ProcessNicenessAdjustmentError as Debug>::fmt(self, f)
	}
}

impl error::Error for ProcessNicenessAdjustmentError
{
	#[inline(always)]
	fn source(&self) ->  Option<&(error::Error + 'static)>
	{
		use self::ProcessNicenessAdjustmentError::*;

		match self
		{
			&CouldNotSetCurrentRealEffectiveUserPriorityNiceness => None,

			&CouldNotSetCurrentProcessGroupPriorityNiceness => None,

			&CouldNotSetCurrentProcessPriorityNiceness => None,

			&CouldNotSetCurrentProcessAutogroupPriorityNiceness(ref error) => Some(error),
		}
	}
}

impl From<io::Error> for ProcessNicenessAdjustmentError
{
	#[inline(always)]
	fn from(error: io::Error) -> Self
	{
		ProcessNicenessAdjustmentError::CouldNotSetCurrentProcessAutogroupPriorityNiceness(error)
	}
}
