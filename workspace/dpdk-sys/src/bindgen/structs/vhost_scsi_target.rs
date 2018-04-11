// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct vhost_scsi_target
{
	pub abi_version: c_int,
	pub vhost_wwpn: [c_char; 224usize],
	pub vhost_tpgt: c_ushort,
	pub reserved: c_ushort,
}

impl Default for vhost_scsi_target
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for vhost_scsi_target
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(
			f,
			"vhost_scsi_target {{ vhost_wwpn: [{}] }}",
			self.vhost_wwpn
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
