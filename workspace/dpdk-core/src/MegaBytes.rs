// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A value in MegaBytes.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
pub struct MegaBytes(u64);

impl Display for MegaBytes
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "{} mb", self.0)
	}
}

impl From<u8> for MegaBytes
{
	#[inline(always)]
	fn from(value: u8) -> Self
	{
		MegaBytes(value as u64)
	}
}

impl From<u16> for MegaBytes
{
	#[inline(always)]
	fn from(value: u16) -> Self
	{
		MegaBytes(value as u64)
	}
}

impl From<u32> for MegaBytes
{
	#[inline(always)]
	fn from(value: u32) -> Self
	{
		MegaBytes(value as u64)
	}
}

impl From<u64> for MegaBytes
{
	#[inline(always)]
	fn from(value: u64) -> Self
	{
		MegaBytes(value)
	}
}

impl From<KiloBytes> for MegaBytes
{
	#[inline(always)]
	fn from(value: KiloBytes) -> Self
	{
		MegaBytes(value.0 / 1024)
	}
}

impl Into<u64> for MegaBytes
{
	#[inline(always)]
	fn into(self) -> u64
	{
		self.0
	}
}

impl MegaBytes
{
	/// Scale up by `scalar`.
	#[inline(always)]
	pub fn scale_by(self, scalar: u64) -> Self
	{
		MegaBytes(self.0 * scalar)
	}
	
	/// Creates a string representation capped at 512 Mb.
	#[inline(always)]
	pub fn to_string_capped_at_dpdk_maximum(self) -> String
	{
		const MaximumMegaBytes: u64 = 512;
		
		format!("{}", min(self.0, MaximumMegaBytes))
	}
}
