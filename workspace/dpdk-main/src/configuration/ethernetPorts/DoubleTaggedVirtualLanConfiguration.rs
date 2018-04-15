// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[serde(default)]
struct DoubleTaggedVirtualLanConfiguration
{
	settings: VirtualLanValue,
	pub innerVirtualLans: HashMap<VirtualLanIdentifier, VirtualLanConfiguration>,
}

impl Default for DoubleTaggedVirtualLanConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			settings: Default::default(),
			innerVirtualLans: Default::default(),
		}
	}
}

impl AppendAdditionalEthernetAddresses for DoubleTaggedVirtualLanConfiguration
{
	#[inline(always)]
	fn appendAdditionalEthernetAddresses(&self, defaultEthernetAddress: &UnicastEthernetAddress, additionalEthernetAddresses: &mut HashSet<UnicastEthernetAddress>)
	{
		self.innerVirtualLans.appendAdditionalEthernetAddresses(defaultEthernetAddress, additionalEthernetAddresses);
	}
}

impl DoubleTaggedVirtualLanConfiguration
{
	pub fn asVirtualLanTrafficClassIndicator(&self, virtual_lan_id: Option<VirtualLanIdentifier>) -> VirtualLanTrafficClassIndicator
	{
		VirtualLanTrafficClassIndicator
		{
			virtual_lan_value: self.settings,
			virtual_lan_id
		}
	}
}
