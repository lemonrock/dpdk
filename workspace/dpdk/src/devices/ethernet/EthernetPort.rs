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
	fn configure_and_start(self, ethernet_frame_length: EthernetFrameLength, receive_side_scaling_toeplitz_hash_function_key_data_strategy: &ReceiveSideScalingToeplitzHashFunctionKeyDataStrategy, available_cores: u16)
	{
		use self::rte_eth_rx_mq_mode::*;
		use self::rte_eth_tx_mq_mode::*;
		use self::rte_fdir_mode::*;
		use self::rte_fdir_pballoc_type::*;
		use self::rte_fdir_status_mode::*;
		
		debug_assert_ne!(available_cores, 0, "available_cores is zero");
		
		/*
			A given NUMA node will have:-
				- one or more ethernet ports
				- zero or more cores 'out-of-bounds'
					- master core
					- dedicated service cores
				
				- we can opt to use a distributor to help with dynamic load balancing
					
					- multiple receiver cores, one per RSS queue
					- one distributor core
					- one or more 'connection' (application) cores
						- whether this includes packet classification & initial processing (eg arp, ipv4, etc) is uncertain
							 - can occur on receiver cores
							 - can occur on application cores
					- one or more transmitter cores; one per TX queue; a transmitter core can be the same as a receiver core.
		
		
		
		*/
		
		
		
		let ethernet_port_identifier = self.ethernet_port_identifier().into();
		
		let mut ethernet_device_information = self.ethernet_device_information();
		let _driver_name = ethernet_device_information.driver_name_and_fix_buggy_maximum_receive_queues_information();
		let number_of_receive_side_scaling_queues_and_receive_side_scaling_hash_key = ethernet_device_information.number_of_receive_side_scaling_queues_and_receive_side_scaling_hash_key();
		let number_of_transmit_queues = ethernet_device_information.number_of_transmit_queues();
		
		// TODO: if RSS isn't supported, then just fallback to a single receive (and transmit) queue.
		
		// TODO: Queues and flows.
		
		// TODO: Mapping to cores - this isn't going to be straightforward; queue numbers for RSS should be powers of 2.
		
			// transmit queues - one per core during transmit
			// capped by number of cores available to numa node
			// an ethernet port is per numa node - rte_eth_dev_socket_id(port)
		
		// TODO: rte_eth_desc_lim - use to size buffers?
		
		let mut rxmode = rte_eth_rxmode
		{
			mq_mode: if receive_side_scaling_is_unavailable
			{
				ETH_MQ_RX_NONE
			}
			else
			{
				ETH_MQ_RX_RSS
			},
			max_rx_pkt_len: min(ethernet_device_information.max_rx_pktlen, ethernet_frame_length.to_u16() as u32),
			split_hdr_size: 0,
			offloads: ethernet_device_information.rx_offload_capa & (DEV_RX_OFFLOAD_CHECKSUM | DEV_RX_OFFLOAD_VLAN_FILTER | DEV_RX_OFFLOAD_VLAN_STRIP | DEV_RX_OFFLOAD_VLAN_EXTEND | DEV_RX_OFFLOAD_QINQ_STRIP | DEV_RX_OFFLOAD_JUMBO_FRAME | DEV_RX_OFFLOAD_CRC_STRIP | DEV_RX_OFFLOAD_TCP_LRO),
			bitfield_1: BindgenBitfieldUnit::new(unsafe { zeroed() }),
			__bindgen_padding_0: unsafe { uninitialized() },
		};
		rxmode.set_ignore_offload_bitfield(1);
		
		const NoPortBasedVirtualLanInsertionAsMostNicsDoNotSupportIt: u64 = 0;
		
		const DisableLoopbackOperationModeAsMostNicsDoNotSupportIt: u32 = 0;
		
		const DisableDataCentreBridgingCapabilityAsMostNicsDoNotSupportIt: u32 = 0;
		
		let ethernet_configuration = rte_eth_conf
		{
			link_speeds: ETH_LINK_SPEED_AUTONEG,
			
			lpbk_mode: DisableLoopbackOperationModeAsMostNicsDoNotSupportIt,
			
			rxmode,
			
			rx_adv_conf: rte_eth_conf_1
			{
				rss_conf: match hash_key
				{
					None => unsafe { zeroed() },
					Some(ref mut hash_key) => rte_eth_rss_conf
					{
						rss_key: rss_hash_key.as_mut_ptr(),
						rss_key_len: rss_hash_key.len() as u8,
						rss_hf: ETH_RSS_PROTO_MASK,
					}
				},
				
				vmdq_dcb_conf: unsafe { zeroed() },
				
				dcb_rx_conf: unsafe { zeroed() },
				
				vmdq_rx_conf: unsafe { zeroed() },
			},
			
			txmode: rte_eth_txmode
			{
				mq_mode: ETH_MQ_TX_NONE,
				
				offloads: ethernet_device_information.tx_offload_capa & (DEV_TX_OFFLOAD_VLAN_INSERT | DEV_TX_OFFLOAD_QINQ_INSERT | DEV_TX_OFFLOAD_IPV4_CKSUM | DEV_TX_OFFLOAD_UDP_CKSUM | DEV_TX_OFFLOAD_TCP_CKSUM | DEV_TX_OFFLOAD_TCP_TSO | DEV_TX_OFFLOAD_UDP_TSO | DEV_TX_OFFLOAD_MBUF_FAST_FREE),
				pvid: NoPortBasedVirtualLanInsertionAsMostNicsDoNotSupportIt,
				bitfield_1: rte_eth_txmode::newbitfield_1(0, 0, 0),
				__bindgen_padding_0: unsafe { uninitialized() },
			},
			
			tx_adv_conf: rte_eth_conf_2
			{
				vmdq_dcb_tx_conf: unsafe { zeroed() },
				
				dcb_tx_conf: unsafe { zeroed() },
				
				vmdq_tx_conf: unsafe { zeroed() },
			},
			
			dcb_capability_en: DisableDataCentreBridgingCapabilityAsMostNicsDoNotSupportIt,
			
			// Is this legacy or not?
			fdir_conf: rte_fdir_conf
			{
				mode: RTE_FDIR_MODE_NONE,
				pballoc: RTE_FDIR_PBALLOC_64K,
				status: RTE_FDIR_NO_REPORT_STATUS,
				drop_queue: 0,
				mask: unsafe { zeroed() },
				flex_conf: unsafe { zeroed() },
			},
			
			intr_conf: rte_intr_conf
			{
				bitfield_1: rte_intr_conf::newbitfield_1(0, 0, 0),
				__bindgen_padding_0: unsafe { uninitialized() },
				__bindgen_align: unsafe { uninitialized() },
			},
		};
		
		
		
		
		
		
		
		
		
		
		assert_eq!(unsafe { rte_eth_dev_configure(ethernet_port_identifier, number_of_receive_queues, number_of_transmit_queues, &ethernet_configuration) }, 0, "Could not configure ethernet_port_identifier '{}' driver '{:?}'", ethernet_port_identifier, driver_name);
		
		
		// rte_eth_rx_queue_setup
		
		// rte_eth_tx_queue_setup
		
		// rte_eth_dev_start
		
	}
	
	/// Stop, close and try to detach a device.
	///
	/// After calling this, the device effectively does not exist.
	///
	/// If an `Err(())` is returned, then the device was stopped and closed but not detached, typically because the device does not support detachment.
	///
	/// To use the device again call `Self::hot_plug_attach_by_device_name()`.
	#[inline(always)]
	fn terminate_started_device_and_hot_plug_detach(self) -> Result<(), ()>
	{
		let ethernet_port_identifier = self.ethernet_port_identifier().into();
		
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
	
	
	/// Number of receive queues.
	#[inline(always)]
	fn number_of_receive_queues(self) -> usize
	{
		self.data().nb_rx_queues as usize
	}
	
	/// Number of transmit queues.
	#[inline(always)]
	fn number_of_transmit_queues(self) -> usize
	{
		self.data().nb_tx_queues as usize
	}
	
	/// NUMA node this ethernet port is on, if any.
	///
	/// This caps the number of cores available that will perform well.
	#[inline(always)]
	fn numa_node(self) -> NumaNodeChoice
	{
		self.data().numa_node
	}
	
	/// Returns a helper for effective receive burst queuing.
	///
	/// Returns `None` if :-
	///
	/// * `receive_queue_identifier` is equal to or greater than `self.number_of_receive_queues()`.
	/// * The receive queue is null.
	#[inline(always)]
	fn receive_burst(self, receive_queue_identifier: ReceiveQueueIdentifier) -> Option<ReceiveBurst>
	{
		let number_of_receive_queues = self.number_of_receive_queues();
		let receive_queue_identifier = receive_queue_identifier.into() as usize;
		
		if number_of_receive_queues >= receive_queue_identifier
		{
			return None
		}
		
		let rx_queues = self.data().rx_queues;
		debug_assert!(rx_queues.is_not_null(), "rx_queues are null");
		let receive_queues = unsafe { from_raw_parts(rx_queues, number_of_receive_queues) };
		let receive_queue = receive_queues[receive_queue_identifier];
		if receive_queue.is_null()
		{
			None
		}
		else
		{
			let mut receive_queue_information = unsafe { zeroed() };
			(self.device_operations().rxq_info_get.expect("rxq_info_get is unsupported"))(self, receive_queue_identifier as u16, &mut receive_queue_information);
			
			debug_assert_eq!(receive_queue_information.scattered_rx, 0, "Packet receive scatter (ie multiple segment, non-contiguous packets) is not supported but this queue has it enabled");
			
			Some
			(
				ReceiveBurst
				{
					receive_burst_function_pointer: self.mutable_reference().rx_pkt_burst,
					receive_queue: unsafe { NonNull::new_unchecked(receive_queue) },
					receive_memory_pool:
					{
						let memory_pool = receive_queue_information.mp;
						debug_assert!(memory_pool.is_not_null(), "memory_pool is null");
						unsafe { NonNull::new_unchecked(memory_pool) }
					},
					maximum_number_of_packets_which_can_be_received_at_once: receive_queue_information.nb_desc as usize,
				}
			)
		}
	}
	
	/// Returns a helper for effective transmit burst queuing.
	///
	/// Returns `None` if :-
	///
	/// * `transmit_queue_identifier` is equal to or greater than `self.number_of_transmit_queues()`.
	/// * The transmit queue is null.
	#[inline(always)]
	fn transmit_burst(self, transmit_queue_identifier: TransmitQueueIdentifier) -> Option<TransmitBurst>
	{
		let number_of_transmit_queues = self.number_of_transmit_queues();
		let transmit_queue_identifier = transmit_queue_identifier.into() as usize;
		
		if number_of_transmit_queues >= transmit_queue_identifier
		{
			return None
		}
		
		let tx_queues = self.data().tx_queues;
		debug_assert!(tx_queues.is_not_null(), "tx_queues are null");
		let transmit_queues = unsafe { from_raw_parts(tx_queues, number_of_transmit_queues) };
		let transmit_queue = transmit_queues[transmit_queue_identifier];
		if transmit_queue.is_null()
		{
			None
		}
		else
		{
			let mut transmit_queue_information = unsafe { zeroed() };
			(self.device_operations().txq_info_get.expect("txq_info_get is unsupported"))(self, transmit_queue_identifier as u16, &mut transmit_queue_information);
			
			Some
			(
				TransmitBurst
				{
					transmit_prepare_function_pointer: match self.mutable_reference().tx_pkt_prepare
					{
						None => TransmitBurst::prepare_is_unsupported,
						Some(transmit_prepare_function_pointer) => transmit_prepare_function_pointer,
					},
					transmit_burst_function_pointer: self.mutable_reference().tx_pkt_burst,
					transmit_queue: unsafe { NonNull::new_unchecked(transmit_queue) },
					maximum_number_of_packets_which_can_be_transmitted_at_once: transmit_queue_information.nb_desc as usize,
				}
			)
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
	
	/// Backing device.
	///
	/// Might be a PCI device or a virtual device.
	#[inline(always)]
	fn backing_device<'a>(self) -> DpdkDevice<'a>
	{
		DpdkDevice::new(self.mutable_reference().device)
	}
	
	/// Interrupt handle.
	#[inline(always)]
	fn interrupt_handle<'a>(self) -> &'a rte_intr_handle
	{
		let intr_handle = self.mutable_reference().intr_handle;
		debug_assert!(intr_handle.is_not_null(), "intr_handle is null");
		
		unsafe { & * intr_handle }
	}
	
	/// Security Context for IPsec (and possibly other protocols in future DPDK revisions).
	#[inline(always)]
	fn security_context(self) -> *mut c_void
	{
		self.mutable_reference().security_ctx
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
