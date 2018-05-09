// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug)]
pub struct HugePageFinisher
{
	mountPath: Option<PathBuf>,
	weCreated: bool,
	weMounted: bool,
}

impl Finisher for HugePageFinisher
{
	#[allow(unused_variables)]
	fn finish(&self)
	{
		if let Some(ref mountPath) = self.mountPath
		{
			if self.weMounted
			{
				if let Err(error) = Mount::unmount(mountPath, UnmountFlags::Detach)
				{
					warn!("Could not unmount {:?} because {:?}", mountPath, error);
				}
			}
			if self.weCreated
			{
				if let Err(error) = remove_dir_all(mountPath)
				{
					warn!("Could not remove mount path {:?} because {:?}", mountPath, error);
				}
			}
		}
	}
}

impl HugePageFinisher
{
	pub const FreeBsd: HugePageFinisher = HugePageFinisher
	{
		mountPath: None,
		weCreated: false,
		weMounted: false,
	};

	pub fn new(mountPoint: &Path, weCreated: bool, weMounted: bool) -> Self
	{
		HugePageFinisher
		{
			mountPath: Some(PathBuf::from(mountPoint)),
			weCreated,
			weMounted,
		}
	}
}
