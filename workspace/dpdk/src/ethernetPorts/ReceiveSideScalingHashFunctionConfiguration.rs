// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReceiveSideScalingHashFunctionConfiguration
{
	pub key: ReceiveSideScalingHashFunctionKeyData,
	pub hashFunctionFlowApplicability: ReceiveSideScalingOffloadFlowTypeSet
}

impl Default for ReceiveSideScalingHashFunctionConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::TcpEthernetDeviceDefault
	}
}

impl ReceiveSideScalingHashFunctionConfiguration
{
	pub const NoneEthernetDeviceDefault: ReceiveSideScalingHashFunctionConfiguration = ReceiveSideScalingHashFunctionConfiguration
	{
		key: ReceiveSideScalingHashFunctionKeyData::EthernetDeviceDefaultKeyData,
		hashFunctionFlowApplicability: ReceiveSideScalingOffloadFlowTypeSet::empty(),
	};
	
	pub const UdpEthernetDeviceDefault: ReceiveSideScalingHashFunctionConfiguration = ReceiveSideScalingHashFunctionConfiguration
	{
		key: ReceiveSideScalingHashFunctionKeyData::EthernetDeviceDefaultKeyData,
		hashFunctionFlowApplicability: ReceiveSideScalingOffloadFlowTypeSet::Udp,
	};
	
	pub const TcpEthernetDeviceDefault: ReceiveSideScalingHashFunctionConfiguration = ReceiveSideScalingHashFunctionConfiguration
	{
		key: ReceiveSideScalingHashFunctionKeyData::EthernetDeviceDefaultKeyData,
		hashFunctionFlowApplicability: ReceiveSideScalingOffloadFlowTypeSet::Tcp,
	};
	
	pub const AllEthernetDeviceDefault: ReceiveSideScalingHashFunctionConfiguration = ReceiveSideScalingHashFunctionConfiguration
	{
		key: ReceiveSideScalingHashFunctionKeyData::EthernetDeviceDefaultKeyData,
		hashFunctionFlowApplicability: ReceiveSideScalingOffloadFlowTypeSet::ProtocolMask,
	};
	
	pub const AllSymmetricForty: ReceiveSideScalingHashFunctionConfiguration = ReceiveSideScalingHashFunctionConfiguration
	{
		key: ReceiveSideScalingHashFunctionKeyData::SymmetricFortyKeyData,
		hashFunctionFlowApplicability: ReceiveSideScalingOffloadFlowTypeSet::ProtocolMask,
	};
	
	// This is a GUESS based on the above
	pub const AllSymmetricFiftyTwo: ReceiveSideScalingHashFunctionConfiguration = ReceiveSideScalingHashFunctionConfiguration
	{
		key: ReceiveSideScalingHashFunctionKeyData::SymmetricFiftyTwoKeyData,
		hashFunctionFlowApplicability: ReceiveSideScalingOffloadFlowTypeSet::ProtocolMask,
	};
	
	pub const AllMellanoxForty: ReceiveSideScalingHashFunctionConfiguration = ReceiveSideScalingHashFunctionConfiguration
	{
		key: ReceiveSideScalingHashFunctionKeyData::DefaultMellanoxKeyData,
		hashFunctionFlowApplicability: ReceiveSideScalingOffloadFlowTypeSet::ProtocolMask,
	};
	
	pub const AllIntelFiftyTwo: ReceiveSideScalingHashFunctionConfiguration = ReceiveSideScalingHashFunctionConfiguration
	{
		key: ReceiveSideScalingHashFunctionKeyData::DefaultIntelI40eKeyData,
		hashFunctionFlowApplicability: ReceiveSideScalingOffloadFlowTypeSet::ProtocolMask,
	};
	
	pub const AllDefaultFiftyTwo: ReceiveSideScalingHashFunctionConfiguration = ReceiveSideScalingHashFunctionConfiguration
	{
		key: ReceiveSideScalingHashFunctionKeyData::EthernetDeviceDefaultKeyData,
		hashFunctionFlowApplicability: ReceiveSideScalingOffloadFlowTypeSet::ProtocolMask,
	};
		
	pub fn as_rte_eth_rss_conf(&self) -> rte_eth_rss_conf
	{
		let data = self.key.as_ref();
		
		rte_eth_rss_conf
		{
			rss_key: match data
			{
				None => null_mut(),
				Some(slice) => slice.as_ptr() as *mut _,
			},
			rss_key_len: self.key.length(),
			rss_hf: self.hashFunctionFlowApplicability.bits()
		}
	}
}
