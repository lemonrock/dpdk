// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2019 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A Linux scheduler.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize)]
pub enum Scheduler
{
	/// For running very low priority background jobs.
	///
	/// Lower priority than `Idle`.
	///
	/// The process' `nice` value is ignored.
	///
	/// This policy is intended for running jobs at extremely low priority (lower even than a `+1`9 nice value with the `Other` or `Batch` policies).
	Idle,

	/// For 'batch' style execution of processes.
	///
	/// Slightly lower priority than `Other`, higher than `Idle`.
	///
	/// Since Linux 2.6.16.
	///
	/// This policy is similar to `Other` in that it schedules the process according to its dynamic priority (based on the nice value).
	/// The difference is that this policy will cause the scheduler to always assume that the process is CPU-intensive.
	/// Consequently, the scheduler will apply a small scheduling penalty with respect to wakeup behaviour, so that this process is mildly disfavored in scheduling decisions.
	/// This policy is useful for workloads that are noninteractive, but do not want to lower their nice value, and for workloads that want a deterministic scheduling policy without interactivity causing extra preemptions (between the workload's tasks).
	Batch(Nice),

	/// The standard (and default) round-robin time-sharing scheduler.
	///
	/// The process to run is chosen from the static priority 0 list based on a dynamic priority that is determined only inside this list.
	///
	/// The dynamic priority is based on the nice value (set by `setpriority()`) and increased for each time quantum the process is ready to run, but denied to run by the scheduler.
	///
	/// This ensures fair progress among all `Other` processes.
	Other(Nice),

	/// A first-in, first-out real time scheduler.
	///
	/// When a `RealTimeFirstInFirstOut` processes becomes runnable, it will always immediately preempt any currently running `Other`, `Batch` or `Idle` process.
	///
	/// A `RealTimeFirstInFirstOut` process that has been preempted by another process of higher priority will stay at the head of the list for its priority and will resume execution as soon as all processes of higher priority are blocked again.
	///
	/// When a `RealTimeFirstInFirstOut` process becomes runnable, it will be inserted at the end of the list for its priority.
	///
	/// A `RealTimeFirstInFirstOut` process runs until either it is blocked by an I/O request, it is preempted by a higher priority process, or it calls `sched_yield()`.
	RealTimeFirstInFirstOut(RealTimeSchedulerPriority),

	/// A round-robin real time scheduler.
	///
	/// Everything described above for `RealTimeFirstInFirstOut` also applies to `RealTimeRoundRobin`, except that each process is only allowed to run for a maximum time quantum.
	///
	/// If a `RealTimeRoundRobin` process has been running for a time period equal to or longer than the time quantum, it will be put at the end of the list for its priority.
	///
	/// A `RealTimeRoundRobin` process that has been preempted by a higher priority process and subsequently resumes execution as a running process will complete the unexpired portion of its round-robin time quantum.
	///
	/// The length of the time quantum can be retrieved using `sched_rr_get_interval()`.
	RealTimeRoundRobin(RealTimeSchedulerPriority),

	/// A real time scheduler that takes precedence over all other schedulers.
	///
	/// Also known as "Earliest-Deadline-First" (EDF).
	///
	/// Since Linux 3.14.
	///
	/// Using a Deadline scheduler is impossible if a thread has an affinity to less than the total CPUs on the system (or in the current cgroup).
	Deadline
	{
		/// Runtime parameter.
		runtime_in_nanoseconds: u64,

		/// Deadline parameter.
		deadline_in_nanoseconds: u64,

		/// Period parameter.
		period_in_nanoseconds: u64,
	},
}

impl Default for Scheduler
{
	#[inline(always)]
	fn default() -> Self
	{
		Scheduler::Other(Nice::Zero)
	}
}

impl Scheduler
{
	/// Returns an error if permission was denied or a deadline scheduler could not be brought into use.
	#[inline(always)]
	pub fn set_for_current_thread(&self)-> Result<(), &'static str>
	{
		const SCHED_DEADLINE: i32 = 6;
		const SCHED_RESET_ON_FORK: u64 = 0x40000000;

		use self::Scheduler::*;

		let mut parameters = match self
		{
			&Idle => sched_attr
			{
				size: sched_attr::SCHED_ATTR_SIZE_VER0,
				sched_policy: SCHED_IDLE as u32,
				sched_flags: SCHED_RESET_ON_FORK,
				sched_nice: 0,
				sched_priority: 0,
				sched_runtime: 0,
				sched_deadline: 0,
				sched_period: 0,
			},

			&Batch(nice) => sched_attr
			{
				size: sched_attr::SCHED_ATTR_SIZE_VER0,
				sched_policy: SCHED_BATCH as u32,
				sched_flags: SCHED_RESET_ON_FORK,
				sched_nice: nice as i32,
				sched_priority: 0,
				sched_runtime: 0,
				sched_deadline: 0,
				sched_period: 0,
			},

			&Other(nice) => sched_attr
			{
				size: sched_attr::SCHED_ATTR_SIZE_VER0,
				sched_policy: SCHED_OTHER as u32,
				sched_flags: SCHED_RESET_ON_FORK,
				sched_nice: nice as i32,
				sched_priority: 0,
				sched_runtime: 0,
				sched_deadline: 0,
				sched_period: 0,
			},

			&RealTimeFirstInFirstOut(real_time_scheduler_priority) => sched_attr
			{
				size: sched_attr::SCHED_ATTR_SIZE_VER0,
				sched_policy: SCHED_FIFO as u32,
				sched_flags: SCHED_RESET_ON_FORK,
				sched_nice: 0,
				sched_priority: real_time_scheduler_priority as u32 as i32,
				sched_runtime: 0,
				sched_deadline: 0,
				sched_period: 0,
			},

			&RealTimeRoundRobin(real_time_scheduler_priority) => sched_attr
			{
				size: sched_attr::SCHED_ATTR_SIZE_VER0,
				sched_policy: SCHED_RR as u32,
				sched_flags: SCHED_RESET_ON_FORK,
				sched_nice: 0,
				sched_priority: real_time_scheduler_priority as u32 as i32,
				sched_runtime: 0,
				sched_deadline: 0,
				sched_period: 0,
			},

			&Deadline { runtime_in_nanoseconds, deadline_in_nanoseconds, period_in_nanoseconds } => sched_attr
			{
				size: sched_attr::SCHED_ATTR_SIZE_VER0,
				sched_policy: SCHED_DEADLINE as u32,
				sched_flags: SCHED_RESET_ON_FORK,
				sched_nice: 0,
				sched_priority: 0,
				sched_runtime: runtime_in_nanoseconds,
				sched_deadline: deadline_in_nanoseconds,
				sched_period: period_in_nanoseconds,
			},
		};

		const CurrentThread: pid_t = 0;
		let result = sched_setattr(CurrentThread, &mut parameters, 0);

		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EPERM => Err("Permission denied, or, for deadline tasks, the CPU affinity mask of the thread (pid) does not include all CPUS in the current cgroup (or system)"),
				EBUSY => Err("Deadline scheduler admission control failure (?)"),

				ESRCH => panic!("The thread whose ID is pid could not be found"),
				EINVAL => panic!("`attr` is NULL; or `pid` is negative; or `flags` is not zero; `attr.sched_policy` is not one of the recognized policies; `attr.sched_flags` contains a flag other than `SCHED_FLAG_RESET_ON_FORK`; or `attr.sched_priority` is invalid; or `attr.sched_policy` is `SCHED_DEADLINE` and the deadline scheduling parameters in `attr` are invalid"),
				E2BIG => panic!("The buffer specified by `size` and `attr` is larger than the kernel structure, and one or more of the excess bytes is nonzero"),

				_ => unreachable!(),

			}
		}
		else
		{
			unreachable!()
		}

		/*

int setpriority(int which, id_t who, int prio)
{
	return syscall(SYS_setpriority, which, who, prio);
}

int sched_setattr(pid_t pid,
              const struct sched_attr *attr,
              unsigned int flags)
{
    return syscall(__NR_sched_setattr, pid, attr, flags);
}

int sched_getattr(pid_t pid,
              struct sched_attr *attr,
              unsigned int size,
              unsigned int flags)
{
    return syscall(__NR_sched_getattr, pid, attr, size, flags);
}

		*/


		/*

EINVAL

The scheduling policy is not one of the recognized policies, param is NULL, or param does not make sense for the policy.

EPERM

The calling process does not have appropriate privileges.

ESRCH

The process whose ID is pid could not be found.

		*/

		// What is SCHED_NORMAL? info on SCHED_DEADLINE?
	}
}

