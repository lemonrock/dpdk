// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct IpV4RoutingTableConfiguration
{
	rules: HashMap<InternetProtocolVersion4NetworkAddress, NextHop>,
	#[serde(deserialize_with = "IpV4RoutingTableConfiguration::serde_deserialize_routes")] routes: Vec<IpV4Route>,
	defaultMaximumTransmissionUnitBeforeVirtualLanAdjustments: MaximumTransmissionUnitSizeInBytes,
}

impl Default for IpV4RoutingTableConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			rules: Default::default(),
			routes: Default::default(),
			defaultMaximumTransmissionUnitBeforeVirtualLanAdjustments: MaximumTransmissionUnitSizeInBytes::EthernetV2,
		}
	}
}

impl IpV4RoutingTableConfiguration
{
	fn serde_deserialize_routes<D: Deserializer>(deserializer: D) -> Result<Vec<IpV4Route>, D::Error>
	{
		let result = Vec::<IpV4Route>::deserialize(deserializer)?;
		if result.is_empty()
		{
			Err(D::Error::custom("There must be at least one route"))
		}
		else
		{
			Ok(result)
		}
	}
	
	pub fn defaultMaximumTransmissionUnitAfterVirtualLanAdjustments(&self, virtualLanSizeCorrection: u16) -> MaximumTransmissionUnitSizeInBytes
	{
		self.defaultMaximumTransmissionUnitBeforeVirtualLanAdjustments.decreaseBy(virtualLanSizeCorrection)
	}
	
	pub fn reconfigure(&self, ipV4RoutingTable: &mut IpV4RoutingTable, virtualLanSizeCorrection: u16)
	{
		let mut routes = Vec::with_capacity(self.routes.len());
		for route in self.routes.iter()
		{
			routes.push(route.decreaseBy(virtualLanSizeCorrection));
		}
		
		ipV4RoutingTable.reconfigureRulesAndRoutes(&self.rules, &routes.as_slice())
	}
}
