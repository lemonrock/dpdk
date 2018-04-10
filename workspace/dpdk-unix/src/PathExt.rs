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
	fn read_hexadecimal_value_with_prefix<P, T>(&self, size: usize, parser: P) -> io::Result<T> where P: Fn(&str) -> Result<T, ParseIntError>;
	
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
	fn read_hexadecimal_value_with_prefix<P, T>(&self, size: usize, parser: P) -> io::Result<T> where P: Fn(&str) -> Result<T, ParseIntError>
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
}

