// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Strategies for allocating memory from NUMA nodes.
///
/// These all round memory requests down to a size that fits the largest huge page size for the processor.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub enum HugePageAllocationStrategy
{
	/// As a per-myriad `ratio` ('percentage') of total free memory.
	TotalFreeRatio
	{
		ratio: PerMyriad
	},
	
	/// As total free memory less a `reserve`).
	TotalFreeLessReserve
	{
		reserve: KiloBytes,
	},
	
	/// A `fixed_amount` allocation, otherwise panic.
	FixedOrFail
	{
		fixed_amount: KiloBytes,
	},
}

impl Default for HugePageAllocationStrategy
{
	#[inline(always)]
	fn default() -> Self
	{
		HugePageAllocationStrategy::TotalFreeRatio
		{
			ratio: PerMyriad::NinetyFivePercent,
		}
	}
}

impl HugePageAllocationStrategy
{
	#[inline(always)]
	pub(crate) fn calculate_number_of_huge_pages(&self, huge_page_size: HugePageSize, total_free: KiloBytes) -> u64
	{
		use self::HugePageAllocationStrategy::*;
		
		let allocation = match *self
		{
			TotalFreeRatio { ratio } => total_free.scale_by(ratio),
			
			TotalFreeLessReserve { reserve } => total_free.subtract_with_zero_floor(reserve),
			
			FixedOrFail { fixed_amount } =>
			{
				assert!(fixed_amount >= total_free, "total_free '{}' is less than the fixed amount of fixed_amount '{}'", total_free, fixed_amount);
				fixed_amount
			}
		};

		huge_page_size.calculate_number_of_huge_pages(allocation.0)
	}
}
