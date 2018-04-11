// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#![allow(non_upper_case_globals)]
#![deny(missing_docs)]


//! #dpdk-serde
//!
//! A small crate to abstract away dependencies on Serde.


extern crate serde;


include!("serde_pub_enum_u8.rs");
include!("serde_pub_enum_u16.rs");
