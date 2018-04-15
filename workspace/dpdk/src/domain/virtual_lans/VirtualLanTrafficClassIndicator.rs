// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Virtual LAN Traffic Class Indicator (TCI).
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VirtualLanTrafficClassIndicator
{
	/// Virtual LAN value.
	pub virtual_lan_value: VirtualLanValue,
	
	/// Virtual LAN identifier.
	pub virtual_lan_id: Option<VirtualLanIdentifier>
}

impl VirtualLanTrafficClassIndicator
{
	/// Writes Layer 2 header data.
	#[inline(always)]
	pub fn write_layer_2_header_data(&self, buffer: *mut u8, ether_type: u16)
	{
		buffer.writeU16AsNetworkByteOrderU16(ether_type);
		
		let traffic_control_information =
		{
			let mut top_bits = (self.virtual_lan_value.class_of_service as u16) << 13;
			if unlikely(self.virtual_lan_value.drop_eligible_indicator)
			{
				top_bits = top_bits | 0x1000;
			}
			top_bits | match self.virtual_lan_id
			{
				None => 0,
				Some(value) => value.0,
			}
		};
		
		const offset: usize = SizeOfEtherType as usize;
		buffer.offsetUp(offset).writeU16AsNetworkByteOrderU16(traffic_control_information);
	}
}
