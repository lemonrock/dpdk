// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct DifferentiatedServiceCodePoint(pub u8);

impl Default for DifferentiatedServiceCodePoint
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::DefaultForwarding
	}
}

impl DifferentiatedServiceCodePoint
{
	// AKA Best Effort
	pub const DefaultForwarding: DifferentiatedServiceCodePoint = DifferentiatedServiceCodePoint(0b000000);
	
	pub const ExpeditedForwarding: DifferentiatedServiceCodePoint = DifferentiatedServiceCodePoint(0b101110);
	
	pub const VoiceAdmit: DifferentiatedServiceCodePoint = DifferentiatedServiceCodePoint(0b101100);
	
	// AKA AF11
	pub const AssuredForwardingClass1LowDropProbability: DifferentiatedServiceCodePoint = DifferentiatedServiceCodePoint(0b001010);
	
	// AKA AF12
	pub const AssuredForwardingClass1MediumDropProbability: DifferentiatedServiceCodePoint = DifferentiatedServiceCodePoint(0b001100);
	
	// AKA AF13
	pub const AssuredForwardingClass1HighDropProbability: DifferentiatedServiceCodePoint = DifferentiatedServiceCodePoint(0b001110);
	
	// AKA AF21
	pub const AssuredForwardingClass2LowDropProbability: DifferentiatedServiceCodePoint = DifferentiatedServiceCodePoint(0b010010);
	
	// AKA AF22
	pub const AssuredForwardingClass2MediumDropProbability: DifferentiatedServiceCodePoint = DifferentiatedServiceCodePoint(0b010100);
	
	// AKA AF23
	pub const AssuredForwardingClass2HighDropProbability: DifferentiatedServiceCodePoint = DifferentiatedServiceCodePoint(0b010110);
	
	// AKA AF31
	pub const AssuredForwardingClass3LowDropProbability: DifferentiatedServiceCodePoint = DifferentiatedServiceCodePoint(0b011010);
	
	// AKA AF32
	pub const AssuredForwardingClass3MediumDropProbability: DifferentiatedServiceCodePoint = DifferentiatedServiceCodePoint(0b011100);
	
	// AKA AF43
	pub const AssuredForwardingClass3HighDropProbability: DifferentiatedServiceCodePoint = DifferentiatedServiceCodePoint(0b011110);
	
	// AKA AF41
	pub const AssuredForwardingClass4LowDropProbability: DifferentiatedServiceCodePoint = DifferentiatedServiceCodePoint(0b100010);
	
	// AKA AF42
	pub const AssuredForwardingClass4MediumDropProbability: DifferentiatedServiceCodePoint = DifferentiatedServiceCodePoint(0b100100);
	
	// AKA AF43
	pub const AssuredForwardingClass4HighDropProbability: DifferentiatedServiceCodePoint = DifferentiatedServiceCodePoint(0b100110);
	
	// ?
	pub const InterNetworkControl: DifferentiatedServiceCodePoint = DifferentiatedServiceCodePoint(0b110000);
	
	// ?
	pub const NetworkControl: DifferentiatedServiceCodePoint = DifferentiatedServiceCodePoint(0b111000);
}
