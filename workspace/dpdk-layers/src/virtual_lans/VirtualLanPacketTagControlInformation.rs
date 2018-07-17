// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Tag control information.
#[repr(C, packed)]
pub struct VirtualLanPacketTagControlInformation(pub NetworkByteOrderEndianU16);

impl VirtualLanPacketTagControlInformation
{
	/// Parse.
	#[inline(always)]
	pub fn parse(self) -> Result<(ClassOfService, DropEligibleIndicator, Option<VirtualLanIdentifier>), ()>
	{
		let value = self.0.to_native_byte_order_value();
		
		let virtual_lan_identifier =
		{
			match value & 0x0FFF
			{
				0 => None,
				0xFFF => return Err(()),
				valid @ _ => Some(VirtualLanIdentifier(valid))
			}
		};
		
		let class_of_service = unsafe { transmute((value & 0b1110_0000_0000_0000 >> 13) as u8) };
		let drop_eligible_indicator = value & 0b0001_0000_0000_0000 == 0b0001_0000_0000_0000;
		Ok((class_of_service, drop_eligible_indicator, virtual_lan_identifier))
	}
}
