// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2019 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A CPU set wrapper.
pub struct CpuSet(cpu_set_t);

impl Default for CpuSet
{
	#[inline(always)]
	fn default() -> Self
	{
		Self(unsafe { zeroed() })
	}
}

impl<'a> From<&'a BTreeSet<HyperThread>> for CpuSet
{
	#[inline(always)]
	fn from(hyper_threads: &BTreeSet<HyperThread>) -> CpuSet
	{
		let mut cpu_set = Self::default();
		for hyper_thread in hyper_threads.iter()
		{
			cpu_set.set_hyper_thread(*hyper_thread);
		}
		cpu_set
	}
}

impl CpuSet
{
	const SizeOfCpuSetT: usize = size_of::<cpu_set_t>();

	/// Set process affinity for current process.
	#[inline(always)]
	pub fn set_current_process_affinity(&self) -> io::Result<()>
	{
		self.set_process_affinity(0)
	}

	/// Set process affinity.
	#[inline(always)]
	pub fn set_process_affinity(&self, process_identifier: pid_t) -> io::Result<()>
	{
		#[link(name = "c")]
		extern "C"
		{
			/// Defined in `system/include/libc/sched.h` in emscripten source code.
			pub(crate) fn sched_setaffinity(tid: pid_t, size: size_t, set: *const cpu_set_t) -> c_int;
		}

		let result = unsafe { sched_setaffinity(process_identifier, Self::SizeOfCpuSetT, &self.0) };
		if result == 0
		{
			Ok(())
		}
		else
		{
			Err(io::Error::from_raw_os_error(result))
		}
	}

	/// Set thread affinity.
	#[cfg(any(target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_env = "uclibc"))]
	#[inline(always)]
	pub fn set_thread_affinity(&self, thread_identifier: pthread_t) -> io::Result<()>
	{
		#[link(name = "c")]
		extern "C"
		{
			/// Whilst present-ish in the libc crate, it is not defined for musl and weirdly seems to have additional definitions for mips and s390x.
			pub(crate) fn pthread_setaffinity_np(thread: pthread_t, cpusetsize: size_t, cpuset: *const cpu_set_t) -> c_int;
		}

		let result = unsafe { pthread_setaffinity_np(thread_identifier, Self::SizeOfCpuSetT, &self.0) };
		if result == 0
		{
			Ok(())
		}
		else
		{
			Err(io::Error::from_raw_os_error(result))
		}
	}

	/// Set thread affinity for current thread.
	#[inline(always)]
	pub fn set_current_thread_affinity(&self) -> io::Result<()>
	{
		self.set_thread_affinity(unsafe { pthread_self() })
	}

	/// Set a hyper thread in the CPU set.
	#[inline(always)]
	pub fn set_hyper_thread(&mut self, hyper_thread: HyperThread)
	{
		unsafe { CPU_SET(hyper_thread.0 as usize, &mut self.0) }
	}

	#[cfg(any(target_os = "android"))]
	#[inline(always)]
	fn _set_thread_affinity(&self, thread_identifier: ThreadIdentifier) -> io::Result<()>
	{
		Ok(())
	}
}
