// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// NUMA node numbers start at zero.
///
/// It is assumed by DPDK code that there is always at least one NUMA node, and, if there is one NUMA node, it is number zero.
///
/// Some DPDK APIs (eg `rte_eth_dev_socket_id`) treat zero as also meaning 'undetermined'.
///
/// NUMA node numbers are not necessarily contiguous but usually are.
///
/// NUMA nodes are also, confusingly, known as sockets. In this sense they represent the socket where a modern CPU with multiple cores resides.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialization)]
pub struct NumaNode(u8);

impl Into<u8> for NumaNode
{
	#[inline(always)]
	fn into(self) -> u8
	{
		self.0
	}
}

impl Into<u16> for NumaNode
{
	#[inline(always)]
	fn into(self) -> u16
	{
		self.0 as u16
	}
}

impl Into<u32> for NumaNode
{
	#[inline(always)]
	fn into(self) -> u32
	{
		self.0 as u32
	}
}

impl Into<u64> for NumaNode
{
	#[inline(always)]
	fn into(self) -> u64
	{
		self.0 as u64
	}
}

impl Into<usize> for NumaNode
{
	#[inline(always)]
	fn into(self) -> usize
	{
		self.0 as usize
	}
}

impl NumaNode
{
	/// Maximum number of `NumaNode`s.
	pub const Maximum: usize = RTE_MAX_NUMA_NODES;
	
	/// Constructs from an `u32` value.
	///
	/// Panics if the value is out-of-range greater than or equal to `RTE_MAX_NUMA_NODES`).
	#[inline(always)]
	pub fn from_u32(value: u32) -> Self
	{
		debug_assert!((Self::Maximum as u32) <= (::std::u8::MAX as u32), "Self::Maximum '{}' exceeds ::std::u8::MAX; the DPDK API is broken", Self::Maximum, ::std::u8::MAX);
		
		assert!(value < (Self::Maximum as u32), "value '{}' equals or exceeds Self::Maximum '{}'", value, Self::Maximum);
		
		NumaNode(value as u8)
	}
	
	/// Valid NUMA nodes.
	#[inline(always)]
	pub fn valid_numa_nodes() -> &'static HashSet<Self>
	{
		lazy_static!
		{
			static ref ValidNumaNodes: HashSet<NumaNode> =
			{
				Self::initialize_libnuma();
				
				let numa_nodes_bitmask = unsafe { numa_allocate_nodemask() };
				
				let number_of_numa_nodes_in_numa_nodes_bitmask = unsafe { numa_num_possible_nodes() } as usize;
				let maximum = min(number_of_numa_nodes_in_numa_nodes_bitmask, Self::Maximum);
				
				let mut valid_numa_nodes = HashSet::with_capacity(maximum);
				for numa_node_index in 0 .. maximum
				{
					let is_unset = unsafe { numa_bitmask_isbitset(numa_nodes_bitmask as *const _, numa_node_index as u32) } == 0;
					if is_unset
					{
						continue
					}
					
					valid_numa_nodes.insert(NumaNode(numa_node_index as u8))
				}
				valid_numa_nodes
			}
		}
		
		ValidNumaNodes
	}
	
	/// Neighbours to this NUMA node ordered in increasing distance order.
	///
	/// The first entry is `self`.
	pub fn neighbouring_numa_nodes_in_increasing_distance_order_with_first_as_self(self) -> IndexSet<Self>
	{
		let mut distances = BTreeSet::new();
		for neighbouring_numa_node in Self::valid_numa_nodes().iter()
		{
			let raw_distance = unsafe { numa_distance(numa_node.0 as i32, neighbouring_numa_node.0 as i32) };
			if raw_distance > 0
			{
				let smaller_is_closer_and_zero_is_self = ((raw_distance as usize) / 10) - 1;
				distances.push((smaller_is_closer_and_zero_is_self, *neighbouring_numa_node))
			}
		}
		
		let mut neighbouring_numa_nodes_in_increasing_distance_order_with_first_as_self = IndexSet::with_capacity(distances.len());
		for (_, neighbouring_numa_node) in distances.drain()
		{
			neighbouring_numa_nodes_in_increasing_distance_order_with_first_as_self.push(neighbouring_numa_node)
		}
		neighbouring_numa_nodes_in_increasing_distance_order_with_first_as_self
	}
	
	/// Hyper threads are similar to `LogicalCore`s, but, since this code often runs before DPDK has been initialized (`rte_eal_init`), we can not use them as their global and thread local statics will not have been initialized.
	///
	/// TODO: ?assume master logical core is current process? (DPDK defaults to first logical core).
	pub fn hyper_threads(self) -> HashSet<HyperThread>
	{
		Self::initialize_libnuma();
		
		let hyper_thread_bitmask = unsafe { numa_allocate_hyper_threadmask() };
		assert_eq!(unsafe { numa_node_to_cpus(numa_node_index as i32, &mut hyper_thread_bitmask) }, 0, "numa_node_to_hyper_threads failed");
		
		let maximum_hyper_threads = min(number_of_hyper_threads_in_hyper_thread_bitmask, LogicalCore::Maximum);
		let mut set = HashSet::with_capacity(maximum_hyper_threads);
		for hyper_thread_index in 0 ..maximum_hyper_threads
		{
			let is_unset = unsafe { numa_bitmask_isbitset(hyper_thread_bitmask as *const _, hyper_thread_index as u32) } == 0;
			if is_unset
			{
				continue
			}
			
			let list = this.0.entry(NumaNode(numa_node_index as u8)).or_insert_with(Vec::with_capacity(likely_number_of_hyper_threads_per_numa_node));
			
			set.push(HyperThread(hyper_thread_index as u16));
		}
		
		unsafe { numa_bitmask_free(hyper_thread_bitmask) };
		
		set
	}
	
	#[inline(always)]
	fn initialize_libnuma()
	{
		static InitializeLibnuma: Once = ONCE_INIT;
		
		InitializeLibnuma.call_once(|| assert_eq!(unsafe { numa_available() }, 0, "numa_available failed"))
	}
}

/// Represents a logical hyper thread, which in Operating System terms is usually a logical CPU (core).
///
/// These usually map 1:1 with `LogicalCore`s
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
pub struct HyperThread(u16);

impl HyperThread
{
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
