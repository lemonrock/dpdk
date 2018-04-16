// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Allows specification of either an internet protocol (IP) version 4 address or an internet protocol (IP) version 6 address or both.
#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct InternetProtocolVersion4OrVersion6OrBoth<A, B>
{
	internet_protocol_version_4: Option<A>,
	internet_protocol_version_6: Option<B>,
}

impl<A, B> InternetProtocolVersion4OrVersion6OrBoth<A, B>
{
	/// Specify both an internet protocol (IP) version 4 address and an internet protocol (IP) version 6 address.
	#[inline(always)]
	pub fn both(internet_protocol_version_4: A, internet_protocol_version_6: B) -> Self
	{
		Self
		{
			internet_protocol_version_4: Some(internet_protocol_version_4),
			internet_protocol_version_6: Some(internet_protocol_version_6),
		}
	}
	
	/// Specify only an internet protocol (IP) version 4 address.
	#[inline(always)]
	pub fn only_internet_protocol_version_4(internet_protocol_version_4: A) -> Self
	{
		Self
		{
			internet_protocol_version_4: Some(internet_protocol_version_4),
			internet_protocol_version_6: None,
		}
	}
	
	/// Specify only an internet protocol (IP) version 6 address.
	#[inline(always)]
	pub fn only_internet_protocol_version_6(ipV6: B) -> Self
	{
		Self
		{
			internet_protocol_version_4: None,
			internet_protocol_version_6: Some(ipV6),
		}
	}
	
	/// Get the internet protocol (IP) version 4 address if specified.
	#[inline(always)]
	pub fn internet_protocol_version_4(&self) -> Option<&A>
	{
		self.internet_protocol_version_4.as_ref()
	}
	
	/// Get the internet protocol (IP) version 6 address if specified.
	#[inline(always)]
	pub fn internet_protocol_version_6(&self) -> Option<&B>
	{
		self.internet_protocol_version_6.as_ref()
	}
	
	/// Is there an internet protocol (IP) version 4 address specified?
	#[inline(always)]
	pub fn has_internet_protocol_version_4(&self) -> bool
	{
		self.internet_protocol_version_4.is_some()
	}
	
	/// Is there an internet protocol (IP) version 6 address specified?
	#[inline(always)]
	pub fn has_internet_protocol_version_6(&self) -> bool
	{
		self.internet_protocol_version_6.is_some()
	}
}
