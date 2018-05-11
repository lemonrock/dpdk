// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Enables parsing of known file system mounts.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct MountsWrapper(*mut FILE);

impl Drop for MountsWrapper
{
	//noinspection SpellCheckingInspection
	#[inline(always)]
	fn drop(&mut self)
	{
		match unsafe { endmntent(self.0) }
		{
			1 => (),
			illegal @ _ => panic!("endmntent() returned '{}'; it should never return anything other than 1", illegal),
		}
	}
}

impl MountsWrapper
{
	//noinspection SpellCheckingInspection
	fn new(mounts_file_path: &Path, read_only: bool) -> Result<Self, io::Error>
	{
		let mounts_file_path = mounts_file_path.to_c_string();
		
		const_cstr!
		{
			ReadOnlyFlag = "r";
			ReadWriteFlag = "ra";
		}
		
		let flag = match read_only
		{
			false => ReadOnlyFlag,
			true => ReadWriteFlag,
		};
		
		let handle = unsafe { setmntent(mounts_file_path.as_ptr(), flag.as_ptr()) };
		if unlikely(handle.is_null())
		{
			Err(io::Error::new(ErrorKind::NotFound, "setmntent() returned NULL - not found or couldn't open or read_only was false and file permissions prevent writing"))
		}
		else
		{
			Ok(MountsWrapper(handle))
		}
	}
	
	fn use_mount<F>(&self, mut called_for_each_mount_point: F) -> Result<(), io::Error>
	where F: FnMut(Mount) -> Result<(), io::Error>
	{
		let mut mount_entry_pointer;
		while
		{
			mount_entry_pointer = unsafe { getmntent(self.0) };
			!mount_entry_pointer.is_null()
		}
		{
			let result = called_for_each_mount_point(Mount::from_mntent(mount_entry_pointer));
			if unlikely(result.is_err())
			{
				return result;
			}
		}
		Ok(())
	}
}
