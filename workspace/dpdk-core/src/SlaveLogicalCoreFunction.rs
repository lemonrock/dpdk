// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A trait for code to implement to be executable on a slave logical core.
///
/// Will be destroyed after a single call to `execute()`.
pub trait SlaveLogicalCoreFunction
{
	/// Execute code.
	///
	/// Whilst this method uses a mutable reference, the logical core function will be dropped immediately after the first call to `execute()`.
	fn execute(&mut self);
}
