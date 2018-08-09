// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Specification for an `PacketMatcher::AddressResolutionProtocolForInternetProtocolVersion4OverEthernet`.
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct VirtualLanHeaderMask
{
	/// Tag Control Information (TCI) mask.
	pub tag_control_information: NetworkEndianU16,
	
	/// Ether Type or legacy ethernet frame size mask.
	pub ether_type_or_legacy_ethernet_frame_size: NetworkEndianU16,
}

impl MaskedPacketMatcher for VirtualLanHeaderMask
{
	type Type = rte_flow_item_vlan;
}

impl Mask for VirtualLanHeaderMask
{
	#[inline(always)]
	fn dpdk_mask(&self) -> &<Self as MaskedPacketMatcher>::Type
	{
		unsafe { transmute(self) }
	}
}
