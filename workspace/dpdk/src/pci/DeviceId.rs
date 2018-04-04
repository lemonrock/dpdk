// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeviceId(pub u16);

impl DeviceId
{
	const AnyOrInvalidRaw: u16 = 0xFFFF;
	
	#[inline(always)]
	pub fn new(deviceId: u16) -> Option<Self>
	{
		if deviceId == Self::AnyOrInvalidRaw
		{
			None
		}
		else
		{
			Some(DeviceId(deviceId))
		}
	}
	
	pub const AnyOrInvalid: DeviceId = DeviceId(Self::AnyOrInvalidRaw);
	
	#[inline(always)]
	pub fn isAnyOrInvalid(&self) -> bool
	{
		self.0 == Self::AnyOrInvalidRaw
	}
	
	#[inline(always)]
	pub fn is(&self, other: u16) -> bool
	{
		self.0 == other
	}
}
