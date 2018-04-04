// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NumaNodesData
{
	pub hasCpu: NumaSocketsActive,
	pub hasMemory: NumaSocketsActive,
	pub hasNormalMemory: NumaSocketsActive,
	pub online: NumaSocketsActive,
	pub possible: NumaSocketsActive,
}

impl NumaNodesData
{
	pub fn nodesThatAreOnlineWithACpuAndMemory(&self) -> NumaSocketsActive
	{
		let usefulNumaNodes = self.online.intersect(&self.hasCpu).intersect(&self.hasMemory);
	
		assert!(usefulNumaNodes.hasAtLeastOneActive(), "Apparently, there are no useful NUMA nodes yet we are running as a program...");
		
		usefulNumaNodes
	}
}
