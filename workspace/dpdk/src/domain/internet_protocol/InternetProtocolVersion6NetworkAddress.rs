// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// An Internet Protocol (IP) version 6 masked network address.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct InternetProtocolVersion6NetworkAddress
{
	network: InternetProtocolVersion6HostAddress,
	mask_bits: InternetProtocolVersion6MaskBits,
}

impl InternetProtocolNetworkAddress for InternetProtocolVersion6NetworkAddress
{
	type InternetProtocolHostAddress = InternetProtocolVersion6HostAddress;
	
	#[inline(always)]
	fn network(&self) -> &Self::InternetProtocolHostAddress
	{
		&self.network
	}
	
	#[inline(always)]
	fn mask_bits_as_depth(&self) -> u8
	{
		self.mask_bits as u8
	}
}

impl InternetProtocolVersion6NetworkAddress
{
	#[inline(always)]
	pub fn contains(&self, internet_protocol_version_4_host_address: Self::InternetProtocolHostAddress) -> bool
	{
		let network_byte_order_value: u128 = unsafe { transmute(self.network.0) };
		
		network_byte_order_value & (self.mask_bits as u128) != 0
	}
}
