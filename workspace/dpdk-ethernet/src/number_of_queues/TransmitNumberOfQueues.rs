// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents a number of transmit queues for a particular ethernet device.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
pub struct TransmitNumberOfQueues(pub(crate) u16);

impl Display for TransmitNumberOfQueues
{
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "{}", self.0)
	}
}

impl Into<u16> for TransmitNumberOfQueues
{
	#[inline(always)]
	fn into(self) -> u16
	{
		self.0
	}
}

impl Into<usize> for TransmitNumberOfQueues
{
	#[inline(always)]
	fn into(self) -> usize
	{
		self.0 as usize
	}
}

impl NumberOfQueues for TransmitNumberOfQueues
{
	const Zero: Self = TransmitNumberOfQueues(0);
}
