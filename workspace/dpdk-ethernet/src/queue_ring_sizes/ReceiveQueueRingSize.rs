// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// An ethernet port's receive queue ring size.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
pub struct ReceiveQueueRingSize(pub(crate) u16);

impl Display for ReceiveQueueRingSize
{
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "{}", self.0)
	}
}

impl TryFrom<u16> for ReceiveQueueRingSize
{
	type Error = ();
	
	#[inline(always)]
	fn try_from(value: u16) -> Result<Self, Self::Error>
	{
		if value >= Self::Maximum as u16
		{
			Err(())
		}
		else
		{
			Ok(ReceiveQueueRingSize(value))
		}
	}
}

impl TryFrom<usize> for ReceiveQueueRingSize
{
	type Error = ();
	
	#[inline(always)]
	fn try_from(value: usize) -> Result<Self, Self::Error>
	{
		if value >= Self::Maximum as usize
		{
			Err(())
		}
		else
		{
			Ok(ReceiveQueueRingSize(value as u16))
		}
	}
}

impl Into<u16> for ReceiveQueueRingSize
{
	#[inline(always)]
	fn into(self) -> u16
	{
		self.0
	}
}

impl Into<usize> for ReceiveQueueRingSize
{
	#[inline(always)]
	fn into(self) -> usize
	{
		self.0 as usize
	}
}

impl QueueRingSize for ReceiveQueueRingSize
{
	const InclusiveMaximum: Self = ReceiveQueueRingSize((Self::Maximum as u16) - 1);
}
