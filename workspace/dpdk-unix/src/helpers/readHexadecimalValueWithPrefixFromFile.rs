// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub fn readHexadecimalValueWithPrefixFromFile<P, T>(filePath: &Path, size: usize, parser: P) -> io::Result<T>
where P: Fn(&str) -> Result<T, ParseIntError>
{
	let mut openFile = try!(File::open(filePath));
	let mut rawString = String::with_capacity(128);
	let bytesRead = try!(openFile.read_to_string(&mut rawString));
	
	// '0x' and '\n', eg '0x1af4'
	let bytesToRead = 3 + size;
	if bytesRead != bytesToRead
	{
		return Err(io::Error::new(ErrorKind::InvalidData, format!("{} bytes not read", bytesToRead)));
	}

	let shouldBeLineFeed = rawString.remove(bytesRead - 1);
	if shouldBeLineFeed != '\n'
	{
		return Err(io::Error::new(ErrorKind::InvalidData, "File lacks terminating line feed"));
	}
	
	match &rawString[..2]
	{
		"0x" => (),
		_ => return Err(io::Error::new(ErrorKind::InvalidData, "value does not start '0x'")),
	}
	
	match parser(&rawString[2..])
	{
		Err(error) => Err(io::Error::new(ErrorKind::InvalidData, error)),
		Ok(value) => Ok(value),
	}
}
