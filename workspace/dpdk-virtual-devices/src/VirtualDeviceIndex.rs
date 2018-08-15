// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A virtual device index, 0 to 31 inclusive.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct VirtualDeviceIndex(u8);

impl TryFrom<u8> for VirtualDeviceIndex
{
	type Error = ();
	
	#[inline(always)]
	fn try_from(value: u8) -> Result<Self, Self::Error>
	{
		if value > 31
		{
			Err(())
		}
		else
		{
			Ok(VirtualDeviceIndex(value))
		}
	}
}

impl Into<u8> for VirtualDeviceIndex
{
	#[inline(always)]
	fn into(self) -> u8
	{
		self.0
	}
}
