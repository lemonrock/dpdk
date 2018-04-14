// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Internet Protocol (IP) traffic class.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct TrafficClass
{
	pub differentiated_service_code_point: DifferentiatedServiceCodePoint,
	pub explicit_congestion_notification: ExplicitCongestionNotification,
}

impl TrafficClass
{
	/// To an u8 value.
	#[inline(always)]
	pub fn as_u8(&self) -> u8
	{
		self.differentiated_service_code_point.into() << 2 | self.explicit_congestion_notification as u8
	}
	
	/// To an u32 value
	///
	/// * not bit shifted
	/// * in native endian order.
	#[inline(always)]
	pub fn as_u32(&self) -> u32
	{
		self.as_u8() as u32
	}
}
