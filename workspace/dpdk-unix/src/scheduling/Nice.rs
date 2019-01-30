// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2019 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.
//


/// Represent a `nice` value.
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
#[derive(Deserialize)]
pub enum Nice
{
	/// Least priority.
	Positive_19 = 19,

	Positive_18 = 18,

	Positive_17 = 17,

	Positive_16 = 16,

	Positive_15 = 15,

	Positive_14 = 14,

	Positive_13 = 13,

	Positive_12 = 12,

	Positive_11 = 11,

	Positive_10 = 10,

	Positive_9 = 9,

	Positive_8 = 8,

	Positive_7 = 7,

	Positive_6 = 6,

	Positive_5 = 5,

	Positive_4 = 4,

	Positive_3 = 3,

	Positive_2 = 2,

	Positive_1 = 1,

	Zero = 0,

	Negative_1 = -1,

	Negative_2 = -2,

	Negative_3 = -3,

	Negative_4 = -4,

	Negative_5 = -5,

	Negative_6 = -6,

	Negative_7 = -7,

	Negative_8 = -8,

	Negative_9 = -9,

	Negative_10 = -10,

	Negative_11 = -11,

	Negative_12 = -12,

	Negative_13 = -13,

	Negative_14 = -14,

	Negative_15 = -15,

	Negative_16 = -16,

	Negative_17 = -17,

	Negative_18 = -18,

	Negative_19 = -19,

	/// Highest priority.
	Negative_20 = -20,
}

impl Display for Nice
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "{}", *self as i32)
	}
}

impl Default for Nice
{
	#[inline(always)]
	fn default() -> Self
	{
		Nice::Negative_20
	}
}

impl Into<i8> for Nice
{
	#[inline(always)]
	fn into(self) -> i8
	{
		self as i32 as i8
	}
}

impl Into<i16> for Nice
{
	#[inline(always)]
	fn into(self) -> i16
	{
		self as i32 as i16
	}
}

impl Into<i32> for Nice
{
	#[inline(always)]
	fn into(self) -> i32
	{
		self as i32
	}
}

impl Into<i64> for Nice
{
	#[inline(always)]
	fn into(self) -> i64
	{
		self as i32 as i64
	}
}

impl Into<isize> for Nice
{
	#[inline(always)]
	fn into(self) -> isize
	{
		self as i32 as isize
	}
}

impl Nice
{
	/// Set the autogroup for the current process.
	#[cfg(any(target_os = "android", target_os = "linux"))]
	#[inline(always)]
	pub fn set_autogroup_for_current_process(self, proc_path: &ProcPath) -> Result<(), io::Error>
	{
		proc_path.adjust_autogroup_nice_value_for_self(self)
	}

	/// Set the autogroup for the current process.
	#[cfg(any(target_os = "android", target_os = "linux"))]
	#[inline(always)]
	pub fn set_autogroup_for_current_process_if_desired(only_set_if_is_some: Option<Self>, proc_path: &ProcPath) -> Result<(), io::Error>
	{
		if let Some(nice) = only_set_if_is_some
		{
			if proc_path.is_autogroup_active()?
			{
				nice.set_autogroup_for_current_process(proc_path)
			}
			else
			{
				Ok(())
			}
		}
		else
		{
			Ok(())
		}
	}

	/// This is a Linux-only thing.
	///
	/// Returns an `Err()` if the user did not have permission to adjust the priority (eg was not privileged or had the capability `CAP_SYS_NICE`).
	#[cfg(any(target_os = "android", target_os = "linux"))]
	#[inline(always)]
	pub fn set_current_thread_priority(self) -> Result<(), ()>
	{
		self.set_priority_for_self(PRIO_PROCESS)
	}

	/// This replaces the use of the legacy `nice()` function.
	///
	/// Returns an `Err()` if the user did not have permission to adjust the priority (eg was not privileged or had the capability `CAP_SYS_NICE`).
	///
	/// On Linux, actually sets the priority for only the current thread.
	#[inline(always)]
	pub fn set_current_process_priority(self) -> Result<(), ()>
	{
		self.set_priority_for_self(PRIO_PROCESS)
	}

	/// Returns an `Err()` if the user did not have permission to adjust the priority (eg was not privileged or had the capability `CAP_SYS_NICE`).
	#[inline(always)]
	pub fn set_current_process_group_priority(self) -> Result<(), ()>
	{
		self.set_priority_for_self(PRIO_PGRP)
	}

	/// Returns an `Err()` if the user did not have permission to adjust the priority (eg was not privileged or had the capability `CAP_SYS_NICE`).
	#[inline(always)]
	pub fn set_current_real_effective_user_priority(self) -> Result<(), ()>
	{
		self.set_priority_for_self(PRIO_USER)
	}

	#[inline(always)]
	fn set_priority_for_self(self, which: i32) -> Result<(), ()>
	{
		let result = unsafe { setpriority(which, 0, self as i32) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EINVAL => panic!("`which` was not one of `PRIO_PROCESS`, `PRIO_PGRP`, or `PRIO_USER`"),
				ESRCH => panic!("no process was located using the `which` and `who` values specified"),
				EACCES | EPERM => Err(()),

				_ => unreachable!(),
			}
		}
		else
		{
			unreachable!()
		}
	}
}
