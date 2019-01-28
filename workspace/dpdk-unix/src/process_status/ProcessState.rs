// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2019 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Process state.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ProcessState
{
	/// Also known as `R (running)`.
	Running,

	/// Also known as `S (sleeping)`.
	Sleeping,

	/// Also known as `D (disk sleep)`, or disk sleep.
	SleepingInAnUninterruptibleWait,

	/// Also known as `T (stopped)`.
	TracedOrStopped,

	/// Also known as `t (tracing stop)`.
	TracingStop,

	/// Also known as `X (dead)`.
	Dead,

	/// Also known as `Z (zombie)`.
	Zombie,

	/// Also known as `P (parked)`.
	Parked,

	/// Also known as `I (idle)`.
	Idle,
}
