// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VirtualLanTrafficClassIndicator
{
	pub virtualLanValue: VirtualLanValue,
	pub virtualLanId: Option<VirtualLanId>
}

impl VirtualLanTrafficClassIndicator
{
	#[inline(always)]
	pub fn writeLayer2HeaderData(&self, buffer: *mut u8, etherType: u16)
	{
		buffer.writeU16AsNetworkByteOrderU16(etherType);
		
		let tci =
		{
			let mut topBits = (self.virtualLanValue.classOfService as u16) << 13;
			if unlikely(self.virtualLanValue.dropEligibleIndicator)
			{
				topBits = topBits | 0x1000;
			}
			topBits | match self.virtualLanId
			{
				None => 0,
				Some(value) => value.0,
			}
		};
		
		const offset: usize = SizeOfEtherType as usize;
		buffer.offsetUp(offset).writeU16AsNetworkByteOrderU16(tci);
	}
}
