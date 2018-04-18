// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct IpV4Route
{
	router: InternetProtocolVersion4HostAddress,
	tldkMtu: MaximumTransmissionUnitSizeInBytes,
}

impl Default for IpV4Route
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			router: 0, // So wrong it's untrue...
			tldkMtu: MaximumTransmissionUnitSizeInBytes::TldkValue,
		}
	}
}

impl IpV4Route
{
	#[inline(always)]
	pub fn decrease_by(&self, virtualLanSizeCorrection: u16) -> Self
	{
		Self
		{
			router: self.router,
			tldkMtu: self.tldkMtu.decrease_by(virtualLanSizeCorrection)
		}
	}
}
