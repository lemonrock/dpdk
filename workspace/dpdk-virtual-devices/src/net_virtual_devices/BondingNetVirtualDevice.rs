// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A bonding net(work) virtual device.
///
/// Bonding slaves can not themselves be a `BondingNetVirtualDevice`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub struct BondingNetVirtualDevice
{
	slaves: HashSet<BondingSlave>,
	mode: UsefulBondingMode,
	numa_node: NumaNode,
	media_access_control_address: MediaAccessControlAddress,
	lsc_poll_period_milliseconds: u32,
	up_delay_milliseconds: u32,
	down_delay_milliseconds: u32,
}

impl VirtualDevice for BondingNetVirtualDevice
{
	type V = NetVirtualDeviceDriverName;

	const DriverName: NetVirtualDeviceDriverName = NetVirtualDeviceDriverName::Bonding;

	#[inline(always)]
	fn formatted_virtual_device_arguments_with_leading_comma(&self) -> String
	{
		let mut result = String::with_capacity(256);

		for slave in &self.slaves
		{
			result.push_str(&format!(",slave={}", slave.as_dpdk_string()));
		}

		let (bonding_mode, active_back_up_primary_slave, balance_bonding_mode_transmit_policy) = self.mode.clone().mode_and_primary_slave_and_transmit_policy();
		if let Some(active_back_up_primary_slave) = active_back_up_primary_slave
		{
			result.push_str(&format!(",primary={}", active_back_up_primary_slave.as_dpdk_string()));
		}

		result.push_str(&format!(",mode={}", bonding_mode as u8));
		
		use self::BalanceBondingModeTransmitPolicy::*;
		
		if let Some(balance_bonding_mode_transmit_policy) = balance_bonding_mode_transmit_policy
		{
			let value = match balance_bonding_mode_transmit_policy
			{
				Layer2Only => "l2",
				Layers2And3 => "l23",
				Layers3And4 => "l34",
			};
			result.push_str(&format!(",xmit_policy={}", value.to_owned()));
		}
		
		let socket_id: u8 = self.numa_node.into();
		
		result.push_str(&format!(",socket_id={}", socket_id));
		result.push_str(&format!(",mac={}", self.media_access_control_address));

		result.push_str(&format!(",lsc_poll_period_ms={}", self.lsc_poll_period_milliseconds));
		result.push_str(&format!(",up_delay={}", self.lsc_poll_period_milliseconds));
		result.push_str(&format!(",down_delay={}", self.lsc_poll_period_milliseconds));

		result
	}
}

impl NetVirtualDevice for BondingNetVirtualDevice
{
}

impl BondingNetVirtualDevice
{
	/// Maximum Ethernet Ports is 32 (ie 31 ports), and since this is an ethernet port, there can be only 30 slaves (in theory).
	/// In practice, more than 4 makes little sense (as this is typically the maximum number of ports for most ethernet cards).
	/// Bonded ports must match in speed and duplex; settings are inherited from first added slave.
	/// Configuration should happen via the bonded device only.
	pub const MaximumSlaves: usize = 31;
	
	/// Creates a new instance.
	///
	/// `lsc_poll_period_milliseconds`, `up_delay_milliseconds`, and `down_delay_milliseconds` are 31-bit unsigned integers.
	pub fn new(slaves: HashSet<BondingSlave>, mode: UsefulBondingMode, numa_node: NumaNode, media_access_control_address: MediaAccessControlAddress, lsc_poll_period_milliseconds: u32, up_delay_milliseconds: u32, down_delay_milliseconds: u32) -> Self
	{
		assert_ne!(slaves.len(), 0, "slaves can not be empty");
		assert!(slaves.len() < Self::MaximumSlaves, "slaves '{}' can not equal or exceed MaximumSlaves '{}'", slaves.len(), Self::MaximumSlaves);
		if let Some(has_primary_slave) = mode.has_primary_slave(&slaves)
		{
			assert!(has_primary_slave, "Slaves do not contain Active-Backup primary slave");
		}

		Self
		{
			slaves,
			mode,
			numa_node,
			media_access_control_address,
			lsc_poll_period_milliseconds,
			up_delay_milliseconds,
			down_delay_milliseconds,
		}
	}
}
