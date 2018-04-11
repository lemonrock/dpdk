// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn rust_tle_dring_count(dr: *const tle_dring) -> u32;
	pub fn rust_tle_dring_dequeue(dr: *mut tle_dring, objs: *mut *const c_void, nb_obj: u32, drbs: *mut *mut tle_drb, nb_drb: *mut u32) -> u32;
	pub fn rust_tle_dring_enqueue(dr: *mut tle_dring, objs: *const *const c_void, nb_obj: u32, drbs: *mut *mut tle_drb, nb_drb: *mut u32) -> u32;
	pub fn rust_tle_dring_mc_dequeue(dr: *mut tle_dring, objs: *mut *const c_void, nb_obj: u32, drbs: *mut *mut tle_drb, nb_drb: *mut u32) -> u32;
	pub fn rust_tle_dring_mp_enqueue(dr: *mut tle_dring, objs: *const *const c_void, nb_obj: u32, drbs: *mut *mut tle_drb, nb_drb: *mut u32) -> u32;
	pub fn rust_tle_dring_reset(dr: *mut tle_dring, flags: u32);
	pub fn rust_tle_dring_sc_dequeue(dr: *mut tle_dring, objs: *mut *const c_void, nb_obj: u32, drbs: *mut *mut tle_drb, nb_drb: *mut u32) -> u32;
	pub fn rust_tle_dring_sp_enqueue(dr: *mut tle_dring, objs: *const *const c_void, nb_obj: u32, drbs: *mut *mut tle_drb, nb_drb: *mut u32) -> u32;
}
