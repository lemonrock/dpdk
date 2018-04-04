// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Mounts(*mut FILE, bool);

impl Drop for Mounts
{
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
	const_cstr!
	{
		ReadOnlyFlag = "r";
		ReadWriteFlag = "ra";
	}
	
	// Ought to be a constant: &Path, but Rust makes this nearly impossible
	#[inline(always)]
	pub fn ProcSelfMounts() -> PathBuf
	{
		PathBuf::from("/proc/self/mounts")
	}
	
	#[inline(always)]
	pub fn ProcMounts() -> PathBuf
	{
		PathBuf::from("/proc/mounts")
	}
	
	#[inline(always)]
	pub fn readOnly(&self) -> bool
	{
		self.1
	}
		
	pub fn parse(procPath: &Path) -> Result<HashMap<PathBuf, Mount>, io::Error>
	{
		let mut mountsFilePath = PathBuf::from(procPath);
		mountsFilePath.push("self/mounts");
		let mounts = try!(Self::new(&mountsFilePath, true));
		
		let mut map = HashMap::with_capacity(64);
		
		try!(mounts.useMounts(|mount|
		{
			let key = mount.mountPoint.clone();
			if let Some(previous) = map.insert(key, mount)
			{
				Err(io::Error::new(ErrorKind::InvalidData, format!("Duplicate mount for mount point '{:?}'", previous.mountPoint)))
			}
			else
			{
				Ok(())
			}
		}));
		
		Ok(map)
	}
		
	pub fn procSelfMountsReadOnly() -> Result<Self, io::Error>
	{
		Self::new(&Self::ProcSelfMounts(), true)
	}
	
	fn new(mountsFilePath: &Path, readOnly: bool) -> Result<Self, io::Error>
	{
		let mountsFilePath = pathToCString(mountsFilePath);
		
		let flag = match readOnly
		{
			false => Self::ReadOnlyFlag,
			true => Self::ReadWriteFlag,
		};
		
		let mountsHandle = unsafe { setmntent(mountsFilePath.as_ptr(), flag.as_ptr()) };
		if unlikely(mountsHandle.is_null())
		{
			Err(io::Error::new(ErrorKind::NotFound, "setmntent() returned NULL - not found or couldn't open or readOnly was false and file permissions prevent writing"))
		}
		else
		{
			Ok(Mounts(mountsHandle, readOnly))
		}
	}
	
	pub fn useMounts<F>(&self, mut calledForEachMount: F) -> Result<(), io::Error>
	where F: FnMut(Mount) -> Result<(), io::Error>
	{
		loop
		{
			let mountEntryPointer = unsafe { getmntent(self.0) };
			if unlikely(mountEntryPointer.is_null())
			{
				break;
			}
			let result = calledForEachMount(Mount::from_mntent(mountEntryPointer));
			if unlikely(result.is_err())
			{
				return result;
			}
		}
		Ok(())
	}
}
