// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct EthernetPortConfiguration
{
	ethernetPortDpdkConfiguration: EthernetPortDpdkConfiguration,
	sourceEthernetAddressBlackList: SourceEthernetAddressBlackListConfiguration,
	defaultVirtualLan: VirtualLanConfiguration,
	singleTaggedVirtualLans: HashMap<VirtualLanId, VirtualLanConfiguration>,
	doubleTaggedVirtualLans: HashMap<VirtualLanId, DoubleTaggedVirtualLanConfiguration>,
	udpFragmentsAndTcpControlPacketsMemoryConfiguration: UdpFragmentsAndTcpControlPacketsMemoryConfiguration,
}

impl Default for EthernetPortConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			ethernetPortDpdkConfiguration: Default::default(),
			sourceEthernetAddressBlackList: Default::default(),
			defaultVirtualLan: Default::default(),
			singleTaggedVirtualLans: Default::default(),
			doubleTaggedVirtualLans: Default::default(),
			udpFragmentsAndTcpControlPacketsMemoryConfiguration: Default::default(),
		}
	}
}

impl AppendAdditionalEthernetAddresses for EthernetPortConfiguration
{
	#[inline(always)]
	fn appendAdditionalEthernetAddresses(&self, defaultEthernetAddress: &UnicastEthernetAddress, additionalEthernetAddresses: &mut HashSet<UnicastEthernetAddress>)
	{
		self.defaultVirtualLan.appendAdditionalEthernetAddresses(defaultEthernetAddress, additionalEthernetAddresses);
		self.singleTaggedVirtualLans.appendAdditionalEthernetAddresses(defaultEthernetAddress, additionalEthernetAddresses);
		self.doubleTaggedVirtualLans.appendAdditionalEthernetAddresses(defaultEthernetAddress, additionalEthernetAddresses);
	}
}

impl ExecutionRoutineCreatorCreator<RwLock<HashMap<VirtualLanKey, ArpCache>>, OurExecutionRoutineCreator> for EthernetPortConfiguration
{
	fn createWhilstOnMasterLogicalCore(&self, data: Arc<RwLock<HashMap<VirtualLanKey, ArpCache>>>, queuePairIdentifier: QueueIdentifier, slaveLogicalCoreToExecuteOn: LogicalCore, ethernetPortInformation: &EthernetPortInformation) -> OurExecutionRoutineCreator
	{
		let ethernetPort = *ethernetPortInformation.ethernetPort();
		
		OurExecutionRoutineCreator
		{
			ethernetPortConfiguration: self.clone(),
			ethernetPort: ethernetPort,
			arpCaches: data,
			queueIdentifier: queuePairIdentifier,
			logicalCoreMemorySocket: slaveLogicalCoreToExecuteOn.optionalNumaSocketId(),
			receiveBurst: ReceiveBurst::new(ethernetPort, queuePairIdentifier),
			transmitBurst: TransmitBurst::new(ethernetPort, queuePairIdentifier),
		}
	}
}

impl EthernetPortConfiguration
{
	// At entry, useLogicalCoreUser() must have been used to allocate the maximum number of logical cores we have
	pub fn configureAndStartEthernetPort(&mut self, ethernetPortInformation: &mut EthernetPortInformation) -> (EthernetPortConfigurationResult, Arc<Mutex<ExecutionRoutineGroup<ReceiveTransmitQueuePairSlaveLogicalCoreTask<OurExecutionRoutineCreator>>>>)
	{
		let (defaultEthernetAddress, additionalEthernetAddresses) = self.ethernetAddresses(*ethernetPortInformation.ethernetPort());
		
		let arpCaches = Arc::new(RwLock::new(HashMap::new()));
		
		self.ethernetPortDpdkConfiguration.configureAndStartEthernetPort(self, arpCaches, ethernetPortInformation, &defaultEthernetAddress, &additionalEthernetAddresses)
	}
	
	pub fn createPerLogicalCore(&self,
		ethernetPort: EthernetPort, arpCaches: Arc<RwLock<HashMap<VirtualLanKey, ArpCache>>>, // Per ethernet core
		queueIdentifier: QueueIdentifier, logicalCoreMemorySocket: Option<NumaSocketId> // for this logical core
	) -> Destinations
	{
		let (defaultEthernetAddress, additionalEthernetAddresses) = self.ethernetAddresses(ethernetPort);
		
		let packetBufferPool = self.udpFragmentsAndTcpControlPacketsMemoryConfiguration.createPacketBufferPool(ethernetPort.portIdentifier(), queueIdentifier, logicalCoreMemorySocket);
		let udpFragmentsAndTcpControlPacketBufferPool = packetBufferPool.0;
		
		let mut ourHardwareAddresses = additionalEthernetAddresses.clone();
		ourHardwareAddresses.insert(defaultEthernetAddress);
		
		let mut destinations = Destinations
		{
			ourHardwareAddresses: ourHardwareAddresses.iter().map(|value| (value.0).0).collect(),
			sourceEthernetAddressBlackList: SourceEthernetAddressBlackList::from(self.sourceEthernetAddressBlackList.clone()),
			ipStates: HashMap::new(),
		};
		
		self.populateIpStates(ethernetPort, queueIdentifier, logicalCoreMemorySocket, &defaultEthernetAddress, udpFragmentsAndTcpControlPacketBufferPool, arpCaches, &mut destinations.ipStates);
		
		destinations
	}
	
	fn populateIpStates(&self, ethernetPort: EthernetPort, queueIdentifier: QueueIdentifier, logicalCoreMemorySocket: Option<NumaSocketId>, defaultEthernetAddress: &UnicastEthernetAddress, udpFragmentsAndTcpControlPacketBufferPool: *mut rte_mempool, arpCaches: Arc<RwLock<HashMap<VirtualLanKey, ArpCache>>>, ipStates: &mut HashMap<VirtualLanKey, IpState>)
	{
		let virtualLanTagging = if self.defaultVirtualLan.settingsAreEquivalentToUnspecified()
		{
			VirtualLanTagging::None
		}
		else
		{
			VirtualLanTagging::Single(self.defaultVirtualLan.asVirtualLanTrafficClassIndicator(None))
		};
		let ipState = self.defaultVirtualLan.createIpState(ethernetPort, queueIdentifier, logicalCoreMemorySocket, &defaultEthernetAddress, udpFragmentsAndTcpControlPacketBufferPool, &virtualLanTagging, arpCaches.clone());
		let virtualLanKey = virtualLanTagging.virtualLanKey();
		ipStates.insert(virtualLanKey, ipState);
		
		for (innerVirtualLanId, virtualLanConfiguration) in self.singleTaggedVirtualLans.iter()
		{
			let inner = virtualLanConfiguration.asVirtualLanTrafficClassIndicator(Some(*innerVirtualLanId));
			let virtualLanTagging = VirtualLanTagging::Single(inner);
			
			let ipState = virtualLanConfiguration.createIpState(ethernetPort, queueIdentifier, logicalCoreMemorySocket, &defaultEthernetAddress, udpFragmentsAndTcpControlPacketBufferPool, &virtualLanTagging, arpCaches.clone());
			let virtualLanKey = virtualLanTagging.virtualLanKey();
			ipStates.insert(virtualLanKey, ipState);
		}
		
		for (outerVirtualLanId, doubleTaggedVirtualLanConfiguration) in self.doubleTaggedVirtualLans.iter()
		{
			let outer = doubleTaggedVirtualLanConfiguration.asVirtualLanTrafficClassIndicator(Some(*outerVirtualLanId));
			
			for (innerVirtualLanId, virtualLanConfiguration) in doubleTaggedVirtualLanConfiguration.innerVirtualLans.iter()
			{
				let inner = virtualLanConfiguration.asVirtualLanTrafficClassIndicator(Some(*innerVirtualLanId));
				let virtualLanTagging = VirtualLanTagging::Double(outer, inner);
				
				let ipState = virtualLanConfiguration.createIpState(ethernetPort, queueIdentifier, logicalCoreMemorySocket, &defaultEthernetAddress, udpFragmentsAndTcpControlPacketBufferPool, &virtualLanTagging, arpCaches.clone());
				let virtualLanKey = virtualLanTagging.virtualLanKey();
				ipStates.insert(virtualLanKey, ipState);
			}
		}
	}
	
	fn ethernetAddresses(&self, ethernetPort: EthernetPort) -> (UnicastEthernetAddress, HashSet<UnicastEthernetAddress>)
	{
		let defaultEthernetAddress = self.ethernetPortDpdkConfiguration.defaultEthernetAddress(ethernetPort);
		let mut additionalEthernetAddresses = HashSet::with_capacity(16);
		self.appendAdditionalEthernetAddresses(&defaultEthernetAddress, &mut additionalEthernetAddresses);
		(defaultEthernetAddress, additionalEthernetAddresses)
	}
}
