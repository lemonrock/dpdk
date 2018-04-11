// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct tle_dest
{
	pub head_mp: *mut rte_mempool,
	pub dev: *mut tle_dev,
	pub ol_flags: u64,
	pub mtu: u16,
	pub l2_len: u8,
	pub l3_len: u8,
	pub hdr: [u8; 96usize],
}

impl Default for tle_dest
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for tle_dest
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(
			f,
			"tle_dest {{ head_mp: {:?}, dev: {:?}, hdr: [{}] }}",
			self.head_mp,
			self.dev,
			self.hdr
				.iter()
				.enumerate()
				.map(|(i, v)| format!(
					"{}{:?}",
					if i > 0
					{
						", "
					}
					else
					{
						""
					},
					v
				))
				.collect::<String>()
		)
	}
}
