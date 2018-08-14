// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A choice of NUMA node to use.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
pub enum NumaNodeChoice
{
	/// Equivalent to DPDK's `SOCKET_ID_ANY`.
	Any,
	
	/// A specific node.
	Specific(NumaNode),
}

impl Into<i32> for NumaNodeChoice
{
	#[inline(always)]
	fn into(self) -> i32
	{
		use self::NumaNodeChoice::*;
		
		match self
		{
			Any => SOCKET_ID_ANY,
			
			Specific(NumaNode(value)) => value as i32,
		}
	}
}

impl Into<u32> for NumaNodeChoice
{
	/// Note that Any (`SOCKET_ID_ANY`) is represented as `::std::u32::MAX`.
	///
	/// This functionality primarily exists because some DPDK APIs (eg `rte_reorder_create`) take an unsigned int when they should take a signed int...
	#[inline(always)]
	fn into(self) -> u32
	{
		use self::NumaNodeChoice::*;
		
		match self
		{
			Any => ::std::u32::MAX,
			
			Specific(NumaNode(value)) => value as u32,
		}
	}
}

impl NumaNodeChoice
{
	/// For current CPU.
	///
	/// Slightly slow as must go via a C function call.
	#[inline(always)]
	pub fn for_current_cpu() -> Self
	{
		Self::from_i32(unsafe { transmute(rte_socket_id()) })
	}
	
	/// Unwraps as a NumaNode.
	///
	/// Panics if not a NumaNode.
	#[inline(always)]
	pub fn unwrap(self) -> NumaNode
	{
		self.expect("This is not a NUMA node")
	}
	
	/// Unwraps as a NumaNode.
	///
	/// Panics if not a NumaNode.
	///
	/// Takes a `message` for the panic.
	#[inline(always)]
	pub fn expect(self, message: &str) -> NumaNode
	{
		use self::NumaNodeChoice::*;
		
		// This is a separate function similar to that used by ::std::option::Option.
		#[inline(never)]
		#[cold]
		fn expect_failed(message: &str) -> !
		{
			panic!("{}", message)
		}
		
		match self
		{
			Any => expect_failed(message),
			
			Specific(numa_node) => numa_node,
		}
	}
	
	/// Unwraps as a NumaNode or returns a default (the Numa Node for zero).
	#[inline(always)]
	pub fn unwrap_or_default(self) -> NumaNode
	{
		use self::NumaNodeChoice::*;
		
		match self
		{
			Any => NumaNode::default(),
			
			Specific(numa_node) => numa_node,
		}
	}
	
	/// Constructs from an `i32` value.
	///
	/// Panics if the value is out-of-range (less than -1 or greater than or equal to `RTE_MAX_NUMA_NODES`).
	#[inline(always)]
	pub fn from_i32(value: i32) -> Self
	{
		use self::NumaNodeChoice::*;
		
		if likely!(value >= 0)
		{
			debug_assert!((RTE_MAX_NUMA_NODES as u16) <= (::std::u8::MAX as u16), "RTE_MAX_NUMA_NODES '{}' exceeds ::std::u8::MAX '{}'; the DPDK API is broken", RTE_MAX_NUMA_NODES, ::std::u8::MAX);
			
			assert!((value as u32) < (RTE_MAX_NUMA_NODES as u32), "value '{}' equals or exceeds RTE_MAX_NUMA_NODES '{}'", value, RTE_MAX_NUMA_NODES);
			
			Specific(NumaNode(value as u8))
		}
		else if unlikely!(value == SOCKET_ID_ANY)
		{
			Any
		}
		else
		{
			panic!("value '{}' is invalid for a NUMA node", value)
		}
	}
	
	/// Allocates memory on the heap.
	#[inline(always)]
	pub fn allocate_uninitialized<T>(&self, alignment_power_of_two: u32) -> Option<DpdkAllocatedMemory<T>>
	{
		debug_assert!(alignment_power_of_two.is_power_of_two(), "alignment_power_of_two '{}' is not a power of two", alignment_power_of_two);
		debug_assert!(alignment_power_of_two >= NumaNode::CacheLineSize, "alignment_power_of_two '{}' is less than CacheLineSize '{}'", alignment_power_of_two, NumaNode::CacheLineSize);
		
		use self::NumaNodeChoice::*;
		
		match self
		{
			Specific(numa_node) => numa_node.allocate_uninitialized(alignment_power_of_two),
			
			Any =>
			{
				let result = unsafe { rte_malloc(null(), size_of::<T>(), alignment_power_of_two) };
				if unlikely!(result.is_null())
				{
					None
				}
				else
				{
					Some(DpdkAllocatedMemory(result as *mut T))
				}
			}
		}
	}
	
	/// Allocates memory on the heap.
	#[inline(always)]
	pub fn allocate_zeroed<T>(&self, alignment_power_of_two: u32) -> Option<DpdkAllocatedMemory<T>>
	{
		debug_assert!(alignment_power_of_two.is_power_of_two(), "alignment_power_of_two '{}' is not a power of two", alignment_power_of_two);
		debug_assert!(alignment_power_of_two >= NumaNode::CacheLineSize, "alignment_power_of_two '{}' is less than CacheLineSize '{}'", alignment_power_of_two, NumaNode::CacheLineSize);
		
		use self::NumaNodeChoice::*;
		
		match self
		{
			Specific(numa_node) => numa_node.allocate_zeroed(alignment_power_of_two),
			
			Any =>
			{
				let result = unsafe { rte_zmalloc(null(), size_of::<T>(), alignment_power_of_two) };
				if unlikely!(result.is_null())
				{
					None
				}
				else
				{
					Some(DpdkAllocatedMemory(result as *mut T))
				}
			}
		}
	}
	
	/// Allocates memory on the heap.
	#[inline(always)]
	pub fn allocate_uninitialized_for_array<T>(&self, number_of_elements: usize, alignment_power_of_two: u32) -> Option<DpdkAllocatedMemory<T>>
	{
		debug_assert!(alignment_power_of_two.is_power_of_two(), "alignment_power_of_two '{}' is not a power of two", alignment_power_of_two);
		debug_assert!(alignment_power_of_two >= NumaNode::CacheLineSize, "alignment_power_of_two '{}' is less than CacheLineSize '{}'", alignment_power_of_two, NumaNode::CacheLineSize);
		
		use self::NumaNodeChoice::*;
		
		match self
		{
			Specific(numa_node) => numa_node.allocate_uninitialized_for_array(number_of_elements, alignment_power_of_two),
			
			Any =>
			{
				let result = unsafe { rte_calloc(null(), number_of_elements, size_of::<T>(), alignment_power_of_two) };
				if unlikely!(result.is_null())
				{
					None
				}
				else
				{
					Some(DpdkAllocatedMemory(result as *mut T))
				}
			}
		}
	}
	
	/// Memory statistics.
	///
	/// Interpret this by multiplying counts by page size.
	#[inline(always)]
	pub fn zoned_virtual_memory_statistics(self, sys_path: &SysPath, proc_path: &ProcPath) -> io::Result<HashMap<VirtualMemoryStatisticName, u64>>
	{
		use self::NumaNodeChoice::*;
		
		match self
		{
			Specific(numa_node) => numa_node.zoned_virtual_memory_statistics(sys_path),
			
			Any => proc_path.global_zoned_virtual_memory_statistics(),
		}
	}
	
	/// Memory information.
	#[inline(always)]
	pub fn memory_information(&self, sys_path: &SysPath, proc_path: &ProcPath, memory_information_name_prefix: &str) -> Result<MemoryInformation, MemoryInformationParseError>
	{
		use self::NumaNodeChoice::*;
		
		match self
		{
			Specific(numa_node) => numa_node.memory_information(sys_path, memory_information_name_prefix),
			
			Any => proc_path.memory_information(memory_information_name_prefix),
		}
	}
	
	
	/// Try to unreserve (clear reservations of) huge pages.
	///
	/// Will only work as root.
	#[inline(always)]
	pub fn unreserve_huge_pages(self, sys_path: &SysPath, huge_page_size: HugePageSize) -> io::Result<()>
	{
		use self::NumaNodeChoice::*;
		
		match self
		{
			Specific(numa_node) => numa_node.unreserve_huge_pages(sys_path, huge_page_size),
			
			Any => huge_page_size.unreserve_global_huge_pages(sys_path),
		}
	}
	
	/// Try to reserve huge pages.
	///
	/// Will only work as root.
	#[inline(always)]
	pub fn reserve_huge_pages(self, sys_path: &SysPath, huge_page_size: HugePageSize, number_to_try_to_reserve: u64) -> io::Result<()>
	{
		use self::NumaNodeChoice::*;
		
		match self
		{
			Specific(numa_node) => numa_node.reserve_huge_pages(sys_path, huge_page_size, number_to_try_to_reserve),
			
			Any => huge_page_size.reserve_global_huge_pages(sys_path, number_to_try_to_reserve),
		}
	}
	
	/// Read number of huge pages of `huge_page_size` size.
	#[inline(always)]
	pub fn number_of_huge_pages(self, sys_path: &SysPath, huge_page_size: HugePageSize) -> io::Result<u64>
	{
		use self::NumaNodeChoice::*;
		
		match self
		{
			Specific(numa_node) => numa_node.number_of_huge_pages(sys_path, huge_page_size),
			
			Any => huge_page_size.number_of_global_huge_pages(sys_path),
		}
	}
	
	/// Read number of free huge pages of `huge_page_size` size.
	#[inline(always)]
	pub fn number_of_free_global_huge_pages(self, sys_path: &SysPath, huge_page_size: HugePageSize) -> io::Result<u64>
	{
		use self::NumaNodeChoice::*;
		
		match self
		{
			Specific(numa_node) => numa_node.number_of_free_global_huge_pages(sys_path, huge_page_size),
			
			Any => huge_page_size.number_of_free_global_huge_pages(sys_path),
		}
	}
	
	/// Read number of surplus huge pages of `huge_page_size` size.
	#[inline(always)]
	pub fn number_of_surplus_huge_pages(self, sys_path: &SysPath, huge_page_size: HugePageSize) -> io::Result<u64>
	{
		use self::NumaNodeChoice::*;
		
		match self
		{
			Specific(numa_node) => numa_node.number_of_surplus_huge_pages(sys_path, huge_page_size),
			
			Any => huge_page_size.number_of_surplus_global_huge_pages(sys_path),
		}
	}
}
