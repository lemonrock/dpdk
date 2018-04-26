// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Ether type or legacy ethernet frame size?
#[repr(C, packed)]
pub union EtherTypeOrLegacyEthernetFrameSize
{
	/// Legacy ethernet frame size.
	pub legacy_ethernet_frame_size: LegacyEthernetFrameSize,
	
	/// Ether Type.
	pub ether_type: EtherType,
}
