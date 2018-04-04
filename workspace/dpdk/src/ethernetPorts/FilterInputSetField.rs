// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(u16)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FilterInputSetField
{
	Layer2SourceMediaAccessControlAddress = 1,
	Layer2DestinationMediaAccessControlAddress = 2,
	Layer2OuterVirtualLan = 3,
	Layer2InnerVirtualLan = 4,
	Layer2EtherType = 5,
	Layer3SourceIpV4 = 129,
	Layer3DestinationIpV4 = 130,
	Layer3SourceIpV6 = 131,
	Layer3DestinationIpV6 = 132,
	Layer3IpV4Tos = 133,
	Layer3IpV4Proto = 134,
	Layer3IpV6Tc = 135,
	Layer3IpV6NextHeader = 136,
	Layer3IpV4TimeToLive = 137,
	Layer3IpV6HopLImits = 138,
	Layer4UdpSourcePort = 257,
	Layer4UdpDestinationPort = 258,
	Layer4TcpSourcePort = 259,
	Layer4TcpDestinationPort = 260,
	Layer4SctpSourcePort = 261,
	Layer4SctpDestinationPort = 262,
	Layer4SctpVerificationTag = 263,
	TunnelLayer2InnerDestinationMediaAccessControlAddress = 385,
	TunnelLayer2InnerSourceMediaAccessControlAddress = 386,
	TunnelLayer2InnerVirtualLan = 387,
	TunnelLayer4UdpKey = 388,
	TunnelGreKey = 389,
	FlexPayloadFirstWord = 641,
	FlexPayloadSecondWord = 642,
	FlexPayloadThirdWord = 643,
	FlexPayloadFourthWord = 644,
	FlexPayloadFifthWord = 645,
	FlexPayloadSixthWord = 646,
	FlexPayloadSeventhWord = 647,
	FlexPayloadEighthWord = 648,
	Default = 65533,
	None = 65534,
}

impl FilterInputSetField
{
	#[inline(always)]
	pub fn as_rte_eth_input_set_field(&self) -> rte_eth_input_set_field
	{
		unsafe { transmute(*self as u32) }
	}
}
