// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[serde(default)]
struct VirtualLanConfiguration
{
	settings: VirtualLanValue,
	ipV4Addresses: HashMap<Ipv4Addr, IpV4AddressConfiguration>,
	ipV6Addresses: HashMap<Ipv6Addr, IpV6AddressConfiguration>,

	// The black lists could operate at the level of a locally bound IP address, but they are not intended to be an access control. Rather, it is intended to let us drop traffic that is coming from misconfigured or DoS'ing hosts
	// This lets us use them to 'defend' the ARP cache (and IPv6 equivalents) from some poisoning attacks
	sourceIpV4NetworksToBlackList: InternetProtocolNetworkAddressBlackListConfiguration<InternetProtocolVersion4NetworkAddress>,
	sourceIpV6NetworksToBlackList: InternetProtocolNetworkAddressBlackListConfiguration<InternetProtocolVersion6NetworkAddress>,

	// Routing could be different for different local IP addresses but this is a niche use case
	ipV4RoutingTableConfiguration: IpV4RoutingTableConfiguration,
	arp: ArpCacheConfiguration,

	// Fragmentation
	ipV4Fragmentation: IpFragmentationConfiguration,
	ipV6Fragmentation: IpFragmentationConfiguration,
}

impl Default for VirtualLanConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			settings: Default::default(),
			ipV4Addresses: Default::default(),
			ipV6Addresses: Default::default(),
			sourceIpV4NetworksToBlackList: Default::default(),
			sourceIpV6NetworksToBlackList: Default::default(),
			ipV4RoutingTableConfiguration: Default::default(),
			arp: Default::default(),
			ipV4Fragmentation: Default::default(),
			ipV6Fragmentation: Default::default(),
		}
	}
}

impl AppendAdditionalEthernetAddresses for VirtualLanConfiguration
{
	#[inline(always)]
	fn appendAdditionalEthernetAddresses(&self, defaultEthernetAddress: &UnicastEthernetAddress, additionalEthernetAddresses: &mut HashSet<UnicastEthernetAddress>)
	{
		self.ipV4Addresses.appendAdditionalEthernetAddresses(defaultEthernetAddress, additionalEthernetAddresses);
		self.ipV6Addresses.appendAdditionalEthernetAddresses(defaultEthernetAddress, additionalEthernetAddresses);
	}
}

impl VirtualLanConfiguration
{
	pub fn settingsAreEquivalentToUnspecified(&self) -> bool
	{
		self.settings.equivalent_to_unspecified()
	}

	pub fn asVirtualLanTrafficClassIndicator(&self, virtual_lan_id: Option<VirtualLanIdentifier>) -> VirtualLanTrafficClassIndicator
	{
		VirtualLanTrafficClassIndicator
		{
			virtual_lan_value: self.settings,
			virtual_lan_id
		}
	}

	fn createArpCache(&self) -> ArpCache
	{
		self.arp.createArpCache()
	}

	pub fn createIpState(&self, ethernetPort: EthernetPort, queueIdentifier: QueueIdentifier, logicalCoreMemorySocket: Option<NumaSocketId>, defaultEthernetAddress: &UnicastEthernetAddress, udpFragmentsAndTcpControlPacketBufferPool: *mut rte_mempool, virtualLanTagging: &VirtualLanTagging, mut arpCaches: Arc<RwLock<HashMap<VirtualLanKey, ArpCache>>>) -> IpState
	{
		let arpCache =
		{
			let mut arpCaches = arpCaches.write().unwrap();
			arpCaches.entry(virtualLanTagging.virtual_lan_key()).or_insert_with(|| self.createArpCache()).clone()
		};

		let name = LongestPrefixMatchName
		{
			ethernetPortIdentifier: ethernetPort.portIdentifier(),
			queueIdentifier,
			virtual_lan_key: virtualLanTagging.virtual_lan_key(),
		};

		IpState
		{
			ipV4State: self.createIpV4State(ethernetPort, &name, logicalCoreMemorySocket, defaultEthernetAddress, udpFragmentsAndTcpControlPacketBufferPool, virtualLanTagging, arpCache),
			ipV6State: self.createIpV6State(ethernetPort, &name, logicalCoreMemorySocket, defaultEthernetAddress, udpFragmentsAndTcpControlPacketBufferPool, virtualLanTagging),
		}
	}

	fn createIpV4State(&self, ethernetPort: EthernetPort, name: &LongestPrefixMatchName, logicalCoreMemorySocket: Option<NumaSocketId>, defaultEthernetAddress: &UnicastEthernetAddress, udpFragmentsAndTcpControlPacketBufferPool: *mut rte_mempool, virtualLanTagging: &VirtualLanTagging, arpCache: ArpCache) -> IpV4State
	{
		let routingTable = Rc::new(RefCell::new(self.createIpV4RoutingTable(name, logicalCoreMemorySocket, arpCache.clone(), virtualLanTagging.size() as u16)));

		let sourceIpV4AddressesBlackList = self.createSourceIpV4AddressesBlackList(name, logicalCoreMemorySocket);

		let mut ourIpV4Addresses = HashMap::with_capacity(self.ipV4Addresses.len());

		// These need to be used ONLY within a Logical Core
		for (ipV4Address, ipV4AddressConfiguration) in self.ipV4Addresses.iter()
		{
			let ethernetAddress = ipV4AddressConfiguration.ethernetAddress(defaultEthernetAddress);

			let (tcpContext, tcpDevice) = ipV4AddressConfiguration.createTcpContextAndDevice(ethernetPort, logicalCoreMemorySocket, udpFragmentsAndTcpControlPacketBufferPool, ipV4Address, virtualLanTagging, routingTable.clone());
			let (udpContext, udpDevice) = ipV4AddressConfiguration.createUdpContextAndDevice(ethernetPort, logicalCoreMemorySocket, udpFragmentsAndTcpControlPacketBufferPool, ipV4Address, virtualLanTagging, routingTable.clone());

			ourIpV4Addresses.insert(InternetProtocolVersion4HostAddress::from_ipv4_addr(ipV4Address), IpAddressInformation
			{
				ourEthernetAddress: ethernetAddress,
				tcpContext,
				tcpReceiveBurstBuffer: ReceiveBurstBuffer::new(tcpDevice),
				udpContext,
				udpReceiveBurstBuffer: ReceiveBurstBuffer::new(udpDevice),
			});
		}

		IpV4State
		{
			sourceIpV4AddressBlackList: sourceIpV4AddressesBlackList,
			arpCache,
			ourIpV4Addresses,
			packetReassemblyTable: self.ipV4Fragmentation.create(logicalCoreMemorySocket),
		}
	}

	fn createIpV6State(&self, ethernetPort: EthernetPort, name: &LongestPrefixMatchName, logicalCoreMemorySocket: Option<NumaSocketId>, defaultEthernetAddress: &UnicastEthernetAddress, udpFragmentsAndTcpControlPacketBufferPool: *mut rte_mempool, virtualLanTagging: &VirtualLanTagging) -> IpV6State
	{
		let sourceIpV6AddressesBlackList = self.createSourceIpV6AddressesBlackList(name, logicalCoreMemorySocket);

		let mut ourIpV6Addresses = HashMap::with_capacity(self.ipV6Addresses.len());

		// These need to be used ONLY within a Logical Core
		for (ipV6Address, ipV6AddressConfiguration) in self.ipV6Addresses.iter()
		{
			let ethernetAddress = ipV6AddressConfiguration.ethernetAddress(defaultEthernetAddress);

			let (tcpContext, tcpDevice) = ipV6AddressConfiguration.createTcpContextAndDevice(ethernetPort, logicalCoreMemorySocket, udpFragmentsAndTcpControlPacketBufferPool, ipV6Address, virtualLanTagging);
			let (udpContext, udpDevice) = ipV6AddressConfiguration.createUdpContextAndDevice(ethernetPort, logicalCoreMemorySocket, udpFragmentsAndTcpControlPacketBufferPool, ipV6Address, virtualLanTagging);

			ourIpV6Addresses.insert(InternetProtocolVersion6HostAddress::from_ipv6_addr(ipV6Address), IpAddressInformation
			{
				ourEthernetAddress: ethernetAddress,
				tcpContext,
				tcpDevice,
				udpContext,
				udpDevice,
			});
		}

		IpV6State
		{
			sourceIpV6AddressBlackList: sourceIpV6AddressesBlackList,
			ourIpV6Addresses,
			packetReassemblyTable: self.ipV6Fragmentation.create(logicalCoreMemorySocket),
		}
	}

	fn createIpV4RoutingTable(&self, name: &LongestPrefixMatchName, logicalCoreMemorySocket: Option<NumaSocketId>, arpCache: ArpCache, virtualLanSizeCorrection: u16) -> IpV4RoutingTable
	{
		let mut routingTable = IpV4RoutingTable::new(name, logicalCoreMemorySocket, arpCache, self.ipV4RoutingTableConfiguration.defaultMaximumTransmissionUnitAfterVirtualLanAdjustments(virtualLanSizeCorrection));
		self.ipV4RoutingTableConfiguration.reconfigure(&mut routingTable, virtualLanSizeCorrection);
		routingTable
	}

	fn createSourceIpV4AddressesBlackList(&self, name: &LongestPrefixMatchName, logicalCoreMemorySocket: Option<NumaSocketId>) -> IpV4AddressBlackList
	{
		self.sourceIpV4NetworksToBlackList.create(name, logicalCoreMemorySocket)
	}

	fn createSourceIpV6AddressesBlackList(&self, name: &LongestPrefixMatchName, logicalCoreMemorySocket: Option<NumaSocketId>) -> IpV6AddressBlackList
	{
		self.sourceIpV6NetworksToBlackList.create(name, logicalCoreMemorySocket)
	}
}
