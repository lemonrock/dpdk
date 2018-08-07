// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use super::*;


// SIMD requires use of target features which aren't enabled by default.
// /// Memory copying.
// pub mod copying;


/// Memory zones.
pub mod zones;


include!("InputOutputVirtualAddress.rs");
include!("MemoryChannels.rs");
include!("MemoryRanks.rs");
