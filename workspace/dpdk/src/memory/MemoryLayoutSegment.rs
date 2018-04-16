// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Memory layout segment.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct MemoryLayoutSegment(NonNull<rte_memseg>);

impl MemoryLayoutSegment
{
	/// IO Virtual Address.
	#[inline(always)]
	pub fn io_virtual_address(&self) -> rte_iova_t
	{
		let address = * self.deref().rte_memseg_1.iova.as_ref();
		debug_assert_ne!(address as i64, -1, "should not be a RTE_BAD_IOVA");
		address
	}
	
	/// Virtual Address.
	#[inline(always)]
	pub fn virtual_address(&self) -> *mut u8
	{
		(* self.deref().rte_memseg_2.addr.as_ref()) as *mut u8
	}
	
	/// Length of memory segment in bytes.
	#[inline(always)]
	pub fn length_in_bytes(&self) -> usize
	{
		self.deref().len
	}
	
	/// Huge page size.
	#[inline(always)]
	pub fn huge_page_size(&self) -> HugePageSize
	{
		HugePageSize::from_proc_mem_info_value(self.deref().hugepage_sz).expect("Unknown huge page size")
	}
	
	/// NUMA socket id.
	#[inline(always)]
	pub fn numa_socket_id(&self) -> Option<NumaSocketId>
	{
		match self.deref().socket_id
		{
			-1 => None,
			valid => Some(valid as u8),
		}
	}
	
	/// Number of memory channels.
	#[inline(always)]
	pub fn number_of_memory_channels(&self) -> MemoryChannels
	{
		let channels = self.deref().nchannel;
		if channels == 0 || channels > 4
		{
			panic!("Invalid number of memory channels '{}'", channels)
		}
		unsafe { transmute(channels) }
	}
	
	/// Number of memory ranks.
	#[inline(always)]
	pub fn number_of_memory_ranks(&self) -> MemoryRanks
	{
		let ranks = self.deref().nrank;
		if ranks == 0 || ranks > 16
		{
			panic!("Invalid number of memory ranks '{}'", ranks)
		}
		unsafe { transmute(ranks) }
	}
	
	#[inline(always)]
	fn deref(&self) -> &rte_memseg
	{
		unsafe { & * self.0.as_ptr() }
	}
	
	pub _1: rte_memseg_1,
	pub _2: rte_memseg_2,
}
