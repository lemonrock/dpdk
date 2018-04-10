// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub enum HugePageAllocationStrategy
{
	RatioOfTotalFree
	{
		numeratorPermyriad: u16, // ie one ten-thousandth
	},
	
	AllLessReserve
	{
		reserveInKiloBytes: u64,
	},
	
	FixedOrFail
	{
		fixedAmountInKiloBytes: u64,
	},
}

impl HugePageAllocationStrategy
{
	pub const TenPercentRatioOfTotalFree: HugePageAllocationStrategy = HugePageAllocationStrategy::RatioOfTotalFree
	{
		numeratorPermyriad: 1_000,
	};
	
	pub const EightyPercentRatioOfTotalFree: HugePageAllocationStrategy = HugePageAllocationStrategy::RatioOfTotalFree
	{
		numeratorPermyriad: 8_000,
	};
	
	pub fn allocateInPages(&self, hugePageSize: HugePageSize, totalNumberOfKiloBytesFree: u64) -> u64
	{
		let numberOfKiloBytes = match *self
		{
			HugePageAllocationStrategy::RatioOfTotalFree { numeratorPermyriad } =>
			{
				(totalNumberOfKiloBytesFree * numeratorPermyriad as u64) / 10_000
			},
			
			HugePageAllocationStrategy::AllLessReserve { reserveInKiloBytes } =>
			{
				totalNumberOfKiloBytesFree.checked_sub(reserveInKiloBytes).unwrap_or(0)
			},
			
			HugePageAllocationStrategy::FixedOrFail { fixedAmountInKiloBytes } =>
			{
				assert!(fixedAmountInKiloBytes >= totalNumberOfKiloBytesFree, "totalNumberOfKiloBytesFree '{}' is less than the fixed amount of fixedAmountInKiloBytes '{}'", totalNumberOfKiloBytesFree, fixedAmountInKiloBytes);
				fixedAmountInKiloBytes
			},
		};

		hugePageSize.calculate_number_of_huge_pages(numberOfKiloBytes)
	}
}
