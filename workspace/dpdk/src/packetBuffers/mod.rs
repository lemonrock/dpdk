// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use ::dpdk_sys::*;
use ::libc::c_char;
use ::libc::c_void;
use ::libc::FILE;
use ::libc_extra::stderr;
use ::rust_extra::likely;
use ::rust_extra::unlikely;
use ::std::ffi::CString;
use ::std::mem::forget;
use ::std::mem::transmute;
use ::std::mem::uninitialized;
use ::syscall_alt::constants::E;
use ::syscall_alt::constants::NegativeE;
use ::E_RTE;
use ::libc_extra::ffi::isTrue;
use ::logicalCores::AnyNumaSocketId;
use ::logicalCores::NumaSocketId;


include!("PacketBuffer.rs");
include!("PacketBufferPool.rs");
include!("rte_mbuf.macros.rs");
include!("rte_mbufEx.rs");
