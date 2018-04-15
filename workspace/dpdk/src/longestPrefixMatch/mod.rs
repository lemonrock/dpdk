// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use ::dpdk_sys::rte_lpm;
use ::dpdk_sys::rte_lpm_config;
use ::dpdk_sys::rte_lpm6;
use ::dpdk_sys::rte_lpm6_config;
use ::dpdk_sys::rust_rte_errno;
use ::E_RTE;
use ::ethernetPorts::EthernetPortIdentifier;
use ::ethernetPorts::QueueIdentifier;
use ::libc::c_int;
use ::logicalCores::AnyNumaSocketId;
use ::logicalCores::NumaSocketId;
use ::domain::internet_protocol::*::*;
use ::domain::internet_protocol::*;
use ::domain::virtual_lans::VirtualLanKey;
use ::rust_extra::likely;
use ::rust_extra::unlikely;
use ::std::ffi::CString;
use ::std::mem::uninitialized;
use ::syscall_alt::constants::E;
use ::syscall_alt::constants::NegativeE;



include!("IpV4LongestPrefixMatchTable.rs");
include!("IpV6LongestPrefixMatchTable.rs");
include!("LongestPrefixMatchName.rs");
include!("LongestPrefixMatchTable.rs");
include!("NextHop.rs");
