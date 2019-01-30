// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2019 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub(crate) struct sched_attr
{
	/// Size of this structure.
	///
	/// Currently always `Self::SCHED_ATTR_SIZE_VER0`
	pub(crate) size: u32,

	/// A scheduler policy, eg `SCHED_DEADLINE`.
	pub(crate) sched_policy: u32,

	/// Flags.
	///
	/// A bit-or of flags.
	///
	/// Currently the only known flag is `SCHED_RESET_ON_FORK`.
	pub(crate) sched_flags: u64,

	/// Value of `nice` for when `sched_policy` is either `SCHED_OTHER` or `SCHED_BATCH`.
	///
	/// The nice value is a number in the range -20 (high priority) to +19 (low priority).
	pub(crate) sched_nice: i32,

	/// Value of static `priority` for when `sched_policy` is either `SCHED_FIFO` or `SCHED_RR`.
	///
	/// A value from 1 (minimum priority) to 99 (maximum priority).
	///
	/// Otherwise 0.
	pub(crate) sched_priority: i32,

	/// Value needed when `sched_policy` is `SCHED_DEADLINE`.
	///
	/// This field specifies the "Runtime" parameter for deadline scheduling.
	/// The value is expressed in nanoseconds.
	pub(crate) sched_runtime: u64,

	/// Value needed when `sched_policy` is `SCHED_DEADLINE`.
	///
	/// This field specifies the "Deadline" parameter for deadline sheduling.
	/// The value is expressed in nanoseconds.
	pub(crate) sched_deadline: u64,

	/// Value needed when `sched_policy` is `SCHED_DEADLINE`.
	///
	/// This field specifies the "Period" parameter for deadline sheduling.
	/// The value is expressed in nanoseconds.
	pub(crate) sched_period: u64,
}

impl sched_attr
{
	/// Value of `size` field.
	pub(crate) const SCHED_ATTR_SIZE_VER0: u32 = 48;
}
