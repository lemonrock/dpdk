// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	#[link_name = "\u{1}rte_cryptodev_globals"] pub static mut rte_cryptodev_globals: *mut rte_cryptodev_global;
	#[link_name = "\u{1}rte_cryptodevs"] pub static mut rte_cryptodevs: *mut rte_cryptodev;
}
