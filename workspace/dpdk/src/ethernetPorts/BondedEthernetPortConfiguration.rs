// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BondedEthernetPortConfiguration
{
	pub name: String,
	pub bondingMode: BondingMode,
	pub balanceBondingModeTransmitPolicy: Option<BalanceBondingModeTransmitPolicy>,
	pub media_access_control_address: Option<MediaAccessControlAddress>,
	pub numa_socket_id: NumaSocketId,
	pub linkStatusMonitoringFrequencyInMilliseconds: Option<u31>,
	pub linkDownDelayBeforeDisablingInMilliseconds: Option<u31>,
	pub linkUpDelayBeforeEnablingInMilliseconds: Option<u31>,
	pub slaves: Vec<EthernetPort>, // first slave is used as primary. No way to enfoce uniqueness (rust doesn't have an ordered hash set)
}

impl BondedEthernetPortConfiguration
{
	pub fn configure(self) -> BondedEthernetPort
	{
		debug_assert!(!self.slaves.is_empty(), "slaves can not be empty; there must be at least one");

		let bondedEthernetPort = BondedEthernetPort::create(&self.name, self.bondingMode, self.numa_socket_id).expect("Could not create BondedEthernetPort");

		let mut isFirst = true;
		for slave in self.slaves
		{
			bondedEthernetPort.addSlave(slave).expect("Could not add slave");
			if unlikely(isFirst)
			{
				bondedEthernetPort.setPrimarySlave(slave).expect("Could not set primary slave");
				isFirst = false;
			}
		}

		if let Some(media_access_control_address) = self.media_access_control_address
		{
			bondedEthernetPort.setMediaAccessControlAddress(media_access_control_address);
		}
		else
		{
			bondedEthernetPort.resetMediaAccessControlAddressToPrimarySlaves();
		}

		if let Some(balanceBondingModeTransmitPolicy) = self.balanceBondingModeTransmitPolicy
		{
			bondedEthernetPort.setBalanceBondingModeTransmitPolicy(balanceBondingModeTransmitPolicy).expect("Could not set balanceBondingModeTransmitPolicy");
		}

		if let Some(linkStatusMonitoringFrequencyInMilliseconds) = self.linkStatusMonitoringFrequencyInMilliseconds
		{
			bondedEthernetPort.setLinkStatusMonitoringFrequency(linkStatusMonitoringFrequencyInMilliseconds).expect("Could not set linkStatusMonitoringFrequencyInMilliseconds");
		}

		if let Some(linkDownDelayBeforeDisablingInMilliseconds) = self.linkDownDelayBeforeDisablingInMilliseconds
		{
			bondedEthernetPort.setLinkDownDelayBeforeDisabling(linkDownDelayBeforeDisablingInMilliseconds).expect("Could not set linkDownDelayBeforeDisablingInMilliseconds");
		}

		if let Some(linkUpDelayBeforeEnablingInMilliseconds) = self.linkUpDelayBeforeEnablingInMilliseconds
		{
			bondedEthernetPort.setLinkUpDelayBeforeEnabling(linkUpDelayBeforeEnablingInMilliseconds).expect("Could not set linkUpDelayBeforeEnablingInMilliseconds");
		}

		bondedEthernetPort
	}
}
