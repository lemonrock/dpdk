// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// An ethernet port identifier.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct EthernetPortIdentifier(u16);

impl EthernetPortIdentifier
{
	//noinspection SpellCheckingInspection
	/// Returns an `Err(())` if the `ethernet_port_identifier` is greater than or equal to `RTE_MAX_ETHPORTS`, currently `32`.
	#[inline(always)]
	pub fn new(ethernet_port_identifier: u16) -> Result<Self, ()>
	{
		if (ethernet_port_identifier as usize) >= RTE_MAX_ETHPORTS
		{
			Err(())
		}
		else
		{
			Ok(EthernetPortIdentifier(ethernet_port_identifier))
		}
	}
	
	#[inline(always)]
	pub(crate) fn ethernet_device(self) -> &'static mut rte_eth_dev
	{
		unsafe { &mut rte_eth_devices[self.0 as usize] }
	}
	
	/// Warning: This method will fail if the device is not PCI-based.
	#[inline(always)]
	pub(crate) fn ethernet_device_as_pci_device(self) -> DpdkPciDevice
	{
		DpdkPciDevice(unsafe { NonNull::new_unchecked(rust_RTE_DEV_TO_PCI(self.ethernet_device().device)) })
	}
	
	/// Warning: This method will fail if the device is not PCI-based.
	#[inline(always)]
	pub(crate) fn ethernet_device_needs_link_status_interrupt(self) -> DpdkPciDevice
	{
		self.ethernet_device_as_pci_device().driver().unwrap().flags.contains(DpdkPciDriverFlags::SupportsLinkStatusInterrupt)
	}
	
	#[inline(always)]
	pub(crate) fn configure(self, ethernet_device_information: &rte_eth_dev_info, number_of_receive_side_scaling_queues_and_receive_side_scaling_hash_key: Option<(u16, Vec<u8>)>, number_of_transmit_queues: u16, driver_name: &CStr) -> (u16, u64, u64)
	{
		use self::rte_eth_rx_mq_mode::*;
		use self::rte_eth_tx_mq_mode::*;
		use self::rte_fdir_mode::*;
		use self::rte_fdir_pballoc_type::*;
		use self::rte_fdir_status_mode::*;
		
		let ethernet_configuration = rte_eth_conf
		{
			link_speeds: ETH_LINK_SPEED_AUTONEG,
			
			lpbk_mode:
			{
				const DisableLoopbackOperationModeAsMostNicsDoNotSupportIt: u32 = 0;
				
				DisableLoopbackOperationModeAsMostNicsDoNotSupportIt
			},
			
			rxmode:
			{
				let mut rxmode = rte_eth_rxmode
				{
					mq_mode: if number_of_receive_side_scaling_queues_and_receive_side_scaling_hash_key.is_none()
					{
						ETH_MQ_RX_NONE
					}
					else
					{
						ETH_MQ_RX_RSS
					},
					max_rx_pkt_len: ethernet_device_information.max_rx_pktlen,
					split_hdr_size: 0,
					offloads:
					{
						debug_assert!(ethernet_device_information.max_rx_pktlen <= ::std::u16::MAX as u32, "ethernet_device_information.max_rx_pktlen '{}' exceeds ::std::u16::MAX", ethernet_device_information.max_rx_pktlen, ::std::u16::MAX);
						let ethernet_frame_length = EthernetFrameLength::try_from_with_jumbo_frames(ethernet_device_information.max_rx_pktlen as u16);
						
						let offload_jumbo_frames_bit = if ethernet_frame_length.implies_jumbo_frames()
						{
							DEV_RX_OFFLOAD_JUMBO_FRAME
						}
						else
						{
							0
						};
						
						ethernet_device_information.rx_offload_capa & (offload_jumbo_frames_bit | DEV_RX_OFFLOAD_CRC_STRIP | DEV_RX_OFFLOAD_VLAN_FILTER | DEV_RX_OFFLOAD_VLAN_STRIP | DEV_RX_OFFLOAD_VLAN_EXTEND | DEV_RX_OFFLOAD_QINQ_STRIP | DEV_RX_OFFLOAD_IPV4_CKSUM | DEV_RX_OFFLOAD_UDP_CKSUM | DEV_RX_OFFLOAD_TCP_CKSUM | DEV_RX_OFFLOAD_TCP_LRO)
					},
					bitfield_1: BindgenBitfieldUnit::new(unsafe { zeroed() }),
					__bindgen_padding_0: unsafe { uninitialized() },
				};
				rxmode.set_ignore_offload_bitfield(1);
				rxmode
			},
			
			rx_adv_conf: rte_eth_conf_1
			{
				rss_conf: match number_of_receive_side_scaling_queues_and_receive_side_scaling_hash_key
				{
					None => unsafe { zeroed() },
					Some(_, ref mut receive_side_scaling_hash_key) => rte_eth_rss_conf
					{
						rss_key: receive_side_scaling_hash_key.as_mut_ptr(),
						rss_key_len: receive_side_scaling_hash_key.len() as u8,
						rss_hf: ETH_RSS_PROTO_MASK,
						//rss_level: xxx, 18.05
					}
				},
				
				vmdq_dcb_conf: unsafe { zeroed() },
				
				dcb_rx_conf: unsafe { zeroed() },
				
				vmdq_rx_conf: unsafe { zeroed() },
			},
			
			txmode: rte_eth_txmode
			{
				mq_mode: ETH_MQ_TX_NONE,
				
				offloads: ethernet_device_information.tx_offload_capa & (DEV_TX_OFFLOAD_VLAN_INSERT | DEV_TX_OFFLOAD_QINQ_INSERT | DEV_TX_OFFLOAD_IPV4_CKSUM | DEV_TX_OFFLOAD_UDP_CKSUM | DEV_TX_OFFLOAD_TCP_CKSUM | DEV_TX_OFFLOAD_TCP_TSO | DEV_TX_OFFLOAD_UDP_TSO),
				pvid:
				{
					const NoPortBasedVirtualLanInsertionAsMostNicsDoNotSupportIt: u64 = 0;
					
					NoPortBasedVirtualLanInsertionAsMostNicsDoNotSupportIt
				},
				bitfield_1: rte_eth_txmode::newbitfield_1(0, 0, 0),
				__bindgen_padding_0: unsafe { uninitialized() },
			},
			
			tx_adv_conf: rte_eth_conf_2
			{
				vmdq_dcb_tx_conf: unsafe { zeroed() },
				
				dcb_tx_conf: unsafe { zeroed() },
				
				vmdq_tx_conf: unsafe { zeroed() },
			},
			
			dcb_capability_en:
			{
				const DisableDataCentreBridgingCapabilityAsMostNicsDoNotSupportIt: u32 = 0;
				
				DisableDataCentreBridgingCapabilityAsMostNicsDoNotSupportIt
			},
			
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
			
			intr_conf:
			{
				let mut interrupt_configuration: rte_intr_conf = rte_intr_conf::default();
				interrupt_configuration.set_lsc(1);
				interrupt_configuration
			}
		};
		
		let number_of_receive_queues = match number_of_receive_side_scaling_queues_and_receive_side_scaling_hash_key
		{
			None => 1,
			Some(number_of_receive_queues, _) => number_of_receive_queues,
		};
		
		assert_eq!(unsafe { rte_eth_dev_configure(self.0, number_of_receive_queues, number_of_transmit_queues, &ethernet_configuration) }, 0, "Could not configure ethernet_port_identifier '{}' driver '{:?}'", ethernet_port_identifier, driver_name);
		
		(number_of_receive_queues, ethernet_configuration.rxmode.offloads, ethernet_configuration.txmode.offloads)
	}
	
	#[inline(always)]
	pub(crate) fn obtain_maximum_receive_and_transmit_queue_depths(self, ethernet_device_information: &rte_eth_dev_info) -> (u16, u16)
	{
		let mut receive_descriptors = ethernet_device_information.rx_desc_lim.nb_max;
		let mut transmit_descriptors = ethernet_device_information.tx_desc_lim.nb_max;
		
		assert_eq!(unsafe { rte_eth_dev_adjust_nb_rx_tx_desc(self.0, &mut receive_descriptors, &mut transmit_descriptors) }, 0, "rte_eth_dev_adjust_nb_rx_tx_desc failed");
		
		(receive_descriptors, transmit_descriptors)
	}
	
	/// `receive_descriptors_queue_ring_numa_node` should ideally be the same as the one for the ethernet port.
	///
	/// `received_packet_buffer_pool` should ideally be on the numa node `receive_descriptors_queue_ring_numa_node`.
	#[inline(always)]
	pub(crate) fn configure_receive_queue(self, receive_queue_identifier: ReceiveQueueIdentifier, ethernet_device_information: &rte_eth_dev_info, receive_device_offloads: u64, receive_descriptors_queue_ring_size: u16, receive_descriptors_queue_ring_numa_node: NumaNode, received_packet_buffer_pool: Non<rte_mempool>)
	{
		let receive_queue_configuration =
		{
			const DropPacketsIfNoReceiveDescriptorsAreAvailable: u8 = 1;
			
			const ImmediateStart: u8 = 0;
			
			rte_eth_rxconf
			{
				rx_thresh: ethernet_device_information.default_rxconf.rx_thresh,
				rx_free_thresh: ethernet_device_information.default_rxconf.rx_free_thresh,
				rx_drop_en: DropPacketsIfNoReceiveDescriptorsAreAvailable,
				rx_deferred_start: ImmediateStart,
				offloads: ethernet_device_information.rx_queue_offload_capa & receive_device_offloads,
			};
		};
		assert_eq!(unsafe { rte_eth_rx_queue_setup(self.0, receive_queue_identifier.into(), receive_descriptors_queue_ring_size, receive_descriptors_queue_ring_numa_node.into(), &receive_queue_configuration, received_packet_buffer_pool.as_ptr()) }, 0, "rte_eth_rx_queue_setup failed");
	}
	
	/// `transmit_descriptors_queue_ring_numa_node` should ideally be the same as the one for the ethernet port.
	#[inline(always)]
	pub(crate) fn configure_transmit_queue(self, transmit_queue_identifier: TransmitQueueIdentifier, ethernet_device_information: &rte_eth_dev_info, transmit_device_offloads: u64, transmit_descriptors_queue_ring_size: u16, transmit_descriptors_queue_ring_numa_node: NumaNode)
	{
		let transmit_queue_configuration = rte_eth_txconf
		{
			tx_thresh: ethernet_device_information.default_txconf.tx_thresh,
			tx_rs_thresh: ethernet_device_information.default_txconf.tx_rs_thresh,
			tx_free_thresh: ethernet_device_information.default_txconf.tx_free_thresh,
			txq_flags: ETH_TXQ_FLAGS_IGNORE,
			tx_deferred_start: 0,
			offloads: ethernet_device_information.tx_queue_offload_capa & transmit_device_offloads,
		};
		assert_eq!(unsafe { rte_eth_tx_queue_setup(self.0, transmit_queue_identifier.into(), transmit_descriptors_queue_ring_size, transmit_descriptors_queue_ring_numa_node.into(), &transmit_queue_configuration) }, 0, "rte_eth_tx_queue_setup failed");
	}
	
	#[inline(always)]
	pub(crate) fn start(self)
	{
		assert_eq!(unsafe { rte_eth_dev_start(ethernet_port_identifier.into()) }, 0, "rte_eth_dev_start failed");
	}
	
	#[inline(always)]
	pub(crate) fn configure_redirection_table(self, ethernet_device_information: &rte_eth_dev_info, number_of_receive_side_scaling_queues_and_receive_side_scaling_hash_key: Option<(u16, Vec<u8>)>)
	{
		if let Some(number_of_receive_side_scaling_queues, _) = number_of_receive_side_scaling_queues_and_receive_side_scaling_hash_key
		{
			let first_receive_side_scaling_queue = ReceiveQueueIdentifer::new(0).unwrap();
			let (redirection_table, redirection_table_size) = ethernet_device_information.redirection_table(number_of_receive_side_scaling_queues, first_receive_side_scaling_queue);
			
			assert_eq!(unsafe { rte_eth_dev_rss_reta_update(self.into(), redirection_table.as_mut_ptr(), redirection_table_size) }, 0, "rte_eth_dev_rss_reta_update failed");
		}
	}
	
	#[inline(always)]
	pub(crate) fn set_default_media_access_control_address(self, media_access_control_address: MediaAccessControlAddress)
	{
		assert_eq!(unsafe { rte_eth_dev_default_mac_addr_set(self.0, &mut media_access_control_address.to_ether_addr()) }, 0, "rte_eth_dev_default_mac_addr_set failed");
	}
	
	#[inline(always)]
	pub(crate) fn wait_for_link_to_come_up<Handler: EthernetPortLinkStatusEventHandler>(self, handler: Handler, should_function_terminate: &Arc<ShouldFunctionTerminate>) -> Option<Box<Handler>>
	{
		let mut handler = Box::new(handler);
		
		let mut link_status = unsafe { unintialized() };
		while
		{
			unsafe { rte_eth_link_get_nowait(ethernet_port_identifier, &mut link_status) }
			Self::link_status_is_down(&link_status)
		}
		{
			if should_function_terminate.sleep_and_check_should_terminate()
			{
				return None
			}
		}
		
		let (is_full_duplex, was_auto_negotiated, speed_in_megabits_per_second) = Self::parse_if_details_if_link_status_is_up(&link_status, &handler);
		
		self.register_receive_link_up_or_down_events(handler);
		
		Some(handler)
	}
	
	// The returned `Box<Handler>` must live until the call to `unregister_receive_link_up_or_down_events()` and must be the same type.
	#[inline(always)]
	pub(crate) fn register_receive_link_up_or_down_events<Handler: EthernetPortLinkStatusEventHandler>(self, handler: Box<Handler>) -> Box<Handler>
	{
		let argument = Box::into_raw(handler);
		let handler = unsafe { Box::from_raw(argument) };
		
		use self::rte_eth_event_type::*;
		assert_eq!(unsafe { rte_eth_dev_callback_register(self.0, RTE_ETH_EVENT_INTR_LSC, Self::<Handler>::link_up_or_down_events_callback, argument) }, 0, "rte_eth_dev_callback_register failed");
		
		handler
	}
	
	#[inline(always)]
	pub(crate) fn unregister_receive_link_up_or_down_events<Handler: EthernetPortLinkStatusEventHandler>(self, handler: Box<Handler>)
	{
		let argument = Box::into_raw(Box::new(handler));
		let handler = unsafe { Box::from_raw(argument) };
		
		use self::rte_eth_event_type::*;
		assert_eq!(unsafe { rte_eth_dev_callback_unregister(self.0, RTE_ETH_EVENT_INTR_LSC, Self::<Handler>::link_up_or_down_events_callback, argument) }, 0, "rte_eth_dev_callback_unregister failed");
		
		drop(handler);
	}
	
	unsafe extern "C" fn link_up_or_down_events_callback<Handler: EthernetPortLinkStatusEventHandler>(ethernet_port_identifier: u16, event: rte_eth_event_type, cb_arg: *mut c_void, ret_param: *mut c_void) -> i32
	{
		debug_assert_eq!(event, rte_eth_event_type::RTE_ETH_EVENT_INTR_LSC, "event '{:?}' was not RTE_ETH_EVENT_INTR_LSC", event);
		debug_assert!(cb_arg.is_not_null(), "cb_arg is null");
		debug_assert!(ret_param.is_null(), "ret_param is not null");
		let handler = unsafe { &mut * (cb_arg as *mut Handler) };
		
		let mut link_status = unsafe { unintialized() };
		unsafe { rte_eth_link_get_nowait(ethernet_port_identifier, &mut link_status) }
		
		let ethernet_port_identifier = EthernetPortIdentifier(ethernet_port_identifier);
		if Self::link_status_is_down(&link_status)
		{
			handler.link_has_gone_down(ethernet_port_identifier);
		}
		else
		{
			let (is_full_duplex, was_auto_negotiated, speed_in_megabits_per_second) = Self::parse_if_details_if_link_status_is_up(&link_status, handler);
		}
		
		0
	}
	
	#[inline(always)]
	fn link_status_is_down(link_status: &link_status) -> bool
	{
		link_status.link_status() == 0
	}
	
	#[inline(always)]
	fn parse_if_details_if_link_status_is_up<Handler: EthernetPortLinkStatusEventHandler>(link_status: &link_status, handler: &mut Handler)
	{
		let is_full_duplex = link_status.link_duplex() == 1;
		let was_auto_negotiated = link_status.link_autoneg() == 1;
		let speed_in_megabits_per_second = link_status.link_speed;
		
		handler.link_has_come_up(ethernet_port_identifier, is_full_duplex, was_auto_negotiated, speed_in_megabits_per_second)
	}
}
