// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReceiveQueue
{
	portIdentifier: EthernetPortIdentifier,
	pub queueIdentifier: QueueIdentifier,
	startQueueWhenEthernetDeviceStarted: bool,
	pub numa_socket_id: Option<NumaSocketId>,
	pub packetBufferPool: PacketBufferPool,
}

// Another option is 32
pub const ReceiveQueueBurstBufferSize: usize = 4096;
pub const MaximumReceiveQueueBurstBufferSize: usize = 65_536;

impl ReceiveQueue
{
	#[inline(always)]
	pub fn new<Q: QueueMemoryConfiguration>(ethernetPortInformation: &EthernetPortInformation, queueIdentifier: QueueIdentifier, queueMemoryConfiguration: &Q, receiveQueueConfiguration: &ReceiveQueueConfiguration, failures: &mut EthernetPortConfigurationFailures) -> Option<ReceiveQueue>
	{
		debug_assert!((queueIdentifier as usize) <= MaximumReceiveQueues, "queueIdentifier '{}' exceeds MaximumReceiveQueues '{}'", queueIdentifier, MaximumReceiveQueues);

		let numberOfReceiveDescriptorsForTheReceiveRingAlsoKnownAsRingSize = receiveQueueConfiguration.numberOfReceiveDescriptorsForTheReceiveRingAlsoKnownAsRingSize;
		let (receiveQueueDescriptorsDmaMemoryAllocatedFromNumaSocketId, packetBufferPool) = queueMemoryConfiguration.receiveQueueDescriptorsDmaMemoryAllocatedFromNumaSocketIdAndMemoryPool(ethernetPortInformation, queueIdentifier, numberOfReceiveDescriptorsForTheReceiveRingAlsoKnownAsRingSize);

		let mut value = receiveQueueConfiguration.overrideDefaultDeviceConfiguration.as_ref().map(|deviceConfiguration| deviceConfiguration.as_rte_eth_rxconf(ethernetPortInformation.new_default_rxconf()));
		let configurationMutRefOption = value.as_mut();

		let pointer = if let Some(configuration) = configurationMutRefOption
		{
			configuration
		}
		else
		{
			null_mut()
		};

		let portIdentifier = ethernetPortInformation.portIdentifier();

		let result = unsafe
		{
			rte_eth_rx_queue_setup
			(
				portIdentifier,
				queueIdentifier,
				receiveQueueConfiguration.numberOfReceiveDescriptorsForTheReceiveRingAlsoKnownAsRingSize,
				receiveQueueDescriptorsDmaMemoryAllocatedFromNumaSocketId.as_c_uint(),
				pointer,
				packetBufferPool.memoryPool()
			)
		};
		if likely!(result == 0)
		{
			let mut receiveQueue = ReceiveQueue
			{
				portIdentifier,
				queueIdentifier,
				numa_socket_id: receiveQueueDescriptorsDmaMemoryAllocatedFromNumaSocketId,
				startQueueWhenEthernetDeviceStarted: receiveQueueConfiguration.startQueueWhenEthernetDeviceStarted(),
				packetBufferPool,
			};

			if let Some(enableVlanStripping) = receiveQueueConfiguration.enableVlanStripping
			{
				if let Err(error) = receiveQueue.changeVlanStripping(enableVlanStripping)
				{
					failures.push(EthernetPortConfigurationFailureKind::ReceiveQueueChangeVirtualLanStripping(queueIdentifier, error))
				}
			}

			Some(receiveQueue)
		}
		else
		{
			match result
			{
				NegativeE::ENOMEM => None,
				NegativeE::EINVAL => panic!("The size of network buffers which can be allocated from the memory pool does not fit the various buffer sizes allowed by the device controller"),

				_ => panic!("rte_eth_rx_queue_setup() returned unexpected result '{}'", result),
			}
		}
	}

	#[inline(always)]
	pub fn startIfDeferred(&self) -> Result<(), ()>
	{
		if self.startQueueWhenEthernetDeviceStarted
		{
			return Ok(());
		}
		let result = unsafe { rte_eth_dev_rx_queue_start(self.portIdentifier, self.queueIdentifier) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else
		{
			match result
			{
				NegativeE::ENOTSUP => Err(()),

				NegativeE::EINVAL => panic!("portIdentifier '{}' or queueIdentifier '{}' out of scope", self.portIdentifier, self.queueIdentifier),

				_ => panic!("Unexpected result '{}' from rte_eth_dev_rx_queue_start()", result),
			}
		}
	}

	#[inline(always)]
	pub fn stop(&self)
	{
		let result = unsafe { rte_eth_dev_rx_queue_stop(self.portIdentifier, self.queueIdentifier) };
		if likely!(result == 0)
		{
			()
		}
		else
		{
			match result
			{
				NegativeE::ENOTSUP => (),

				NegativeE::EINVAL => panic!("portIdentifier '{}' or queueIdentifier '{}' out of scope", self.portIdentifier, self.queueIdentifier),

				_ => panic!("Unexpected result '{}' from rte_eth_dev_rx_queue_stop()", result),
			}
		}
	}

	#[inline(always)]
	pub fn changeVlanStripping(&mut self, enable: bool) -> Result<(), UnsupportedByHardwareError>
	{
		let enable = if enable
		{
			1
		}
		else
		{
			0
		};

		let result = unsafe { rte_eth_dev_set_vlan_strip_on_queue(self.portIdentifier, self.queueIdentifier, enable) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else
		{
			match result
			{
				NegativeE::ENOTSUP => Err(UnsupportedByHardwareError::IsUnsupportedByTheHardware),

				NegativeE::ENODEV => panic!("portIdentifier '{}' is inbalid", self.portIdentifier),
				NegativeE::EINVAL => panic!("queueIdentifier '{}' is invalid", self.queueIdentifier),

				_ => panic!("Unexpected error code '{}' from second call to rte_eth_dev_set_vlan_strip_on_queue()", result),
			}
		}
	}

	#[inline(always)]
	pub fn setStatisticMappingIndex(&self, index: u8) -> bool
	{
		let result = unsafe { rte_eth_dev_set_rx_queue_stats_mapping(self.portIdentifier, self.queueIdentifier, index) };
		if likely!(result == 0)
		{
			true
		}
		else
		{
			match result
			{
				result if result < 0 => false,

				_ => panic!("Unexpected result '{}' from rte_eth_dev_set_rx_queue_stats_mapping()", result),
			}
		}
	}

	#[inline(always)]
	pub fn receiveBurstArrayVec(&self, readIntoPacketBuffer: &mut ArrayVec<[*mut rte_mbuf; ReceiveQueueBurstBufferSize]>) -> u16
	{
		let capacity = readIntoPacketBuffer.capacity();
		debug_assert!(capacity < MaximumReceiveQueueBurstBufferSize, "ReceiveQueueBurstBufferSize '{}' exceeds MaximumReceiveQueueBurstBufferSize '{}'", capacity, MaximumReceiveQueueBurstBufferSize);
		let remainder = capacity - ReceiveQueueBurstBufferSize;

		let numberRead = unsafe { rust_rte_eth_rx_burst(self.portIdentifier, self.queueIdentifier, readIntoPacketBuffer.as_mut_ptr(), remainder as u16) };
		let numberReadUsize = numberRead as usize;
		debug_assert!(numberReadUsize <= remainder, "numberRead '{}' exceeds remainder '{}'", numberRead, remainder);
		unsafe { readIntoPacketBuffer.set_len(ReceiveQueueBurstBufferSize + numberReadUsize) };

		numberRead as u16
	}

	#[inline(always)]
	pub fn receiveBurstVec(&self, readIntoPacketBuffer: &mut Vec<*mut rte_mbuf>) -> u16
	{
		let length = readIntoPacketBuffer.len();
		let capacity = readIntoPacketBuffer.capacity();
		debug_assert!(capacity < MaximumReceiveQueueBurstBufferSize, "readIntoPacketBuffer.capacity() '{}' exceeds MaximumReceiveQueueBurstBufferSize '{}'", capacity, MaximumReceiveQueueBurstBufferSize);
		let remainder = capacity - length;

		let numberRead = unsafe { rust_rte_eth_rx_burst(self.portIdentifier, self.queueIdentifier, readIntoPacketBuffer.as_mut_ptr(), remainder as u16) };
		let numberReadUsize = numberRead as usize;
		debug_assert!(numberReadUsize <= remainder, "numberRead '{}' exceeds remainder '{}'", numberRead, remainder);
		unsafe { readIntoPacketBuffer.set_len(length + numberReadUsize) };

		numberRead as u16
	}

	#[inline(always)]
	pub fn receiveBurstSlice(&self, readIntoPacketBuffer: &mut [*mut rte_mbuf]) -> u16
	{
		let length = readIntoPacketBuffer.len();
		debug_assert!(length < MaximumReceiveQueueBurstBufferSize, "readIntoPacketBuffer.len() '{}' exceeds MaximumReceiveQueueBurstBufferSize '{}'", length, MaximumReceiveQueueBurstBufferSize);

		unsafe { rust_rte_eth_rx_burst(self.portIdentifier, self.queueIdentifier, readIntoPacketBuffer.as_mut_ptr(), length as u16) }
	}

	#[inline(always)]
	pub fn receiveBurstArray(&self, readIntoPacketBuffer: &mut [*mut rte_mbuf; ReceiveQueueBurstBufferSize]) -> u16
	{
		unsafe { rust_rte_eth_rx_burst(self.portIdentifier, self.queueIdentifier, readIntoPacketBuffer.as_mut_ptr(), ReceiveQueueBurstBufferSize as u16) }
	}

	#[inline(always)]
	pub fn numberOfUsedReceiveDescriptors(&self) -> Result<u16, UnsupportedByHardwareError>
	{
		match unsafe { rust_rte_eth_rx_queue_count(self.portIdentifier, self.queueIdentifier) }
		{
			number if number >=0 && number <= ::std::u16::MAX as i32 => Ok(number as u16),

			NegativeE::ENOTSUP => Err(UnsupportedByHardwareError::IsUnsupportedByTheHardware),

			NegativeE::EINVAL => panic!("portIdentifier '{}' or queueIdentifier '{}' out of scope", self.portIdentifier, self.queueIdentifier),

			result @ _ => panic!("Unexpected result '{}' from rte_eth_rx_queue_count()", result),
		}
	}

	#[inline(always)]
	pub fn isDdBitOfReceiveDescriptorSet(&self, receiveDescriptorOffsetFromTail: u16) -> Result<bool, UnsupportedByHardwareError>
	{
		let result = unsafe { rust_rte_eth_rx_descriptor_done(self.portIdentifier, self.queueIdentifier, receiveDescriptorOffsetFromTail) };
		if likely!(result >= 0)
		{
			Ok(isTrue(result))
		}
		else
		{
			match result
			{
				NegativeE::ENOTSUP => Err(UnsupportedByHardwareError::IsUnsupportedByTheHardware),

				NegativeE::EINVAL => panic!("portIdentifier '{}' or queueIdentifier '{}' out of scope", self.portIdentifier, self.queueIdentifier),

				_ => panic!("Unexpected result '{}' from rte_eth_rx_descriptor_done()", result),
			}
		}
	}

	// See http://www.dpdk.org/doc/api/l3fwd-power_2main_8c-example.html
	/*
                        if (lcore_idle_hint < SUSPEND_THRESHOLD)
                                rte_delay_us(lcore_idle_hint);
                        else {
                                /* suspend until rx interrupt trigges */
                                if (intr_en) {
                                        turn_on_intr(qconf);
                                        sleep_until_rx_interrupt(
                                                qconf->n_rx_queue);
                                }
                                /* start receiving packets immediately */
                                goto start_rx;
                        }
	*/

	#[inline(always)]
	pub fn enableSleepWithInterrupt(&mut self) -> Result<(), UnsupportedByHardwareError>
	{
		let result = unsafe { rte_eth_dev_rx_intr_enable(self.portIdentifier, self.queueIdentifier) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else
		{
			match result
			{
				NegativeE::ENOTSUP => Err(UnsupportedByHardwareError::IsUnsupportedByTheHardware),

				NegativeE::EINVAL => panic!("portIdentifier '{}' or queueIdentifier '{}' out of scope", self.portIdentifier, self.queueIdentifier),

				_ => panic!("Unexpected result '{}' from rte_eth_dev_rx_intr_enable()", result),
			}
		}
	}

	#[inline(always)]
	pub fn disableSleepWithInterrupt(&mut self) -> Result<(), UnsupportedByHardwareError>
	{
		let result = unsafe { rte_eth_dev_rx_intr_disable(self.portIdentifier, self.queueIdentifier) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else
		{
			match result
			{
				NegativeE::ENOTSUP => Err(UnsupportedByHardwareError::IsUnsupportedByTheHardware),

				NegativeE::EINVAL => panic!("portIdentifier '{}' or queueIdentifier '{}' out of scope", self.portIdentifier, self.queueIdentifier),

				_ => panic!("Unexpected result '{}' from rte_eth_dev_rx_intr_disable()", result),
			}
		}
	}

	// userData could, be say, &self, or a structure holding self
	#[inline(always)]
	pub fn receiveInterruptEpollControl(&self, epollFileDescriptor: Option<i32>, ePollInterruptEvent: EPollInterruptEvent, userData: *mut c_void) -> bool
	{
		let epollFileDescriptor: i32 = epollFileDescriptor.unwrap_or(RTE_EPOLL_PER_THREAD);
		let result = unsafe { rte_eth_dev_rx_intr_ctl_q(self.portIdentifier, self.queueIdentifier, epollFileDescriptor, ePollInterruptEvent as i32, userData) };
		if likely!(result == 0)
		{
			true
		}
		else
		{
			match result
			{
				negative if negative < 0 => false,

				_ => panic!("Illegal result '{}' from rte_eth_dev_rx_intr_ctl_q()"),
			}
		}
	}

	#[inline(always)]
	pub fn getInformation(&self) -> Result<rte_eth_rxq_info, UnsupportedByHardwareError>
	{
		let mut information = unsafe { uninitialized() };

		let result = unsafe { rte_eth_rx_queue_info_get(self.portIdentifier, self.queueIdentifier, &mut information) };
		if likely!(result == 0)
		{
			Ok(information)
		}
		else
		{
			forget(information);

			match result
			{
				NegativeE::ENOTSUP => Err(UnsupportedByHardwareError::IsUnsupportedByTheHardware),

				NegativeE::EINVAL => panic!("portIdentifier '{}' or queueIdentifier '{}' out of scope", self.portIdentifier, self.queueIdentifier),

				_ => panic!("Unexpected error code '{}' from rte_eth_rx_queue_info_get()", result),
			}
		}
	}

	/*
		rte_eth_add_rx_callback QUEUE
		rte_eth_remove_rx_callback QUEUE
		rte_eth_add_first_rx_callback(port_id: uint8_t, queue_id: uint16_t, fn_: rte_rx_callback_fn, user_param: *mut c_void) -> *mut c_void   (removed with rte_eth_remove_rx_callback)
	*/
}
