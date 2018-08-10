// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Mask for an `Pattern::EthernetHeader`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(C, packed)]
pub struct EthernetHeaderMask
{
	/// Source and destination addresses.
	pub ethernet_addresses: EthernetAddressesMask,
	
	/// Ether Type or legacy ethernet frame size.
	pub ether_type_or_legacy_ethernet_frame_size: NetworkEndianU16,
}

impl MaskedPattern for EthernetHeaderMask
{
	type Type = rte_flow_item_eth;
}

impl Mask for EthernetHeaderMask
{
	#[inline(always)]
	fn dpdk_mask(&self) -> &<Self as MaskedPattern>::Type
	{
		unsafe { transmute(self) }
	}
}
