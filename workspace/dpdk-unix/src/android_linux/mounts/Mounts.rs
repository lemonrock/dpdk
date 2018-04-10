// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Enables parsing of known file system mounts.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Mounts(*mut FILE);

impl Drop for Mounts
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

impl Mounts
{
	/// Parses pseudo-file of current mounts.
	pub fn parse(proc_path: &Path) -> Result<HashMap<PathBuf, Mount>, io::Error>
	{
		let mut mounts_file_path = PathBuf::from(proc_path);
		mounts_file_path.push("self/mounts");
		let mounts = Self::new(&mounts_file_path, true)?;
		
		let mut map = HashMap::with_capacity(64);
		
		mounts.use_mount(|mount_point|
		{
			let key = mount_point.mount_point.clone();
			if let Some(previous) = map.insert(key, mount_point)
			{
				Err(io::Error::new(ErrorKind::InvalidData, format!("Duplicate mount_point for mount_point point '{:?}'", previous.mount_point)))
			}
			else
			{
				Ok(())
			}
		})?;
		
		Ok(map)
	}
	
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
			Ok(Mounts(handle))
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
