// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern crate dpdk_sys;


use ::dpdk_sys::rte_net_crc_calc;
use ::dpdk_sys::rte_net_crc_type;


include!("cyclic_redundancy_check_16_CCITT.rs");
include!("cyclic_redundancy_check_32.rs");
