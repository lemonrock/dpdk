// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents `/dev`.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
pub struct DevPath(PathBuf);

impl Default for DevPath
{
	#[inline(always)]
	fn default() -> Self
	{
		DevPath(PathBuf::from("/dev"))
	}
}

impl DevPath
{
	/// `/dev/null`.
	#[inline(always)]
	pub fn null(&self) -> PathBuf
	{
		self.file_path("null")
	}

	#[inline(always)]
	fn file_path(&self, file_name: &str) -> PathBuf
	{
		let mut path = self.path();
		path.push(file_name);
		path
	}

	#[inline(always)]
	fn path(&self) -> PathBuf
	{
		self.0.to_owned()
	}
}
