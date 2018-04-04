// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum BypassWatchdogTimeout
{
	TimerIsOff = 0,
	OneAndAHalfSeconds = 1,
	TwoSeconds = 2,
	ThreeSeconds = 3,
	FourSeconds = 4,
	EightSeconds = 5,
	SixteenSeconds = 6,
	ThirtyTwoSeconds = 7,
}

impl BypassWatchdogTimeout
{
	#[inline(always)]
	fn fromC(bypassWatchdogTimeoutValue: uint32_t, function: &str) -> BypassWatchdogTimeout
	{
		match bypassWatchdogTimeoutValue
		{
			0 => BypassWatchdogTimeout::TimerIsOff,
			1 => BypassWatchdogTimeout::OneAndAHalfSeconds,
			2 => BypassWatchdogTimeout::TwoSeconds,
			3 => BypassWatchdogTimeout::ThreeSeconds,
			4 => BypassWatchdogTimeout::FourSeconds,
			5 => BypassWatchdogTimeout::EightSeconds,
			6 => BypassWatchdogTimeout::SixteenSeconds,
			7 => BypassWatchdogTimeout::ThirtyTwoSeconds,
			
			_ => panic!("{}() returned an invalid bypass watchdog timeout value '{}'", function, bypassWatchdogTimeoutValue),
		}
	}
	
	#[inline(always)]
	pub fn toMilliseconds(&self) -> Option<usize>
	{
		match *self
		{
			BypassWatchdogTimeout::TimerIsOff => None,
			BypassWatchdogTimeout::OneAndAHalfSeconds => Some(1_500),
			BypassWatchdogTimeout::TwoSeconds => Some(2_000),
			BypassWatchdogTimeout::ThreeSeconds => Some(3_000),
			BypassWatchdogTimeout::FourSeconds => Some(4_000),
			BypassWatchdogTimeout::EightSeconds => Some(8_000),
			BypassWatchdogTimeout::SixteenSeconds => Some(16_000),
			BypassWatchdogTimeout::ThirtyTwoSeconds => Some(32_000),
		}
	}
}
