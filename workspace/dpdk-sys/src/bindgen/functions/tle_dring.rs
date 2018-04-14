// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn rust___tle_dring_copy_objs(dst: *mut *const c_void, src: *const *const c_void, num: u32);
	pub fn rust___tle_dring_dequeue(dr: *mut tle_dring, head: u32, objs: *mut *const c_void, nb_obj: u32, drbs: *mut *mut tle_drb, nb_drb: u32) -> u32;
	pub fn rust___tle_dring_enqueue(dr: *mut tle_dring, head: u32, objs: *const *const c_void, nb_obj: u32, drbs: *mut *mut tle_drb, nb_drb: u32) -> u32;
	pub fn tle_dring_dump(f: *mut FILE, verb: i32, dr: *const tle_dring);
}
