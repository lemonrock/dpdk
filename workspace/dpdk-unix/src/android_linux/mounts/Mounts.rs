// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Mounts (or mount points) for the current process.
#[derive(Debug)]
pub struct Mounts(HashMap<PathBuf, Mount>);

impl Mounts
{
	pub(crate) fn parse(file_path: &Path) -> Result<Self, io::Error>
	{
		let mounts_wrapper = MountsWrapper::new(file_path, true)?;
		
		let mut map = HashMap::with_capacity(64);
		
		mounts_wrapper.use_mount(|mount_point|
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
		
		Ok(Mounts(map))
	}
	
	/// Returns a path for an existing `hugetlbfs` mount, if any.
	#[inline(always)]
	pub fn existing_hugetlbfs_mount<'a>(&self) -> Option<PathBuf>
	{
		for mount in self.0.values()
		{
			if mount.has_file_system_type(&FileSystemType::hugetlbfs)
			{
				let mount_point = &mount.mount_point;
				if mount_point.is_dir()
				{
					return Some(mount_point.to_owned());
				}
			}
		}
		None
	}
}
