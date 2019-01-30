// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2019 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[inline(always)]
fn dpdk_provided_kernel_modules_path() -> PathBuf
{
	#[inline(always)]
	fn parent_folder_path() -> PathBuf
	{
		if let Ok(path) = current_exe()
		{
			if let Ok(path) = path.canonicalize()
			{
				if let Some(parent) = path.parent()
				{
					return parent.to_path_buf()
				}
			}
		}
		PathBuf::from("/")
	}

	#[inline(always)]
	fn append_dpdk_relative_modules_path(mut parent_folder_path: PathBuf) -> PathBuf
	{
		if parent_folder_path.to_str().map(|path| path.ends_with("/bin") || path.ends_with("/sbin")).unwrap_or(false)
		{
			parent_folder_path.set_file_name("lib");
		}
		else
		{
			parent_folder_path.push("lib");
		}
		parent_folder_path.push("modules/dpdk");
		parent_folder_path
	}

	append_dpdk_relative_modules_path(parent_folder_path()),

}
