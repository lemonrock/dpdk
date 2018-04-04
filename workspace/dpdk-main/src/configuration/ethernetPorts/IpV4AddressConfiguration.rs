// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[serde(default)]
struct IpV4AddressConfiguration
{
	ethernetAddress: Option<UnicastEthernetAddress>,
	tcp: Layer4ProtocolConfiguration,
	udp: Layer4ProtocolConfiguration,
}

impl Default for IpV4AddressConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			ethernetAddress: None,
			tcp: Default::default(),
			udp: Default::default(),
		}
	}
}

impl AppendAdditionalEthernetAddresses for IpV4AddressConfiguration
{
	#[inline(always)]
	fn appendAdditionalEthernetAddresses(&self, defaultEthernetAddress: &UnicastEthernetAddress, additionalEthernetAddresses: &mut HashSet<UnicastEthernetAddress>)
	{
		if let Some(ethernetAddress) = self.ethernetAddress.as_ref()
		{
			if ethernetAddress != defaultEthernetAddress
			{
				if !additionalEthernetAddresses.contains(ethernetAddress)
				{
					additionalEthernetAddresses.insert(*ethernetAddress);
				}
			}
		}
	}
}

impl IpV4AddressConfiguration
{
	pub fn ethernetAddress(&self, defaultEthernetAddress: &UnicastEthernetAddress) -> ether_addr
	{
		match self.ethernetAddress
		{
			None => (defaultEthernetAddress.0).0,
			Some(ref ours) => (ours.0).0,
		}
	}
	
	pub fn createTcpContextAndDevice(&self, ethernetPort: EthernetPort, logicalCoreMemorySocket: Option<NumaSocketId>, udpFragmentsAndTcpControlPacketBufferPool: *mut rte_mempool, ipV4Address: &Ipv4Addr, virtualLanTagging: &VirtualLanTagging, ipV4RoutingTable: Rc<RefCell<IpV4RoutingTable>>) -> (TcpContext<IpV4AddressLookUpForSendCallback, NeverRouteAddressLookUpForSendCallback>, TcpDevice)
	{
		self.tcp.createTldkContextIpV4(ethernetPort, logicalCoreMemorySocket, udpFragmentsAndTcpControlPacketBufferPool, ipV4Address, virtualLanTagging, Layer4Protocol::Tcp, ipV4RoutingTable)
	}
	
	pub fn createUdpContextAndDevice(&self, ethernetPort: EthernetPort, logicalCoreMemorySocket: Option<NumaSocketId>, udpFragmentsAndTcpControlPacketBufferPool: *mut rte_mempool, ipV4Address: &Ipv4Addr, virtualLanTagging: &VirtualLanTagging, ipV4RoutingTable: Rc<RefCell<IpV4RoutingTable>>) -> (UdpContext<IpV4AddressLookUpForSendCallback, NeverRouteAddressLookUpForSendCallback>, UdpDevice)
	{
		self.udp.createTldkContextIpV4(ethernetPort, logicalCoreMemorySocket, udpFragmentsAndTcpControlPacketBufferPool, ipV4Address, virtualLanTagging, Layer4Protocol::Udp, ipV4RoutingTable)
	}
}
