// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Provides a longest prefix match table for internet protocol (IP) version 4 network addresses.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InternetProtocolVersion6LongestPrefixMatchTable(NonNull<rte_lpm6>);

impl Drop for InternetProtocolVersion6LongestPrefixMatchTable
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.delete_all_rules();
		
		unsafe { rte_lpm6_free(self.pointer()) };
	}
}

impl LongestPrefixMatchTable for InternetProtocolVersion6LongestPrefixMatchTable
{
	type InternetProtocolHostAddress = InternetProtocolVersion6HostAddress;

	type InternetProtocolNetworkAddress = InternetProtocolVersion6NetworkAddress;
	
	type Underlying = rte_lpm6;

	#[inline(always)]
	fn new(name: &str, maximum_number_of_rules: u32, number_of_table8s_to_allocate: u32, numa_node_choice: NumaNodeChoice) -> Option<Self>
	{
		const FlagsAreCurrentlyUnused: c_int = 0;

		let configuration = rte_lpm6_config
		{
			max_rules: maximum_number_of_rules,
			number_tbl8s: number_of_table8s_to_allocate,
			flags: FlagsAreCurrentlyUnused,
		};

		let c_name = CString::new(name).expect("InternetProtocolVersion6LongestPrefixMatchTable.new() name contained an interior ASCII NUL and couldn't be converted to a CString");

		let result = unsafe { rte_lpm6_create(c_name.as_ptr(), numa_node_choice.into(), &configuration) };
		if unlikely(result.is_null())
		{
			match unsafe { rust_rte_errno() }
			{
				E_RTE::NO_CONFIG => panic!("rte_lpm6_create() could not get pointer to rte_config structure"),
				E_RTE::SECONDARY => panic!("rte_lpm6_create() was called from a secondary process instance"),

				E::EINVAL => panic!("rte_lpm6_create(): invalid parameter"),
				E::ENOSPC => panic!("rte_lpm6_create(): the maximum number of memzones has already been allocated"),
				E::EEXIST => panic!("rte_lpm6_create(): a memzone with the same name already exists"),
				E::ENOMEM => panic!("rte_lpm6_create(): no appropriate memory area found in which to create memzone"),

				unexpected @ _ => panic!("Unexpected error code '{}' from rte_lpm6_create()", unexpected),
			}
		}
		else
		{
			Some(InternetProtocolVersion6LongestPrefixMatchTable(unsafe { NonNull::new_unchecked(result) }))
		}
	}

	#[inline(always)]
	fn look_up(&self, internet_protocol_address: &Self::InternetProtocolHostAddress) -> Option<RoutingTableKey>
	{
		let mut routing_table_key = unsafe { uninitialized() };

		let result = unsafe { rte_lpm6_lookup(self.pointer(), internet_protocol_address as *const _ as *mut _, &mut routing_table_key) };
		
		if likely(result == 0)
		{
			Some(routing_table_key)
		}
		else if likely(result == NegativeE::ENOENT)
		{
			None
		}
		else
		{
			match result
			{
				NegativeE::EINVAL => panic!("rte_lpm6_lookup(): invalid parameter"),
				
				unexpected @ _ => panic!("Unexpected error code '{}' from rte_lpm6_lookup()", unexpected),
			}
		}
	}

	#[inline(always)]
	fn add_rule(&mut self, network_address: &Self::InternetProtocolNetworkAddress, routing_table_key: RoutingTableKey) -> bool
	{
		let internet_protocol_address = network_address.network();
		let depth = network_address.mask_bits_as_depth();

		let result = unsafe { rte_lpm6_add(self.pointer(), internet_protocol_address as *const _ as *mut _, depth, routing_table_key) };
		if likely(result == 0)
		{
			true
		}
		else if likely(result < 0)
		{
			false
		}
		else
		{
			panic!("Unexpected positive value '{}' from rte_lpm6_add()", result);
		}
	}

	#[inline(always)]
	fn has_rule(&self, network_address: &Self::InternetProtocolNetworkAddress) -> Option<RoutingTableKey>
	{
		let internet_protocol_address = network_address.network();
		let depth = network_address.mask_bits_as_depth();

		let mut routing_table_key = unsafe { uninitialized() };

		let result = unsafe { rte_lpm6_is_rule_present(self.pointer(), internet_protocol_address as *const _ as *mut _, depth, &mut routing_table_key) };
		
		match result
		{
			1 => Some(routing_table_key),
			0 => None,
			negative if negative < 0 => panic!("Failure '{}' of some sort in rte_lpm6_is_rule_present()", negative),
			unexpected @ _ => panic!("Unexpected positive value '{}' from rte_lpm6_add()", unexpected),
		}
	}

	#[inline(always)]
	fn delete_rule(&mut self, network_address: &Self::InternetProtocolNetworkAddress) -> bool
	{
		let internet_protocol_address = network_address.network();
		let depth = network_address.mask_bits_as_depth();

		let result = unsafe { rte_lpm6_delete(self.pointer(), internet_protocol_address as *const _ as *mut _, depth) };
		if likely(result == 0)
		{
			true
		}
		else if likely(result < 0)
		{
			false
		}
		else
		{
			panic!("Unexpected positive value '{}' from rte_lpm6_add()", result);
		}
	}

	#[inline(always)]
	fn delete_all_rules(&mut self)
	{
		unsafe { rte_lpm6_delete_all(self.pointer()) }
	}
	
	#[inline(always)]
	fn find(name: &str) -> Option<NonNull<Self::Underlying>>
	{
		let c_name = CString::new(name).expect("InternetProtocolVersion6LongestPrefixMatchTable.find() name contained an interior ASCII NUL and couldn't be converted to a CString");
		
		let result = unsafe { rte_lpm6_find_existing(c_name.as_ptr()) };
		if result.is_null()
		{
			None
		}
		else
		{
			Some(unsafe { NonNull::new_unchecked(result) })
		}
	}
}

impl InternetProtocolVersion6LongestPrefixMatchTable
{
	#[inline(always)]
	fn pointer(&self) -> *mut rte_lpm6
	{
		self.0.as_ptr()
	}
}
