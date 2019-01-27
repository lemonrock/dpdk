// This file is part of dpdk-unix. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk-unix/master/COPYRIGHT. No part of dpdk-unix, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk-unix. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk-unix/master/COPYRIGHT.


/// List parse error.
#[derive(Debug)]
pub enum ListParseError
{
	/// An IO error.
	IoError(io::Error),
	
	/// Contains an empty index or range.
	ContainsAnEmptyIndexOrRange,
	
	/// Could not parse index (not a string).
	CouldNotParseIndexAsNotAString
	{
		/// Description.
		description: &'static str,

		/// Unparsable index.
		unparsable_index: Box<[u8]>,

		/// Cause.
		cause: Utf8Error,
	},

	/// Could not parse index.
	CouldNotParseIndex
	{
		/// Description.
		description: &'static str,

		/// Unparsable index.
		unparsable_index: String,

		/// Cause.
		cause: ParseIntError,
	},
	
	/// Contains mis-sorted indices.
	ContainsMisSortedIndices
	{
		/// First part of index.
		first: u16,

		/// Minimum expected for next index.
		next_minimum_index_expected: u16
	},
	
	/// Range is not an ascending range with more than one element.
	RangeIsNotAnAscendingRangeWithMoreThanOneElement
	{
		/// First part of index.
		first: u16,

		/// Second part of index.
		second: u16
	},
}

impl Display for ListParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		<ListParseError as Debug>::fmt(self, f)
	}
}

impl error::Error for ListParseError
{
	#[inline(always)]
	fn source(&self) ->  Option<&(error::Error + 'static)>
	{
		use self::ListParseError::*;

		match self
		{
			&IoError(ref error) => Some(error),

			&ContainsAnEmptyIndexOrRange => None,

			&CouldNotParseIndexAsNotAString { ref cause, .. } => Some(cause),

			&CouldNotParseIndex { ref cause, .. } => Some(cause),

			&ContainsMisSortedIndices { .. } => None,

			&RangeIsNotAnAscendingRangeWithMoreThanOneElement { .. } => None,
		}
	}
}

impl From<io::Error> for ListParseError
{
	#[inline(always)]
	fn from(error: io::Error) -> Self
	{
		ListParseError::IoError(error)
	}
}

impl ListParseError
{
	/// Parses a Linux list string used for cpu sets, core masks and NUMA nodes such as "2,4-31,32-63" and "1,2,10-20,100-2000:2/25" (see <https://www.kernel.org/doc/html/latest/admin-guide/kernel-parameters.html> for an awful description of this mad syntax).
	///
	/// Returns a BTreeSet with the zero-based indices found in the string. For example, "2,4-31,32-63" would return a set with all values between 0 to 63 except 0, 1 and 3.
	pub fn parse_linux_list_string<Mapper: Fn(u16) -> R, R: Ord>(linux_list_string: &[u8], mapper: Mapper) -> Result<BTreeSet<R>, ListParseError>
	{
		#[inline(always)]
		fn parse_index(index_string: &[u8], description: &'static str) -> Result<u16, ListParseError>
		{
			use self::ListParseError::*;

			let index_string = match from_utf8(index_string)
			{
				Ok(index_string) => index_string,
				Err(cause) => return Err(CouldNotParseIndexAsNotAString { description, unparsable_index: index_string.to_vec().into_boxed_slice(), cause }),
			};

			match index_string.parse()
			{
				Ok(index) => Ok(index),
				Err(cause) => Err(CouldNotParseIndex { description, unparsable_index: index_string.to_owned(), cause }),
			}
		}
		
		let mut result = BTreeSet::new();
		
		use self::ListParseError::*;
		
		// Prevents mis-sorted strings
		let mut next_minimum_index_expected = 0;
		for index_or_range in split(linux_list_string, b',')
		{
			if index_or_range.is_empty()
			{
				return Err(ContainsAnEmptyIndexOrRange);
			}
			
			let mut range_iterator = splitn(index_or_range, 2, b'-');
			
			let first =
			{
				let index = parse_index(range_iterator.next().unwrap(), "first")?;
				if index < next_minimum_index_expected
				{
					return Err(ContainsMisSortedIndices { first: index, next_minimum_index_expected });
				}
				index
			};
			
			if let Some(second) = range_iterator.last()
			{
				// There is a weird, but rare, syntax used of `100-2000:2/25` for some ranges.
				let mut range_or_range_with_groups = splitn(second, 2, b':');
				
				let second =
				{
					let index = parse_index(range_or_range_with_groups.next().unwrap(), "second")?;
					if first >= index
					{
						return Err(RangeIsNotAnAscendingRangeWithMoreThanOneElement { first, second: index });
					}
					index
				};
				
				match range_or_range_with_groups.last()
				{
					None =>
					{
						for index in first .. (second + 1)
						{
							result.insert(mapper(index));
						}
						
						next_minimum_index_expected = second;
					}
					
					Some(weird_but_rare_group_syntax) =>
					{
						let mut weird_but_rare_group_syntax = splitn(weird_but_rare_group_syntax, 2, b'/');
						let used_size = parse_index(weird_but_rare_group_syntax.next().unwrap(), "used_size")?;
						let group_size = parse_index(weird_but_rare_group_syntax.last().expect("a group does not have group_size"), "group_size")?;
						
						assert_ne!(used_size, 0, "used_size is zero");
						assert_ne!(group_size, 0, "group_size is zero");
						
						let mut base_cpu_index = first;
						while base_cpu_index < second
						{
							for cpu_index_increment in 0 .. used_size
							{
								let cpu_index = base_cpu_index + cpu_index_increment;
								result.insert(mapper(cpu_index));
							}
							
							base_cpu_index += group_size;
						}
					}
				}
			}
			else
			{
				let sole = first;
				result.insert(mapper(sole));
				next_minimum_index_expected = sole;
			}
		}
		
		Ok(result)
	}
}
