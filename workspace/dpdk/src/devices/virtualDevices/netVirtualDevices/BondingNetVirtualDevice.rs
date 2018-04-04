// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub struct BondingNetVirtualDevice
{
	index: u5,
	slaves: HashSet<BondingSlave>,
	mode: UsefulBondingMode,
	numaSocketId: NumaSocketId,
	mediaAccessControlAddress: MediaAccessControlAddress,
	lscPollPeriodMilliseconds: u31,
	upDelayMilliseconds: u31,
	downDelayMilliseconds: u31,
}

impl VirtualDevice for BondingNetVirtualDevice
{
	type V = NetVirtualDeviceDriverName;
	
	const DriverName: NetVirtualDeviceDriverName = NetVirtualDeviceDriverName::Bonding;
	
	#[inline(always)]
	fn index(&self) -> u5
	{
		self.index
	}
	
	#[inline(always)]
	fn formattedVirtualDeviceArgumentsWithLeadingComma(&self) -> String
	{
		let mut result = String::with_capacity(256);
		
		for slave in &self.slaves
		{
			result.push_str(&format!(",slave={}", slave.asDpdkString()));
		}
		
		let (bondingMode, activeBackupPrimarySlave, balanceBondingModeTransmitPolicy) = self.mode.clone().modeAndPrimarySlaveAndTransmitPolicy();
		if let Some(activeBackupPrimarySlave) = activeBackupPrimarySlave
		{
			result.push_str(&format!(",primary={}", activeBackupPrimarySlave.asDpdkString()));
		}
		
		result.push_str(&format!(",mode={}", bondingMode as u8));
		
		if let Some(balanceBondingModeTransmitPolicy) = balanceBondingModeTransmitPolicy
		{
			let value = match balanceBondingModeTransmitPolicy
			{
				BalanceBondingModeTransmitPolicy::Layer2Only => "l2",
				BalanceBondingModeTransmitPolicy::Layers2And3 => "l23",
				BalanceBondingModeTransmitPolicy::Layers3And4 => "l34",
			};
			result.push_str(&format!(",xmit_policy={}", value.to_owned()));
		}
		
		result.push_str(&format!(",socket_id={}", self.numaSocketId.as_u8()));
		result.push_str(&format!(",mac={}", self.mediaAccessControlAddress));
		
		
		result.push_str(&format!(",lsc_poll_period_ms={}", self.lscPollPeriodMilliseconds));
		result.push_str(&format!(",up_delay={}", self.lscPollPeriodMilliseconds));
		result.push_str(&format!(",down_delay={}", self.lscPollPeriodMilliseconds));
		
		result
	}
}

impl NetVirtualDevice for BondingNetVirtualDevice
{
}

impl BondingNetVirtualDevice
{
	// Maximum Ethernet Ports is 32 (ie 31 ports), and since this is an ethernet port, there can be only 30 slaves (in theory)
	// In practice, more than 4 makes little sense (as this is typically the maximum number of ports for most ethernet cards)
	// Bonded ports must match in speed and duplex; settings are inherited from first added slave
	// Configuration should happen via the bonded device only
	pub const MaximumSlaves: usize = 31;
	
	pub fn new
	(
		index: u5,
		slaves: HashSet<BondingSlave>,
		mode: UsefulBondingMode,
		numaSocketId: NumaSocketId,
		mediaAccessControlAddress: MediaAccessControlAddress,
		lscPollPeriodMilliseconds: u31,
		upDelayMilliseconds: u31,
		downDelayMilliseconds: u31,
	) -> Self
	{
		assert!(index < VirtualDeviceName::<NetVirtualDeviceDriverName>::MaximumIndex, "index '{}' can not equal or exceed MaximumIndex '{}'", index, VirtualDeviceName::<NetVirtualDeviceDriverName>::MaximumIndex);
		assert!(slaves.len() != 0, "slaves can not be empty");
		assert!(slaves.len() < Self::MaximumSlaves, "slaves '{}' can not equal or exceed MaximumSlaves '{}'", slaves.len(), Self::MaximumSlaves);
		if let Some(hasPrimarySlave) = mode.hasPrimarySlave(&slaves)
		{
			assert!(hasPrimarySlave, "Slaves do not contain Active-Backup primary slave");
		}
		
		BondingNetVirtualDevice
		{
			index: index,
			slaves: slaves,
			mode: mode,
			numaSocketId: numaSocketId,
			mediaAccessControlAddress: mediaAccessControlAddress,
			lscPollPeriodMilliseconds: lscPollPeriodMilliseconds,
			upDelayMilliseconds: upDelayMilliseconds,
			downDelayMilliseconds: downDelayMilliseconds,
		}
	}
}
