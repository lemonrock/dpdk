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
	/// CPUs (hyper threaded logical cores) that could possibly be online at some point.
	///
	/// Not reliable, as includes CPUs that can never be brought online; simply reports the number that could be used by the Linux kernel upto the `CONFIG_` number of CPUs
	///
	/// Consider using libnuma instead of this call.
	#[inline(always)]
	pub fn possible(sys_path: &SysPath) -> BTreeSet<Self>
	{
		Self::parse_list_mask(sys_path, "possible")
	}
	
	/// CPUs (hyper threaded logical cores) that could possibly be online at some point.
	///
	/// Consider using libnuma instead of this call.
	#[inline(always)]
	pub fn online(sys_path: &SysPath) -> BTreeSet<Self>
	{
		Self::parse_list_mask(sys_path, "online")
	}
	
	/// CPUs (hyper threaded logical cores) that are present.
	#[inline(always)]
	pub fn present(sys_path: &SysPath) -> BTreeSet<Self>
	{
		Self::parse_list_mask(sys_path, "present")
	}
	
	/// CPUs (hyper threaded logical cores) that are offline.
	#[inline(always)]
	pub fn offline(sys_path: &SysPath) -> BTreeSet<Self>
	{
		Self::parse_list_mask(sys_path, "offline")
	}
	
	/// CPUs (hyper threaded logical cores) that are siblings of this one.
	///
	/// Will include `self`.
	#[inline(always)]
	pub fn siblings(self, sys_path: &SysPath) -> BTreeSet<Self>
	{
		sys_path.cpu_node_path(self.into(), "topology/core_siblings_list").read_linux_core_or_numa_mask().unwrap().map(|value| HyperThread(value)).collect()
	}
	
	/// CPUs (hyper threaded logical cores) that are thread-siblings of this one.
	///
	/// Will include `self`.
	#[inline(always)]
	pub fn thread_siblings(self, sys_path: &SysPath) -> BTreeSet<Self>
	{
		sys_path.cpu_node_path(self.into(), "topology/thread_siblings_list").read_linux_core_or_numa_mask().unwrap().map(|value| HyperThread(value)).collect()
	}
	
	/// Core identifier.
	#[inline(always)]
	pub fn core_identifier(self, sys_path: &SysPath) -> io::Result<u16>
	{
		sys_path.cpu_node_path(self.into(), "topology/core_id").read_value()
	}
	
	/// Physical package identifier.
	#[inline(always)]
	pub fn physical_package_identifier(self, sys_path: &SysPath) -> io::Result<u16>
	{
		sys_path.cpu_node_path(self.into(), "topology/physical_package_id").read_value()
	}
	
	/// Simply reports the maximum *identifier* that could be used by the Linux kernel upto the `CONFIG_` number of CPUs
	///
	/// Add one to this to get the exclusive maximum.
	///
	/// Consider using libnuma instead of this call.
	#[inline(always)]
	pub fn kernel_maximum(sys_path: &SysPath) -> io::Result<u16>
	{
		sys_path.cpu_nodes_path("kernel_max").read_value()
	}
	
	#[inline(always)]
	fn parse_list_mask(sys_path: &SysPath, file_name: &str) -> BTreeSet<Self>
	{
		sys_path.cpu_nodes_path(file_name).read_linux_core_or_numa_mask().unwrap().map(|value| HyperThread(value)).collect()
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
	
	pub fn hexadecimal_core_mask_c_string(hyper_threads: &HashSet<HyperThread>) -> CString
	{
		
		pub struct NumaNodeLayout(HashMap<NumaNode, Vec<(LogicalCore, HyperThreadAssignment)>>);
		
		impl NumaNodeLayout
		{
			pub fn new() -> Self
			{
				use self::HyperThreadAssignment::*;
				
				initialize();
				
				let current_core = Self::current_hyper_thread_index();
				
				let number_of_numa_nodes = NumaNode::valid_numa_nodes();
				
				let number_of_hyper_threads_in_hyper_thread_bitmask = unsafe { numa_num_possible_hyper_threads() } as usize;
				
				let likely_number_of_hyper_threads_per_numa_node = (number_of_hyper_threads_in_hyper_thread_bitmask + number_of_numa_nodes - 1) / number_of_numa_nodes;
				
				let mut this = NumaNodeLayout(HashMap::with_capacity(number_of_hyper_threads_in_hyper_thread_bitmask));
				
				for numa_node_index in 0 .. number_of_numa_nodes
				{
					
					
					
					
					
					
					
					
					let list = this.0.entry(NumaNode(numa_node_index as u8)).or_insert(Vec::with_capacity(likely_number_of_hyper_threads_per_numa_node));
					
				}
				
				this
			}
		}
		
		pub enum HyperThreadAssignment
		{
			Available,
			
			Master,
			
			ServiceCore,
			
			ReceiveSlave
			{
				ethernet_port_identifier: EthernetPortIdentifier,
				receive_queue_identifier: ReceiveQueueIdentifier,
			},
			
			TransmitSlave
			{
				ethernet_port_identifier: EthernetPortIdentifier,
				transmit_queue_identifier: TransmitQueueIdentifier,
			},
		}
		
		
		let core_map: HashMap<BTreeSet<HyperThread>, BTreeSet<LogicalCore>>;
		
		
		// We should prefer `--lcores COREMAP` or `-l CORELIST` to `-c COREMASK`.
		
		// `--lcores` would allow us to handle the complexities of 'scaling down'.
		
		// `--master-lcore ID` - can be wherever, so we can put it on a lesser-loaded NUMA core.
		
		
		/*
			
			If we have more than one numa node, we can reasonably assume there are at least 4 cores per node.
			
			(LogicalCore5.LogicalCore7.LogicalCore8)@(CpuHyperThread0.CpuHyperThread5),()@()
			
			
			
			
		*/
		
		
		
		
		
		let mut bits: u64 = 0;
		for hyper_thread in hyper_threads.iter()
		{
			bits |= 1 << hyper_thread.0
		}
		
		let mut setBits = 0;
		for index in 0..Maximum
		{
			if self.isEnabled(index)
			{
				setBits |= 1 << index
			}
		}
		
		debug_assert!(Self::Maximum <= 256 && Self::Maximum >= 16, "Change format string size parameter from 2 to something else, as Maximum '{}' is outside of the range expected", Self::Maximum);
		
		CString::new(format!("0x{:02}", setBits)).unwrap()
	}
	
	/*
		Usage
			- master; needed for signal handling, liveness, is not a slave and so can not be used as a service core.
			
			- 'operating system reserved' - a core to be used by the OS for all other things (SSH, admin, cron jobs, etc)
				- could overlap with master in systems with low core counts
				- would not be available to DPDK
			
			// how is master_lcore used?
				eal_thread_init_master
	
		See
			static void
			eal_check_mem_on_local_socket(void)
			{
				const struct rte_memseg *ms;
				int i, socket_id;
			
				socket_id = rte_lcore_to_socket_id(rte_config.master_lcore);
			
				ms = rte_eal_get_physmem_layout();
			
				for (i = 0; i < RTE_MAX_MEMSEG; i++)
					if (ms[i].socket_id == socket_id &&
							ms[i].len > 0)
						return;
			
				RTE_LOG(WARNING, EAL, "WARNING: Master core has no "
						"memory on local socket!\n");
			}
	*/
}

