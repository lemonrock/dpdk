// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A trait for operations on longest prefix match tables for internet protocol version 4 or version 4 host addresses.
pub trait LongestPrefixMatchTable : Drop
{
	/// Internet Protocol (IP) version 4 or version 4 host address.
	type InternetProtocolHostAddress: Sized;
	
	/// Internet Protocol (IP) version 4 or version 4 network address.
	type InternetProtocolNetworkAddress: InternetProtocolNetworkAddress<InternetProtocolHostAddress=Self::InternetProtocolHostAddress> + Sized;
	
	/// Underlying DPDK type of Longest Prefix Match table.
	type Underlying: Sized;
	
	/// Create a new instance.
	///
	/// Names need to be unique.
	///
	/// It is recommended to create a new look up table per logical socket.
	///
	/// Virtual LAN and outgoing port ethernet address should also be considerations.
	#[inline(always)]
	fn new(name: &str, maximum_number_of_rules: u32, number_of_table8s_to_allocate: u32, numa_node_choice: NumaNodeChoice) -> Option<Self> where Self: Sized;

	/// Look up a host address to try to find a routing table.
	///
	/// It is advisable to add a default route.
	///
	/// There is an alternative version (not supported yet by us) that allows bulk look up in multiples of 8 addresses for better performance.
	#[inline(always)]
	fn look_up(&self, internet_protocol_address: &Self::InternetProtocolHostAddress) -> Option<RoutingTableKey>;
	
	/// Add a rule.
	#[inline(always)]
	fn add_rule(&mut self, network_address: &Self::InternetProtocolNetworkAddress, routing_table_key: RoutingTableKey) -> bool;
	
	/// Does this table have this rule?
	#[inline(always)]
	fn has_rule(&self, network_address: &Self::InternetProtocolNetworkAddress) -> Option<RoutingTableKey>;
	
	/// Delete a rule.
	#[inline(always)]
	fn delete_rule(&mut self, network_address: &Self::InternetProtocolNetworkAddress) -> bool;

	/// Delete all rules.
	#[inline(always)]
	fn delete_all_rules(&mut self);
	
	/// Try to find an existing longest prefix match table for internet protocol (IP) version 4 host addresses.
	#[inline(always)]
	fn find(name: &str) -> Option<NonNull<Self::Underlying>>;
}
