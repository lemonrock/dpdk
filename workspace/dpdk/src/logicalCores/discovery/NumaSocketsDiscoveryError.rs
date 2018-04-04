// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


quick_error!
{
	#[derive(Debug)]
	pub enum NumaSocketsDiscoveryError
	{
		CouldNotParseListFile(cause: ListParseError)
		{
			display("Could not parse NUMA nodes or CPUs list file ({})", cause)
			cause(cause)
			from()
		}
		
		CpuIsInMoreThanOneNumaNode(cpuIndex: usize)
		{
			display("CPU index '{}' is in more than one NUMA node", cpuIndex)
		}
		
		UnassignedCpuIndices(shouldNotContainAnyLogicalCoresWhenAllNumaNodesConsidered: LogicalCoresActive)
		{
			display("Unassigned CPU indices '{:?}'", shouldNotContainAnyLogicalCoresWhenAllNumaNodesConsidered)
		}
	}
}
