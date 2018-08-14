// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// An ethernet port identifier.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
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
	fn try_from(potential_ethernet_port_identifier: u16) -> Result<Self, Self::Error>
	{
		if potential_ethernet_port_identifier >= Self::Maximum as u16
		{
			Err(())
		}
		else
		{
			Self::try_from_u16_unchecked(potential_ethernet_port_identifier)
		}
	}
}

impl TryFrom<usize> for EthernetPortIdentifier
{
	type Error = ();
	
	#[inline(always)]
	fn try_from(potential_ethernet_port_identifier: usize) -> Result<Self, Self::Error>
	{
		if potential_ethernet_port_identifier >= Self::Maximum
		{
			Err(())
		}
		else
		{
			Self::try_from_u16_unchecked(potential_ethernet_port_identifier as u16)
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
	
	/// Finds an ethernet port by its device name.
	///
	/// A `device_name` is either a formatted PCI Device Address (eg `0000:01:00.0`) or a virtual device name (eg `net_pcap,iface=eth0`)
	/// In the case of a virtual device, the `device_name` (a) includes device arguments and (b) is the same as that passed to RTE init's `--vdev` option.
	///
	/// `device_name` can be upto 64 characters long (excluding the trailing null).
	/// This is not checked.
	#[inline(always)]
	pub fn find_by_device_name(device_name: &CStr) -> Option<Self>
	{
		let mut potential_ethernet_port_identifier = unsafe { uninitialized() };
		let result = unsafe { rte_eth_dev_get_port_by_name(device_name.as_ptr(), &mut potential_ethernet_port_identifier) };
		Self::parse_find_result(potential_ethernet_port_identifier, result)
	}
	
	/// Attaches an ethernet port by its device name and assigns an `EthernetPortIdentifier` for it.
	///
	/// Attachment assumes the ethernet port was not attached when the PCI bus was scanned, say (ie after `rte_eal_init()`).
	///
	/// A `device_name` is either a formatted PCI Device Address (eg `0000:01:00.0`) or a virtual device name (eg `net_pcap,iface=eth0`).
	/// In the case of a virtual device, the `device_name` (a) includes device arguments and (b) is the same as that passed to RTE init's `--vdev` option.
	///
	/// `device_name` is passed to `rte_eal_parse_devargs_str` then to `rte_eal_dev_attach`.
	///
	/// `device_name` can be upto 64 characters long (excluding the trailing null).
	/// This is not checked.
	///
	/// After calling this function call `self.configure_and_start()`.
	#[inline(always)]
	pub fn hot_plug_attach_by_device_name(device_name: &CStr) -> Option<Self>
	{
		let mut potential_ethernet_port_identifier = unsafe { uninitialized() };
		let result = unsafe { rte_eth_dev_attach(device_name.as_ptr(), &mut potential_ethernet_port_identifier) };
		Self::parse_find_result(potential_ethernet_port_identifier, result)
	}
	
	/// Next valid ethernet port.
	#[inline(always)]
	pub fn next(self) -> Option<Self>
	{
		let mut potential_ethernet_port_identifier = self.0;
		while potential_ethernet_port_identifier < Self::Maximum as u16
		{
			if Self::is_valid(potential_ethernet_port_identifier)
			{
				return Some(EthernetPortIdentifier(potential_ethernet_port_identifier))
			}
			
			potential_ethernet_port_identifier += 1;
		}
		None
	}
	
	/// Previous valid ethernet port.
	#[inline(always)]
	pub fn previous(self) -> Option<Self>
	{
		let mut potential_ethernet_port_identifier = self.0;
		while potential_ethernet_port_identifier != 0
		{
			if Self::is_valid(potential_ethernet_port_identifier)
			{
				return Some(EthernetPortIdentifier(potential_ethernet_port_identifier))
			}
			
			potential_ethernet_port_identifier -= 1;
		}
		None
	}
	
	/// Underlying DPDK type.
	#[inline(always)]
	pub fn ethernet_device(self) -> &'static rte_eth_dev
	{
		Self::ethernet_device_from_port_id(self.0)
	}
	
	/// Ethernet device capabilities
	#[inline(always)]
	pub fn ethernet_device_capabilities(self) -> EthernetDeviceCapabilities
	{
		let mut dpdk_information: rte_eth_dev_info = unsafe { uninitialized() };
		unsafe { rte_eth_dev_info_get(self.0, &mut dpdk_information) };
		
		EthernetDeviceCapabilities::from(dpdk_information, self.extended_statistic_names(), self.maximum_transmission_unit(), self.firmware_version())
	}
	
	/// Warning: This method will fail if the device is not PCI-based, eg is virtual.
	#[inline(always)]
	pub fn ethernet_device_as_pci_device(self) -> DpdkPciDevice
	{
		DpdkPciDevice::from(unsafe { NonNull::new_unchecked(self.ethernet_device().device) })
	}
	
	/// Warning: This method will fail if the device is not virtual, eg is PCI-based.
	#[inline(always)]
	pub fn ethernet_device_as_virtual_device(self) -> DpdkVirtualDevice
	{
		DpdkVirtualDevice::from(unsafe { NonNull::new_unchecked(self.ethernet_device().device) })
	}
	
	/// Warning: This method will fail if the device is not PCI-based, eg is virtual.
	#[inline(always)]
	pub fn ethernet_device_needs_link_status_interrupt(self) -> bool
	{
		self.ethernet_device_as_pci_device().driver().unwrap().flags().contains(DpdkPciDriverFlags::SupportsLinkStatusInterrupt)
	}
	
	// NOTE: Similar logic to rte_eth_dev_socket_id but inlined and without unnecessary checks (as we know 'self' is a valid ethernet port identifier already).
	/// NUMA node that ethernet port is associated with.
	#[inline(always)]
	pub fn numa_node_choice(self) -> NumaNodeChoice
	{
		NumaNodeChoice::from_i32(self.data().numa_node)
	}
	
	#[inline(always)]
	fn try_from_u16_unchecked(potential_ethernet_port_identifier: u16) -> Result<Self, ()>
	{
		if Self::is_invalid(potential_ethernet_port_identifier)
		{
			Err(())
		}
		else
		{
			let result = EthernetPortIdentifier(potential_ethernet_port_identifier);
			debug_assert_eq!(result.data().port_id, potential_ethernet_port_identifier, "Self consistency check failed");
			Ok(result)
		}
	}
	
	#[inline(always)]
	fn parse_find_result(potential_ethernet_port_identifier: u16, result: i32) -> Option<Self>
	{
		if result == 0
		{
			if Self::is_valid(potential_ethernet_port_identifier)
			{
				Some(EthernetPortIdentifier(potential_ethernet_port_identifier))
			}
			else
			{
				None
			}
		}
		else
		{
			None
		}
	}
	
	#[inline(always)]
	fn is_valid(potential_ethernet_port_identifier: u16) -> bool
	{
		if unlikely!(potential_ethernet_port_identifier >= Self::Maximum as u16)
		{
			return false
		}
		
		Self::ethernet_device_from_port_id(potential_ethernet_port_identifier).state == rte_eth_dev_state::RTE_ETH_DEV_ATTACHED
	}
	
	#[inline(always)]
	fn is_invalid(potential_ethernet_port_identifier: u16) -> bool
	{
		!Self::is_valid(potential_ethernet_port_identifier)
	}
	
	#[inline(always)]
	fn ethernet_device_from_port_id(port_id: u16) -> &'static rte_eth_dev
	{
		unsafe { &rte_eth_devices[port_id as usize] }
	}
	
	/// Underlying DPDK type.
	#[inline(always)]
	fn ethernet_device_mutable(self) -> &'static mut rte_eth_dev
	{
		unsafe { &mut rte_eth_devices[self.0 as usize] }
	}
	
	#[inline(always)]
	fn receive_queue_information(self, receive_queue_identifier: ReceiveQueueIdentifier) -> rte_eth_rxq_info
	{
		let mut receive_queue_information = unsafe { zeroed() };
		unsafe { (self.device_operations().rxq_info_get.expect("rxq_info_get is unsupported"))(self.ethernet_device_mutable(), receive_queue_identifier.into(), &mut receive_queue_information) };
		receive_queue_information
	}
	
	#[inline(always)]
	fn receive_queues<'a>(self) -> &'a [*mut c_void]
	{
		let rx_queues = self.data().rx_queues;
		debug_assert!(!rx_queues.is_null(), "rx_queues are null");
		let number_of_receive_queues = self.data().nb_rx_queues;
		debug_assert_ne!(number_of_receive_queues, 0, "number_of_receive_queues is zero");
		unsafe { from_raw_parts(rx_queues, number_of_receive_queues as usize) }
	}
	
	#[inline(always)]
	fn receive_queue<'a>(self, queue_identifier: ReceiveQueueIdentifier) -> NonNull<c_void>
	{
		let into: usize = queue_identifier.into();
		let receive_queue: *mut c_void = self.receive_queues()[into];
		debug_assert!(!receive_queue.is_null(), "receive_queue is null");
		unsafe { NonNull::new_unchecked(receive_queue) }
	}
	
	#[allow(dead_code)]
	#[inline(always)]
	fn transmit_queue_information(self, queue_identifier: TransmitQueueIdentifier) -> rte_eth_txq_info
	{
		let mut transmit_queue_information = unsafe { zeroed() };
		unsafe { (self.device_operations().txq_info_get.expect("txq_get is unsupported"))(self.ethernet_device_mutable(), queue_identifier.into(), &mut transmit_queue_information) };
		transmit_queue_information
	}
	
	#[inline(always)]
	fn transmit_queues<'a>(self) -> &'a [*mut c_void]
	{
		let tx_queues = self.data().tx_queues;
		debug_assert!(!tx_queues.is_null(), "tx_queues are null");
		let number_of_transmit_queues = self.data().nb_tx_queues;
		debug_assert_ne!(number_of_transmit_queues, 0, "number_of_transmit_queues is zero");
		unsafe { from_raw_parts(tx_queues, number_of_transmit_queues as usize) }
	}
	
	#[inline(always)]
	fn transmit_queue<'a>(self, queue_identifier: TransmitQueueIdentifier) -> NonNull<c_void>
	{
		let into: usize = queue_identifier.into();
		let transmit_queue: *mut c_void = self.transmit_queues()[into];
		debug_assert!(!transmit_queue.is_null(), "transmit_queue is null");
		unsafe { NonNull::new_unchecked(transmit_queue) }
	}
	
	#[inline(always)]
	fn device_operations<'a>(self) -> &'a eth_dev_ops
	{
		let dev_ops = self.ethernet_device().dev_ops;
		debug_assert!(!dev_ops.is_null(), "dev_ops is null");
		
		unsafe { & * dev_ops }
	}
	
	#[inline(always)]
	fn data(self) -> &'static mut rte_eth_dev_data
	{
		let data = self.ethernet_device_mutable().data;
		debug_assert!(!data.is_null(), "data is null");
		
		unsafe { &mut * data }
	}
	
	#[inline(always)]
	fn firmware_version(self) -> Option<String>
	{
		let result = unsafe { rte_eth_dev_fw_version_get(self.0, null_mut(), 0) };
		if likely!(result > 0)
		{
			let size = result as usize;
			let mut buffer: Vec<u8> = Vec::with_capacity(size);
			unsafe { buffer.set_len(size) };
			let result = unsafe { rte_eth_dev_fw_version_get(self.0, buffer.as_mut_ptr() as *mut _, size) };
			
			if likely!(result == 0)
			{
				return Some(CStr::from_bytes_with_nul(&buffer[..]).unwrap().to_str().unwrap().to_owned())
			}
			
			match result
			{
				NegativeE::EIO => panic!("rte_eth_dev_fw_version_get for ethernet port '{}' reported device removed", self),
				
				_ => panic!("rte_eth_dev_fw_version_get for ethernet port '{}' returned an expected result '{}'", self, result)
			}
		}
		else
		{
			match result
			{
				NegativeE::ENOTSUP => None,
				
				NegativeE::ENODEV => panic!("rte_eth_dev_fw_version_get for ethernet port '{}' reported no device", self),
				NegativeE::EIO => panic!("rte_eth_dev_fw_version_get for ethernet port '{}' reported device removed", self),
				
				0 => panic!("Firmware version string should never be zero length including terminating NUL"),
				
				_ => panic!("rte_eth_dev_fw_version_get for ethernet port '{}' returned an expected result '{}'", self, result),
			}
		}
	}
	
	#[inline(always)]
	fn maximum_transmission_unit(self) -> MaximumTransmissionUnitSize
	{
		let mut maximum_transmission_unit = unsafe { uninitialized() };
		let result = unsafe { rte_eth_dev_get_mtu(self.0, &mut maximum_transmission_unit) };
		if likely!(result == 0)
		{
			MaximumTransmissionUnitSize::try_from(maximum_transmission_unit).expect("ethernet device very oddly has a maximum transmission unit (MTU) less than the RFC 791 minimum")
		}
		else if unlikely!(result > 0)
		{
			panic!("rte_eth_dev_get_mtu for ethernet port '{}' returned a positive result '{}'", self, result)
		}
		else
		{
			match result
			{
				NegativeE::ENODEV => panic!("rte_eth_dev_get_mtu for ethernet port '{}' reported no device", self),
				_ => panic!("rte_eth_dev_get_mtu for ethernet port '{}' returned an expected result '{}'", self, result),
			}
		}
	}
}

/// Configuration related functionality.
impl EthernetPortIdentifier
{
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
	
	/// Returns an error message and error number on failure.
	#[inline(always)]
	pub fn configure_flow_isolation(self) -> Result<(), (rte_flow_error, i32)>
	{
		let mut error = unsafe { zeroed() };
		
		const EnterIsolatedMode: i32 = 1;
		
		let result = unsafe { rte_flow_isolate(self.0, EnterIsolatedMode, &mut error) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else if unlikely!(result > 0)
		{
			panic!("result of rte_flow_isolate was positive '{}'", result)
		}
		else
		{
			Err((error, LogicalCore::current_logical_core_error_number()))
		}
	}
	
//	/// Configure an ethernet device's global hash function.
//	#[inline(always)]
//	pub fn configure_receive_side_scaling_hash_function(self)
//	{
//		let mut configuration = rte_eth_hash_filter_info
//		{
//			info_type: rte_eth_hash_filter_info_type::RTE_ETH_HASH_FILTER_GLOBAL_CONFIG,
//			info:
//			{
//				let mut info = rte_eth_hash_filter_info_1::default();
//				let global_conf: &mut rte_eth_hash_global_conf = info.global_conf.as_mut();
//				global_conf.hash_func = rte_eth_hash_function::RTE_ETH_HASH_FUNCTION_TOEPLITZ;
//				global_conf.sym_hash_enable_mask = [0];
//				global_conf.valid_bit_mask = [::std::u64::MAX];
//				info
//			},
//		};
//
//		let result = unsafe { rte_eth_dev_filter_ctrl(self.0, rte_filter_type::RTE_ETH_FILTER_HASH, rte_filter_op::RTE_ETH_FILTER_SET, &mut configuration as *mut rte_eth_hash_filter_info as *mut _) };
//
//		if likely!(result == 0)
//		{
//			return
//		}
//
//		match result
//		{
//			NegativeE::ENODEV => panic!("This ethernet port '{}' is not a device", self),
//			NegativeE::ENOTSUP => panic!("rte_eth_dev_filter_ctrl global hash filter setting is not supported"),
//			NegativeE::EIO => panic!("This ethernet port '{}' is removed", self),
//
//			_ => panic!("rte_eth_dev_filter_ctrl global hash filter setting error '{}'", result),
//		}
//	}
	
	/// Configure an ethernet device.
	#[inline(always)]
	pub fn configure_ethernet_device<'a>(self, ethernet_device_capabilities: &EthernetDeviceCapabilities, number_of_receive_queues: TransmitNumberOfQueues, number_of_transmit_queues: TransmitNumberOfQueues, receive_side_scaling_hash_key: Option<&mut ReceiveSideScalingHashKey<'a>>)
	{
		use self::rte_eth_rx_mq_mode::*;
		use self::rte_eth_tx_mq_mode::*;
		use self::rte_fdir_mode::*;
		use self::rte_fdir_pballoc_type::*;
		use self::rte_fdir_status_mode::*;
		
		let device_receive_offloads =
		{
			let offload_jumbo_frames_bit = if ethernet_device_capabilities.maximum_receive_packet_length().implies_jumbo_frames()
			{
				ReceiveHardwareOffloadingFlags::common_flags()
			}
			else
			{
				ReceiveHardwareOffloadingFlags::common_flags_with_jumbo_frames_support()
			};
			
			ethernet_device_capabilities.receive_device_hardware_offloading_flags() & offload_jumbo_frames_bit
		};
		
		let device_transmit_offloads = ethernet_device_capabilities.transmit_device_hardware_offloading_flags() & TransmitHardwareOffloadingFlags::common_flags();
		
		// TODO: If using the flow API, does this matter?
		let (mq_mode, rss_conf) = match receive_side_scaling_hash_key
		{
			None => (ETH_MQ_RX_NONE, unsafe { zeroed() }),
			Some(receive_side_scaling_hash_key) =>
			{
				let (pointer, length) = receive_side_scaling_hash_key.pointer_and_length();
				let rss_conf = rte_eth_rss_conf
				{
					rss_key: pointer,
					rss_key_len: length,
					rss_hf: ethernet_device_capabilities.receive_side_scaling_offload_flow().bits(),
				};
				(ETH_MQ_RX_RSS, rss_conf)
			}
		};
		
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
					mq_mode,
					
					max_rx_pkt_len: ethernet_device_capabilities.maximum_receive_packet_length().into(),
					
					split_hdr_size: 0,
					
					offloads: device_receive_offloads.bits(),
					
					bitfield_1:
					{
						let legacy_value = BindgenBitfieldUnit::new(unsafe { zeroed() });
						legacy_value
					},
					
					__bindgen_padding_0: unsafe { uninitialized() },
				};
				rxmode.set_ignore_offload_bitfield(1);
				rxmode
			},
			
			// TODO: If using the flow API, does this matter?
			rx_adv_conf: rte_eth_conf_1
			{
				rss_conf,
				
				vmdq_dcb_conf: unsafe { zeroed() },
				
				dcb_rx_conf: unsafe { zeroed() },
				
				vmdq_rx_conf: unsafe { zeroed() },
			},
			
			txmode: rte_eth_txmode
			{
				mq_mode: ETH_MQ_TX_NONE,
				
				offloads: device_transmit_offloads.bits(),
				
				pvid:
				{
					const NoPortBasedVirtualLanInsertionAsMostNicsDoNotSupportIt: u16 = 0;
					NoPortBasedVirtualLanInsertionAsMostNicsDoNotSupportIt
				},
				
				bitfield_1:
				{
					const DisableHardwareVirtualLanRejectTagged: u8 = 0;
					const DisableHardwareVirtualLanRejectUntagged: u8 = 0;
					const DisableHardwareVirtualLanInsertPortBasedVirtualLanIdentifier: u8 = 0;
					rte_eth_txmode::newbitfield_1(DisableHardwareVirtualLanRejectTagged, DisableHardwareVirtualLanRejectUntagged, DisableHardwareVirtualLanInsertPortBasedVirtualLanIdentifier)
				},
				
				__bindgen_padding_0: unsafe { uninitialized() },
			},
			
			tx_adv_conf: unsafe { zeroed() },
			
			dcb_capability_en:
			{
				const DisableDataCentreBridgingCapabilityAsMostNicsDoNotSupportIt: u32 = 0;
				DisableDataCentreBridgingCapabilityAsMostNicsDoNotSupportIt
			},
			
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
				const EnableLinkStatusInterrupt: u32 = 1;
				rte_intr_conf
				{
					bitfield_1: rte_intr_conf::newbitfield_1(EnableLinkStatusInterrupt, 0, 0),
					__bindgen_padding_0: unsafe { uninitialized() },
					__bindgen_align: unsafe { uninitialized() },
				}
			},
		};
		
		let result = unsafe { rte_eth_dev_configure(self.0, number_of_receive_queues.into(), number_of_transmit_queues.into(), &ethernet_configuration) };
		if likely!(result == 0)
		{
			return
		}
		else if likely!(result < 0)
		{
			panic!("rte_eth_dev_configure configure failed with code '{}'", result)
		}
		else
		{
			panic!("rte_eth_dev_configure configure failed with unexpected positive code '{}'", result)
		}
	}
	
	/// Configure the redirection table.
	#[inline(always)]
	pub fn configure_redirection_table(self, redirection_table: &mut RedirectionTable)
	{
		redirection_table.configure(self)
	}
	
	/// Configure a transmit queue.
	///
	/// Should only be called after configuring the network card and before starting it.
	#[inline(always)]
	pub fn configure_transmit_queue(self, queue_identifier: TransmitQueueIdentifier, transmit_queue_configuration: &TransmitQueueConfiguration, default_ethernet_device_transmit_queue_capabilities: &EthernetDeviceTransmitQueueCapabilities) -> TransmitBurst
	{
		transmit_queue_configuration.configure(self, queue_identifier, default_ethernet_device_transmit_queue_capabilities)
	}
	
	/// Configure a receive queue.
	///
	/// Should only be called after configuring the network card and before starting it.
	///
	/// `queue_packet_buffer_pool` should ideally be on the numa node `receive_queue_configuration.queue_ring_numa_node`.
	#[inline(always)]
	pub fn configure_receive_queue(self, queue_identifier: ReceiveQueueIdentifier, receive_queue_configuration: &ReceiveQueueConfiguration, default_ethernet_device_receive_queue_capabilities: &EthernetDeviceReceiveQueueCapabilities, queue_packet_buffer_pool: NonNull<rte_mempool>) -> ReceiveBurst
	{
		receive_queue_configuration.configure(self, queue_identifier, default_ethernet_device_receive_queue_capabilities, queue_packet_buffer_pool)
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
}

/// Life cycle; start and stop like behaviours.
impl EthernetPortIdentifier
{
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
	
	/// Destroys all flow rules.
	///
	/// Returns an error message and error number on failure.
	#[inline(always)]
	pub fn destroy_all_flow_rules(self) -> Result<(), (rte_flow_error, i32)>
	{
		let mut error = unsafe { uninitialized() };
		
		let result = unsafe { rte_flow_flush(self.0, &mut error) };
		
		if likely!(result == 0)
		{
			Ok(())
		}
		else if unlikely!(result > 0)
		{
			panic!("result of rte_flow_flush was positive '{}'", result)
		}
		else
		{
			Err((error, LogicalCore::current_logical_core_error_number()))
		}
	}
}

/// Statistics
impl EthernetPortIdentifier
{
	/// Get simple statistics.
	#[inline(always)]
	pub fn get_simple_statistics(self) -> Result<EthernetPortSimpleStatistics, i32>
	{
		let mut statistics = unsafe { uninitialized() };
		let result = unsafe { rte_eth_stats_get(self.0, &mut statistics) };
		if likely!(result == 0)
		{
			Ok(EthernetPortSimpleStatistics::from(statistics))
		}
		else
		{
			Err(result)
		}
	}
	
	/// Reset simple statistics.
	#[inline(always)]
	pub fn reset_simple_statistics(self)
	{
		// We ignore any errors as they don't contribute anything useful to the use of this functionality.
		unsafe { rte_eth_stats_reset(self.0) };
	}
	
	/// Obtain an `extended_statistics_iterator` from `EthernetDeviceCapabilities.extended_statistics_iterator()`.
	///
	/// Resets the `extended_statistics_iterator` with new statistics ready for iteration.
	#[inline(always)]
	pub fn get_extended_statistics<'a>(self, mut extended_statistics_iterator: ExtendedStatisticsIterator<'a>) -> ExtendedStatisticsIterator<'a>
	{
		let result = unsafe { rte_eth_xstats_get_by_id(self.0, null_mut(), extended_statistics_iterator.values_pointer(), 0) };
		
		if unlikely!(result < 0)
		{
			panic!("rte_eth_xstats_get failed with error '{}' when trying to retrieve extended statistics", result);
		}
		
		debug_assert!(result == extended_statistics_iterator.size() as i32, "result '{}' did not match number of extended statistics '{}' when trying to retrieve extended statistic names", result, extended_statistics_iterator.size());
		
		extended_statistics_iterator.reset();
		
		extended_statistics_iterator
	}
	
	/// Reset extended statistics.
	#[inline(always)]
	pub fn reset_extended_statistics(self)
	{
		unsafe { rte_eth_xstats_reset(self.0) }
	}
	
	#[inline(always)]
	fn extended_statistic_names(self) -> Vec<&'static str>
	{
		let number_of_extended_statistic_names = unsafe { rte_eth_xstats_get_names(self.0, null_mut(), 0) };
		if unlikely!(number_of_extended_statistic_names < 0)
		{
			panic!("rte_eth_xstats_get_names failed with error '{}' when trying to retrieve the number of extended statistic names", number_of_extended_statistic_names);
		}
		
		let number_of_extended_statistic_names_usize = number_of_extended_statistic_names as usize;
		
		let mut extended_statistic_c_names = Vec::with_capacity(number_of_extended_statistic_names_usize);
		let result = unsafe { rte_eth_xstats_get_names(self.0, extended_statistic_c_names.as_mut_ptr(), number_of_extended_statistic_names as u32) };
		if unlikely!(result < 0)
		{
			panic!("rte_eth_xstats_get_names failed with error '{}' when trying to retrieve extended statistic names", result);
		}
		debug_assert!(result == number_of_extended_statistic_names, "result '{}' did not match number_of_extended_statistic_names '{}' when trying to retrieve extended statistic names", result, number_of_extended_statistic_names);
		unsafe { extended_statistic_c_names.set_len(number_of_extended_statistic_names_usize) };
		
		let mut extended_statistic_names = Vec::with_capacity(number_of_extended_statistic_names_usize);
		for extended_statistic_c_name in extended_statistic_c_names.iter()
		{
			let c_name = unsafe { CStr::from_ptr((&extended_statistic_c_name.name[..]).as_ptr()) };
			let extended_statistic_name = c_name.to_str().unwrap();
			extended_statistic_names.push(extended_statistic_name);
		}
		
		extended_statistic_names
	}
}
