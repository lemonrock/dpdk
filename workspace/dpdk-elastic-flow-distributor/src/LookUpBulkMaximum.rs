// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Maximum number of keys that can be looked up at once.
///
/// Does not exist as an associated constant in ElasticFlowDistributorTable because Rust is weird when referencering associated constants in function array arguments in structs with generic parameters.
pub const LookUpBulkMaximum: usize = RTE_EFD_BURST_MAX as usize;
