// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// An ethernet port identifier.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ReceiveQueueIdentifier(u16);

impl Into<u16> for ReceiveQueueIdentifier
{
	#[inline(always)]
	fn into(self) -> u16
	{
		self.0
	}
}

impl ReceiveQueueIdentifier
{
	//noinspection SpellCheckingInspection
	/// Returns an `Err(())` if the `receive_queue_identifier` is greater than or equal to `RTE_MAX_QUEUES_PER_PORT`, currently `1024`.
	#[inline(always)]
	pub fn new(receive_queue_identifier: u16) -> Result<Self, ()>
	{
		if (receive_queue_identifier as usize) >= RTE_MAX_QUEUES_PER_PORT
		{
			Err(())
		}
		else
		{
			Ok(ReceiveQueueIdentifier(receive_queue_identifier))
		}
	}
}
