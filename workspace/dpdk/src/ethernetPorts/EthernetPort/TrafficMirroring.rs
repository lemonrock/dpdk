// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


impl EthernetPort
{
	#[inline(always)]
	fn privateSetTrafficMirroringRule(&self, trafficMirroringRuleNumber: TrafficMirroringRuleNumber, trafficMirroringRule: &TrafficMirroringRule, enable: bool) -> Result<(), UnsupportedByHardwareError>
	{
		let on = if enable
		{
			1
		}
		else
		{
			0
		};
		let mut mirrorConfiguration = trafficMirroringRule.as_rte_eth_mirror_conf();
		let result = unsafe { rte_eth_mirror_rule_set(self.portIdentifier, &mut mirrorConfiguration, trafficMirroringRuleNumber as u8, on) };
		if likely(result == 0)
		{
			Ok(())
		}
		else
		{
			match result
			{
				NegativeE::ENOTSUP => Err(UnsupportedByHardwareError::IsUnsupportedByTheHardware),

				NegativeE::ENODEV => panic!("The port identifier '{}' is invalid", self.portIdentifier()),
				NegativeE::EINVAL => panic!("mirrorConfiguration was invalid"),

				unexpected @ _ => panic!("Unexpected error code '{}' from rte_eth_mirror_rule_set()", unexpected),
			}
		}
	}

	#[inline(always)]
	pub fn enableTrafficMirroringRule(&self, trafficMirroringRuleNumber: TrafficMirroringRuleNumber, trafficMirroringRule: &TrafficMirroringRule) -> Result<(), UnsupportedByHardwareError>
	{
		self.privateSetTrafficMirroringRule(trafficMirroringRuleNumber, trafficMirroringRule, true)
	}

	#[inline(always)]
	pub fn disableTrafficMirroringRule(&self, trafficMirroringRuleNumber: TrafficMirroringRuleNumber, trafficMirroringRule: &TrafficMirroringRule) -> Result<(), UnsupportedByHardwareError>
	{
		self.privateSetTrafficMirroringRule(trafficMirroringRuleNumber, trafficMirroringRule, false)
	}

	#[inline(always)]
	pub fn clearTrafficMirroringRule(&self, trafficMirroringRuleNumber: TrafficMirroringRuleNumber) -> Result<(), UnsupportedByHardwareError>
	{
		let result = unsafe { rte_eth_mirror_rule_reset(self.portIdentifier, trafficMirroringRuleNumber as u8) };
		if likely(result == 0)
		{
			Ok(())
		}
		else
		{
			match result
			{
				NegativeE::ENOTSUP => Err(UnsupportedByHardwareError::IsUnsupportedByTheHardware),

				NegativeE::ENODEV => panic!("The port identifier '{}' is invalid", self.portIdentifier()),
				NegativeE::EINVAL => panic!("mirrorConfiguration was invalid?"),

				unexpected @ _ => panic!("Unexpected error code '{}' from rte_eth_mirror_rule_reset()", unexpected),
			}
		}
	}
}
