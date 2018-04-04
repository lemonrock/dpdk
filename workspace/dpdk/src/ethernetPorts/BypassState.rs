// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum BypassState
{
	NormalMode = 1,
	BypassMode = 2,
	IsolateMode = 3,
}

impl BypassState
{
	#[inline(always)]
	fn fromC(bypassStateValue: uint32_t, function: &str) -> BypassState
	{
		match bypassStateValue
		{
			1 => BypassState::NormalMode,
			2 => BypassState::BypassMode,
			3 => BypassState::IsolateMode,
			
			_ => panic!("{}() returned an invalid bypass state value '{}'", function, bypassStateValue),
		}
	}
}
