// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn rte_eal_devargs_add(devtype: rte_devtype, devargs_str: *const c_char) -> c_int;
	pub fn rte_eal_devargs_dump(f: *mut FILE);
	pub fn rte_eal_devargs_insert(da: *mut rte_devargs) -> c_int;
	pub fn rte_eal_devargs_parse(dev: *const c_char, da: *mut rte_devargs) -> c_int;
	pub fn rte_eal_devargs_remove(busname: *const c_char, devname: *const c_char) -> c_int;
	pub fn rte_eal_devargs_type_count(devtype: rte_devtype) -> c_uint;
	pub fn rte_eal_parse_devargs_str(devargs_str: *const c_char, drvname: *mut *mut c_char, drvargs: *mut *mut c_char) -> c_int;
}
