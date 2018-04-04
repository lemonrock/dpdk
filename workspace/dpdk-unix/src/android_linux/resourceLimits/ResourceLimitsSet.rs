// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct ResourceLimitsSet(HashMap<ResourceName, SoftAndHardResourceLimit>);

impl ResourceLimitsSet
{
	pub fn defaultish(maximumNumberOfOpenFileDescriptors: ResourceLimit) -> Self
	{
		// Ideally, these should be constants, but Rust's const fn is too limited and does not allow assert! or if
		let _64_000: SoftAndHardResourceLimit = SoftAndHardResourceLimit::both(ResourceLimit::Finite(64_000));
		let _262_144: SoftAndHardResourceLimit = SoftAndHardResourceLimit::both(ResourceLimit::Finite(262_144));
		
		let mut map = HashMap::with_capacity(16);

		map.insert(ResourceName::MaximumSizeOfFilesThatProcessCanCreateInBytes, SoftAndHardResourceLimit::BothInfinite);
		map.insert(ResourceName::MaximumSizeOfVirtualMemoryAddressSpaceInBytes, SoftAndHardResourceLimit::BothInfinite);
		map.insert(ResourceName::CpuTimeLimitInSeconds, SoftAndHardResourceLimit::BothInfinite);
		map.insert(ResourceName::MaximumNumberOfFileDescriptors, SoftAndHardResourceLimit::both(maximumNumberOfOpenFileDescriptors));
		map.insert(ResourceName::MaximumSizeOfProcessResidentSetSizeInBytes, SoftAndHardResourceLimit::BothInfinite); // Ignored on Linux kernels after 2.4.30
		map.insert(ResourceName::MaximumNumberOfProcessAndThreads, _64_000);
		map.insert(ResourceName::MaximumSizeOfACoreDumpFileInBytes, SoftAndHardResourceLimit::BothZero);
		
		map.insert(ResourceName::MaximumNumberOfBytesForPosixMessageQueues, SoftAndHardResourceLimit::BothZero);
		map.insert(ResourceName::MaximumNumberOfBytesThatProcessCanMemLock, SoftAndHardResourceLimit::BothInfinite);
		map.insert(ResourceName::MaximumSizeOfProcessStackInBytes, _262_144); // 256Kb stacks!
		
		ResourceLimitsSet(map)
	}
	
	pub fn change(&self)
	{
		for (resourceName, softAndHardResourceLimit) in &self.0
		{
			resourceName.set(softAndHardResourceLimit);
		}
	}
}
