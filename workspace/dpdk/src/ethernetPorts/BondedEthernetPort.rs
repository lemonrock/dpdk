// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BondedEthernetPort
{
	name: Option<CString>,
	ethernetPort: EthernetPort,
}

impl Drop for BondedEthernetPort
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if let Some(ref name) = self.name
		{
			match unsafe { rte_eth_bond_free(name.as_ptr()) }
			{
				0 => (),

				error if error < 0 => (),

				illegal @ _ => panic!("Unexpected return code '{}' from rte_eth_bond_free()", illegal),
			}
		}
	}
}

impl BondedEthernetPort
{
	#[inline(always)]
	pub fn underlying_ethernet_port(&self) -> EthernetPort
	{
		self.ethernetPort
	}

	#[inline(always)]
	pub fn fromEthernetPort(ethernetPort: EthernetPort) -> Option<BondedEthernetPort>
	{
		let deviceInformation = ethernetPort.underlying_ethernet_device();
		let data = unsafe { *(deviceInformation.data) };
		let driverName = unsafe { CStr::from_ptr(data.drv_name) };
		match driverName.to_str()
		{
			Err(_) => None,

			Ok(value) => match value
			{
				"rte_bond_pmd" => Some
				(
					BondedEthernetPort
					{
						name: None,
						ethernetPort,
					}
				),

				_ => None,
			}
		}
	}

	#[inline(always)]
	pub fn create(name: &str, bondingMode: BondingMode, numa_socket_id: NumaSocketId) -> Result<BondedEthernetPort, i32>
	{
		let name = CString::new(name).unwrap();;

		match unsafe { rte_eth_bond_create(name.as_ptr(), bondingMode as u8, numa_socket_id.as_u8()) }
		{
			portIdentifier if portIdentifier >= 0 && portIdentifier <= ::std::u8::MAX as i32 => Ok
			(
				BondedEthernetPort
				{
					name: Some(name),
					ethernetPort: EthernetPort::new(portIdentifier as u8).unwrap(),
				}
			),

			errorCode if errorCode < 0 => Err(errorCode),

			illegal @ _ => panic!("rte_eth_bond_create() returned an invalid positive value '{}'", illegal),
		}
	}

	#[inline(always)]
	pub fn addSlave(&self, ethernetPort: EthernetPort) -> Result<(), i32>
	{
		match unsafe { rte_eth_bond_slave_add(self.ethernetPort.portIdentifier(), ethernetPort.portIdentifier) }
		{
			0 => Ok(()),

			error if error < 0 => Err(error),

			illegal @ _ => panic!("Unexpected return code '{}' from rte_eth_bond_slave_add()", illegal),
		}
	}

	#[inline(always)]
	pub fn removeSlave(&self, ethernetPort: EthernetPort) -> Result<(), i32>
	{
		match unsafe { rte_eth_bond_slave_remove(self.ethernetPort.portIdentifier(), ethernetPort.portIdentifier) }
		{
			0 => Ok(()),

			error if error < 0 => Err(error),

			illegal @ _ => panic!("Unexpected return code '{}' from rte_eth_bond_slave_remove()", illegal),
		}
	}

	#[inline(always)]
	pub fn setPrimarySlave(&self, ethernetPort: EthernetPort) -> Result<(), i32>
	{
		match unsafe { rte_eth_bond_primary_set(self.ethernetPort.portIdentifier(), ethernetPort.portIdentifier) }
		{
			0 => Ok(()),

			error if error < 0 => Err(error),

			illegal @ _ => panic!("Unexpected return code '{}' from rte_eth_bond_primary_set()", illegal),
		}
	}

	#[inline(always)]
	pub fn getPrimarySlave(&self) -> Option<EthernetPort>
	{
		match unsafe { rte_eth_bond_primary_get(self.ethernetPort.portIdentifier()) }
		{
			primarySlavePortIdentifier if primarySlavePortIdentifier >= 0 && primarySlavePortIdentifier <= ::std::u8::MAX as i32 => Some(EthernetPort { portIdentifier: primarySlavePortIdentifier as u8 }),

			-1 => None,

			illegal @ _ => panic!("Unexpected return code '{}' from rte_eth_bond_slave_add()", illegal),
		}
	}

	#[inline(always)]
	pub fn setBondingMode(&self, bondingMode: BondingMode) -> Result<(), i32>
	{
		match unsafe { rte_eth_bond_mode_set(self.ethernetPort.portIdentifier(), bondingMode as u8) }
		{
			0 => Ok(()),

			error if error < 0 => Err(error),

			illegal @ _ => panic!("Unexpected return code '{}' from rte_eth_bond_mode_set()", illegal),
		}
	}

	#[inline(always)]
	pub fn getBondingMode(&self) -> Result<BondingMode, i32>
	{
		match unsafe { rte_eth_bond_mode_get(self.ethernetPort.portIdentifier()) }
		{
			0 => Ok(BondingMode::RoundRobin),
			1 => Ok(BondingMode::ActiveBackup),
			2 => Ok(BondingMode::Balance),
			3 => Ok(BondingMode::Broadcast),
			4 => Ok(BondingMode::Lacp),
			5 => Ok(BondingMode::AdaptiveTransmitLoadBalancing),
			6 => Ok(BondingMode::AdaptiveLoadBalancing),

			error if error < 0 => Err(error),

			illegal @ _ => panic!("Unexpected return code '{}' from rte_eth_bond_mode_get()", illegal),
		}
	}

	#[inline(always)]
	pub fn getAllSlaves(&self) -> Vec<EthernetPort>
	{
		const length: u8 = ::std::u8::MAX;
		let mut slaves: Vec<EthernetPort> = Vec::with_capacity(length as usize);

		match unsafe { rte_eth_bond_slaves_get(self.ethernetPort.portIdentifier(), slaves.as_mut_ptr() as *mut u8, length) }
		{
			actualLength if actualLength >=0 && actualLength <= length as i32 =>
			{
				unsafe
				{
					slaves.set_len(actualLength as usize);
				}
				slaves.shrink_to_fit();
				slaves
			},

			error if error < 0 => panic!("Some sort of error calling rte_eth_bond_slaves_get(), '{}'", error),

			illegal @ _ => panic!("Unexpected result code '{}' from rte_eth_bond_slaves_get()", illegal),
		}
	}

	#[inline(always)]
	pub fn getAllActiveSlaves(&self) -> Vec<EthernetPort>
	{
		const length: u8 = ::std::u8::MAX;
		let mut slaves: Vec<EthernetPort> = Vec::with_capacity(length as usize);

		match unsafe { rte_eth_bond_active_slaves_get(self.ethernetPort.portIdentifier(), slaves.as_mut_ptr() as *mut u8, length) }
		{
			actualLength if actualLength >=0 && actualLength <= length as i32 =>
			{
				unsafe
				{
					slaves.set_len(actualLength as usize);
				}
				slaves.shrink_to_fit();
				slaves
			},

			error if error < 0 => panic!("Some sort of error calling rte_eth_bond_active_slaves_get(), '{}'", error),

			illegal @ _ => panic!("Unexpected result code '{}' from rte_eth_bond_active_slaves_get()", illegal),
		}
	}

	#[inline(always)]
	pub fn setMediaAccessControlAddress(&self, media_access_control_address: MediaAccessControlAddress)
	{
		let mut value = media_access_control_address.0;
		match unsafe { rte_eth_bond_mac_address_set(self.ethernetPort.portIdentifier(), &mut value) }
		{
			0 => (),

			error if error < 0 => panic!("Some sort of error calling rte_eth_bond_mac_address_set(), '{}'", error),

			illegal @ _ => panic!("Unexpected result code '{}' from rte_eth_bond_mac_address_set()", illegal),
		}
	}

	#[inline(always)]
	pub fn resetMediaAccessControlAddressToPrimarySlaves(&self)
	{
		match unsafe { rte_eth_bond_mac_address_reset(self.ethernetPort.portIdentifier()) }
		{
			0 => (),

			error if error < 0 => panic!("Some sort of error calling rte_eth_bond_mac_address_reset(), '{}'", error),

			illegal @ _ => panic!("Unexpected result code '{}' from rte_eth_bond_mac_address_reset()", illegal),
		}
	}

	#[inline(always)]
	pub fn setBalanceBondingModeTransmitPolicy(&self, balanceBondingModeTransmitPolicy: BalanceBondingModeTransmitPolicy) -> Result<(), i32>
	{
		match unsafe { rte_eth_bond_xmit_policy_set(self.ethernetPort.portIdentifier(), balanceBondingModeTransmitPolicy as u8) }
		{
			0 => Ok(()),

			error if error < 0 => Err(error),

			illegal @ _ => panic!("Unexpected return code '{}' from rte_eth_bond_xmit_policy_set()", illegal),
		}
	}

	#[inline(always)]
	pub fn getBalanceBondingModeTransmitPolicy(&self) -> Result<BalanceBondingModeTransmitPolicy, i32>
	{
		match unsafe { rte_eth_bond_xmit_policy_get(self.ethernetPort.portIdentifier()) }
		{
			0 => Ok(BalanceBondingModeTransmitPolicy::Layer2Only),
			1 => Ok(BalanceBondingModeTransmitPolicy::Layers2And3),
			2 => Ok(BalanceBondingModeTransmitPolicy::Layers3And4),

			error if error < 0 => Err(error),

			illegal @ _ => panic!("Unexpected return code '{}' from rte_eth_bond_xmit_policy_get()", illegal),
		}
	}

	#[inline(always)]
	pub fn setLinkStatusMonitoringFrequency(&self, milliseconds: u31) -> Result<(), i32>
	{
		match unsafe { rte_eth_bond_link_monitoring_set(self.ethernetPort.portIdentifier(), milliseconds) }
		{
			0 => Ok(()),

			error if error < 0 => Err(error),

			illegal @ _ => panic!("Unexpected return code '{}' from rte_eth_bond_link_monitoring_set()", illegal),
		}
	}

	#[inline(always)]
	pub fn getLinkStatusMonitoringFrequency(&self) -> Result<u31, i32>
	{
		match unsafe { rte_eth_bond_link_monitoring_get(self.ethernetPort.portIdentifier()) }
		{
			milliseconds if milliseconds >= 0 => Ok(milliseconds as u31),

			error @ _ => Err(error),
		}
	}

	#[inline(always)]
	pub fn setLinkDownDelayBeforeDisabling(&self, milliseconds: u31) -> Result<(), i32>
	{
		match unsafe { rte_eth_bond_link_down_prop_delay_set(self.ethernetPort.portIdentifier(), milliseconds) }
		{
			0 => Ok(()),

			error if error < 0 => Err(error),

			illegal @ _ => panic!("Unexpected return code '{}' from rte_eth_bond_link_down_prop_delay_set()", illegal),
		}
	}

	#[inline(always)]
	pub fn getLinkDownDelayBeforeDisabling(&self) -> Result<u31, i32>
	{
		match unsafe { rte_eth_bond_link_down_prop_delay_get(self.ethernetPort.portIdentifier()) }
		{
			milliseconds if milliseconds >= 0 => Ok(milliseconds as u31),

			error @ _ => Err(error),
		}
	}

	#[inline(always)]
	pub fn setLinkUpDelayBeforeEnabling(&self, milliseconds: u31) -> Result<(), i32>
	{
		match unsafe { rte_eth_bond_link_up_prop_delay_set(self.ethernetPort.portIdentifier(), milliseconds) }
		{
			0 => Ok(()),

			error if error < 0 => Err(error),

			illegal @ _ => panic!("Unexpected return code '{}' from rte_eth_bond_link_up_prop_delay_set()", illegal),
		}
	}

	#[inline(always)]
	pub fn getLinkUpDelayBeforeEnabling(&self) -> Result<u31, i32>
	{
		match unsafe { rte_eth_bond_link_up_prop_delay_get(self.ethernetPort.portIdentifier()) }
		{
			milliseconds if milliseconds >= 0 => Ok(milliseconds as u31),

			error @ _ => Err(error),
		}
	}
}
