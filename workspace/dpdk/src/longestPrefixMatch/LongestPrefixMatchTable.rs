// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait LongestPrefixMatchTable : Drop
{
	type InternetProtocolHostAddress: Sized;

	type InternetProtocolNetworkAddress: InternetProtocolNetworkAddress<InternetProtocolHostAddress=Self::InternetProtocolHostAddress> + Sized;

	#[inline(always)]
	fn new(name: &str, maximumRules: u32, numberOfTable8sToAllocate: u32, numa_socket_id: Option<NumaSocketId>) -> Option<Self> where Self: Sized;

	#[inline(always)]
	fn look_up(&self, internet_protocol_address: &Self::InternetProtocolHostAddress) -> Option<NextHop>;

	#[inline(always)]
	fn addRule(&mut self, networkAddress: &Self::InternetProtocolNetworkAddress, nextHop: NextHop) -> bool;

	#[inline(always)]
	fn hasRule(&self, networkAddress: &Self::InternetProtocolNetworkAddress) -> Option<NextHop>;

	#[inline(always)]
	fn deleteRule(&mut self, networkAddress: &Self::InternetProtocolNetworkAddress) -> bool;

	#[inline(always)]
	fn deleteAllRules(&mut self);
}
