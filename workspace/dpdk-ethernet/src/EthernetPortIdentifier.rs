// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// An ethernet port identifier.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct EthernetPortIdentifier(pub(crate) u16);

impl Display for EthernetPortIdentifier
{
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "{}", self.0)
	}
}

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
			if Self::is_invalid(value)
			{
				Err(())
			}
			else
			{
				Ok(EthernetPortIdentifier(value))
			}
		}
	}
}

impl TryFrom<usize> for EthernetPortIdentifier
{
	type Error = ();
	
	#[inline(always)]
	fn try_from(value: usize) -> Result<Self, Self::Error>
	{
		if value >= Self::Maximum
		{
			Err(())
		}
		else
		{
			let value = value as u16;
			if Self::is_invalid(value)
			{
				Err(())
			}
			else
			{
				Ok(EthernetPortIdentifier(value))
			}
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

impl EthernetPortIdentifier
{
	/// Maximum.
	pub const Maximum: usize = RTE_MAX_ETHPORTS;
	
	#[inline(always)]
	fn is_valid(port_id: u16) -> bool
	{
		let value = unsafe { rte_eth_dev_is_valid_port(port_id) };
		debug_assert!(value < 2, "rte_eth_dev_is_valid_port returned a value '{}' which was not 0 or 1", value);
		value == 1
	}
	
	#[inline(always)]
	fn is_invalid(port_id: u16) -> bool
	{
		let value = unsafe { rte_eth_dev_is_valid_port(port_id) };
		debug_assert!(value < 2, "rte_eth_dev_is_valid_port returned a value '{}' which was not 0 or 1", value);
		value == 0
	}
	
	/// Next valid ethernet port.
	#[inline(always)]
	pub fn next(self) -> Option<Self>
	{
		let mut potentially_next = self.0;
		while potentially_next <= Self::Maximum as u16
		{
			if Self::is_valid(potentially_next)
			{
				return Some(EthernetPortIdentifier(potentially_next))
			}
			
			potentially_next += 1;
		}
		None
	}
	
	/// Previous valid ethernet port.
	#[inline(always)]
	pub fn previous(self) -> Option<Self>
	{
		let mut potentially_previous = self.0;
		while potentially_previous != 0
		{
			if Self::is_valid(potentially_previous)
			{
				return Some(EthernetPortIdentifier(potentially_previous))
			}
			
			potentially_previous -= 1;
		}
		None
	}
	
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
	
	/// NUMA node that ethernet port is associated with.
	///
	/// Returns NumaNode of zero if not known; there is no way to distinguish this.
	pub fn numa_node(self) -> NumaNode
	{
		let result = unsafe { rte_eth_dev_socket_id(self.0) };
		debug_assert!(result != -1, "port_id out of range");
		debug_assert!(result >= 0, "invalid result '{}' from rte_eth_dev_socket_id", result);
		
		NumaNode::from_u32(result as u32)
	}
	
	/// Maximum receive and transmit queue depths.
	#[inline(always)]
	pub fn obtain_maximum_receive_and_transmit_queue_depths(self, ethernet_device_information: &rte_eth_dev_info) -> (ReceiveQueueRingSize, TransmitQueueRingSize)
	{
		let mut receive_descriptors = ethernet_device_information.rx_desc_lim.nb_max;
		let mut transmit_descriptors = ethernet_device_information.tx_desc_lim.nb_max;
		
		let result = unsafe { rte_eth_dev_adjust_nb_rx_tx_desc(self.0, &mut receive_descriptors, &mut transmit_descriptors) };
		
		if likely!(result == 0)
		{
			return (ReceiveQueueRingSize(receive_descriptors), TransmitQueueRingSize(transmit_descriptors))
		}
		
		match result
		{
			NegativeE::ENODEV => panic!("This ethernet port '{}' is not a device", self),
			NegativeE::ENOTSUP => panic!("rte_eth_dev_adjust_nb_rx_tx_desc is not supported"),
			NegativeE::EINVAL => panic!("rte_eth_dev_adjust_nb_rx_tx_desc reports bad arguments"),
			
			_ => panic!("rte_eth_dev_adjust_nb_rx_tx_desc returned an unknown error '{}'", result)
		}
	}
	
	/// Configure the default media access control address.
	#[inline(always)]
	pub fn configure_default_media_access_control_address(self, mut media_access_control_address: MediaAccessControlAddress)
	{
		let result = unsafe { rte_eth_dev_default_mac_addr_set(self.0, &mut media_access_control_address as *mut MediaAccessControlAddress as *mut ether_addr) };
		
		if likely!(result == 0)
		{
			return
		}
		
		match result
		{
			NegativeE::ENODEV => panic!("This ethernet port '{}' is not a device", self),
			NegativeE::ENOTSUP => panic!("rte_eth_dev_default_mac_addr_set is not supported"),
			NegativeE::EINVAL => panic!("rte_eth_dev_default_mac_addr_set reports bad arguments"),
			
			_ => panic!("rte_eth_dev_default_mac_addr_set returned an unknown error '{}'", result)
		}
	}
	
	/// Configure a receive queue.
	///
	/// Should only be called after configuring the network card and before starting it.
	///
	/// `queue_ring_numa_node` should ideally be the same as the one for the ethernet port.
	///
	/// `queue_packet_buffer_pool` should ideally be on the numa node `queue_ring_numa_node`.
	#[inline(always)]
	pub fn configure_receive_queue(self, ethernet_device_capabilities: &EthernetDeviceCapabilities, queue_identifier: ReceiveQueueIdentifier, receive_hardware_offloading_flags: ReceiveHardwareOffloadingFlags, queue_ring_size: ReceiveQueueRingSize, queue_ring_numa_node: NumaNode, queue_packet_buffer_pool: NonNull<rte_mempool>)
	{
		let queue_configuration =
		{
			const DropPacketsIfNoReceiveDescriptorsAreAvailable: u8 = 1;
			
			rte_eth_rxconf
			{
				rx_thresh: ethernet_device_capabilities.receive_threshold(),
				rx_free_thresh: ethernet_device_capabilities.receive_free_threshold(),
				rx_drop_en: DropPacketsIfNoReceiveDescriptorsAreAvailable,
				rx_deferred_start: EthernetDeviceCapabilities::ImmediateStart,
				offloads: (ethernet_device_capabilities.receive_queue_hardware_offloading_flags() & receive_hardware_offloading_flags).bits,
			}
		};
		
		let result = unsafe { rte_eth_rx_queue_setup(self.0, queue_identifier.into(), queue_ring_size.into(), queue_ring_numa_node.into(), &queue_configuration, queue_packet_buffer_pool.as_ptr()) };
		
		if likely!(result == 0)
		{
			return
		}
		
		match result
		{
			// NOTE: This is not listed in the documentation but it seems likely to occur.
			NegativeE::ENODEV => panic!("This ethernet port '{}' is not a device", self),
			
			NegativeE::EIO => panic!("This ethernet port '{}' is removed", self),
			NegativeE::EINVAL => panic!("rte_eth_rx_queue_setup: the size of network buffers which can be allocated from the memory pool does not fit the various buffer sizes allowed by the device controller"),
			NegativeE::ENOMEM => panic!("rte_eth_rx_queue_setup: unable to allocate the receive ring descriptors or to allocate network packet buffers from the queue_packet_buffer_pool when initializing receive descriptors"),
			
			_ => panic!("rte_eth_rx_queue_setup returned an unknown error '{}'", result)
		}
	}
	
	/// Configure a transmit queue.
	///
	/// Should only be called after configuring the network card and before starting it.
	///
	/// `queue_ring_numa_node` should ideally be the same as the one for the ethernet port.
	#[inline(always)]
	pub fn configure_transmit_queue(self, ethernet_device_capabilities: &EthernetDeviceCapabilities, queue_identifier: TransmitQueueIdentifier, transmit_hardware_offloading_flags: TransmitHardwareOffloadingFlags, queue_ring_size: TransmitQueueRingSize, queue_ring_numa_node: NumaNode)
	{
		let queue_configuration = rte_eth_txconf
		{
			tx_thresh: ethernet_device_capabilities.transmit_threshold(),
			tx_rs_thresh: ethernet_device_capabilities.transmit_rs_threshold(),
			tx_free_thresh: ethernet_device_capabilities.transmit_free_threshold(),
			txq_flags: ETH_TXQ_FLAGS_IGNORE,
			tx_deferred_start: EthernetDeviceCapabilities::ImmediateStart,
			offloads: (ethernet_device_capabilities.transmit_queue_hardware_offloading_flags() & transmit_hardware_offloading_flags).bits,
		};
		
		let result = unsafe { rte_eth_tx_queue_setup(self.0, queue_identifier.into(), queue_ring_size.into(), queue_ring_numa_node.into(), &queue_configuration) };
		
		if likely!(result == 0)
		{
			return
		}
		
		match result
		{
			// NOTE: This is not listed in the documentation but it seems likely to occur.
			NegativeE::ENODEV => panic!("This ethernet port '{}' is not a device", self),
			
			// NOTE: This is not listed in the documentation but it seems likely to occur.
			NegativeE::EIO => panic!("This ethernet port '{}' is removed", self),
			
			NegativeE::ENOMEM => panic!("rte_eth_tx_queue_setup: unable to allocate the transmit ring descriptors"),
			
			_ => panic!("rte_eth_rx_queue_setup returned an unknown error '{}'", result)
		}
	}
	
	/// Starts the underlying ethernet device.
	///
	/// Returns a device-specific error number in the event of failure.
	#[inline(always)]
	pub fn start(self) -> Result<(), u32>
	{
		let result = unsafe { rte_eth_dev_start(self.into()) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else if unlikely!(result > 0)
		{
			panic!("Unexpected result '{}' from rte_eth_dev_start")
		}
		else
		{
			Err((-result) as u32)
		}
	}
	
	/// Stops the underlying ethernet device.
	///
	/// Never panics.
	#[inline(always)]
	pub fn stop(self)
	{
		unsafe { rte_eth_dev_stop(self.into()) };
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
