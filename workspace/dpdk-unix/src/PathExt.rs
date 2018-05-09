// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// An extension trait to make it easier to work with sys and proc files and folders.
pub trait PathExt
{
	/// Converts a `Path` to a `CString`.
	#[cfg(unix)]
	#[inline(always)]
	fn to_c_string(&self) -> CString;
	
	/// Makes a file read-write to all.
	#[cfg(unix)]
	#[inline(always)]
	fn make_file_read_write_all(&self) -> io::Result<()>;
	
	/// Makes a folder searchable to all (ie gives it read and execute permissions).
	#[cfg(unix)]
	#[inline(always)]
	fn make_folder_searchable_to_all(&self) -> io::Result<()>;
	
	/// Reads a value from a file which is line-feed terminated and is hexadecimal using a parser.
	#[inline(always)]
	fn read_hexadecimal_value_with_prefix<P: Fn(&str) -> Result<T, ParseIntError>, T>(&self, size: usize, parser: P) -> io::Result<T>;
	
	/// Reads a value from a file which is line-feed terminated and is hexadecimal into an u16.
	#[inline(always)]
	fn read_hexadecimal_value_with_prefix_u16(&self) -> io::Result<u16>
	{
		self.read_hexadecimal_value_with_prefix(4, |raw_string| u16::from_str_radix(raw_string, 16))
	}
	
	/// Reads a value from a file which is line-feed terminated.
	#[inline(always)]
	fn read_value<F>(&self) -> io::Result<F> where F: FromStr, <F as FromStr>::Err: 'static + Send + Sync + Error;
	
	/// Writes a value to a file which is line-feed terminated.
	#[inline(always)]
	fn write_value<D: Display>(&self, value: D) -> io::Result<()>;
	
	/// Reads and parses a linux core or numa mask string from a file.
	///
	/// Returns a BTreeSet with the zero-based indices found in the string. For example, "2,4-31,32-63" would return a set with all values between 0 to 63 except 0, 1 and 3.
	#[inline(always)]
	fn read_linux_core_or_numa_mask(&self) -> Result<BTreeSet<u16>, ListParseError>;
	
	/// Parses a virtual memory statistics file (`vmstat`).
	#[inline(always)]
	fn parse_virtual_memory_statistics_file(&self) -> io::Result<HashMap<VirtualMemoryStatisticName, u64>>;
	
	/// Parses a memory information file (`meminfo`).
	#[inline(always)]
	fn parse_memory_information_file(&self, memory_statistic_name_prefix: &str) -> Result<MemoryStatistics, MemoryStatisticsParseError>;
}

impl PathExt for Path
{
	#[cfg(unix)]
	#[inline(always)]
	fn to_c_string(&self) -> CString
	{
		CString::new(self.as_os_str().as_bytes()).expect("Paths should not contain interior ASCII NULs")
	}
	
	#[cfg(unix)]
	#[inline(always)]
	fn make_file_read_write_all(&self) -> io::Result<()>
	{
		#[inline(always)]
		fn add_read_write_permissions(permissions: Permissions) -> Permissions
		{
			Permissions::from_mode(permissions.mode() | 0o666)
		}
		let metadata = metadata(self)?;
		set_permissions(self, add_read_write_permissions(metadata.permissions()))
	}
	
	#[cfg(unix)]
	#[inline(always)]
	fn make_folder_searchable_to_all(&self) -> io::Result<()>
	{
		#[inline(always)]
		fn add_read_and_execute_permissions(permissions: Permissions) -> Permissions
		{
			Permissions::from_mode(permissions.mode() | 0o555)
		}
		let metadata = metadata(self)?;
		set_permissions(self, add_read_and_execute_permissions(metadata.permissions()))
	}
	
	#[inline(always)]
	fn read_hexadecimal_value_with_prefix<P: Fn(&str) -> Result<T, ParseIntError>, T>(&self, size: usize, parser: P) -> io::Result<T>
	{
		let mut opened_file = File::open(self)?;
		let mut raw_string = String::with_capacity(128);
		let bytes_read = opened_file.read_to_string(&mut raw_string)?;
		
		// '0x' and '\n', eg '0x1af4'
		let bytes_to_read = 3 + size;
		if bytes_read != bytes_to_read
		{
			return Err(io::Error::new(ErrorKind::InvalidData, format!("{} bytes not read", bytes_to_read)));
		}
		
		let should_be_line_feed = raw_string.remove(bytes_read - 1);
		if should_be_line_feed != '\n'
		{
			return Err(io::Error::new(ErrorKind::InvalidData, "File lacks terminating line feed"));
		}
		
		match &raw_string[..2]
		{
			"0x" => (),
			_ => return Err(io::Error::new(ErrorKind::InvalidData, "value does not start '0x'")),
		}
		
		match parser(&raw_string[2..])
		{
			Err(error) => Err(io::Error::new(ErrorKind::InvalidData, error)),
			Ok(value) => Ok(value),
		}
	}
	
	#[inline(always)]
	fn read_value<F>(&self) -> io::Result<F> where F: FromStr, <F as FromStr>::Err: 'static + Send + Sync + Error
	{
		let mut opened_file = File::open(self)?;
		let mut raw_string = String::with_capacity(256);
		let bytes_read = opened_file.read_to_string(&mut raw_string)?;
		
		if bytes_read == 0
		{
			return Err(io::Error::new(ErrorKind::InvalidData, "Empty file"));
		}
		
		let should_be_line_feed = raw_string.remove(bytes_read - 1);
		if should_be_line_feed != '\n'
		{
			return Err(io::Error::new(ErrorKind::InvalidData, "File lacks terminating line feed"));
		}
		
		match raw_string.parse::<F>()
		{
			Err(error) => Err(io::Error::new(ErrorKind::InvalidData, error)),
			Ok(value) => Ok(value),
		}
	}
	
	#[inline(always)]
	fn write_value<D: Display>(&self, value: D) -> io::Result<()>
	{
		let value = format!("{}\n", value).into_bytes();
		let mut file = OpenOptions::new().write(true).open(self)?;
		file.write_all(value.as_slice())
	}
	
	#[inline(always)]
	fn read_linux_core_or_numa_mask(&self) -> Result<BTreeSet<u16>, ListParseError>
	{
		let mut file = File::open(self)?;
		let mut raw_string = String::with_capacity(256);
		let bytes_read = file.read_to_string(&mut raw_string)?;
		
		use self::ListParseError::*;
		
		let is_empty_file = bytes_read == 0;
		
		if is_empty_file
		{
			return Err(EmptyFile);
		}
		
		let should_be_line_feed = raw_string.remove(bytes_read - 1);
		if should_be_line_feed != '\n'
		{
			return Err(FileContentsDidNotEndWithATrailingLineFeed);
		}
		
		ListParseError::parse_linux_list_string(&raw_string)
	}
	
	#[inline(always)]
	fn parse_virtual_memory_statistics_file(&self) -> io::Result<HashMap<VirtualMemoryStatisticName, u64>>
	{
		let file = File::open(self)?;
		
		let mut reader = BufReader::with_capacity(4096, file);
		
		let mut statistics = HashMap::with_capacity(6);
		let mut zero_based_line_number = 0;
		let mut line = String::with_capacity(64);
		while reader.read_line(&mut line)? > 0
		{
			{
				use self::ErrorKind::InvalidData;
				
				let mut split = line.splitn(2, ' ');
				
				let statistic_name = VirtualMemoryStatisticName::parse(split.next().unwrap());
				
				let statistic_value = match split.next()
				{
					None => return Err(io::Error::new(InvalidData, format!("Zero based line '{}' does not have a value second column", zero_based_line_number))),
					Some(value) =>
					{
						match value.parse::<u64>()
						{
							Err(parse_error) => return Err(io::Error::new(InvalidData, parse_error)),
							Ok(value) => value,
						}
					}
				};
				
				if let Some(previous) = statistics.insert(statistic_name, statistic_value)
				{
					return Err(io::Error::new(InvalidData, format!("Zero based line '{}' has a duplicate statistic (was '{}')", zero_based_line_number, previous)))
				}
			}
			
			line.clear();
			zero_based_line_number += 1;
		}
		
		Ok(statistics)
	}
	
	/// Parses the `meminfo` file.
	fn parse_memory_information_file(&self, memory_statistic_name_prefix: &str) -> Result<MemoryStatistics, MemoryStatisticsParseError>
	{
		let mut reader = BufReader::with_capacity(4096, File::open(self)?);
		
		let mut memory_statistics = HashMap::new();
		let mut line_number = 0;
		let mut line = String::with_capacity(512);
		while reader.read_line(&mut line)? > 0
		{
			{
				let mut split = line.splitn(2, ':');
				
				let memory_statistic_name = MemoryStatisticName::parse(split.next().unwrap(), memory_statistic_name_prefix);
				
				let memory_statistic_value = match split.next()
				{
					None => return Err(MemoryStatisticsParseError::CouldNotParseMemoryStatisticValue(line_number, memory_statistic_name)),
					Some(raw_value) =>
						{
							let trimmed_raw_value = raw_value.trim();
							let ends_with = memory_statistic_name.unit().ends_with();
							
							if !trimmed_raw_value.ends_with(ends_with)
							{
								return Err(MemoryStatisticsParseError::CouldNotParseMemoryStatisticValue(line_number, memory_statistic_name));
							}
							
							let trimmed = &raw_value[0..raw_value.len() - ends_with.len()];
							
							match trimmed.parse::<u64>()
							{
								Ok(value) => value,
								Err(int_parse_error) => return Err(MemoryStatisticsParseError::CouldNotParseMemoryStatisticValueAsU64(line_number, memory_statistic_name, raw_value.to_owned(), int_parse_error))
							}
						}
				};
				
				if memory_statistics.contains_key(&memory_statistic_name)
				{
					return Err(MemoryStatisticsParseError::DuplicateMemoryStatistic(line_number, memory_statistic_name, memory_statistic_value));
				}
				
				memory_statistics.insert(memory_statistic_name, memory_statistic_value);
			}
			
			line.clear();
			line_number += 1;
		}
		
		Ok(MemoryStatistics(memory_statistics))
	}
}

