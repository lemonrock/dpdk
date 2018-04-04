// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[allow(missing_debug_implementations)]
pub struct EthernetPortConfiguration
{
	pub receiveQueueConfigurations: ArrayVec<[ReceiveQueueConfiguration; MaximumReceiveQueues]>,
	pub transmitQueueConfigurations: ArrayVec<[TransmitQueueConfiguration; MaximumTransmitQueues]>,
	
	pub linkSpeeds: LinkSpeeds,
	pub receiveModeConfiguration: EthernetPortReceiveModeConfiguration,
	pub transmitModeConfiguration: EthernetPortTransmitModeConfiguration,
	pub loopBackOperationMode: u32,
	
	pub receiveSideScalingHashFunctionConfiguration: ReceiveSideScalingHashFunctionConfiguration,
	pub receiveSideScalingHashFilter: Option<HashFilter>,
	pub receiveSideScalingRetaIndirectionTable: Option<ReceiveSideScalingRetaIndirectionTable>,
	
	pub receiveVmdQDataCentreBridgingConfiguration: ReceiveVmdQDataCentreBridgingConfiguration,
	pub receiveDataCentreBridgingConfiguration: ReceiveDataCentreBridgingConfiguration,
	pub receiveVmdQConfiguration: ReceiveVmdQConfiguration,
	pub transmitAdvancedConfiguration: TransmitAdvancedConfiguration,
	pub dataCentreBridgingCapability: DataCentreBridgingCapability,
	pub flowDirectorConfiguration: FlowDirectorConfiguration,
	pub enableDeviceIscInterrupts: bool,
	pub deviceInterruptReceiveQueue: QueueIdentifier,
	
	pub flowControl: Option<FlowControl>,
	pub dataCentreBridgingPriorityFlowControl: Option<DataCentreBridgingPriorityFlowControl>,
	
	pub isPromiscuous: bool,
	pub enableAllMulticastReceive: bool,
	pub maximumTransmissionUnitSizeInBytes: MaximumTransmissionUnitSizeInBytes,
	pub multicastMediaAccessControlAddressesToFilter: HashSet<MediaAccessControlAddress>,
	pub udpTunnelsToOffload: HashSet<UdpTunnelConfiguration>,
	pub enableTimestamping: bool,
	
	pub linkIsUp: bool,
	pub ledIsLit: bool,
	pub defaultMediaAccessControlAddress: Option<MediaAccessControlAddress>,
	pub additionalMediaAccessControlAddresses: HashSet<(MediaAccessControlAddress, Option<u6>)>,
	pub trafficMirroringRules: HashMap<TrafficMirroringRuleNumber, TrafficMirroringRule>,
	pub virtualLanOffloadFeatures: Option<VirtualLanOffloadFeatures>,
}

impl EthernetPortConfiguration
{
	pub fn restrictNumberOfQueuePairsToMaximum(&mut self, maximumNumberOfReceiveThenTransmitQueuePairs: u16)
	{
		let maximumNumberOfReceiveThenTransmitQueuePairs = maximumNumberOfReceiveThenTransmitQueuePairs as usize;
		
		let borrowReceive = &mut self.receiveQueueConfigurations;
		while borrowReceive.len() > maximumNumberOfReceiveThenTransmitQueuePairs
		{
			borrowReceive.pop();
		}
		
		let borrowTransmit = &mut self.transmitQueueConfigurations;
		while borrowTransmit.len() > maximumNumberOfReceiveThenTransmitQueuePairs
		{
			borrowTransmit.pop();
		}
	}
	
	pub fn configureAndStartWithQueues<Q: QueueMemoryConfiguration>(&mut self, ethernetPortInformation: &EthernetPortInformation, queueMemoryConfiguration: &Q) -> EthernetPortConfigurationResult
	{
		debug_assert!(self.receiveQueueConfigurations.len() > 0, "receiveQueueConfigurations.len() can not be zero");
		debug_assert!(self.transmitQueueConfigurations.len() > 0, "transmitQueueConfigurations.len() can not be zero");
		
		let numberOfReceiveQueues = self.receiveQueueConfigurations.len();
		let numberOfTransmitQueues = self.transmitQueueConfigurations.len();
		
		self.configure(ethernetPortInformation.portIdentifier(), numberOfReceiveQueues as u16, numberOfTransmitQueues as u16).expect("Could not configure ethernet port; check PMD_INIT_LOG, level INFO or NOTICE");
		
		let mut failures = EthernetPortConfigurationFailures::new();
		
		let mut receiveQueues = ArrayVec::<[ReceiveQueue; MaximumReceiveQueues]>::new();
		for queueIdentifier in 0..numberOfReceiveQueues
		{
			let receiveQueueConfiguration = self.receiveQueueConfigurations.get(queueIdentifier).unwrap();
			let queueIdentifier = queueIdentifier as QueueIdentifier;
			
			if let Some(receiveQueue) = ReceiveQueue::new(ethernetPortInformation, queueIdentifier, queueMemoryConfiguration, &receiveQueueConfiguration, &mut failures)
			{
				receiveQueues.push(receiveQueue);
			}
			else
			{
				failures.push(EthernetPortConfigurationFailureKind::CouldNotCreateReceiveQueueBecauseOutOfMemory(queueIdentifier))
			}
		}
		
		let ethernetPort = ethernetPortInformation.ethernetPort();
		
		let mut transmitQueues = ArrayVec::<[TransmitQueue; MaximumTransmitQueues]>::new();
		for queueIdentifier in 0..numberOfTransmitQueues
		{
			let transmitQueueConfiguration = self.transmitQueueConfigurations.get(queueIdentifier).unwrap();
			let queueIdentifier = queueIdentifier as QueueIdentifier;
			
			if let Some(transmitQueue) = TransmitQueue::new(ethernetPortInformation, queueIdentifier, queueMemoryConfiguration, &transmitQueueConfiguration, &mut failures)
			{
				transmitQueues.push(transmitQueue);
			}
			else
			{
				failures.push(EthernetPortConfigurationFailureKind::CouldNotCreateTransmitQueueBecauseOutOfMemory(queueIdentifier))
			}
		}

		if let Some(flowControl) = self.flowControl
		{
			failures.ifError(ethernetPort.setFlowControl(flowControl), EthernetPortConfigurationFailureKind::SetFlowControl);
		}
		
		if let Some(dataCentreBridgingPriorityFlowControl) = self.dataCentreBridgingPriorityFlowControl
		{
			failures.ifError(ethernetPort.setDataCentreBridgingPriorityFlowControl(dataCentreBridgingPriorityFlowControl), EthernetPortConfigurationFailureKind::SetDataCentreBridgingPriorityFlowControl);
		}
		
		ethernetPort.start().expect("Could not start EthernetPort");
		
		if let Some(ref receiveSideScalingHashFilter) = self.receiveSideScalingHashFilter
		{
			failures.ifError(ethernetPort.setHashFilter(receiveSideScalingHashFilter), EthernetPortConfigurationFailureKind::ReceiveSideScalingHashFilter)
		}
		
		if let Some(ref mut receiveSideScalingRetaIndirectionTable) = self.receiveSideScalingRetaIndirectionTable
		{
			failures.ifError(ethernetPort.updateRetaIndirectionTable(receiveSideScalingRetaIndirectionTable), EthernetPortConfigurationFailureKind::UpdateRetaIndirectionTable)
		}
		
		if self.isPromiscuous
		{
			ethernetPort.enablePromiscuousReceive();
		}
		else
		{
			ethernetPort.disablePromiscuousReceive();
		}
		
		if self.enableAllMulticastReceive
		{
			ethernetPort.enableAllMulticastReceive();
		}
		else
		{
			ethernetPort.disableAllMulticastReceive();
		}
		
		failures.ifError(ethernetPort.clearMulticastMediaAccessControlAddressesToFilter(), EthernetPortConfigurationFailureKind::ClearMulticastMediaAccessControlAddressesToFilter);
		failures.ifError(ethernetPort.setMulticastMediaAccessControlAddressesToFilter(&self.multicastMediaAccessControlAddressesToFilter), EthernetPortConfigurationFailureKind::SetMulticastMediaAccessControlAddressesToFilter);
		failures.ifError(ethernetPort.setMaximumTransmissionUnit(self.maximumTransmissionUnitSizeInBytes.as_u16()), EthernetPortConfigurationFailureKind::SetMaximumTransmissionUnit);
		
		for udpTunnelConfiguration in &self.udpTunnelsToOffload
		{
			let udpTunnelConfiguration = *udpTunnelConfiguration;
			if let Err(error) = ethernetPort.addUdpTunnelOffload(udpTunnelConfiguration)
			{
				failures.push(EthernetPortConfigurationFailureKind::AddUdpTunnelOffload(error, udpTunnelConfiguration));
			}
		}
	
		if self.enableTimestamping
		{
			failures.ifError(ethernetPort.enableTimestamping(), EthernetPortConfigurationFailureKind::EnableTimestamping);
		}
		else
		{
			failures.ifError(ethernetPort.disableTimestamping(), EthernetPortConfigurationFailureKind::DisableTimestamping);
		}
		
		for receiveQueue in &receiveQueues
		{
			if receiveQueue.startIfDeferred().is_err()
			{
				failures.push(EthernetPortConfigurationFailureKind::ReceiveQueueDeferredStart(receiveQueue.queueIdentifier))
			}
		}
		
		for transmitQueue in &transmitQueues
		{
			if transmitQueue.startIfDeferred().is_err()
			{
				failures.push(EthernetPortConfigurationFailureKind::TransmitQueueDeferredStart(transmitQueue.queueIdentifier))
			}
		}
		
		if self.linkIsUp
		{
			failures.ifError(ethernetPort.setLinkUp(), EthernetPortConfigurationFailureKind::LinkUp);
		}
		else
		{
			failures.ifError(ethernetPort.setLinkDown(), EthernetPortConfigurationFailureKind::LinkDown);
		}
		
		if self.ledIsLit
		{
			failures.ifError(ethernetPort.turnLedOn(), EthernetPortConfigurationFailureKind::LedOn);
		}
		else
		{
			failures.ifError(ethernetPort.turnLedOff(), EthernetPortConfigurationFailureKind::LedOff);
		}
		
		if let Some(defaultMediaAccessControlAddress) = self.defaultMediaAccessControlAddress
		{
			failures.ifError(ethernetPort.setDefaultMediaAccessControlAddress(defaultMediaAccessControlAddress), EthernetPortConfigurationFailureKind::SetDefaultMediaAccessControlAddress);
		}
		
		for &(additionalMediaAccessControlAddress, vmdqPortIndex) in &self.additionalMediaAccessControlAddresses
		{
			if let Err(error) = ethernetPort.addMediaAccessControlAddress(additionalMediaAccessControlAddress, vmdqPortIndex)
			{
				failures.push(EthernetPortConfigurationFailureKind::AddMediaAccessControlAddress(error, (additionalMediaAccessControlAddress, vmdqPortIndex)));
			}
		}
		
		for (trafficMirroringRuleNumber, trafficMirroringRule) in self.trafficMirroringRules.iter()
		{
			if let Err(error) = ethernetPort.enableTrafficMirroringRule(*trafficMirroringRuleNumber, trafficMirroringRule)
			{
				failures.push(EthernetPortConfigurationFailureKind::EnableTrafficMirroringRule(error, *trafficMirroringRuleNumber));
			}
		}
		
		if let Some(virtualLanOffloadFeatures) = self.virtualLanOffloadFeatures
		{
			failures.ifError(ethernetPort.setVirtualLanOffloading(virtualLanOffloadFeatures), EthernetPortConfigurationFailureKind::SetVirtualLanOffloadFeatures);
		}
		
		(receiveQueues, transmitQueues, failures)
	}
	
	#[inline(always)]
	fn configure(&self, portIdentifier: EthernetPortIdentifier, numberOfReceiveQueues: u16, numberOfTransmitQueues: u16) -> Result<(), i32>
	{
		debug_assert!(numberOfReceiveQueues > 0, "numberOfReceiveQueues can not be zero");
		debug_assert!(numberOfTransmitQueues > 0, "numberOfTransmitQueues can not be zero");
		
		let mustLiveAtLeastAsLongAs_receiveSideScalingHashFunctionConfiguration = self.receiveSideScalingHashFunctionConfiguration.as_rte_eth_rss_conf();
		
		let ethernetInternalConfiguration = rte_eth_conf
		{
			link_speeds: self.linkSpeeds.as_uint32_t(),
			rxmode: self.receiveModeConfiguration.as_rte_eth_rxmode(),
			txmode: self.transmitModeConfiguration.as_rte_eth_txmode(),
			lpbk_mode: self.loopBackOperationMode,
			rx_adv_conf: rte_eth_conf_AnonymousStruct_rx_adv_conf
			{
				rss_conf: mustLiveAtLeastAsLongAs_receiveSideScalingHashFunctionConfiguration,
				vmdq_dcb_conf: self.receiveVmdQDataCentreBridgingConfiguration.as_rte_eth_vmdq_dcb_conf(),
				dcb_rx_conf: self.receiveDataCentreBridgingConfiguration.as_rte_eth_dcb_rx_conf(),
				vmdq_rx_conf: self.receiveVmdQConfiguration.as_rte_eth_vmdq_rx_conf(),
			},
			tx_adv_conf: self.transmitAdvancedConfiguration.as_rte_eth_conf_AnonymousUnion_tx_adv_conf(),
			dcb_capability_en: self.dataCentreBridgingCapability.bits(),
			fdir_conf: self.flowDirectorConfiguration.as_rte_fdir_conf(),
			intr_conf: rte_intr_conf
			{
				lsc: if self.enableDeviceIscInterrupts
				{
					1
				}
				else
				{
					0
				},
				rxq: self.deviceInterruptReceiveQueue,
			},
		};

		match unsafe { ::dpdk_sys::rte_eth_dev_configure(portIdentifier, numberOfReceiveQueues, numberOfTransmitQueues, &ethernetInternalConfiguration) }
		{
			0 => Ok(()),

			result if result < 0 => Err(result),

			unexpected @ _ => panic!("Unexpected positive error result '{}' from rte_eth_dev_configure()", unexpected),
		}
	}
}
