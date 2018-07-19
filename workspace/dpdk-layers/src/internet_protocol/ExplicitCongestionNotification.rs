// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Explicit congestion notification (ECN) value.
///
/// Defaults to `NotCapableTransport`.
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub enum ExplicitCongestionNotification
{
	#[allow(missing_docs)]
	NotCapableTransport = 0b00,
	
	#[allow(missing_docs)]
	CapableTransportEctZero = 0b10,
	
	#[allow(missing_docs)]
	CapableTransportEctOne = 0b01,
	
	#[allow(missing_docs)]
	CongestionEncountered = 0b11,
}

impl Display for ExplicitCongestionNotification
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		use self::ExplicitCongestionNotification::*;
		
		let string = match *self
		{
			NotCapableTransport => "None",
			
			CapableTransportEctZero => "ECT(0)",
			
			CapableTransportEctOne => "ECT(1)",
			
			CongestionEncountered => "CE",
		};
		write!(f, "{}", string)
	}
}

impl Default for ExplicitCongestionNotification
{
	#[inline(always)]
	fn default() -> Self
	{
		ExplicitCongestionNotification::NotCapableTransport
	}
}

impl Into<u8> for ExplicitCongestionNotification
{
	#[inline(always)]
	fn into(self) -> u8
	{
		self as u8
	}
}
