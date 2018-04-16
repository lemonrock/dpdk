// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[serde(default)]
struct Layer4ProtocolConfiguration
{
	differentiatedServiceCodePoint: DifferentiatedServiceCodePoint,
	hopLimits: u8,
	maximumNumberOfStreams: u32,
	maximumNumberOfReceiveBuffers: u32,
	maximumNumberOfSendBuffers: u32,
	sendMaximumBulkNumber: u32,
	services: HashMap<Layer4Port, Service>,
}

impl Default for Layer4ProtocolConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			differentiatedServiceCodePoint: Default::default(),
			hopLimits: 64,
			maximumNumberOfStreams: 4096,
			maximumNumberOfReceiveBuffers: 1, // Defaults to 1 if set to 0
			maximumNumberOfSendBuffers: 1, // Defaults to 1 if set to 0
			sendMaximumBulkNumber: 32, // Defaults to 32 if set to 0
			services: Default::default(),
		}
	}
}

impl Layer4ProtocolConfiguration
{
	fn createTldkContextIpV4<C: Context<IpV4AddressLookUpForSendCallback, NeverRouteAddressLookUpForSendCallback>>(&self, ethernetPort: EthernetPort, logicalCoreMemorySocket: Option<NumaSocketId>, udpFragmentsAndTcpControlPacketBufferPool: *mut rte_mempool, ipV4Address: &Ipv4Addr, virtualLanTagging: &VirtualLanTagging, layer4Protocol: Layer4Protocol, ipV4RoutingTable: Rc<RefCell<IpV4RoutingTable>>) -> (C, C::Device)
	{
		let ethernetPortInformation = ethernetPort.information();

		let openPorts: HashSet<Layer4Port> = self.services.keys().map(|port| *port).collect();

		let addressLookUpForSendToIpV4 = Rc::new(RefCell::new(IpV4AddressLookUpForSendCallback::new(ipV4RoutingTable, udpFragmentsAndTcpControlPacketBufferPool, virtualLanTagging, self.differentiatedServiceCodePoint, self.hopLimits, layer4Protocol)));
		let addressLookUpForSendToIpV6 = Rc::new(RefCell::new(NeverRouteAddressLookUpForSendCallback));
		let deviceConfiguration = DeviceConfiguration
		{
			deviceOffloadingIsActive: ethernetPortInformation.receiveIpV4TcpAndUdpChecksumOffloadSupported(),
			localSocketAddresses: InternetProtocolVersion4OrVersion6OrBoth::only_internet_protocol_version_4(AddressWithListOfOpenLocalLayer4Ports::new(ipV4Address.clone(), openPorts))
		};
		let deviceConfigurations = vec![deviceConfiguration];

		if let Some((context, mut optionDeviceList)) = C::create(logicalCoreMemorySocket, self.maximumNumberOfStreams, self.maximumNumberOfReceiveBuffers, self.maximumNumberOfSendBuffers, self.sendMaximumBulkNumber, addressLookUpForSendToIpV4.clone(), addressLookUpForSendToIpV6, deviceConfigurations)
		{
			let (deviceConfiguration, mut device) = optionDeviceList.pop().unwrap();
			let handle = device.handle();
			addressLookUpForSendToIpV4.borrow_mut().assignTleDeviceAfterContextCreated(handle);

			(context, device)
		}
		else
		{
			panic!("Could not create Context and Device");
		}
	}

	fn createTldkContextIpV6<C: Context<IpV4AddressLookUpForSendCallback, NeverRouteAddressLookUpForSendCallback>>(&self, ethernetPort: EthernetPort, logicalCoreMemorySocket: Option<NumaSocketId>, udpFragmentsAndTcpControlPacketBufferPool: *mut rte_mempool, ipV6Address: &Ipv6Addr, virtualLanTagging: &VirtualLanTagging, layer4Protocol: Layer4Protocol) -> (C, C::Device)
	{
		unimplemented!();
	}
}
