// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents a machine or a set of valid NUMA nodes.
///
/// This abstraction is to handle non-NUMA and NUMA machines.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum MachineOrNumaNodes<T>
{
	Machine(T),
	
	NumaNodes(BTreeMap<NumaNode, T>),
}

impl MachineOrNumaNodes<()>
{
	/// New instance.
	#[inline(always)]
	pub fn new(sys_path: &SysPath) -> Self
	{
		use self::MachineOrNumaNodes::*;
		
		if sys_path.is_a_numa_machine(sys_path)
		{
			NumaNodes(NumaNode::valid_numa_nodes_map())
		}
		else
		{
			Machine(())
		}
	}
	
	/// Tries to:-
	///
	/// * If this is a NUMA machine
	/// 	* garbage collect memory (pages) on NUMA nodes;
	/// 	* free per-NUMA node pages;
	/// 	* tries to clear (unreserve) all huge-page reservations on all NUMA nodes;
	/// * If this is not a NUMA machine
	/// 	* tries to clear (unreserve) all huge-page reservations;
	///
	/// NUMA nodes without CPUs are not considered for the above actions.
	///
	/// Safe to call on a non-NUMA machine.
	#[inline(always)]
	pub fn garbage_collect_memory(&self, sys_path: &SysPath)
	{
		#[inline(always)]
		fn unreserve_huge_pages<F: Fn(HugePageSize, &SysPath)>(sys_path: &SysPath, unreserve: F)
		{
			for huge_page_size in HugePageSize::supported_huge_page_sizes(sys_path).iter()
			{
				unreserve(*huge_page_size, sys_path)
			}
		}
		
		use self::MachineOrNumaNodes::*;
		
		match *self
		{
			Machine(()) => unreserve_huge_pages(sys_path, HugePageSize::unreserve_global_huge_pages),
			
			NumaNodes(valid_numa_nodes_map) => for numa_node in valid_numa_nodes_map.keys()
			{
				let numa_node = (*numa_node);
				for _garbage_collection_iteration in 0 .. 2
				{
					numa_node.compact_pages();
					numa_node.evict_pages();
				}
				
				unreserve_huge_pages(sys_path, |huge_page_size, sys_path| huge_page_size.unreserve_numa_huge_pages(sys_path, numa_node.into()))
			},
		}
	}
	
	/// Tries to reserve huge page memory on NUMA nodes, if present, otherwise on the whole machine.
	///
	/// Returns the amount of memory which should be passed to the '--socket-mem' EAL option or the '-m' EAL option.
	///
	/// Safe to call on a non-NUMA machine.
	#[inline(always)]
	pub fn reserve_huge_page_memory(&self, sys_path: &SysPath, proc_path: &ProcPath, huge_page_allocation_strategy: &HugePageAllocationStrategy) -> MachineOrNumaNodes<MegaBytes>
	{
		#[inline(always)]
		fn memory_information_to_number_of_pages(largest_supported_huge_page_size: HugePageSize, memory_information: Result<MemoryInformation, MemoryInformationParseError>) -> (u64, MegaBytes)
		{
			let total_free = KiloBytes(memory_information.expect("Could not parse `meminfo` file").free_physical_ram().expect("Free physical RAM memory information was not in `meminfo` file"));
			let number_of_pages = huge_page_allocation_strategy.calculate_number_of_huge_pages(largest_supported_huge_page_size, total_free);
			
			(number_of_pages, MegaBytes::from(total_free).scale_by(number_of_pages))
		}
		
		const MemoryInformationNamePrefix: &'static str = "";
		
		let largest_supported_huge_page_size = HugePageSize::largest_supported_huge_page_size(sys_path);
		
		use self::MachineOrNumaNodes::*;
		
		match *self
		{
			Machine(()) =>
			{
				let (number_of_pages, megabytes_limit) = memory_information_to_number_of_pages(largest_supported_huge_page_size, proc_path.memory_information(MemoryInformationNamePrefix));
				largest_supported_huge_page_size.reserve_global_huge_pages(sys_path, number_of_pages);
				Machine(megabytes_limit)
			}
			
			NumaNodes(valid_numa_nodes_map) =>
			{
				let mut map = BTreeMap::new();
				
				for numa_node in valid_numa_nodes_map.keys()
				{
					let numa_node = *numa_node;
					
					let (number_of_pages, megabytes_limit) = memory_information_to_number_of_pages(largest_supported_huge_page_size, numa_node.memory_information(sys_path, MemoryInformationNamePrefix));
					largest_supported_huge_page_size.reserve_numa_huge_pages(sys_path, numa_node.into(), number_of_pages);
					map.insert(numa_node, megabytes_limit)
				}
				
				NumaNodes(map)
			}
		}
	}
}
