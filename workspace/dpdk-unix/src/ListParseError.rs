// This file is part of dpdk-unix. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk-unix/master/COPYRIGHT. No part of dpdk-unix, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk-unix. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk-unix/master/COPYRIGHT.


quick_error!
{
	/// List parse error.
	#[derive(Debug)]
	pub enum ListParseError
	{
		/// An IO error.
		IoError(err: io::Error)
		{
			cause(err)
			from()
		}
		
		/// Empty file.
		EmptyFile
		{
		}
		
		/// File contents did not end with a trailing line feed.
		FileContentsDidNotEndWithATrailingLineFeed
		{
		}
		
		/// Contains an empty index or range.
		ContainsAnEmptyIndexOrRange
		{
		}
		
		/// Could not parse index.
		CouldNotParseIndex(description: &'static str, unparsable_index: String, cause: ParseIntError)
		{
			display("Could not parse {} index '{}' because '{}'", description, unparsable_index, cause)
			cause(cause)
		}
		
		/// Contains mis-sorted indices.
		ContainsMisSortedIndices(first: u16, next_minimum_index_expected: u16)
		{
			display("Did not expect '{}' when minimum expectation is '{}'", first, next_minimum_index_expected)
		}
		
		/// Range is not an ascending range with more than one element.
		RangeIsNotAnAscendingRangeWithMoreThanOneElement(first: u16, second: u16)
		{
			display("range is not an ascending range with more than one element; it is '{}-{}'!", first, second)
		}
	}
}

impl ListParseError
{
	/// Parses a Linux list string used for cpu sets, core masks and NUMA nodes such as "2,4-31,32-63".
	///
	/// Returns a BTreeSet with the zero-based indices found in the string. For example, "2,4-31,32-63" would return a set with all values between 0 to 63 except 0, 1 and 3.
	pub fn parse_linux_list_string(linux_list_string: &str) -> Result<BTreeSet<u16>, ListParseError>
	{
		#[inline(always)]
		fn parse_index(index_string: &str, description: &'static str) -> Result<u16, ListParseError>
		{
			match index_string.parse()
			{
				Ok(index) => Ok(index),
				Err(error) => Err(ListParseError::CouldNotParseIndex(description, index_string.to_owned(), error)),
			}
		}
		
		let mut result = BTreeSet::new();
		
		use self::ListParseError::*;
		
		// Prevents mis-sorted strings
		let mut next_minimum_index_expected = 0;
		for index_or_range in linux_list_string.split(',')
		{
			if index_or_range.is_empty()
			{
				return Err(ContainsAnEmptyIndexOrRange);
			}
			
			let mut range_iterator = index_or_range.splitn(2, '-');
			
			let first = parse_index(range_iterator.next().unwrap(), "first")?;
			
			if first < next_minimum_index_expected
			{
				return Err(ContainsMisSortedIndices(first, next_minimum_index_expected));
			}
			
			if let Some(second) = range_iterator.last()
			{
				let second = parse_index(second, "second")?;
				if first >= second
				{
					return Err(RangeIsNotAnAscendingRangeWithMoreThanOneElement(first, second));
				}
				
				for index in first .. (second + 1)
				{
					result.insert(index);
				}
				
				next_minimum_index_expected = second;
			}
			else
			{
				let sole = first;
				result.insert(sole);
				next_minimum_index_expected = sole;
			}
		}
		
		Ok(result)
	}
}
