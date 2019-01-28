// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2019 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Signal queue status.
#[derive(Default, Debug, Copy,Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SignalQueueStatus
{
	/// Number of signals queued.
	pub number_of_signals_queued: u64,

	/// Maximum number of signals that can be queued (maximum queue depth).
	pub maximum_number_of_signals_that_can_be_queued: u64,
}
