// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// An ethernet port (link).
pub trait EthernetPort
{
	// ETH_RSS_L2_PAYLOAD is rarely supported and ETH_RSS_PORT is almost unused.
	const DesiredReceiveSideScalingHashProtocols: u64 = ETH_RSS_IP | ETH_RSS_UDP | ETH_RSS_TCP;
	
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
							32
							36
							40
							44
							48
							52
							56
							(60)
							64
							
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
}
