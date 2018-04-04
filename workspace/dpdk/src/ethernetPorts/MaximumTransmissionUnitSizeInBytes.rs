// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct MaximumTransmissionUnitSizeInBytes(u16);

impl MaximumTransmissionUnitSizeInBytes
{
	pub const Minimum: MaximumTransmissionUnitSizeInBytes = MaximumTransmissionUnitSizeInBytes(ETHER_MIN_MTU);
	pub const DsLiteOverPPPoEOverEthernetV2: MaximumTransmissionUnitSizeInBytes = MaximumTransmissionUnitSizeInBytes(1452);
	pub const PPPoEOverEthernetV2: MaximumTransmissionUnitSizeInBytes = MaximumTransmissionUnitSizeInBytes(1492);
	pub const EthernetV2WithLlcAndSnap: MaximumTransmissionUnitSizeInBytes = MaximumTransmissionUnitSizeInBytes(1492);
	pub const EthernetV2: MaximumTransmissionUnitSizeInBytes = MaximumTransmissionUnitSizeInBytes(ETHER_MTU);
	pub const TldkValue: MaximumTransmissionUnitSizeInBytes = MaximumTransmissionUnitSizeInBytes(1514);
	pub const MaximumJumboValue: MaximumTransmissionUnitSizeInBytes = MaximumTransmissionUnitSizeInBytes(ETHER_MAX_JUMBO_FRAME_LEN - ETHER_CRC_LEN);

	// Typical values are 1500, 1492 (PPPoE), 1501 - 9198 (Jumbo Frames), 1493 - 9190 (Jumbo Frames PPoE); a safe default is 1500, with 1492 a fallback
	#[inline(always)]
	pub fn new(maximumTransmissionUnitSizeInBytes: u16) -> Self
	{
		MaximumTransmissionUnitSizeInBytes(Self::guard(maximumTransmissionUnitSizeInBytes))
	}
	
	#[inline(always)]
	pub fn newForPPPoE(maximumTransmissionUnitSizeInBytesBeforePPPoEHeader: u16) -> Self
	{
		const PPPoEHeaderLength: u16 = 8;
		
		MaximumTransmissionUnitSizeInBytes(Self::guard(maximumTransmissionUnitSizeInBytesBeforePPPoEHeader - PPPoEHeaderLength))
	}
	
	#[inline(always)]
	pub fn decreaseBy(&self, virtualLanSizeCorrection: u16) -> Self
	{
		MaximumTransmissionUnitSizeInBytes(self.0 - virtualLanSizeCorrection)
	}
	
	#[inline(always)]
	pub fn as_u16(&self) -> u16
	{
		self.0
	}
	
	#[inline(always)]
	fn guard(maximumTransmissionUnitSizeInBytes: u16) -> u16
	{
		assert!(maximumTransmissionUnitSizeInBytes >= ETHER_MIN_MTU, "The maximumTransmissionUnitSizeInBytes, '{}', must be greather than ETHER_MIN_MTU ({})", maximumTransmissionUnitSizeInBytes, ETHER_MIN_MTU);
		assert!(maximumTransmissionUnitSizeInBytes <= ETHER_MAX_JUMBO_FRAME_LEN - ETHER_CRC_LEN, "The maximumTransmissionUnitSizeInBytes, '{}', must be less than (ETHER_MAX_JUMBO_FRAME_LEN ({}) - ETHER_CRC_LEN ({}))", maximumTransmissionUnitSizeInBytes, ETHER_MAX_JUMBO_FRAME_LEN, ETHER_CRC_LEN);
		
		maximumTransmissionUnitSizeInBytes
	}
	
	// conservative, ie consider Jumbo frames as being needed if MTU > 1500
	#[inline(always)]
	fn conservativelyRequiresJumboFrames(&self) -> bool
	{
		self.0 > MaximumTransmissionUnitSizeInBytes::EthernetV2.0
	}
	
	#[inline(always)]
	pub fn conservativeJumboFrameLength(&self) -> Option<u16>
	{
		if self.conservativelyRequiresJumboFrames()
		{
			Some(self.0 + ETHER_CRC_LEN)
		}
		else
		{
			None
		}
	}
}
