// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Errors possible when parsing memory statistics.
#[derive(Debug,)]
pub enum MemoryInformationParseError
{
	/// Could not open a file of memory statistics.
	CouldNotOpenFile(io::Error),

	/// Could not parse a memory statistic.
	CouldNotParseMemoryInformationValue
	{
		/// Zero-based line number in the file the error occurred at.
		zero_based_line_number: usize,

		/// Memory item it occurred for.
		memory_information_name: MemoryInformationName,
	},

	/// Could not parse a memory statistic as a UTF-8 string.
	CouldNotParseAsUtf8
	{
		/// Zero-based line number in the file the error occurred at.
		zero_based_line_number: usize,

		/// Memory item it occurred for.
		memory_information_name: MemoryInformationName,

		/// Bad value.
		bad_value: Box<[u8]>,

		cause: Utf8Error,
	},

	/// Could not parse a memory statistic (trimmed).
	CouldNotParseMemoryInformationValueTrimmed
	{
		/// Zero-based line number in the file the error occurred at.
		zero_based_line_number: usize,

		/// Memory item it occurred for.
		memory_information_name: MemoryInformationName,

		/// Bad value.
		bad_value: String,
	},

	/// Could not parse a memory statistic as a u64 value.
	CouldNotParseMemoryInformationValueAsU64
	{
		/// Zero-based line number in the file the error occurred at.
		zero_based_line_number: usize,

		/// Memory item it occurred for.
		memory_information_name: MemoryInformationName,

		/// Bad value.
		bad_value: String,

		/// Underlying parse error.
		cause: ParseIntError,
	},

	/// Could not parse a memory statistic because it was a duplicate.
	DuplicateMemoryInformation
	{
		/// Zero-based line number in the file the error occurred at.
		zero_based_line_number: usize,

		/// Memory item it occurred for.
		memory_information_name: MemoryInformationName,

		/// New value.
		new_value: u64,
	},
}

impl Display for MemoryInformationParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		<MemoryInformationParseError as Debug>::fmt(self, f)
	}
}

impl error::Error for MemoryInformationParseError
{
	#[inline(always)]
	fn source(&self) ->  Option<&(error::Error + 'static)>
	{
		use self::MemoryInformationParseError::*;

		match self
		{
			&CouldNotOpenFile(ref error) => Some(error),

			&CouldNotParseMemoryInformationValue { .. } => None,

			&CouldNotParseAsUtf8 { ref cause, .. } => Some(cause),

			&CouldNotParseMemoryInformationValueTrimmed { .. } => None,

			&CouldNotParseMemoryInformationValueAsU64 { ref cause, .. } => Some(cause),

			&DuplicateMemoryInformation { .. } => None,
		}
	}
}

impl From<io::Error> for MemoryInformationParseError
{
	#[inline(always)]
	fn from(error: io::Error) -> Self
	{
		MemoryInformationParseError::CouldNotOpenFile(error)
	}
}
