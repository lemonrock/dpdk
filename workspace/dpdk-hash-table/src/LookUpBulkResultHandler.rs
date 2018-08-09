// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Handles results of a bulk look up.
pub trait LookUpBulkResultHandler<Key: Copy + Sized + Hash, Value: Sized>
{
	/// Key found.
	#[inline(always)]
	fn key_found<'a>(&'a mut self, keys: &ArrayVec<[&Key; LookUpBulkMaximum]>, index: usize, value: Value);
	
	/// Key not present.
	#[inline(always)]
	fn key_not_present<'a>(&'a mut self, keys: &ArrayVec<[&Key; LookUpBulkMaximum]>, index: usize);
}
