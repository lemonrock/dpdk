// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct IpV4AndOrIpV6<A, B>
{
	ipV4: Option<A>,
	ipV6: Option<B>,
}

impl<A, B> IpV4AndOrIpV6<A, B>
{
	#[inline(always)]
	pub fn both(ipV4: A, ipV6: B) -> Self
	{
		Self
		{
			ipV4: Some(ipV4),
			ipV6: Some(ipV6),
		}
	}
	
	#[inline(always)]
	pub fn onlyIpV4(ipV4: A) -> Self
	{
		Self
		{
			ipV4: Some(ipV4),
			ipV6: None,
		}
	}
	
	#[inline(always)]
	pub fn onlyIpV6(ipV6: B) -> Self
	{
		Self
		{
			ipV4: None,
			ipV6: Some(ipV6),
		}
	}
	
	#[inline(always)]
	pub fn ipV4(&self) -> Option<&A>
	{
		self.ipV4.as_ref()
	}
	
	#[inline(always)]
	pub fn ipV6(&self) -> Option<&B>
	{
		self.ipV6.as_ref()
	}
	
	#[inline(always)]
	pub fn hasIpV4(&self) -> bool
	{
		self.ipV4.is_some()
	}
	
	#[inline(always)]
	pub fn hasIpV6(&self) -> bool
	{
		self.ipV6.is_some()
	}
}

impl IpV4AndOrIpV6<Ipv4Addr, Ipv6Addr>
{
	#[inline(always)]
	pub fn oneOrTheOther(internet_protocol_address: IpAddr) -> Self
	{
		match internet_protocol_address
		{
			IpAddr::V4(value) => Self::onlyIpV4(value),
			IpAddr::V6(value) => Self::onlyIpV6(value),
		}
	}
}
