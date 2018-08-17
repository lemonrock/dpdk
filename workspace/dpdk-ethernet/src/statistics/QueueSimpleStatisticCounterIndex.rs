// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Transmit and receive queues can calculate statistics.
///
/// These are stored in a fixed range of statistic counters (`Self::Maximum`, usually 16), indexed by this struct.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub struct QueueSimpleStatisticCounterIndex(u8);

impl TryFrom<u8> for QueueSimpleStatisticCounterIndex
{
	type Error = ();
	
	#[inline(always)]
	fn try_from(value: u8) -> Result<Self, Self::Error>
	{
		if value < Self::Maximum as u8
		{
			Ok(QueueSimpleStatisticCounterIndex(value))
		}
		else
		{
			Err(())
		}
	}
}

impl TryFrom<usize> for QueueSimpleStatisticCounterIndex
{
	type Error = ();
	
	#[inline(always)]
	fn try_from(value: usize) -> Result<Self, Self::Error>
	{
		if value < Self::Maximum
		{
			Ok(QueueSimpleStatisticCounterIndex(value as u8))
		}
		else
		{
			Err(())
		}
	}
}

impl Into<u8> for QueueSimpleStatisticCounterIndex
{
	#[inline(always)]
	fn into(self) -> u8
	{
		self.0
	}
}

impl Into<usize> for QueueSimpleStatisticCounterIndex
{
	#[inline(always)]
	fn into(self) -> usize
	{
		self.0 as usize
	}
}

impl Step for QueueSimpleStatisticCounterIndex
{
	#[inline(always)]
	fn steps_between(start: &Self, end: &Self) -> Option<usize>
	{
		u8::steps_between(&start.0, &end.0)
	}
	
	#[inline(always)]
	fn replace_one(&mut self) -> Self
	{
		replace(self, QueueSimpleStatisticCounterIndex(1))
	}
	
	#[inline(always)]
	fn replace_zero(&mut self) -> Self
	{
		replace(self, QueueSimpleStatisticCounterIndex(0))
	}
	
	#[inline(always)]
	fn add_one(&self) -> Self
	{
		QueueSimpleStatisticCounterIndex(self.0.add_one())
	}
	
	#[inline(always)]
	fn sub_one(&self) -> Self
	{
		QueueSimpleStatisticCounterIndex(self.0.sub_one())
	}
	
	#[inline(always)]
	fn add_usize(&self, n: usize) -> Option<Self>
	{
		self.0.add_usize(n).map(|value| QueueSimpleStatisticCounterIndex(value as u8))
	}
}

impl Add<u8> for QueueSimpleStatisticCounterIndex
{
	type Output = Self;
	
	#[inline(always)]
	fn add(self, rhs: u8) -> Self::Output
	{
		QueueSimpleStatisticCounterIndex(min(self.0.saturating_add(rhs), Self::Maximum as u8))
	}
}

impl Add<usize> for QueueSimpleStatisticCounterIndex
{
	type Output = Self;
	
	#[inline(always)]
	fn add(self, rhs: usize) -> Self::Output
	{
		QueueSimpleStatisticCounterIndex(min(self.0.saturating_add(rhs as u8), Self::Maximum as u8))
	}
}

impl AddAssign<u8> for QueueSimpleStatisticCounterIndex
{
	#[inline(always)]
	fn add_assign(&mut self, rhs: u8)
	{
		*self = (*self).add(rhs)
	}
}

impl AddAssign<usize> for QueueSimpleStatisticCounterIndex
{
	#[inline(always)]
	fn add_assign(&mut self, rhs: usize)
	{
		*self = (*self).add(rhs as u8)
	}
}

impl Sub<u8> for QueueSimpleStatisticCounterIndex
{
	type Output = Self;
	
	#[inline(always)]
	fn sub(self, rhs: u8) -> Self::Output
	{
		QueueSimpleStatisticCounterIndex(self.0.saturating_sub(rhs))
	}
}

impl Sub<usize> for QueueSimpleStatisticCounterIndex
{
	type Output = Self;
	
	#[inline(always)]
	fn sub(self, rhs: usize) -> Self::Output
	{
		QueueSimpleStatisticCounterIndex(self.0.saturating_sub(rhs as u8))
	}
}

impl SubAssign<u8> for QueueSimpleStatisticCounterIndex
{
	#[inline(always)]
	fn sub_assign(&mut self, rhs: u8)
	{
		self.0 = self.0.saturating_sub(rhs)
	}
}

impl SubAssign<usize> for QueueSimpleStatisticCounterIndex
{
	#[inline(always)]
	fn sub_assign(&mut self, rhs: usize)
	{
		self.0 = self.0.saturating_sub(rhs as u8)
	}
}

impl QueueSimpleStatisticCounterIndex
{
	/// Zero.
	pub const Zero: Self = QueueSimpleStatisticCounterIndex(0);
	
	/// Maximum queue statistic counters.
	pub const Maximum: usize = RTE_ETHDEV_QUEUE_STAT_CNTRS as usize;
	
	const InclusiveMaximum: Self = QueueSimpleStatisticCounterIndex(Self::Maximum as u8 - 1);
	
	/// Gets a reference from an array of entries.
	#[inline(always)]
	pub fn get<T>(self, array: &[T; Self::Maximum]) -> &T
	{
		let into: usize = self.into();
		unsafe { array.get_unchecked(into) }
	}
	
	/// Gets a mutable reference from an array of entries.
	#[inline(always)]
	pub fn get_mut<T>(self, array: &mut [T; Self::Maximum]) -> &mut T
	{
		let into: usize = self.into();
		unsafe { array.get_unchecked_mut(into) }
	}
	
	/// Gets a value from an array of entries.
	#[inline(always)]
	pub fn get_value<T: Copy>(self, array: &[T; Self::Maximum]) -> T
	{
		*self.get(array)
	}
	
	/// Sets a value in an array of entries.
	#[inline(always)]
	pub fn set_value<T>(self, array: &mut [T; Self::Maximum], value: T)
	{
		*self.get_mut(array) = value
	}
}
