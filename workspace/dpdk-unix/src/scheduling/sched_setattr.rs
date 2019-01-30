// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2019 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// The `flags` argument is provided to allow for future extensions to the interface; in the current implementation it must be specified as `0`.
#[inline(always)]
pub(crate) fn sched_setattr(pid: pid_t, attr: *mut sched_attr, flags: c_uint) -> c_int
{
	#[cfg(target_arch = "aarch64")] const __NR_sched_setattr: c_long = 274;
	#[cfg(target_arch = "arm")] const __NR_sched_setattr: c_long = 380;
	#[cfg(target_arch = "x86")] const __NR_sched_setattr: c_long = 351;
	#[cfg(target_arch = "mips")] const __NR_sched_setattr: c_long = 4349;
	#[cfg(target_arch = "mips64")] const __NR_sched_setattr: c_long = 5309;
	#[cfg(target_arch = "powerpc")] const __NR_sched_setattr: c_long = 355;
	#[cfg(target_arch = "s390")] const __NR_sched_setattr: c_long = 345;
	#[cfg(target_arch = "x86_64")] const __NR_sched_setattr: c_long = 314;

	return unsafe { syscall(__NR_sched_setattr, pid, attr, flags) as c_int }
}
