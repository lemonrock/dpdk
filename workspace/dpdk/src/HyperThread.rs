// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents a logical hyper thread, which in Operating System terms is usually a logical CPU (core).
///
/// These usually map 1:1 with `LogicalCore`s
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
pub struct HyperThread(u16);

impl HyperThread
{
	/// CPUs (hyper threaded logical cores) that are present and that could become online.
	///
	/// Consider using libnuma instead of this call.
	///
	/// See <https://www.kernel.org/doc/Documentation/cputopology.txt>.
	#[inline(always)]
	pub fn present(sys_path: &SysPath) -> BTreeSet<Self>
	{
		Self::parse_list_mask(sys_path, "present")
	}
	
	/// CPUs (hyper threaded logical cores) that are online at some point.
	///
	/// Consider using libnuma instead of this call.
	///
	/// See <https://www.kernel.org/doc/Documentation/cputopology.txt>.
	#[inline(always)]
	pub fn online(sys_path: &SysPath) -> BTreeSet<Self>
	{
		Self::parse_list_mask(sys_path, "online")
	}
	
	/// CPUs (hyper threaded logical cores) that are offline.
	///
	/// The maximum CPU index in this list ***can exceed the kernel's maximum in `self.kernel_maximum_index`***.
	///
	/// Close to useless.
	///
	/// See <https://www.kernel.org/doc/Documentation/cputopology.txt>.
	#[inline(always)]
	pub fn offline(sys_path: &SysPath) -> BTreeSet<Self>
	{
		Self::parse_list_mask(sys_path, "offline")
	}
	
	/// CPUs (hyper threaded logical cores) that could possibly be online at some point.
	///
	/// Close to very useless.
	///
	/// See <https://www.kernel.org/doc/Documentation/cputopology.txt>.
	#[inline(always)]
	pub fn possible(sys_path: &SysPath) -> BTreeSet<Self>
	{
		Self::parse_list_mask(sys_path, "possible")
	}
	
	/// Is this CPU online?
	///
	/// See <https://www.kernel.org/doc/Documentation/core-api/cpu_hotplug.rst>.
	#[inline(always)]
	pub fn is_online(self, sys_path: &SysPath) -> bool
	{
		match &self.online_file_path().read_string()
		{
			"0" => false,
			"1" => true,
			invalid @ _ => panic!("Invalid value for CPU online '{}'", invalid),
		}
	}
	
	/// Is this CPU offline?
	///
	/// See <https://www.kernel.org/doc/Documentation/core-api/cpu_hotplug.rst>.
	#[inline(always)]
	pub fn is_offline(self, sys_path: &SysPath) -> bool
	{
		!self.is_online(sys_path)
	}
	
	/// Disable (offline) this CPU.
	///
	/// Requires root.
	///
	/// CPU zero (0) is special on x86 / x86-64 and can not ordinarily be offlined.
	///
	/// See <https://www.kernel.org/doc/Documentation/core-api/cpu_hotplug.rst>.
	#[inline(always)]
	pub fn set_offline(self, sys_path: &SysPath) -> bool
	{
		assert_effective_user_id_is_root(&format!("Offline CPU '{}'", self.0));
		
		self.online_file_path(sys_path).write_value(0)
	}
	
	/// Enable (online) this CPU.
	///
	/// Requires root.
	///
	/// See <https://www.kernel.org/doc/Documentation/core-api/cpu_hotplug.rst>.
	#[inline(always)]
	pub fn set_online(self, sys_path: &SysPath) -> bool
	{
		assert_effective_user_id_is_root(&format!("Online CPU '{}'", self.0));
		
		self.online_file_path(sys_path).write_value(1)
	}
	
	#[inline(always)]
	fn online_file_path(self, sys_path: &SysPath)
	{
		sys_path.cpu_node_path(self.into(), "online")
	}
	
	/// CPUs (hyper threaded logical cores) that are siblings of this one.
	///
	/// Will include `self`.
	///
	/// See <https://www.kernel.org/doc/Documentation/cputopology.txt>.
	#[inline(always)]
	pub fn siblings(self, sys_path: &SysPath) -> BTreeSet<Self>
	{
		sys_path.cpu_node_path(self.into(), "topology/core_siblings_list").read_linux_core_or_numa_list().unwrap().map(|value| HyperThread(value)).collect()
	}
	
	/// CPUs (hyper threaded logical cores) that are thread-siblings of this one.
	///
	/// Will include `self`.
	///
	/// See <https://www.kernel.org/doc/Documentation/cputopology.txt>.
	#[inline(always)]
	pub fn thread_siblings(self, sys_path: &SysPath) -> BTreeSet<Self>
	{
		sys_path.cpu_node_path(self.into(), "topology/thread_siblings_list").read_linux_core_or_numa_list().unwrap().map(|value| HyperThread(value)).collect()
	}
	
	/// Underlying hardware, not Linux, core identifier.
	///
	/// See <https://www.kernel.org/doc/Documentation/cputopology.txt>.
	#[inline(always)]
	pub fn underlying_hardware_physical_core_identifier(self, sys_path: &SysPath) -> io::Result<u16>
	{
		sys_path.cpu_node_path(self.into(), "topology/core_id").read_value()
	}
	
	/// Underlying hardware, not Linux, socket identifier.
	///
	/// See <https://www.kernel.org/doc/Documentation/cputopology.txt>.
	#[inline(always)]
	pub fn underlying_hardware_physical_socket_identifier(self, sys_path: &SysPath) -> io::Result<u16>
	{
		sys_path.cpu_node_path(self.into(), "topology/physical_package_id").read_value()
	}
	
	/// Simply reports the maximum *identifier* that could be used by the Linux kernel upto the `CONFIG_` number of CPUs.
	///
	/// Add one to this to get the exclusive maximum.
	///
	/// Consider using libnuma instead of this call.
	#[inline(always)]
	pub fn kernel_maximum_index(sys_path: &SysPath) -> io::Result<HyperThread>
	{
		sys_path.cpu_nodes_path("kernel_max").read_value().map(|value| HyperThread(value))
	}
	
	#[inline(always)]
	fn parse_list_mask(sys_path: &SysPath, file_name: &str) -> BTreeSet<Self>
	{
		sys_path.cpu_nodes_path(file_name).read_linux_core_or_numa_list().unwrap().map(|value| HyperThread(value)).collect()
	}
	
	/// Current hyper thread index that this thread is running on.
	///
	/// Unless this thread has been scheduled to only run on this hyper thread, then the result is close to useless.
	///
	/// Topology is not available on FreeBSD; value will always be zero.
	#[cfg(any(target_os = "android", target_os = "dragonfly", target_os = "linux"))]
	pub(crate) fn current_hyper_thread() -> u16
	{
		extern "C"
		{
			fn sched_getcpu() -> c_int;
		}
		
		let result = unsafe { sched_getcpu() };
		debug_assert!(result >= 0, "sched_getcpu() was negative");
		debug_assert!(result <= ::std::u16::MAX as i32, "sched_getcpu() was too large");
		result as u16
	}
	
	/// Current hyper thread index that this thread is running on.
	///
	/// Unless this thread has been scheduled to only run on this hyper thread, then the result is close to useless.
	///
	/// Topology is not available on FreeBSD; value will always be zero.
	#[cfg(target_os = "freebsd")]
	pub(crate) fn current_hyper_thread() -> u16
	{
		0
	}
}

