// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Internet Protocol (IP) traffic class.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct TrafficClass
{
	/// Differentiated service code point.
	pub differentiated_service_code_point: DifferentiatedServiceCodePoint,
	
	/// Explicit congestion notification.
	pub explicit_congestion_notification: ExplicitCongestionNotification,
}

impl Display for TrafficClass
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "{}, {}", self.differentiated_service_code_point, self.explicit_congestion_notification)
	}
}

impl Into<u8> for TrafficClass
{
	#[inline(always)]
	fn into(self) -> u8
	{
		self.differentiated_service_code_point.into() << 2 | self.explicit_congestion_notification.into()
	}
}

impl From<u8> for TrafficClass
{
	#[inline(always)]
	fn from(value: u8) -> Self
	{
		Self
		{
			differentiated_service_code_point: DifferentiatedServiceCodePoint::from(value >> 2),
			explicit_congestion_notification: unsafe { transmute(value & 0b11) },
		}
	}
}
