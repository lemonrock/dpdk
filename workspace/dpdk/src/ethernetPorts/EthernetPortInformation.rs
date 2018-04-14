// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[allow(missing_debug_implementations)]
#[allow(dead_code)]
pub struct EthernetPortInformation
{
	ethernetPortIdentifier: EthernetPortIdentifier,
	ethernetPort: EthernetPort,
	driverName: String,
	deviceInformation: rte_eth_dev_info,
	underlying_ethernet_device: rte_eth_dev,

	requiresIntelI40eEightFragmentsWorkaround: bool,
	requiresVmWareVmxNet3SixteenFragmentsWorkaround: bool,

	parentNumaSocketId: Option<NumaSocketId>,
	maximumNumberOfReceiveThenTransmitQueuePairs: u16,
	receiveSideScalingRetaIndirectionTableNumberOfBits_and_supportedReceiveSideScalingHashKeySize: Option<(PowerOfTwoSixteenBit, ReceiveSideScalingHashKeySize)>,
	logicalCoreUser: LogicalCoreUser,

	supportedFilterTypes: HashSet<FilterType>,
	deviceReceiveOffloadCapabilities: DeviceReceiveOffloadCapabilities,
	deviceTransmitOffloadCapabilities: DeviceTransmitOffloadCapabilities,

	receiveLargeReceiveOffloadSupported: bool, // aka LRO
	receiveIpV4TcpAndUdpChecksumOffloadSupported: bool,
	transmitTcpSegmentationOffloadSupported: bool, // aka TSO
	transmitUdpSegmentationOffloadSupported: bool, // aka UFO
	transmitIpV4ChecksumOffloadSupported: bool,
	transmitTcpAndUdpChecksumOffloadSupported: bool,

	device_name: String,
	dataCentreBridgingInformation: Option<rte_eth_dcb_info>,
	eepromSize: Option<u31>,
	eepromInformation: Option<EepromInformation>,
	deviceRegisters: Option<(u32, u32, u32, Vec<u8>)>,
}

impl EthernetPortInformation
{
	#[inline(always)]
	pub fn underlying_ethernet_device(&self) -> &rte_eth_dev
	{
		&self.underlying_ethernet_device
	}

	#[inline(always)]
	pub fn device_name(&self) -> &str
	{
		&self.device_name
	}

	#[inline(always)]
	pub fn portIdentifier(&self) -> EthernetPortIdentifier
	{
		self.ethernetPortIdentifier
	}

	#[inline(always)]
	pub fn media_access_control_address(&self) -> MediaAccessControlAddress
	{
		self.ethernetPort().getDefaultMediaAccessControlAddress()
	}

	#[inline(always)]
	pub fn ethernetPort(&self) -> &EthernetPort
	{
		&self.ethernetPort
	}

	#[inline(always)]
	pub fn useLogicalCoreUser(&mut self) -> &mut LogicalCoreUser
	{
		&mut self.logicalCoreUser
	}

	#[inline(always)]
	pub fn waitUntilLinkIsUp(&self) -> Result<LinkStatus, ()>
	{
		self.ethernetPort.linkStatusWaitingUpToNineSeconds()
	}

	#[inline(always)]
	pub fn receiveIpV4TcpAndUdpChecksumOffloadSupported(&self) -> bool
	{
		self.receiveIpV4TcpAndUdpChecksumOffloadSupported
	}

	#[inline(always)]
	pub fn createReceiveQueueConfigurations(&self, receiveQueueConfigurationTemplate: &ReceiveQueueConfiguration) -> ArrayVec<[ReceiveQueueConfiguration; MaximumReceiveQueues]>
	{
		let size = self.logicalCoreUser.numberOfReceiveThenTransmitQueuePairs() as usize;
		let mut receiveQueueConfigurations = ArrayVec::new();
		for _ in 0..size
		{
			receiveQueueConfigurations.push(receiveQueueConfigurationTemplate.clone());
		}
		receiveQueueConfigurations
	}

	#[inline(always)]
	pub fn createTransmitQueueConfigurations(&self, transmitQueueConfigurationTemplate: &TransmitQueueConfiguration) -> ArrayVec<[TransmitQueueConfiguration; MaximumTransmitQueues]>
	{
		let size = self.logicalCoreUser.numberOfReceiveThenTransmitQueuePairs() as usize;
		let mut transmitQueueConfigurations = ArrayVec::new();
		for _ in 0..size
		{
			transmitQueueConfigurations.push(transmitQueueConfigurationTemplate.clone());
		}
		transmitQueueConfigurations
	}

	#[inline(always)]
	pub fn logicalCoreFor(&self, queueIdentifier: QueueIdentifier) -> LogicalCore
	{
		*(self.logicalCoreUser.logicalCore(queueIdentifier).unwrap())
	}

	#[inline(always)]
	pub fn startProcessingQueuePairs<Creator: ExecutionRoutineCreatorCreator<D, EC>, D, EC: ExecutionRoutineCreator>(&self, executionRoutineCreatorCreator: &Creator, data: Arc<D>)
	-> Arc<Mutex<ExecutionRoutineGroup<ReceiveTransmitQueuePairSlaveLogicalCoreTask<EC>>>>
	{
		debug_assert!(LogicalCore::isCurrentMaster(), "Can not call tasks() on a slave logical core");

		let taskCount = self.logicalCoreUser.numberOfReceiveThenTransmitQueuePairs() as usize;

		let executionRoutineGroupWrapped = ExecutionRoutineGroup::new(taskCount);
		{
			let mut executionRoutineGroup = executionRoutineGroupWrapped.lock().unwrap();

			for queueIdentifier in 0..(taskCount as QueueIdentifier)
			{
				let slaveLogicalCoreToExecuteOn = self.logicalCoreFor(queueIdentifier);

				let task = ReceiveTransmitQueuePairSlaveLogicalCoreTask::new(executionRoutineGroup.canContinueClone(), executionRoutineGroupWrapped.clone(), executionRoutineCreatorCreator, data.clone(), queueIdentifier, slaveLogicalCoreToExecuteOn, self);
				executionRoutineGroup.pushAndRunOnSlave(task);
			}
		}

		executionRoutineGroupWrapped
	}

	#[inline(always)]
	pub fn adjustConfiguration<'a, ReceiveSideScalingHashChooser: Fn(u16, ReceiveSideScalingHashKeySize, &'a str, &'a str) -> (Option<HashFilter>, ReceiveSideScalingHashFunctionConfiguration)>(&'a self, configuration: &mut EthernetPortConfiguration, receiveSideScalingHashChooser: &ReceiveSideScalingHashChooser)
	{
		self.adjustReceiveSideScalingConfiguration(configuration, receiveSideScalingHashChooser);

		let receiveModeConfiguration = &mut configuration.receiveModeConfiguration;
		self.adjustLargeReceiveOffloadConfiguration(receiveModeConfiguration);
		self.adjustReceiveVlanStripping(receiveModeConfiguration);
		Self::enableHardwareCrcStripping(receiveModeConfiguration);
		self.adjustReceiveChecksumOffloadingConfiguration(receiveModeConfiguration);
	}

	#[inline(always)]
	fn adjustReceiveSideScalingConfiguration<'a, ReceiveSideScalingHashChooser: Fn(u16, ReceiveSideScalingHashKeySize, &'a str, &'a str) -> (Option<HashFilter>, ReceiveSideScalingHashFunctionConfiguration)>(&'a self, configuration: &mut EthernetPortConfiguration, receiveSideScalingHashChooser: &ReceiveSideScalingHashChooser)
	{
		let numberOfReceiveThenTransmitQueuePairs = self.logicalCoreUser.numberOfReceiveThenTransmitQueuePairs();
		assert_ne!(numberOfReceiveThenTransmitQueuePairs, 0, "numberOfReceiveThenTransmitQueuePairs can not be zero");

		configuration.restrictNumberOfQueuePairsToMaximum(numberOfReceiveThenTransmitQueuePairs);

		let receiveModeConfiguration = &mut configuration.receiveModeConfiguration;

		// Receive Side Scaling isn't possible if not supported or there is only one queue pair
		if self.receiveSideScalingRetaIndirectionTableNumberOfBits_and_supportedReceiveSideScalingHashKeySize.is_none() || numberOfReceiveThenTransmitQueuePairs == 1
		{
			receiveModeConfiguration.disableReceiveSideScaling();
			configuration.receiveSideScalingHashFunctionConfiguration = ReceiveSideScalingHashFunctionConfiguration::NoneEthernetDeviceDefault;
			configuration.receiveSideScalingHashFilter = None;
			configuration.receiveSideScalingRetaIndirectionTable = None;
		}
		else
		{
			let (receiveSideScalingRetaIndirectionTableNumberOfBits, supportedReceiveSideScalingHashKeySize) = self.receiveSideScalingRetaIndirectionTableNumberOfBits_and_supportedReceiveSideScalingHashKeySize.unwrap();
			let (receiveSideScalingHashFilter, receiveSideScalingHashFunctionConfiguration) = receiveSideScalingHashChooser(numberOfReceiveThenTransmitQueuePairs, supportedReceiveSideScalingHashKeySize, &self.driverName, &self.device_name);

			receiveModeConfiguration.enableReceiveSideScaling();
			configuration.receiveSideScalingHashFunctionConfiguration = receiveSideScalingHashFunctionConfiguration;
			configuration.receiveSideScalingHashFilter = receiveSideScalingHashFilter;
			configuration.receiveSideScalingRetaIndirectionTable = Some(ReceiveSideScalingRetaIndirectionTable::new(receiveSideScalingRetaIndirectionTableNumberOfBits, numberOfReceiveThenTransmitQueuePairs));
		}
	}

	#[inline(always)]
	fn adjustLargeReceiveOffloadConfiguration(&self, receiveModeConfiguration: &mut EthernetPortReceiveModeConfiguration)
	{
		if self.receiveLargeReceiveOffloadSupported
		{
			receiveModeConfiguration.enableTcpLargeReceiveOffload()
		}
		else
		{
			receiveModeConfiguration.disableTcpLargeReceiveOffload()
		}
	}

	#[inline(always)]
	fn adjustReceiveVlanStripping(&self, receiveModeConfiguration: &mut EthernetPortReceiveModeConfiguration)
	{
		if self.deviceReceiveOffloadCapabilities.supportsVlanOrQinQStripping()
		{
			receiveModeConfiguration.enableHardwareVlanStripping()
		}
		else
		{
			receiveModeConfiguration.disableHardwareVlanStripping()
		}
	}

	#[inline(always)]
	fn enableHardwareCrcStripping(receiveModeConfiguration: &mut EthernetPortReceiveModeConfiguration)
	{
		// ThunderX can't disable HW CRC stripping
		// Many Intel VF drivers can't change HW CRC stripping
		// Default is to try to have it enabled
		receiveModeConfiguration.enableHardwareCyclicRedundancyChecksumStripping()
	}

	#[inline(always)]
	fn adjustReceiveChecksumOffloadingConfiguration(&self, receiveModeConfiguration: &mut EthernetPortReceiveModeConfiguration)
	{
		if self.receiveIpV4TcpAndUdpChecksumOffloadSupported
		{
			receiveModeConfiguration.enableIpV4TcpAndUdpChecksumOffload()
		}
		else
		{
			receiveModeConfiguration.disableIpV4TcpAndUdpChecksumOffload()
		}
	}

	#[inline(always)]
	pub fn new_default_rxconf(&self) -> rte_eth_rxconf
	{
		self.deviceInformation.default_rxconf
	}

	#[inline(always)]
	pub fn new_default_txconf(&self) -> rte_eth_txconf
	{
		self.deviceInformation.default_txconf
	}

	#[inline(always)]
	pub fn new
	(
		ethernetPort: EthernetPort,
		deviceInformation: rte_eth_dev_info,
		underlying_ethernet_device: rte_eth_dev,
		device_name: String,
		parentNumaSocketId: Option<NumaSocketId>,
		supportedFilterTypes: HashSet<FilterType>,
		dataCentreBridgingInformation: Option<rte_eth_dcb_info>,
		eepromSize: Option<u31>,
		eepromInformation: Option<EepromInformation>,
		deviceRegisters: Option<(u32, u32, u32, Vec<u8>)>) -> Self
	{
		let driverName = unsafe { CStr::from_ptr(deviceInformation.driver_name) }.to_str().expect("deviceInformation.driver_name contains non-Unicode data");

		let deviceReceiveOffloadCapabilities = DeviceReceiveOffloadCapabilities::from_bits(deviceInformation.rx_offload_capa).expect("Unsupported rx_offload_capa value");

		let deviceTransmitOffloadCapabilities = DeviceTransmitOffloadCapabilities::from_bits(deviceInformation.tx_offload_capa).expect("Unsupported tx_offload_capa value");

		fn deviceMaximumReceiveQueuesInclusive(deviceInformation: &rte_eth_dev_info, device_name: &str) -> u16
		{
			let deviceMaximumReceiveQueuesInclusive = deviceInformation.max_rx_queues;

			assert_ne!(deviceMaximumReceiveQueuesInclusive, 0, "deviceMaximumReceiveQueuesInclusive is zero; this shouldn't be possible but makes this device '{}' useless", device_name);
			assert!(deviceMaximumReceiveQueuesInclusive <= MaximumReceiveQueues as u16, "deviceMaximumReceiveQueuesInclusive '{}' exceeds MaximumReceiveQueues '{}' device '{}'", deviceMaximumReceiveQueuesInclusive, MaximumReceiveQueues, device_name);

			deviceMaximumReceiveQueuesInclusive
		}

		fn deviceMaximumTransmitQueuesInclusive(deviceInformation: &rte_eth_dev_info, device_name: &str) -> u16
		{
			let deviceMaximumTransmitQueuesInclusive = deviceInformation.max_tx_queues;

			assert_ne!(deviceMaximumTransmitQueuesInclusive, 0, "deviceMaximumTransmitQueuesInclusive is zero; this shouldn't be possible but makes this device '{}' useless", device_name);
			assert!(deviceMaximumTransmitQueuesInclusive <= MaximumTransmitQueues as u16, "deviceMaximumTransmitQueuesInclusive '{}' exceeds MaximumTransmitQueues '{}' for device '{}'", deviceMaximumTransmitQueuesInclusive, MaximumTransmitQueues, device_name);

			deviceMaximumTransmitQueuesInclusive
		}

		println!("TODO: Driver name debugging, as we're not sure net_mlx4 is actually rte_mlx4_pmd, etc: '{}'", driverName);

		let (requiresIntelI40eEightFragmentsWorkaround, requiresVmWareVmxNet3SixteenFragmentsWorkaround, maximumReceiveQueuesInclusive) = match driverName
		{
			"rte_i40e_pmd" => (true, false, 64),

			"rte_i40evf_pmd" => (true, false, 16),

			"rte_ixgbe_pmd" => (false, false, 16),

			"rte_ixgbevf_pmd" => (false, false, 4),

			"rte_vmxnet3_pmd" => (false, true, deviceMaximumReceiveQueuesInclusive(&deviceInformation, &device_name)),

			_ => (false, false, deviceMaximumReceiveQueuesInclusive(&deviceInformation, &device_name)),
		};

		use self::PowerOfTwoSixteenBit::*;

		let receiveSideScalingRetaIndirectionTableNumberOfBits = match deviceInformation.reta_size
		{
			0 => None,
			ETH_RSS_RETA_SIZE_64 => Some(_64),
			ETH_RSS_RETA_SIZE_128 => Some(_128),
			ETH_RSS_RETA_SIZE_256 => Some(_256),
			ETH_RSS_RETA_SIZE_512 => Some(_512),
			illegal @ _ => panic!("reta_size is not 64, 128, 256 or 512 bits but '{}'; this shouldn't be possible but makes this device '{}' useless", illegal, device_name),
		};

		// The number of RSS bits in the RETA table dictates a maximum number of CPUs. Most cards support at least 128 bits => 128 CPUs, but may not support enough queues (eg many Intel cards may support 128 bits but only 64 queues)
		let maximumNumberOfReceiveThenTransmitQueuePairs = if let Some(receiveSideScalingRetaIndirectionTableNumberOfBits) = receiveSideScalingRetaIndirectionTableNumberOfBits
		{
			min(receiveSideScalingRetaIndirectionTableNumberOfBits as u16, min(maximumReceiveQueuesInclusive, deviceMaximumTransmitQueuesInclusive(&deviceInformation, &device_name)))
		}
		else
		{
			1
		};

		let receiveSideScalingRetaIndirectionTableNumberOfBits_and_supportedReceiveSideScalingHashKeySize = match ReceiveSideScalingHashKeySize::fromNumberOrPanicAndZeroLengthIsNone(deviceInformation.hash_key_size)
		{
			Some(supportedReceiveSideScalingHashKeySize) =>
			{
				assert!(receiveSideScalingRetaIndirectionTableNumberOfBits.is_some(), "device '{}' supports a receive side scaling hash key size but does not support a RETA table", device_name);

				assert!(supportedFilterTypes.contains(&FilterType::Hash), "device '{}' supports a receive side scaling hash key size and a RETA table but does not support a Hash filter", device_name);

				Some((receiveSideScalingRetaIndirectionTableNumberOfBits.unwrap(), supportedReceiveSideScalingHashKeySize))
			},
			None =>
			{
				assert!(receiveSideScalingRetaIndirectionTableNumberOfBits.is_none(), "device '{}' does not support a receive side scaling hash key size but DOES support a RETA table", device_name);

				None
			},
		};

		EthernetPortInformation
		{
			ethernetPortIdentifier: ethernetPort.portIdentifier(),
			ethernetPort,
			driverName: driverName.to_owned(),
			deviceInformation,
			underlying_ethernet_device,

			requiresIntelI40eEightFragmentsWorkaround,
			requiresVmWareVmxNet3SixteenFragmentsWorkaround,

			parentNumaSocketId,
			maximumNumberOfReceiveThenTransmitQueuePairs,
			receiveSideScalingRetaIndirectionTableNumberOfBits_and_supportedReceiveSideScalingHashKeySize,
			logicalCoreUser: LogicalCoreUser::new(parentNumaSocketId, maximumNumberOfReceiveThenTransmitQueuePairs as usize),

			supportedFilterTypes,
			deviceReceiveOffloadCapabilities,
			deviceTransmitOffloadCapabilities,

			receiveLargeReceiveOffloadSupported: deviceReceiveOffloadCapabilities.supportsTcpLargeReceiveOffload(),
			receiveIpV4TcpAndUdpChecksumOffloadSupported:
			{
				debug_assert!(deviceReceiveOffloadCapabilities.supportsIpV4TcpAndUdpChecksumOffload() || deviceReceiveOffloadCapabilities.supportsNoneOfIpV4TcpAndUdpChecksumOffload(), "device '{}' supports a mixture of IPv4, TCP and UDP receive checksum offloading");
				deviceReceiveOffloadCapabilities.supportsIpV4TcpAndUdpChecksumOffload()
			},
			transmitTcpSegmentationOffloadSupported: deviceTransmitOffloadCapabilities.supportsTcpSegmentationOffload(),
			transmitUdpSegmentationOffloadSupported: deviceTransmitOffloadCapabilities.supportsUdpSegmentationOffload(),
			transmitIpV4ChecksumOffloadSupported: deviceTransmitOffloadCapabilities.supportsIpV4ChecksumOffload(),
			transmitTcpAndUdpChecksumOffloadSupported:
			{
				debug_assert!(deviceTransmitOffloadCapabilities.supportsTcpAndUdpChecksumOffload() || deviceTransmitOffloadCapabilities.supportsNoneOfTcpAndUdpChecksumOffload(), "device '{}' supports a mixture of TCP and UDP transmit checksum offloading");
				deviceTransmitOffloadCapabilities.supportsTcpAndUdpChecksumOffload()
			},

			device_name,
			dataCentreBridgingInformation,
			eepromSize,
			eepromInformation,
			deviceRegisters,
		}
	}
}
