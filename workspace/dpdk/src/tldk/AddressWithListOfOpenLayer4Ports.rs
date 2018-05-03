// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddressWithListOfOpenLocalLayer4Ports<A>
{
	internet_protocol_address: A,
	whitelist_of_ports: HashSet<Layer4Port>,
}

impl<A> AddressWithListOfOpenLocalLayer4Ports<A>
{
	#[inline(always)]
	pub fn new(internet_protocol_address: A, whitelist_of_ports: HashSet<Layer4Port>) -> Self
	{
		Self
		{
			internet_protocol_address,
			whitelist_of_ports,
		}
	}

	#[inline(always)]
	pub fn blockedPortsForTldk(&self) -> TldkBlockedPortsList
	{
		const MinimumPortInclusive: usize = 1;
		const MaximumPortExclusive: usize = 65536;

		let mut blocked_ports = Vec::with_capacity(MaximumPortExclusive - self.whitelist_of_ports.len());
		// We include Port 0 for TLDK, even though it's not a valid TCP or UDP port
		blocked_ports.push(0);

		for port in MinimumPortInclusive .. MaximumPortExclusive
		{
			let port = port as u16;
			if !self.whitelist_of_ports.contains(&Layer4Port::convert_from_tcp_or_udp_or_sctp_port_value_in_layer_4_header(port).unwrap())
			{
				blocked_ports.push(port);
			}
		}

		blocked_ports.sort();

		return blocked_ports
	}
}

impl AddressWithListOfOpenLocalLayer4Ports<Ipv4Addr>
{
	#[inline(always)]
	pub fn in_addr(&self) -> in_addr
	{
		InternetProtocolVersion4HostAddress::from_ipv4_addr_to_in_addr(&self.internet_protocol_address)
	}
}

impl AddressWithListOfOpenLocalLayer4Ports<Ipv6Addr>
{
	#[inline(always)]
	pub fn in6_addr(&self) -> in6_addr
	{
		InternetProtocolVersion6HostAddress::from_rust_address_to_libc_address(&self.internet_protocol_address)
	}
}
