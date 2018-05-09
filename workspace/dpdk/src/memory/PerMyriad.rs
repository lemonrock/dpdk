// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Per-myriad, one ten-thousandth
///
/// Akin to percent and permille.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
pub struct PerMyriad(u16);

impl From<u8> for PerMyriad
{
	#[inline(always)]
	fn from(value: u8) -> Self
	{
		PerMyriad(value as u16)
	}
}

impl Into<u16> for PerMyriad
{
	#[inline(always)]
	fn into(self) -> u16
	{
		self.0
	}
}

impl PerMyriad
{
	/// Minimum (a ratio of 0:1).
	pub const Minimum: Self = PerMyriad(0);
	
	/// Maximum (a ratio of 1:1).
	pub const Maximum: Self = PerMyriad(10_000);
	
	/// New value or error.
	#[inline(always)]
	pub fn new(value: u16) -> Result<Self, ()>
	{
		if value > Self::Maximum.0
		{
			Err(())
		}
		else
		{
			Ok(PerMyriad(value))
		}
	}
	
	/// Scale down.
	#[inline(always)]
	pub fn scale_down_u64(self, value: u64) -> u64
	{
		(self.0 as u64) / Self::Maximum.0
	}
}
