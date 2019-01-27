// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use super::*;


include!("adjust_performance_events.rs");
include!("adjust_transparent_huge_pages.rs");
include!("disable_dumpable.rs");
include!("enable_strict_sec_comp.rs");
include!("enable_tsc_clock.rs");
include!("lock_secure_bits_and_remove_ambient_capability_raise_and_keep_capabilities.rs");
include!("no_new_privileges.rs");
