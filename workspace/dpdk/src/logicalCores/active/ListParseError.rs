// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


quick_error!
{
	#[derive(Debug)]
	pub enum ListParseError
	{
		IoError(err: Error)
		{
			cause(err)
			from()
		}
		
		EmptyFile
		
		FileContentsDidNotEndWithATrailingLineFeed
		
		ContainsAnEmptyIndexOrRange
		
		CouldNotParseIndex(description: &'static str, unparsableIndex: String, cause: ParseIntError)
		{
			display("Could not parse {} index '{}' because '{}'", description, unparsableIndex, cause)
			cause(cause)
		}
		
		IndexExceedsMaximum(description: &'static str, index: usize, maximum: usize)
		{
			display("Could not use {} index '{}' because it exceeds the maximum of '{}'", description, index, maximum)
		}
		
		ContainsMisSortedIndices(first: usize, nextMinimumIndex: usize)
		{
			display("Did not expect '{}' when minimum expectation is '{}'", first, nextMinimumIndex)
		}
		
		RangeIsNotAnAscendingRangeWithMoreThanOneElement(first: usize, second: usize)
		{
			display("range is not an ascending range with more than one element; it is '{}-{}'!", first, second)
		}
	}
}
