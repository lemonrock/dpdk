// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TransmitQueue
{
	pub portIdentifier: EthernetPortIdentifier,
	pub queueIdentifier: QueueIdentifier,
	startQueueWhenEthernetDeviceStarted: bool,
	pub numa_socket_id: Option<NumaSocketId>,
	pub packetBufferPool: PacketBufferPool,
}

impl TransmitQueue
{
	#[inline(always)]
	pub fn new<Q: QueueMemoryConfiguration>(ethernetPortInformation: &EthernetPortInformation, queueIdentifier: QueueIdentifier, queueMemoryConfiguration: &Q, transmitQueueConfiguration: &TransmitQueueConfiguration, failures: &mut EthernetPortConfigurationFailures) -> Option<TransmitQueue>
	{
		debug_assert!((queueIdentifier as usize) <= MaximumTransmitQueues, "queueIdentifier '{}' exceeds MaximumTransmitQueues '{}'", queueIdentifier, MaximumTransmitQueues);

		let mut value = transmitQueueConfiguration.overrideDefaultDeviceConfiguration.as_ref().map(|deviceConfiguration| deviceConfiguration.as_rte_eth_txconf(ethernetPortInformation.new_default_txconf()));
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
		let numberOfTransmitDescriptorsForTheTransmitRingAlsoKnownAsRingSize = transmitQueueConfiguration.numberOfTransmitDescriptorsForTheTransmitRingAlsoKnownAsRingSize;
		let (transmitQueueDescriptorsDmaMemoryAllocatedFromNumaSocketId, packetBufferPool) = queueMemoryConfiguration.receiveQueueDescriptorsDmaMemoryAllocatedFromNumaSocketIdAndMemoryPool(ethernetPortInformation, queueIdentifier, numberOfTransmitDescriptorsForTheTransmitRingAlsoKnownAsRingSize);

		let result = unsafe
		{
			rte_eth_tx_queue_setup
			(
				portIdentifier,
				queueIdentifier,
				transmitQueueConfiguration.numberOfTransmitDescriptorsForTheTransmitRingAlsoKnownAsRingSize,
				transmitQueueDescriptorsDmaMemoryAllocatedFromNumaSocketId.as_c_uint(),
				pointer
			)
		};

		if likely!(result == 0)
		{
			let mut transmitQueue = TransmitQueue
			{
				portIdentifier,
				queueIdentifier,
				startQueueWhenEthernetDeviceStarted: transmitQueueConfiguration.startQueueWhenEthernetDeviceStarted(),
				numa_socket_id: transmitQueueDescriptorsDmaMemoryAllocatedFromNumaSocketId,
				packetBufferPool,
			};

			if let Some(maximumTransmissionRateInMbps) = transmitQueueConfiguration.maximumTransmissionRateInMbps
			{
				if let Err(error) = transmitQueue.setRateLimit(maximumTransmissionRateInMbps)
				{
					failures.push(EthernetPortConfigurationFailureKind::TransmitQueueSetRateLimit(queueIdentifier, error))
				}
			}

			Some(transmitQueue)
		}
		else
		{
			match result
			{
				NegativeE::ENOMEM => None,

				_ => panic!("rte_eth_tx_queue_setup() returned unexpected result '{}'", result),
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

		let result = unsafe { rte_eth_dev_tx_queue_start(self.portIdentifier, self.queueIdentifier) };
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

				_ => panic!("Unexpected result '{}' from rte_eth_dev_tx_queue_start()", result),
			}
		}
	}

	#[inline(always)]
	pub fn stop(&self)
	{
		let result = unsafe { rte_eth_dev_tx_queue_stop(self.portIdentifier, self.queueIdentifier) };
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

				_ => panic!("Unexpected result '{}' from rte_eth_dev_tx_queue_stop()", result),
			}
		}
	}

	#[inline(always)]
	pub fn setStatisticMappingIndex(&mut self, index: u8) -> bool
	{
		let result = unsafe { rte_eth_dev_set_tx_queue_stats_mapping(self.portIdentifier, self.queueIdentifier, index) };
		if likely!(result == 0)
		{
			true
		}
		else
		{
			match result
			{
				result if result < 0 => false,

				_ => panic!("Unexpected result '{}' from rte_eth_dev_set_tx_queue_stats_mapping()", result),
			}
		}
	}

	#[inline(always)]
	pub fn transmitInBurst4096(&self, writeFromPacketBuffer: &mut [*mut rte_mbuf; 4096], inclusiveIndexFrom: u16, length: u16) -> u16
	{
		debug_assert!(inclusiveIndexFrom + length <= 4096, "inclusiveFrom '{}' + length '{}' <= 4096", inclusiveIndexFrom, length);

		unsafe { rust_rte_eth_tx_burst(self.portIdentifier, self.queueIdentifier, writeFromPacketBuffer.as_mut_ptr().offset(inclusiveIndexFrom as isize), length) }
	}

	// transmissionRateInMbps must be equal to or less than the total port link speed
	#[inline(always)]
	pub fn setRateLimit(&mut self, maximumTransmissionRateInMbps: u16) -> Result<(), UnsupportedByHardwareError>
	{
		let result = unsafe { rte_eth_set_queue_rate_limit(self.portIdentifier, self.queueIdentifier, maximumTransmissionRateInMbps) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else
		{
			match result
			{
				NegativeE::ENOTSUP => Err(UnsupportedByHardwareError::IsUnsupportedByTheHardware),

				NegativeE::ENODEV => panic!("portIdentifier '{}' or queueIdentifier '{}' out of scope", self.portIdentifier, self.queueIdentifier),
				NegativeE::EINVAL => panic!("Bad maximumTransmissionRateInMbps parameter '{}'", maximumTransmissionRateInMbps),

				_ => panic!("Unexpected error code '{}' from second call to rte_eth_dev_flow_ctrl_set()", result),
			}
		}
	}

	#[inline(always)]
	pub fn getInformation(&self) -> Result<rte_eth_txq_info, UnsupportedByHardwareError>
	{
		let mut information = unsafe { uninitialized() };

		let result = unsafe { rte_eth_tx_queue_info_get(self.portIdentifier, self.queueIdentifier, &mut information) };
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

				_ => panic!("Unexpected error code '{}' from rte_eth_tx_queue_info_get()", result),
			}
		}
	}
}
