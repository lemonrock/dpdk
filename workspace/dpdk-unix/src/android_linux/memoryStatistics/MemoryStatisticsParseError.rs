// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


quick_error!
{
	#[derive(Debug)]
	pub enum MemoryStatisticsParseError
	{
		CouldNotOpenFile(cause: Error)
		{
			cause(cause)
			from()
		}
		
		CouldNotParseMemoryStatisticValue(lineCount: usize, memoryStatisticName: MemoryStatisticName)
		{
			display("Zero-based line '{}' statistic '{:?}' did not contain a memory statistic value", lineCount, memoryStatisticName)
		}
		
		CouldNotParseMemoryStatisticValueAsU64(lineCount: usize, memoryStatisticName: MemoryStatisticName, badValue: String, cause: ParseIntError)
		{
			display("Zero-based line '{}' statistic '{:?}' did not contain a valid memory statistic value as u64 '{}' because '{}'", lineCount, memoryStatisticName, badValue, cause)
		}
		
		DuplicateMemoryStatistic(lineCount: usize, memoryStatisticName: MemoryStatisticName,newValue: u64)
		{
			display("Zero-based line '{}' statistic '{:?}' was duplicated (this value is '{}')", lineCount, memoryStatisticName, newValue)
		}
	}
}
