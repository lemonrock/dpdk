// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


//// Becomes an rte_flow_item instance.
//pub enum PatternItem<PM: PatternMatcher>
//{
//	Unspecified,
//
//	Specified
//	{
//		example: PM::Example,
//
//		// Problematic. Similar to the example but a mask of bits to apply from the example.
//		// We could make this PatternMatcher, have it take an example.
//		bitmask: (),
//	},
//
//	SpecifiedWithRange
//	{
//		// from
//		example: PM::Example,
//
//		// to; must always be >= from
//		last: PM::Example,
//
//
//		bitmask: (),
//	}
//}
