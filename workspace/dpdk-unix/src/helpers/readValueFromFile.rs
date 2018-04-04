// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub fn readValueFromFile<F>(filePath: &Path) -> io::Result<F>
where F: FromStr, <F as FromStr>::Err: 'static + Send + Sync + Error
{
	let mut openFile = try!(File::open(filePath));
	let mut rawString = String::with_capacity(256);
	let bytesRead = try!(openFile.read_to_string(&mut rawString));
	
	if bytesRead == 0
	{
		return Err(io::Error::new(ErrorKind::InvalidData, "Empty file"));
	}
	
	let shouldBeLineFeed = rawString.remove(bytesRead - 1);
	if shouldBeLineFeed != '\n'
	{
		return Err(io::Error::new(ErrorKind::InvalidData, "File lacks terminating line feed"));
	}
	
	match rawString.parse::<F>()
	{
		Err(error) => Err(io::Error::new(ErrorKind::InvalidData, error)),
		Ok(value) => Ok(value),
	}
}
