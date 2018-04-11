// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[derive(Hash)]
pub struct rte_rawdev_ops
{
	pub dev_info_get: rawdev_info_get_t,
	pub dev_configure: rawdev_configure_t,
	pub dev_start: rawdev_start_t,
	pub dev_stop: rawdev_stop_t,
	pub dev_close: rawdev_close_t,
	pub dev_reset: rawdev_reset_t,
	pub queue_def_conf: rawdev_queue_conf_get_t,
	pub queue_setup: rawdev_queue_setup_t,
	pub queue_release: rawdev_queue_release_t,
	pub enqueue_bufs: rawdev_enqueue_bufs_t,
	pub dequeue_bufs: rawdev_dequeue_bufs_t,
	pub dump: rawdev_dump_t,
	pub attr_get: rawdev_get_attr_t,
	pub attr_set: rawdev_set_attr_t,
	pub xstats_get: rawdev_xstats_get_t,
	pub xstats_get_names: rawdev_xstats_get_names_t,
	pub xstats_get_by_name: rawdev_xstats_get_by_name_t,
	pub xstats_reset: rawdev_xstats_reset_t,
	pub firmware_status_get: rawdev_firmware_status_get_t,
	pub firmware_version_get: rawdev_firmware_version_get_t,
	pub firmware_load: rawdev_firmware_load_t,
	pub firmware_unload: rawdev_firmware_unload_t,
	pub dev_selftest: rawdev_selftest_t,
}

impl Default for rte_rawdev_ops
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_rawdev_ops
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_rawdev_ops {{  }}")
	}
}
