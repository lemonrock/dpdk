// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use super::super::ProcPath;
use ::errno::errno;
use ::libc::getrlimit64;
use ::libc::rlimit64;
use ::libc::rlim64_t;
use ::libc::setrlimit64;
use ::std::collections::HashMap;
use ::std::io;
use ::syscall_alt::constants::E;


include!("ResourceLimit.rs");
include!("ResourceLimitsSet.rs");
include!("SoftAndHardResourceLimit.rs");
include!("ResourceName.rs");
