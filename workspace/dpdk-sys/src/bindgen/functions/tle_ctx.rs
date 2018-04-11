// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn tle_add_dev(ctx: *mut tle_ctx, dev_prm: *const tle_dev_param) -> *mut tle_dev;
	pub fn tle_ctx_create(ctx_prm: *const tle_ctx_param) -> *mut tle_ctx;
	pub fn tle_ctx_destroy(ctx: *mut tle_ctx);
	pub fn tle_ctx_invalidate(ctx: *mut tle_ctx);
	pub fn tle_del_dev(dev: *mut tle_dev) -> c_int;
}
