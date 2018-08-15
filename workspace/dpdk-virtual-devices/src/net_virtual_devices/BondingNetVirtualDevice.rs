// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A bonding net(work) virtual device.
///
/// Bonding slaves can not themselves be a `BondingNetVirtualDevice`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub struct BondingNetVirtualDevice
{
	/// Slaves.
	pub slaves: HashSet<BondingSlave>,
	
	/// Bonding mode.
	#[serde(default)]
	pub mode: UsefulBondingMode,
	
	/// NUMA node for data structures for bonded device.
	#[serde(default)]
	pub numa_node: NumaNode,
	
	/// Media access control (MAC) address.
	pub media_access_control_address: MediaAccessControlAddress,
	
	/// Link status change poll period in milliseconds.
	///
	/// Maximum value is 2^31 - 1.
	pub link_status_change_poll_period_milliseconds: NonZeroU32,
	
	/// Delay coming up in milliseconds.
	///
	/// Maximum value is 2^31 - 1.
	#[serde(default)]
	pub up_delay_milliseconds: u32,
	
	/// Delay going down in milliseconds.
	///
	/// Maximum value is 2^31 - 1.
	#[serde(default)]
	pub down_delay_milliseconds: u32,
}

impl VirtualDevice for BondingNetVirtualDevice
{
	type V = NetVirtualDeviceDriverName;

	const DriverName: NetVirtualDeviceDriverName = NetVirtualDeviceDriverName::Bonding;

	#[inline(always)]
	fn formatted_virtual_device_arguments_with_leading_comma(&self) -> String
	{
		assert!(self.slaves.len() >= 2, "there must be at least two slaves");
		
		if let Some(has_primary_slave) = self.mode.has_primary_slave(&self.slaves)
		{
			assert!(has_primary_slave, "slaves does not contain Active-Backup primary slave");
		}
		
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

		result.push_str(&format!(",lsc_poll_period_ms={}", cap_u32_to_u31(self.link_status_change_poll_period_milliseconds.get())));
		result.push_str(&format!(",up_delay={}", cap_u32_to_u31(self.link_status_change_poll_period_milliseconds.get())));
		result.push_str(&format!(",down_delay={}", cap_u32_to_u31(self.link_status_change_poll_period_milliseconds.get())));

		result
	}
}

impl NetVirtualDevice for BondingNetVirtualDevice
{
}
