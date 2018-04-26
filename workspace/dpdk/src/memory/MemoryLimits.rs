// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents total memory available per NUMA socket.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct MemoryLimits
{
	/// Adjust as desired.
	pub per_numa_node_memory_in_megabytes: [Option<u31>; NumaNode::MaximumNumaSockets],
}

impl MemoryLimits
{
	// If not a NUMA machine, total memory is memory assigned to first valid Numa Socket (usually zero).
	// If there are no valid sockets, or the memory total is zero, then None is returned.
	// If total exceeds 512Gb, capped at 512Gb.
	pub(crate) fn total_memory_in_megabytes(&self, numa_sockets: &NumaSockets) -> Option<u31>
	{
		const _512GbInMegabytes: u31 = 524_288;
		
		let memory_total = if numa_sockets.isANumaMachine
		{
			let mut memory_total = 0;
			for numaMemoryIndex in 0..NumaNode::MaximumNumaSockets
			{
				if let Some(megabytes) = self.per_numa_node_memory_in_megabytes[numaMemoryIndex]
				{
					if numa_sockets.isValidNumaSocket(numaMemoryIndex)
					{
						memory_total += megabytes;
					}
				}
			}
			memory_total
		}
		else
		{
			let mut memory_total = 0;
			for index in 0..NumaNode::MaximumNumaSockets
			{
				if let Some(megabytes) = self.per_numa_node_memory_in_megabytes[index]
				{
					memory_total = megabytes;
					break;
				}
			}
			memory_total
		};
		
		match memory_total
		{
			0 => None,
			does_not_exceed_512Gb if does_not_exceed_512Gb < _512GbInMegabytes => Some(does_not_exceed_512Gb),
			_ => Some(_512GbInMegabytes),
		}
	}
	
	// Need to pass in valid numa nodes to do this.
	pub(crate) fn as_initialisation_string_if_is_a_numa_machine(&self, use_huge_pages: bool, numa_sockets: &NumaSockets) -> (Option<CString>, Option<u31>)
	{
		const SOCKET_MEM_STRLEN: usize = NumaNode::MaximumNumaSockets * 10;
		
		if use_huge_pages && numa_sockets.isANumaMachine
		{
			assert!(use_huge_pages, "Can not have per NUMA socket memory (memoryLimits) and then have use_huge_pages as false");
			
			let mut numaMemoryString = String::with_capacity(SOCKET_MEM_STRLEN);
			let mut hasValidEntries = false;
			for numaMemoryIndex in 0..NumaNode::MaximumNumaSockets
			{
				if likely(numaMemoryIndex != 0)
				{
					numaMemoryString.push(',')
				}
				
				if let Some(megabytes) = self.per_numa_node_memory_in_megabytes[numaMemoryIndex]
				{
					if numa_sockets.isValidNumaSocket(numaMemoryIndex)
					{
						hasValidEntries = true;
						
						let x = format!("{}", megabytes);
						numaMemoryString.push_str(&x);
					}
				}
			}
			if hasValidEntries
			{
				(Some(CString::new(numaMemoryString).unwrap()), None)
			}
			else
			{
				(None, None)
			}
		}
		else
		{
			(None, self.total_memory_in_megabytes(numa_sockets))
		}
	}
}
