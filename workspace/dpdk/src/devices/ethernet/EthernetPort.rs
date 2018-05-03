// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// An ethernet port (link).
pub trait EthernetPort
{
	/// Finds an ethernet port by its port identifier.
	#[inline(always)]
	fn find_by_ethernet_port_identifier(ethernet_port_identifier: EthernetPortIdentifier) -> Option<Self>
	{
		let ethernet_device = ethernet_port_identifier.ethernet_device();
		
		use self::rte_eth_dev_state::*;
		
		match ethernet_device.state
		{
			RTE_ETH_DEV_UNUSED => Ok(None),
			RTE_ETH_DEV_ATTACHED => Ok(Some(Self::from_mutable_reference(ethernet_device))),
			RTE_ETH_DEV_DEFERRED => Ok(None),
			RTE_ETH_DEV_REMOVED => Ok(Some(Self::from_mutable_reference(ethernet_device))),
		}
	}
	
	/// Finds an ethernet port by its device name.
	///
	/// A `device_name` is either a formatted PCI Device Address (eg `0000:01:00.0`) or a virtual device name (eg `net_pcap,iface=eth0`). In the case of virtual device, the `device_name` (a) include device arguments and (v) is the same as that passed to RTE init's `--vdev` option.
	///
	/// `device_name` can be upto 64 characters long (excluding the trailing null). This is not checked.
	#[inline(always)]
	fn find_by_device_name(device_name: &CStr) -> Option<Self>
	{
		let mut ethernet_port_identifier = unsafe { uninitialized() };
		let result = unsafe { rte_eth_dev_get_port_by_name(name.as_ptr(), &mut ethernet_port_identifier) };
		if result == 0
		{
			Self::find_by_ethernet_port_identifier(EthernetPortIdentifier::new(ethernet_port_identifier).unwrap())
		}
		else
		{
			None
		}
	}
	
	/// Attaches an ethernet port by its device name and assigns an `EthernetPortIdentifier` for it.
	///
	/// Attachment assumes the ethernet port was not attached when the PCI bus was scanned, say (ie after `rte_eal_init()`).
	///
	/// A `device_name` is either a formatted PCI Device Address (eg `0000:01:00.0`) or a virtual device name (eg `net_pcap,iface=eth0`). In the case of virtual device, the `device_name` (a) include device arguments and (v) is the same as that passed to RTE init's `--vdev` option.
	///
	/// `device_name` is passed to `rte_eal_parse_devargs_str` then to `rte_eal_dev_attach`.
	///
	/// `device_name` can be upto 64 characters long (excluding the trailing null). This is not checked.
	///
	/// After calling this function call `self.configure_and_start()`.
	#[inline(always)]
	fn hot_plug_attach_by_device_name(device_name: &CStr) -> Option<Self>
	{
		let mut ethernet_port_identifier = unsafe { uninitialized() };
		let result = unsafe { rte_eth_dev_attach(device_name.as_ptr(), &mut ethernet_port_identifier) };
		if result == 0
		{
			Self::find_by_ethernet_port_identifier(EthernetPortIdentifier::new(ethernet_port_identifier).unwrap())
		}
		else
		{
			None
		}
	}
	
	/// After `rte_eal_init()` is called, it is safe to call this function.
	///
	/// After calling this function, it can not be re-called until the device is stopped.
	///
	/// When this function returns with a `Some()`, then the link is fully configured and up. It should be freed by calling `self.terminate_started_device_and_hot_plug_detach()`.
	///
	/// If it returns with `None`, then the calling process should terminate.
	fn configure_and_start<Handler: EthernetPortLinkStatusEventHandler>(self, receive_side_scaling_toeplitz_hash_function_key_data_strategy: &ReceiveSideScalingToeplitzHashFunctionKeyDataStrategy, handler: Handler, should_function_terminate: &Arc<ShouldFunctionTerminate>, media_access_control_address: MediaAccessControlAddress) -> Option<Box<Handler>>
	{
		/*
			Cap on rss queues - size of RETA table (64, 128 or 512 entries)
			RSS will fall through for bad hashes to RETA table index 0
				- we may want a special extra queue to handle this
			
			Cap on rss queues - max_rx_queue
			
			
			Soft cap on rss queues - what's available on a NUMA node
				- we can steal from other nodes potentially.
			
			We may have 1, 2 or 4 ports on a NUMA node (or more)
				- these ordinarily will have the same limits and constraints (eg no of queues)
				
			We have to choose our processing model
				- Rx, process, Tx on one logical core (Cap on rx queues - max_tx_queue (and vice versa) if using a 'paired' model)
				- Rx, Tx on one logical core, process on another
				- Rx - distribute - tx - all on different cores
					- distributors work best when on the same NUMA node
					- a given lcore cannot act as both a distributor lcore and a worker lcore for the same distributor instance, otherwise deadlock will result.
					- one distributor per ethernet port per numa node
					- so a distributor runs on a core
						- gets received packets from an inbound queue (packets were enqueued by receiver threads)
						- 'sends' them to workers
						- then gets packets returned by workers and enqueues them into an outbound queue
							- transmit workers get these packets and enqueue them.
							- the outbound queue could be a shared one or it could be a queue per transmit worker
				
				- There is no reason why more than distributor-processor 'threads' (lcore) could not run on one logical hyperthread
					- So we only need to reserve a minimum of one logical core to be used for the distributor processors for N or more ethernet ports
					- Sadly, this means we need to know ethernet port characteristics before calling rte_eal_init...
				
				- Distributor processor can run on a receive logical core, BUT then there can be only one receive logical core
					- this model is for ethernet nics that don't support RSS
					
					
				Scale-Down models
					- We could assume that each NUMA node has 64 cores (or may be 63; we assume one is used by OS)
						- we then 'scale down' to the actual amount, eg 56, 32, 4, etc.
							4
							8
							12
							16
							20
							24
							28
							30
							32
							36
							40
							44
							48
							52
							56
							
						- scaling down could happen by assigning multiple logical DPDK cores to one hyper-threaded core.
					
					
			
			We probably should have one 'multipurpose' core per numa node, which maps to [master + service cores]
				- ie two of DPDK's logical cores is actually one hyperthread, ie thread affinity is not 1:1
			
			
			does the distributor use the membership library? (rte_member)
				- seems not
				- perhaps we could write our own that does.
				
			All master, slave and service cores will need to respond to a global exit signal.
		
		*/
		
		
		
		
		let ethernet_port_identifier = self.ethernet_port_identifier();
		
		let mut ethernet_device_information = self.ethernet_device_information();
		let driver_name = ethernet_device_information.driver_name_and_fix_buggy_information();
		let number_of_receive_side_scaling_queues_and_receive_side_scaling_hash_key = ethernet_device_information.number_of_receive_side_scaling_queues_and_receive_side_scaling_hash_key();
		let number_of_transmit_queues = ethernet_device_information.number_of_transmit_queues();
		
		// All u16; 18.05
		// Upcoming DPDK 18.05 provides 'preferred' defaults.
		// ring sizes are for configuring queues.
		// ethernet_device_information.preferred_size.rx_ring
		// ethernet_device_information.preferred_size.tx_ring
		
		// TODO: if RSS isn't supported, then just fallback to a single receive (and transmit) queue.
		// It is possible to have more transmit queues than receive, perhaps even 1:64.
		
		// TODO: Queues and flows.
		
		// TODO: Mapping to cores - this isn't going to be straightforward
			// we can use less preferable numa nodes for an ethernet card...
		
		
			// transmit queues - one per core during transmit
			// capped by number of cores available to numa node
			// an ethernet port is per numa node - rte_eth_dev_socket_id(port)
		
		let (number_of_receive_queues, receive_device_offloads, transmit_device_offloads) = ethernet_port_identifier.configure(&ethernet_device_information, number_of_receive_side_scaling_queues_and_receive_side_scaling_hash_key, number_of_transmit_queues, driver_name);
		let (receive_descriptors_queue_ring_size, transmit_descriptors_queue_ring_size) = ethernet_device_information.obtain_maximum_receive_and_transmit_queue_depths(&ethernet_device_information);
		
		// Flow API and devargs changes: https://github.com/DPDK/dpdk/blob/master/doc/guides/rel_notes/release_18_05.rst
		
		// Configuring service cores - who USES them? eg rte_timer?
		
		
		// At this point, do we know that a numa node has been assigned?
		
		let receive_descriptors_queue_ring_numa_node = XXXXX;
		let received_packet_buffer_pool = XXXXX;
		let transmit_descriptors_queue_ring_numa_node = XXXXX;
		
		for receive_queue_identifier in ReceiveQueueIdentifier::all(number_of_receive_queues)
		{
			ethernet_port_identifier.configure_receive_queue(receive_queue_identifier, &ethernet_device_information, receive_device_offloads, receive_descriptors_queue_ring_size, receive_descriptors_queue_ring_numa_node, received_packet_buffer_pool);
		}
		
		for transmit_queue_identifier in TransmitQueueIdentifier::all(number_of_transmit_queues)
		{
			ethernet_port_identifier.configure_transmit_queue(transmit_queue_identifier, &ethernet_device_information, transmit_device_offloads, transmit_descriptors_queue_ring_size, transmit_descriptors_queue_ring_numa_node);
		}
		
		ethernet_port_identifier.start();
		
		ethernet_port_identifier.set_default_media_access_control_address(media_access_control_address);
		
		ethernet_port_identifier.configure_redirection_table(&ethernet_device_information, number_of_receive_side_scaling_queues_and_receive_side_scaling_hash_key);
		
		// Do we care?
		let handler = ethernet_port_identifier.wait_for_link_to_come_up(handler);
		if handler.is_none()
		{
			return None
		}
		
		for receive_queue_identifier in ReceiveQueueIdentifier::all(number_of_receive_queues)
		{
			let receive_burst = self.receive_burst(receive_queue_identifier, &ethernet_device_information);
		}
		
		for transmit_queue_identifier in TransmitQueueIdentifier::all(number_of_transmit_queues)
		{
			let transmit_burst = self.transmit_burst(transmit_queue_identifier, &ethernet_device_information);
		}
		
		XXXXXX:;
		
		// TODO: recv, transmit bursts need to go where?
		// Ideally, they need to be created using memory on the same NUMA node as the thread / logical core that is going to use them.
		
		
		handler
	}
	
	/// Stop, close and try to detach a device.
	///
	/// After calling this, the device effectively does not exist.
	///
	/// If an `Err(())` is returned, then the device was stopped and closed but not detached, typically because the device does not support detachment.
	///
	/// To use the device again call `Self::hot_plug_attach_by_device_name()`.
	#[inline(always)]
	fn terminate_started_device_and_hot_plug_detach<Handler: EthernetPortLinkStatusEventHandler>(self, handler: Box<Handler>) -> Result<(), ()>
	{
		let ethernet_port_identifier = self.ethernet_port_identifier();
		
		ethernet_port_identifier.unregister_receive_link_up_or_down_events(handler);
		
		let ethernet_port_identifier = ethernet_port_identifier.into();
		
		unsafe { rte_eth_dev_stop(ethernet_port_identifier) };
		
		unsafe { rte_eth_dev_close(ethernet_port_identifier) };
		
		let mut device_name = Vec::with_capacity(RTE_DEV_NAME_MAX_LEN as usize);
		if likely(unsafe { rte_eth_dev_detach(ethernet_port_identifier, device_name.as_mut_ptr()) } == 0)
		{
			Ok(())
		}
		else
		{
			Err(())
		}
	}
	
	/// NUMA node this ethernet port is on, if any.
	#[inline(always)]
	fn numa_node(self) -> NumaNodeChoice
	{
		self.data().numa_node
	}
	
	/// Returns a helper for effective receive burst queuing.
	#[inline(always)]
	fn receive_burst(self, receive_queue_identifier: ReceiveQueueIdentifier, _ethernet_device_information: &rte_eth_dev_info) -> ReceiveBurst
	{
		let receive_queue_information = self.receive_queue_information(receive_queue_identifier);
		debug_assert_eq!(receive_queue_information.scattered_rx, 0, "Packet receive scatter (ie multiple segment, non-contiguous packets) is not supported but this queue has it enabled");
		
		ReceiveBurst
		{
			receive_burst_function_pointer: self.mutable_reference().rx_pkt_burst,
			receive_queue: self.receive_queue(receive_queue_identifier),
			maximum_number_of_packets_which_can_be_received_at_once:
			{
				// DPDK 18.05
				//_ethernet_device_information.preferred_size.rx_burst
				receive_queue_information.nb_desc as usize
			},
			receive_memory_pool: NonNull::new(receive_queue_information.mp).expect("receive queue memory pool is null"),
		}
	}
	
	/// Returns a helper for effective transmit burst queuing.
	#[inline(always)]
	fn transmit_burst(self, transmit_queue_identifier: TransmitQueueIdentifier, _ethernet_device_information: &rte_eth_dev_info) -> TransmitBurst
	{
		let transmit_queue_information = self.transmit_queue_information(transmit_queue_identifier);
		
		TransmitBurst
		{
			transmit_burst_function_pointer: self.mutable_reference().tx_pkt_burst,
			transmit_queue: self.transmit_queue(transmit_queue_identifier),
			maximum_number_of_packets_which_can_be_transmitted_at_once:
			{
				// DPDK 18.05
				//_ethernet_device_information.preferred_size.tx_burst
				transmit_queue_information.nb_desc as usize
			},
			transmit_prepare_function_pointer: match self.mutable_reference().tx_pkt_prepare
			{
				None => TransmitBurst::prepare_is_unsupported,
				Some(transmit_prepare_function_pointer) => transmit_prepare_function_pointer,
			},
		}
	}
	
	/// EthernetPortIdentifier for this ethernet port.
	#[inline(always)]
	fn ethernet_port_identifier(self) -> EthernetPortIdentifier
	{
		EthernetPortIdentifier(self.data().port_id)
	}
	
	#[inline(always)]
	fn device_operations<'a>(self) -> &'a eth_dev_ops
	{
		let dev_ops = self.mutable_reference().dev_ops;
		debug_assert!(dev_ops.is_not_null(), "dev_ops is null");
		
		unsafe { & * dev_ops }
	}
	
//	/// Backing device.
//	///
//	/// Might be a PCI device or a virtual device.
//	#[inline(always)]
//	fn backing_device<'a>(self) -> DpdkDevice<'a>
//	{
//		DpdkDevice::new(self.mutable_reference().device)
//	}
//
//	/// Interrupt handle.
//	#[inline(always)]
//	fn interrupt_handle<'a>(self) -> &'a rte_intr_handle
//	{
//		let intr_handle = self.mutable_reference().intr_handle;
//		debug_assert!(intr_handle.is_not_null(), "intr_handle is null");
//
//		unsafe { & * intr_handle }
//	}
//
//	/// Security Context for IPsec (and possibly other protocols in future DPDK revisions).
//	#[inline(always)]
//	fn security_context(self) -> *mut c_void
//	{
//		self.mutable_reference().security_ctx
//	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn receive_queue_information(self, receive_queue_identifier: ReceiveQueueIdentifier) -> rte_eth_rxq_info
	{
		let mut receive_queue_information = unsafe { zeroed() };
		(self.device_operations().rxq_info_get.expect("rxq_info_get is unsupported"))(self.mutable_reference(), receive_queue_identifier.into(), &mut receive_queue_information);
		receive_queue_information
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn transmit_queue_information(self, transmit_queue_identifier: TransmitQueueIdentifier) -> rte_eth_txq_info
	{
		let mut transmit_queue_information = unsafe { zeroed() };
		(self.device_operations().txq_info_get.expect("txq_get is unsupported"))(self.mutable_reference(), transmit_queue_identifier.into(), &mut transmit_queue_information);
		transmit_queue_information
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn receive_queue<'a>(self, receive_queue_identifier: ReceiveQueueIdentifier) -> NonNull<c_void>
	{
		let receive_queue = self.receive_queues()[receive_queue_identifier.into()];
		assert!(receive_queue.is_not_null(), "receive_queue is null");
		unsafe { NonNull::new_unchecked(receive_queue) }
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn transmit_queue<'a>(self, transmit_queue_identifier: TransmitQueueIdentifier) -> NonNull<c_void>
	{
		let transmit_queue = self.transmit_queues()[transmit_queue_identifier.into()];
		assert!(transmit_queue.is_not_null(), "transmit_queue is null");
		unsafe { NonNull::new_unchecked(transmit_queue) }
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn receive_queues<'a>(self) -> &'a [*mut c_void]
	{
		let rx_queues = self.data().rx_queues;
		debug_assert!(rx_queues.is_not_null(), "rx_queues are null");
		let number_of_receive_queues = self.data().nb_rx_queues;
		debug_assert_ne!(number_of_receive_queues, 0, "number_of_receive_queues is zero");
		unsafe { from_raw_parts(rx_queues, number_of_receive_queues as usize) }
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn transmit_queues<'a>(self) -> &'a [*mut c_void]
	{
		let tx_queues = self.data().tx_queues;
		debug_assert!(tx_queues.is_not_null(), "tx_queues are null");
		let number_of_transmit_queues = self.data().nb_tx_queues;
		debug_assert_ne!(number_of_transmit_queues, 0, "number_of_transmit_queues is zero");
		unsafe { from_raw_parts(tx_queues, number_of_transmit_queues as usize) }
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn ethernet_device_information(self) -> rte_eth_dev_info
	{
		let mut ethernet_device_information: rte_eth_dev_info = uninitialized();
		rte_eth_dev_info_get(ethernet_port_identifier, &mut ethernet_device_information);
		ethernet_device_information
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn data<'a>(self) -> &'a mut rte_eth_dev_data
	{
		let data = self.mutable_reference().data;
		debug_assert!(data.is_not_null(), "data is null");
		
		unsafe { &mut * data }
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn mutable_reference(self) -> &'static mut rte_eth_dev
	{
		let mutable_reference = self.mutable_reference_hidden();
		
		if cfg!(debug_assertions)
		{
			use self::rte_eth_dev_state::*;
			
			let state = mutable_reference.state;
			debug_assert_ne!(state, RTE_ETH_DEV_UNUSED, "Device is unused");
			debug_assert_ne!(state, RTE_ETH_DEV_DEFERRED, "Device is deferred");
			
			self.debug_assert_callbacks_to_post_process_after_receive_burst_are_all_null();
			self.debug_assert_callbacks_to_pre_process_before_transmit_burst_are_all_null();
		}
		
		mutable_reference
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn debug_assert_callbacks_to_post_process_after_receive_burst_are_all_null(self)
	{
		let callbacks = &mut self.mutable_reference().post_rx_burst_cbs;
		let mut index = 0;
		for callback in callbacks
		{
			debug_assert!(callback.is_null(), "Post-process callback after receive burst at index '{}' is not null", index);
			index +=1;
		}
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn debug_assert_callbacks_to_pre_process_before_transmit_burst_are_all_null<'a>(self)
	{
		let callbacks = &mut self.mutable_reference().pre_tx_burst_cbs;
		let mut index = 0;
		for callback in callbacks
		{
			debug_assert!(callback.is_null(), "Pre-process callback before transmit burst at index '{}' is not null", index);
			index +=1;
		}
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn mutable_reference_hidden(self) -> &'static mut rte_eth_dev;
	
	#[doc(hidden)]
	#[inline(always)]
	fn from_mutable_reference(mutable_reference: &'static mut rte_eth_dev) -> Self;
}

impl EthernetPort for &'static mut rte_eth_dev
{
	#[doc(hidden)]
	#[inline(always)]
	fn mutable_reference_hidden(self) -> &'static mut rte_eth_dev
	{
		self
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn from_mutable_reference(mutable_reference: &'static mut rte_eth_dev) -> Self
	{
		mutable_reference
	}
}
