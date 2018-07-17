// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A value in KiloBytes.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
pub struct KiloBytes(u64);

impl Display for KiloBytes
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "{} kb", self.0)
	}
}

impl From<u8> for KiloBytes
{
	#[inline(always)]
	fn from(value: u8) -> Self
	{
		KiloBytes(value as u64)
	}
}

impl From<u16> for KiloBytes
{
	#[inline(always)]
	fn from(value: u16) -> Self
	{
		KiloBytes(value as u64)
	}
}

impl From<u32> for KiloBytes
{
	#[inline(always)]
	fn from(value: u32) -> Self
	{
		KiloBytes(value as u64)
	}
}

impl From<u64> for KiloBytes
{
	#[inline(always)]
	fn from(value: u64) -> Self
	{
		KiloBytes(value)
	}
}

impl Into<u64> for KiloBytes
{
	#[inline(always)]
	fn into(self) -> u64
	{
		self.0
	}
}

impl From<MegaBytes> for KiloBytes
{
	#[inline(always)]
	fn from(value: MegaBytes) -> Self
	{
		KiloBytes(value.0 * 1024)
	}
}

impl KiloBytes
{
	/// Scale down by `ratio`.
	#[inline(always)]
	pub fn scale_by(self, ratio: PerMyriad) -> Self
	{
		KiloBytes(ratio.scale_down_u64(self.0))
	}
	
	/// Subtract with a floor of zero.
	#[inline(always)]
	pub fn subtract_with_zero_floor(self, value: Self) -> Self
	{
		KiloBytes(self.0.checked_sub(value.0).unwrap_or(0))
	}
}
