// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[derive(Hash)]
pub struct vhost_device_ops
{
	pub new_device: Option<unsafe extern "C" fn(vid: c_int) -> c_int>,
	pub destroy_device: Option<unsafe extern "C" fn(vid: c_int)>,
	pub vring_state_changed: Option<unsafe extern "C" fn(vid: c_int, queue_id: u16, enable: c_int) -> c_int>,
	pub features_changed: Option<unsafe extern "C" fn(vid: c_int, features: u64) -> c_int>,
	pub new_connection: Option<unsafe extern "C" fn(vid: c_int) -> c_int>,
	pub destroy_connection: Option<unsafe extern "C" fn(vid: c_int)>,
	pub reserved: [*mut c_void; 2usize],
}

impl Default for vhost_device_ops
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for vhost_device_ops
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "vhost_device_ops {{ new_device: {:?}, destroy_device: {:?}, vring_state_changed: {:?}, features_changed: {:?}, new_connection: {:?}, destroy_connection: {:?}, reserved: {:?} }}", self.new_device, self.destroy_device, self.vring_state_changed, self.features_changed, self.new_connection, self.destroy_connection, self.reserved)
	}
}
