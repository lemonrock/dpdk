// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct EthernetPortDpdkConfiguration
{
	defaultEthernetAddress: Option<UnicastEthernetAddress>,
	isPromiscuous: bool,
	maximumTransmissionUnit: MaximumTransmissionUnitSize,
	numberOfReceiveDescriptors: u16,
	numberOfTransmitDescriptors: u16,
	queueMemorySettings: EthernetPortQueueMemoryConfiguration,
}

impl Default for EthernetPortDpdkConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			defaultEthernetAddress: None,
			isPromiscuous: false,
			maximumTransmissionUnit: MaximumTransmissionUnitSize::TldkValue,
			numberOfReceiveDescriptors: ReceiveQueueConfiguration::TldkNumberOfReceiveDescriptorsForTheReceiveRingAlsoKnownAsRingSize,
			numberOfTransmitDescriptors: TransmitQueueConfiguration::TldkNumberOfTransmitDescriptorsForTheTransmitRingAlsoKnownAsRingSize,
			queueMemorySettings: Default::default(),
		}
	}
}

impl EthernetPortDpdkConfiguration
{
	pub fn defaultEthernetAddress(&self, ethernetPort: EthernetPort) -> UnicastEthernetAddress
	{
		match self.defaultEthernetAddress
		{
			None => UnicastEthernetAddress(ethernetPort.getDefaultMediaAccessControlAddress()),
			Some(address) => address,
		}
	}

	// At entry, useLogicalCoreUser() must have been used to allocate the maximum number of logical cores we have
	pub fn configureAndStartEthernetPort<E: ExecutionRoutineCreatorCreator<D, EC>, D, EC: ExecutionRoutineCreator>(&self, executionRoutineCreatorCreator: &E, data: Arc<D>, ethernetPortInformation: &mut EthernetPortInformation, defaultEthernetAddress: &UnicastEthernetAddress, additionalEthernetAddresses: &HashSet<UnicastEthernetAddress>) -> (EthernetPortConfigurationResult, Arc<Mutex<ExecutionRoutineGroup<ReceiveTransmitQueuePairSlaveLogicalCoreTask<EC>>>>)
	{
		let ethernetPortConfigurationResult =
		{
			let receiveSideScalingHashChooser = symmetricAllReceiveSideScalingHashChooser;

			let mut ethernetPortConfiguration = self.createEthernetPortConfiguration(ethernetPortInformation, receiveSideScalingHashChooser, defaultEthernetAddress, &additionalEthernetAddresses);
			let receiveTransmitQueueMemoryConfiguration = self.queueMemorySettings.receiveTransmitQueueMemoryConfiguration();
			ethernetPortConfiguration.configureAndStartWithQueues(ethernetPortInformation, &receiveTransmitQueueMemoryConfiguration)
		};

		let linkStatusResult = ethernetPortInformation.waitUntilLinkIsUp();
		if linkStatusResult.is_err()
		{
			warn!("Could not bring link up for ethernet port '{}'", ethernetPortInformation.portIdentifier());
		}

		let executionRoutineGroup = ethernetPortInformation.startProcessingQueuePairs(executionRoutineCreatorCreator, data);

		(ethernetPortConfigurationResult, executionRoutineGroup)
	}

	fn createEthernetPortConfiguration<ReceiveSideScalingHashChooser>(&self, ethernetPortInformation: &EthernetPortInformation, receiveSideScalingHashChooser: ReceiveSideScalingHashChooser, defaultEthernetAddress: &UnicastEthernetAddress, additionalEthernetAddresses: &HashSet<UnicastEthernetAddress>) -> ::dpdk::ethernetPorts::EthernetPortConfiguration
	where ReceiveSideScalingHashChooser: for<'a> Fn(u16, ReceiveSideScalingHashKeySize, &'a str, &'a str) -> (Option<HashFilter>, ReceiveSideScalingHashFunctionConfiguration)
	{
		// NOTE: Things such as multiQueuePacketDistributionMode and the receiveSideScaling* fields are automatically adjusted later by EthernetPortConfiguration using EthernetPortInformation
		// NOTE: The reason create() is passed ethernetPortInformation is to allow decisions to be made based on things like ethernet port defaults and maximums

		let receiveQueueConfigurations =
		{
			let receiveQueueConfigurationTemplate = ReceiveQueueConfiguration::new
			(
				self.numberOfReceiveDescriptors,
				Some(ReceiveQueueDeviceConfiguration::overrideDropPacketsIfNoDescriptorsAreAvailable()),
				None
			);

			ethernetPortInformation.createReceiveQueueConfigurations(&receiveQueueConfigurationTemplate)
		};

		let transmitQueueConfigurations =
		{
			fn startFreeingTransmitBuffersIfThereAreLessFreeDescriptorsThanThis(numberOfTransmitDescriptors: u16) -> u16
			{
				// (1) startFreeingTransmitBuffersIfThereAreLessFreeDescriptorsThanThis < numberOfTransmitDescriptors - 3
				// (2) startFreeingTransmitBuffersIfThereAreLessFreeDescriptorsThanThis = numberOfTransmitDescriptors / 2
				// Hence 6
				assert!(numberOfTransmitDescriptors > 6, "There must be 7 or more numberOfTransmitDescriptors, not '{}'", numberOfTransmitDescriptors);
				numberOfTransmitDescriptors / 2
			}

			let numberOfTransmitDescriptors = self.numberOfTransmitDescriptors;

			let transmitQueueConfigurationTemplate = TransmitQueueConfiguration::new
			(
				numberOfTransmitDescriptors,
				Some(TransmitQueueDeviceConfiguration::overrideForTldk(startFreeingTransmitBuffersIfThereAreLessFreeDescriptorsThanThis(numberOfTransmitDescriptors))),
				None
			);

			ethernetPortInformation.createTransmitQueueConfigurations(&transmitQueueConfigurationTemplate)
		};

		let defaultMediaAccessControlAddress = defaultEthernetAddress.0;

		let additionalMediaAccessControlAddresses: HashSet<(MediaAccessControlAddress, Option<u6>)> = additionalEthernetAddresses.iter().map(|value| (value.0, None)).collect();

		let mut configuration = ::dpdk::ethernetPorts::EthernetPortConfiguration
		{
			receiveQueueConfigurations,
			transmitQueueConfigurations,

			linkSpeeds: LinkSpeeds::default(),
			receiveModeConfiguration: EthernetPortReceiveModeConfiguration
			{
				multiQueuePacketDistributionMode: MultiQueuePacketReceiveMode::ReceiveSideScaling,
				maximumReceivePacketLengthOnlyUsedIfJumboFramesEnabled: self.maximumTransmissionUnit.conservative_jumbo_frame_length(),
				splitHeaderSizeIfHeaderSplitEnabled: None,
				hardwareIpUdpOrTcpChecksumOffloadEnabled: true,
				hardwareVlanFilterEnabled: false,
				hardwareVlanStripEnabled: true,
				extendedVlanEnabled: true,
				hardwareCyclicRedundancyChecksumStrippingEnabled: true,
				scatterPacketsReceiveHandlerEnabled: false,
				largeReceiveOffloadEnabled: true,
			},
			transmitModeConfiguration: EthernetPortTransmitModeConfiguration
			{
				multiQueuePacketTransmitMode: MultiQueuePacketTransmitMode::empty(),
				portBasedVlanInsertId: 0,
				hardwareShouldRejectSendingOutVlanTaggedPackets: false,
				hardwareShouldRejectSendingOutVlanUntaggedPackets: false,
				hardwarePortBasedVlanInsertionEnabled: false,
			},
			loopBackOperationMode: 0,
			receiveSideScalingHashFunctionConfiguration: ReceiveSideScalingHashFunctionConfiguration::NoneEthernetDeviceDefault,
			receiveSideScalingHashFilter: None,
			receiveSideScalingRetaIndirectionTable: None,
			receiveVmdQDataCentreBridgingConfiguration: ReceiveVmdQDataCentreBridgingConfiguration
			{
			},
			receiveDataCentreBridgingConfiguration: ReceiveDataCentreBridgingConfiguration
			{
			},
			receiveVmdQConfiguration: ReceiveVmdQConfiguration
			{
			},
			transmitAdvancedConfiguration: TransmitAdvancedConfiguration::None,
			dataCentreBridgingCapability: DataCentreBridgingCapability::empty(),
			flowDirectorConfiguration: FlowDirectorConfiguration
			{
				mode: rte_fdir_mode::RTE_FDIR_MODE_NONE,
				allocationType: rte_fdir_pballoc_type::RTE_FDIR_PBALLOC_64K,
				statusMode: rte_fdir_status_mode::RTE_FDIR_NO_REPORT_STATUS,
				dropQueue: 0,
				mask: rte_eth_fdir_masks::default(),
			},
			enableDeviceIscInterrupts: false,
			deviceInterruptReceiveQueue: 0,

			flowControl: Some(FlowControl::default()),
			dataCentreBridgingPriorityFlowControl: None,

			isPromiscuous: self.isPromiscuous,
			enableAllMulticastReceive: false,
			maximumTransmissionUnitSizeInBytes: self.maximumTransmissionUnit,
			multicastMediaAccessControlAddressesToFilter: HashSet::new(),
			udpTunnelsToOffload: HashSet::new(),
			enableTimestamping: false,

			linkIsUp: true,
			ledIsLit: false,
			defaultMediaAccessControlAddress: Some(defaultMediaAccessControlAddress),
			additionalMediaAccessControlAddresses,
			trafficMirroringRules: HashMap::new(),
			virtualLanOffloadFeatures: None,
		};

		ethernetPortInformation.adjustConfiguration(&mut configuration, &receiveSideScalingHashChooser);

		configuration
	}
}


#[allow(unused_variables)]
pub fn tldkTcpReceiveSideScalingHashChooser<'a>(numberOfReceiveQueues: u16, receiveSideScalingHashKeySize: ReceiveSideScalingHashKeySize, driverName: &'a str, device_name: &'a str) -> ((Option<HashFilter>, ReceiveSideScalingHashFunctionConfiguration))
{
	(
		None,
		ReceiveSideScalingHashFunctionConfiguration
		{
			key: ReceiveSideScalingHashFunctionKeyData::tldkRssKeyData(numberOfReceiveQueues, receiveSideScalingHashKeySize),
			hashFunctionFlowApplicability: ReceiveSideScalingOffloadFlowTypeSet::Tcp,
		}
	)
}

#[allow(unused_variables)]
pub fn tldkUdpReceiveSideScalingHashChooser<'a>(numberOfReceiveQueues: u16, receiveSideScalingHashKeySize: ReceiveSideScalingHashKeySize, driverName: &'a str, device_name: &'a str) -> ((Option<HashFilter>, ReceiveSideScalingHashFunctionConfiguration))
{
	(
		None,
		ReceiveSideScalingHashFunctionConfiguration
		{
			key: ReceiveSideScalingHashFunctionKeyData::tldkRssKeyData(numberOfReceiveQueues, receiveSideScalingHashKeySize),
			hashFunctionFlowApplicability: ReceiveSideScalingOffloadFlowTypeSet::Udp,
		}
	)
}

#[allow(unused_variables)]
pub fn symmetricAllReceiveSideScalingHashChooser<'a>(numberOfReceiveQueues: u16, receiveSideScalingHashKeySize: ReceiveSideScalingHashKeySize, driverName: &'a str, device_name: &'a str) -> ((Option<HashFilter>, ReceiveSideScalingHashFunctionConfiguration))
{
	(
		Some(HashFilter::GenericToeplitzHashFilter),
		match receiveSideScalingHashKeySize
		{
			ReceiveSideScalingHashKeySize::Forty => ReceiveSideScalingHashFunctionConfiguration::AllSymmetricForty,
			ReceiveSideScalingHashKeySize::FiftyTwo => ReceiveSideScalingHashFunctionConfiguration::AllSymmetricFiftyTwo,
		}
	)
}

#[allow(unused_variables)]
pub fn defaultAllReceiveSideScalingHashChooser<'a>(numberOfReceiveQueues: u16, receiveSideScalingHashKeySize: ReceiveSideScalingHashKeySize, driverName: &'a str, device_name: &'a str) -> ((Option<HashFilter>, ReceiveSideScalingHashFunctionConfiguration))
{
	println!("TODO: Please double check defaultAllReceiveSideScalingHashChooser() for correct driverName, as we're not sure net_mlx4 is actually rte_mlx4_pmd. This driverName is '{}'", driverName);

	(
		Some(HashFilter::GenericToeplitzHashFilter),
		match receiveSideScalingHashKeySize
		{
			ReceiveSideScalingHashKeySize::Forty => match driverName
			{
				// Not sure this is right
				"rte_mlx4_pmd" => ReceiveSideScalingHashFunctionConfiguration::AllMellanoxForty,
				"rte_mlx5_pmd" => ReceiveSideScalingHashFunctionConfiguration::AllMellanoxForty,
				_ => ReceiveSideScalingHashFunctionConfiguration::AllEthernetDeviceDefault,
			},
			ReceiveSideScalingHashKeySize::FiftyTwo =>
				{
					match driverName
					{
						"rte_i40e_pmd" => ReceiveSideScalingHashFunctionConfiguration::AllIntelFiftyTwo,
						"rte_i40evf_pmd" => ReceiveSideScalingHashFunctionConfiguration::AllIntelFiftyTwo,
						_ => panic!("Only Intel i40e devices are supported for 52-byte RSS keys"),
					}
				},
		}
	)
}
