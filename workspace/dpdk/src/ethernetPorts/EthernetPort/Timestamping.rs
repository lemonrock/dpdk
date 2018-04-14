// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


impl EthernetPort
{
	#[inline(always)]
	pub fn enableTimestamping(&self) -> Result<(), UnsupportedByHardwareError>
	{
		let result = unsafe { rte_eth_timesync_enable(self.portIdentifier()) };
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

				_ => panic!("Unexpected error code '{}' from rte_eth_timesync_enable()", result),
			}
		}
	}

	#[inline(always)]
	pub fn disableTimestamping(&self) -> Result<(), UnsupportedByHardwareError>
	{
		let result = unsafe { rte_eth_timesync_disable(self.portIdentifier()) };
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

				_ => panic!("Unexpected error code '{}' from rte_eth_timesync_disable()", result),
			}
		}
	}

	#[inline(always)]
	pub fn readReceiveTimestamp(&self, deviceSpecificValueEgRxTimestampRegisterForIntelI40e: Option<u32>) -> Result<Option<timespec>, UnsupportedByHardwareError>
	{
		let mut time = unsafe { uninitialized( )};

		let result = unsafe { rte_eth_timesync_read_rx_timestamp(self.portIdentifier(), &mut time, deviceSpecificValueEgRxTimestampRegisterForIntelI40e.unwrap_or(0)) };
		if likely(result == 0)
		{
			Ok(Some(time))
		}
		else
		{
			forget(time);

			match result
			{
				NegativeE::EINVAL => Ok(None),
				NegativeE::ENOTSUP => Err(UnsupportedByHardwareError::IsUnsupportedByTheHardware),

				NegativeE::ENODEV => panic!("The port identifier '{}' is invalid", self.portIdentifier()),

				_ => panic!("Unexpected error code '{}' from rte_eth_timesync_read_rx_timestamp()", result),
			}
		}
	}

	#[inline(always)]
	pub fn readTransmissionTimestamp(&self) -> Result<Option<timespec>, UnsupportedByHardwareError>
	{
		let mut time = unsafe { uninitialized( )};

		let result = unsafe { rte_eth_timesync_read_tx_timestamp(self.portIdentifier(), &mut time) };
		if likely(result == 0)
		{
			Ok(Some(time))
		}
		else
		{
			forget(time);

			match result
			{
				NegativeE::EINVAL => Ok(None),
				NegativeE::ENOTSUP => Err(UnsupportedByHardwareError::IsUnsupportedByTheHardware),

				NegativeE::ENODEV => panic!("The port identifier '{}' is invalid", self.portIdentifier()),

				_ => panic!("Unexpected error code '{}' from rte_eth_timesync_read_tx_timestamp()", result),
			}
		}
	}

	#[inline(always)]
	pub fn adjustTimesyncClock(&self, deltaInNanoseconds: i64) -> Result<(), UnsupportedByHardwareError>
	{
		let result = unsafe { rte_eth_timesync_adjust_time(self.portIdentifier(), deltaInNanoseconds) };
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

				_ => panic!("Unexpected error code '{}' from rte_eth_timesync_adjust_time()", result),
			}
		}
	}

	#[inline(always)]
	pub fn readTimesyncClock(&self) -> Result<timespec, UnsupportedByHardwareError>
	{
		let mut time = unsafe { uninitialized( )};

		let result = unsafe { rte_eth_timesync_read_time(self.portIdentifier(), &mut time) };
		if likely(result == 0)
		{
			Ok(time)
		}
		else
		{
			forget(time);

			match result
			{
				NegativeE::ENOTSUP => Err(UnsupportedByHardwareError::IsUnsupportedByTheHardware),

				NegativeE::ENODEV => panic!("The port identifier '{}' is invalid", self.portIdentifier()),

				_ => panic!("Unexpected error code '{}' from rte_eth_timesync_read_time()", result),
			}
		}
	}

	#[inline(always)]
	pub fn writeTimesyncClock(&self, time: timespec) -> Result<(), UnsupportedByHardwareError>
	{
		let result = unsafe { rte_eth_timesync_write_time(self.portIdentifier(), &time) };
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

				_ => panic!("Unexpected error code '{}' from rte_eth_timesync_write_time()", result),
			}
		}
	}
}
