// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


impl EthernetPort
{
	// returns true if successfully initialised, false if could not be
	#[inline(always)]
	pub fn initialiseBypass(&self) -> Result<(), UnsupportedByHardwareError>
	{
		let result = unsafe { rte_eth_dev_bypass_init(self.portIdentifier()) };
		self.bypassResult(result, "rte_eth_dev_bypass_init")
	}

	#[inline(always)]
	pub fn setBypassEventWhenGivenBypassEventOccurs(&self, bypassEvent: BypassEvent, bypassState: BypassState) -> Result<(), UnsupportedByHardwareError>
	{
		let result = unsafe { rte_eth_dev_bypass_event_store(self.portIdentifier(), bypassEvent as uint32_t, bypassState as uint32_t) };
		self.bypassResult(result, "rte_eth_dev_bypass_event_store")
	}

	#[inline(always)]
	pub fn getBypassEventWhenGivenBypassEventOccurs(&self, bypassEvent: BypassEvent) -> Result<BypassState, UnsupportedByHardwareError>
	{
		let mut bypassStateValue = unsafe { uninitialized() };

		let result = unsafe { rte_eth_dev_bypass_event_show(self.portIdentifier(), bypassEvent as uint32_t, &mut bypassStateValue) };
		if likely!(result == 0)
		{
			Ok(BypassState::fromC(bypassStateValue, "rte_eth_dev_bypass_event_show"))
		}
		else
		{
			forget(bypassStateValue);

			self.bypassResultError(result, "rte_eth_dev_bypass_event_show")
		}
	}

	#[inline(always)]
	pub fn setBypassState(&self, bypassState: BypassState) -> Result<(), UnsupportedByHardwareError>
	{
		let mut bypassStateValue = bypassState as uint32_t;

		let result = unsafe { rte_eth_dev_bypass_state_set(self.portIdentifier(), &mut bypassStateValue) };
		self.bypassResult(result, "rte_eth_dev_bypass_state_set")
	}

	#[inline(always)]
	pub fn getBypassState(&self) -> Result<BypassState, UnsupportedByHardwareError>
	{
		let mut bypassStateValue = unsafe { uninitialized() };

		let result = unsafe { rte_eth_dev_bypass_state_show(self.portIdentifier(), &mut bypassStateValue) };
		if likely!(result == 0)
		{
			Ok(BypassState::fromC(bypassStateValue, "rte_eth_dev_bypass_state_show"))
		}
		else
		{
			forget(bypassStateValue);

			self.bypassResultError(result, "rte_eth_dev_bypass_state_show")
		}
	}

	#[inline(always)]
	pub fn getBypassFirmwareVersion(&self) -> Result<BypassFirmwareVersion, UnsupportedByHardwareError>
	{
		let mut bypassFirmwareVersionValue = unsafe { uninitialized() };

		let result = unsafe { rte_eth_dev_bypass_ver_show(self.portIdentifier(), &mut bypassFirmwareVersionValue) };
		if likely!(result == 0)
		{
			Ok(bypassFirmwareVersionValue)
		}
		else
		{
			forget(bypassFirmwareVersionValue);

			self.bypassResultError(result, "rte_eth_dev_bypass_ver_show")
		}
	}

	#[inline(always)]
	pub fn resetBypassWatchdogTimer(&self) -> Result<(), UnsupportedByHardwareError>
	{
		let result = unsafe { rte_eth_dev_bypass_wd_reset(self.portIdentifier()) };
		self.bypassResult(result, "rte_eth_dev_bypass_wd_reset")
	}

	#[inline(always)]
	pub fn getBypassWatchdogTimeout(&self) -> Result<BypassWatchdogTimeout, UnsupportedByHardwareError>
	{
		let mut bypassWatchdogTimeoutValue = unsafe { uninitialized() };

		let result = unsafe { rte_eth_dev_bypass_wd_timeout_show(self.portIdentifier(), &mut bypassWatchdogTimeoutValue) };
		if likely!(result == 0)
		{
			Ok(BypassWatchdogTimeout::fromC(bypassWatchdogTimeoutValue, "rte_eth_dev_bypass_wd_timeout_show"))
		}
		else
		{
			forget(bypassWatchdogTimeoutValue);

			self.bypassResultError(result, "rte_eth_dev_bypass_wd_timeout_show")
		}
	}

	#[inline(always)]
	pub fn setBypassWatchdogTimeout(&self, bypassWatchdogTimeout: BypassWatchdogTimeout) -> Result<(), UnsupportedByHardwareError>
	{
		// DPDK C function seems not to follow the naming convention of all other bypass functions
		let result = unsafe { rte_eth_dev_wd_timeout_store(self.portIdentifier(), bypassWatchdogTimeout as u32) };
		self.bypassResult(result, "rte_eth_dev_wd_timeout_store")
	}

	#[inline(always)]
	fn bypassResult(&self, result: c_int, function: &str) -> Result<(), UnsupportedByHardwareError>
	{
		if likely!(result == 0)
		{
			Ok(())
		}
		else
		{
			self.bypassResultError(result, function)
		}
	}

	#[inline(always)]
	fn bypassResultError<T>(&self, result: c_int, function: &str) -> Result<T, UnsupportedByHardwareError>
	{
		match result
		{
			NegativeE::ENOTSUP => Err(UnsupportedByHardwareError::IsUnsupportedByTheHardware),
			NegativeE::EINVAL => panic!("The port identifier '{}' is invalid", self.portIdentifier()),

			unexpected @ _ => panic!("Unexpected error code '{}' from {}()", unexpected, function),
		}
	}
}
