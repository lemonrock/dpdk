// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// An ethernet port identifier.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct EthernetPortIdentifier(pub(crate) u16);

impl TryFrom<u16> for EthernetPortIdentifier
{
	type Error = ();
	
	#[inline(always)]
	fn try_from(value: u16) -> Result<Self, Self::Error>
	{
		if value >= Self::Maximum as u16
		{
			Err(())
		}
		else
		{
			Ok(EthernetPortIdentifier(value))
		}
	}
}

impl TryFrom<usize> for EthernetPortIdentifier
{
	type Error = ();
	
	#[inline(always)]
	fn try_from(value: usize) -> Result<Self, Self::Error>
	{
		if value >= Self::Maximum as usize
		{
			Err(())
		}
		else
		{
			Ok(EthernetPortIdentifier(value as u16))
		}
	}
}

impl Into<u16> for EthernetPortIdentifier
{
	#[inline(always)]
	fn into(self) -> u16
	{
		self.0
	}
}

impl Into<usize> for EthernetPortIdentifier
{
	#[inline(always)]
	fn into(self) -> usize
	{
		self.0 as usize
	}
}

impl Step for EthernetPortIdentifier
{
	#[inline(always)]
	fn steps_between(start: &Self, end: &Self) -> Option<usize>
	{
		u16::steps_between(&start.0, &end.0)
	}
	
	#[inline(always)]
	fn replace_one(&mut self) -> Self
	{
		replace(self, EthernetPortIdentifier(1))
	}
	
	#[inline(always)]
	fn replace_zero(&mut self) -> Self
	{
		replace(self, EthernetPortIdentifier(0))
	}
	
	#[inline(always)]
	fn add_one(&self) -> Self
	{
		EthernetPortIdentifier(self.0.add_one())
	}
	
	#[inline(always)]
	fn sub_one(&self) -> Self
	{
		EthernetPortIdentifier(self.0.sub_one())
	}
	
	#[inline(always)]
	fn add_usize(&self, n: usize) -> Option<Self>
	{
		self.0.add_usize(n).map(|value| EthernetPortIdentifier(value))
	}
}

impl Add<u16> for EthernetPortIdentifier
{
	type Output = Self;
	
	#[inline(always)]
	fn add(self, rhs: u16) -> Self::Output
	{
		EthernetPortIdentifier(min(self.0.saturating_add(rhs), Self::Maximum as u16))
	}
}

impl Add<usize> for EthernetPortIdentifier
{
	type Output = Self;
	
	#[inline(always)]
	fn add(self, rhs: usize) -> Self::Output
	{
		EthernetPortIdentifier(min(self.0.saturating_add(rhs as u16), Self::Maximum as u16))
	}
}

impl AddAssign<u16> for EthernetPortIdentifier
{
	#[inline(always)]
	fn add_assign(&mut self, rhs: u16)
	{
		*self = (*self).add(rhs)
	}
}

impl AddAssign<usize> for EthernetPortIdentifier
{
	#[inline(always)]
	fn add_assign(&mut self, rhs: usize)
	{
		*self = (*self).add(rhs)
	}
}

impl Sub<u16> for EthernetPortIdentifier
{
	type Output = Self;
	
	#[inline(always)]
	fn sub(self, rhs: u16) -> Self::Output
	{
		EthernetPortIdentifier(self.0.saturating_sub(rhs))
	}
}

impl Sub<usize> for EthernetPortIdentifier
{
	type Output = Self;
	
	#[inline(always)]
	fn sub(self, rhs: usize) -> Self::Output
	{
		EthernetPortIdentifier(self.0.saturating_sub(rhs as u16))
	}
}

impl SubAssign<u16> for EthernetPortIdentifier
{
	#[inline(always)]
	fn sub_assign(&mut self, rhs: u16)
	{
		self.0 = self.0.saturating_sub(rhs)
	}
}

impl SubAssign<usize> for EthernetPortIdentifier
{
	#[inline(always)]
	fn sub_assign(&mut self, rhs: usize)
	{
		self.0 = self.0.saturating_sub(rhs as u16)
	}
}

impl EthernetPortIdentifier
{
	/// Maximum.
	pub const Maximum: usize = RTE_MAX_ETHPORTS;
	
	/// Underlying DPDK type.
	#[inline(always)]
	pub fn ethernet_device(self) -> &'static rte_eth_dev
	{
		unsafe { &rte_eth_devices[self.0 as usize] }
	}
	
	/// Warning: This method will fail if the device is not PCI-based, eg is virtual.
	#[inline(always)]
	pub fn ethernet_device_as_pci_device(self) -> DpdkPciDevice
	{
		DpdkPciDevice::from(unsafe { NonNull::new_unchecked(self.ethernet_device().device) })
	}
	
	/// Warning: This method will fail if the device is not PCI-based, eg is virtual.
	#[inline(always)]
	pub fn ethernet_device_needs_link_status_interrupt(self) -> bool
	{
		self.ethernet_device_as_pci_device().driver().unwrap().flags().contains(DpdkPciDriverFlags::SupportsLinkStatusInterrupt)
	}
	
	/// Maximum receive and transmit queue depths.
	#[inline(always)]
	pub fn obtain_maximum_receive_and_transmit_queue_depths(self, ethernet_device_information: &rte_eth_dev_info) -> (u16, u16)
	{
		let mut receive_descriptors = ethernet_device_information.rx_desc_lim.nb_max;
		let mut transmit_descriptors = ethernet_device_information.tx_desc_lim.nb_max;
		
		assert_eq!(unsafe { rte_eth_dev_adjust_nb_rx_tx_desc(self.0, &mut receive_descriptors, &mut transmit_descriptors) }, 0, "rte_eth_dev_adjust_nb_rx_tx_desc failed");
		
		(receive_descriptors, transmit_descriptors)
	}
	
	/// Set the default media access control address.
	#[inline(always)]
	pub fn set_default_media_access_control_address(self, mut media_access_control_address: MediaAccessControlAddress)
	{
		assert_eq!(unsafe { rte_eth_dev_default_mac_addr_set(self.0, &mut media_access_control_address as *mut MediaAccessControlAddress as *mut ether_addr) }, 0, "rte_eth_dev_default_mac_addr_set failed");
	}
	
	/// Starts the underlying ethernet device.
	#[inline(always)]
	pub fn start(self)
	{
		assert_eq!(unsafe { rte_eth_dev_start(self.into()) }, 0, "rte_eth_dev_start failed");
	}
	
	/// Register a handler for link up or link down events.
	///
	/// The handler may be run on a service core; generically, it mya be run on any thread.
	///
	/// The returned `EthernetPortLinkStatusEventHandlerGuard` guard, when dropped, will unregister the event handler.
	#[inline(always)]
	pub fn receive_link_up_or_down_events<Handler: LinkStatusEventHandler>(self, handler: Handler) -> LinkStatusEventHandlerGuard<Handler>
	{
		LinkStatusEventHandlerGuard::register(self, handler)
	}
	
	/// Waits for link to come up.
	///
	/// Returns early with `None` if should terminate becomes true.
	///
	/// Returns with `Some(is_full_duplex, was_auto_negotiated, speed_in_megabits_per_second)` when the link comes up.
	#[inline(always)]
	pub fn wait_for_link_to_come_up(self, should_function_terminate: &Arc<ShouldFunctionTerminate>) -> Option<(bool, bool, u32)>
	{
		let mut link_status = unsafe { uninitialized() };
		while
		{
			unsafe { rte_eth_link_get_nowait(self.0, &mut link_status) }
			link_status.is_down()
		}
		{
			if should_function_terminate.sleep_and_check_should_terminate()
			{
				return None
			}
		}
		Some(link_status.if_is_up())
	}
	
}
