// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct MemoryLimits
{
	perNumaNodeMemoryInMegabytes: [Option<u31>; MaximumNumaSockets],
}

impl MemoryLimits
{
	/// If not a NUMA machine, total memory is memory assigned to first valid Numa Socket (usually zero)
	/// If there are no valid sockets, or the memory total is zero, then None is returned
	/// If total exceeds 512Gb, capped at 512Gb
	pub fn totalMemoryInMegabytes(&self, numaSockets: &NumaSockets) -> Option<u31>
	{
		const _512GbInMegabytes: u31 = 524_288;
		
		let memoryTotal = if numaSockets.isANumaMachine
		{
			let mut memoryTotal = 0;
			for numaMemoryIndex in 0..MaximumNumaSockets
			{
				if let Some(megabytes) = self.perNumaNodeMemoryInMegabytes[numaMemoryIndex]
				{
					if numaSockets.isValidNumaSocket(numaMemoryIndex)
					{
						memoryTotal += megabytes;
					}
				}
			}
			memoryTotal
		}
		else
		{
			let mut memoryTotal = 0;
			for index in 0..MaximumNumaSockets
			{
				if let Some(megabytes) = self.perNumaNodeMemoryInMegabytes[index]
				{
					memoryTotal = megabytes;
					break;
				}
			}
			memoryTotal
		};
		
		match memoryTotal
		{
			0 => None,
			doesNotExceed512Gb if doesNotExceed512Gb < _512GbInMegabytes => Some(doesNotExceed512Gb),
			_ => Some(_512GbInMegabytes),
		}
	}
	
	// Need to pass in valid numa nodes to do this
	pub fn asInitialisationStringIfIsANumaMachine(&self, useHugePages: bool, numaSockets: &NumaSockets) -> (Option<CString>, Option<u31>)
	{
		const SOCKET_MEM_STRLEN: usize = MaximumNumaSockets * 10;
		
		if useHugePages && numaSockets.isANumaMachine
		{
			assert!(useHugePages, "Can not have per NUMA socket memory (memoryLimits) and then have useHugePages as false");
			
			let mut numaMemoryString = String::with_capacity(SOCKET_MEM_STRLEN);
			let mut hasValidEntries = false;
			for numaMemoryIndex in 0..MaximumNumaSockets
			{
				if likely(numaMemoryIndex != 0)
				{
					numaMemoryString.push(',')
				}
				
				if let Some(megabytes) = self.perNumaNodeMemoryInMegabytes[numaMemoryIndex]
				{
					if numaSockets.isValidNumaSocket(numaMemoryIndex)
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
			(None, self.totalMemoryInMegabytes(numaSockets))
		}
	}
}
