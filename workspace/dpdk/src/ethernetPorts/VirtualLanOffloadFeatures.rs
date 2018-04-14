// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk; including this file; may be copied; modified; propagated; or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


bitflags!
{
	pub struct VirtualLanOffloadFeatures: i32
	{
		const Strip = ETH_VLAN_STRIP_OFFLOAD;
		const Filter = ETH_VLAN_FILTER_OFFLOAD;
		const Extend = ETH_VLAN_EXTEND_OFFLOAD;
	}
}

impl Default for VirtualLanOffloadFeatures
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::empty()
	}
}
