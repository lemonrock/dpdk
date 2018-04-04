// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct IpFragmentationConfiguration
{
	numberOfBuckets: u16,
	#[serde(serialize_with = "IpFragmentationConfiguration::serde_serialize_entriesPerBucket", deserialize_with = "IpFragmentationConfiguration::serde_deserialize_entriesPerBucket")] entriesPerBucket: PowerOfTwoSixteenBit,
	maximumFlowTimeToLiveInMilliseconds: u64
}

impl Default for IpFragmentationConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			numberOfBuckets: 4094,
			entriesPerBucket: PowerOfTwoSixteenBit::_8,
			maximumFlowTimeToLiveInMilliseconds: 100 * 1000, // 100 seconds
		}
	}
}

impl IpFragmentationConfiguration
{
	fn serde_serialize_entriesPerBucket<S: Serializer>(value: &PowerOfTwoSixteenBit, serializer: S) -> Result<S::Ok, S::Error>
	{
		let underlying = value as u16;
		underlying.serialize(serializer)
	}
	
	fn serde_deserialize_entriesPerBucket<D: Deserializer>(deserializer: D) -> Result<PowerOfTwoSixteenBit, D::Error>
	{
		let underlying = u16::deserialize(deserializer)?;
		if PowerOfTwoSixteenBit::isPowerOfTwo(underlying)
		{
			Ok(PowerOfTwoSixteenBit::from_u16_unchecked(underlying))
		}
		else
		{
			Err(D::Error::custom("Underlying value as not a power of two between 1 and 32768 inclusive"))
		}
	}
	
	pub fn create(&self, logicalCoreMemorySocket: Option<NumaSocketId>) -> IpV4PacketReassemblyTable
	{
		IpV4PacketReassemblyTable::create(self.numberOfBuckets, self.entriesPerBucket, self.maximumFlowTimeToLiveInMilliseconds, logicalCoreMemorySocket).expect("Could not allocate enough memory")
	}
}
