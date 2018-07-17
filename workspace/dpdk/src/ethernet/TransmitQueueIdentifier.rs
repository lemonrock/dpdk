// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// An ethernet port identifier.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct TransmitQueueIdentifier(u16);

impl Into<u16> for TransmitQueueIdentifier
{
	#[inline(always)]
	fn into(self) -> u16
	{
		self.0
	}
}

impl Into<usize> for TransmitQueueIdentifier
{
	#[inline(always)]
	fn into(self) -> usize
	{
		self.0 as usize
	}
}

impl Step for TransmitQueueIdentifier
{
	#[inline(always)]
	fn steps_between(start: &Self, end: &Self) -> Option<usize>
	{
		u16::steps_between(&start.0, &end.0)
	}
	
	#[inline(always)]
	fn replace_one(&mut self) -> Self
	{
		replace(self, ReceiveQueueIdentifier(1))
	}
	
	#[inline(always)]
	fn replace_zero(&mut self) -> Self
	{
		replace(self, ReceiveQueueIdentifier(0))
	}
	
	#[inline(always)]
	fn add_one(&self) -> Self
	{
		ReceiveQueueIdentifier(self.0.add_one())
	}
	
	#[inline(always)]
	fn sub_one(&self) -> Self
	{
		ReceiveQueueIdentifier(self.0.sub_one())
	}
	
	#[inline(always)]
	fn add_usize(&self, n: usize) -> Option<Self>
	{
		self.0.add_usize(n).map(|value| ReceiveQueueIdentifier(value))
	}
}

impl TransmitQueueIdentifier
{
	/// Maximum.
	pub const Maximum: u16 = RTE_MAX_QUEUES_PER_PORT;
	
	//noinspection SpellCheckingInspection
	/// Returns an `Err(())` if the `transmit_queue_identifier` is greater than or equal to `RTE_MAX_QUEUES_PER_PORT`, currently `1024`.
	#[inline(always)]
	pub fn new(transmit_queue_identifier: u16) -> Result<Self, ()>
	{
		if (transmit_queue_identifier as usize) >= Self::Maximum
		{
			Err(())
		}
		else
		{
			Ok(TransmitQueueIdentifier(transmit_queue_identifier))
		}
	}
	
	/// All possible transmit queue identifiers.
	#[inline(always)]
	pub fn all(exclusive_maximum: u16) -> Range<Self>
	{
		TransmitQueueIdentifier(0) .. TransmitQueueIdentifier::new(exclusive_maximum).unwrap()
	}
}
