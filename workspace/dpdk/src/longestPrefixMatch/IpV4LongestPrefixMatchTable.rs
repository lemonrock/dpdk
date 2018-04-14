// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IpV4LongestPrefixMatchTable(*mut rte_lpm);

impl Drop for IpV4LongestPrefixMatchTable
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { rte_lpm_free(self.0) };
	}
}

impl LongestPrefixMatchTable for IpV4LongestPrefixMatchTable
{
	type IpHostAddress = InternetProtocolVersion4HostAddress;

	type IpNetworkAddress = IpV4NetworkAddress;

	#[inline(always)]
	fn new(name: &str, maximumRules: u32, numberOfTable8sToAllocate: u32, numa_socket_id: Option<NumaSocketId>) -> Option<Self>
	{
		const FlagsAreCurrentlyUnused: c_int = 0;

		let configuration = rte_lpm_config
		{
			max_rules: maximumRules,
			number_tbl8s: numberOfTable8sToAllocate,
			flags: FlagsAreCurrentlyUnused,
		};

		let cName = CString::new(name).expect("IpV4LongestPrefixMatchTable.new() name contained an interior ASCII NUL and couldn't be converted to a CString");

		let result = unsafe { rte_lpm_create(cName.as_ptr(), numa_socket_id.as_c_int(), &configuration) };
		if unlikely(result.is_null())
		{
			match unsafe { rust_rte_errno() }
			{
				E_RTE::NO_CONFIG => panic!("rte_lpm_create() could not get pointer to rte_config structure"),
				E_RTE::SECONDARY => panic!("rte_lpm_create() was called from a secondary process instance"),

				E::EINVAL => panic!("rte_lpm_create(): invalid parameter"),
				E::ENOSPC => panic!("rte_lpm_create(): the maximum number of memzones has already been allocated"),
				E::EEXIST => panic!("rte_lpm_create(): a memzone with the same name already exists"),
				E::ENOMEM => panic!("rte_lpm_create(): no appropriate memory area found in which to create memzone"),

				unexpected @ _ => panic!("Unexpected error code '{}' from rte_lpm_create()", unexpected),
			}
		}
		else
		{
			Some(IpV4LongestPrefixMatchTable(result))
		}
	}

	#[inline(always)]
	fn lookUp(&self, internet_protocol_address: &Self::IpHostAddress) -> Option<NextHop>
	{
		let mut nextHop: NextHop = unsafe { uninitialized() };

		let result = unsafe { rust_rte_lpm_lookup(self.0, *internet_protocol_address, &mut nextHop) };

		match result
		{
			0 => Some(nextHop),
			NegativeE::ENOENT => None,

			NegativeE::EINVAL => panic!("rust_rte_lpm_lookup(): invalid parameter"),

			unexpected @ _ => panic!("Unexpected error code '{}' from rust_rte_lpm_lookup()", unexpected),
		}
	}

	#[inline(always)]
	fn addRule(&mut self, networkAddress: &Self::IpNetworkAddress, nextHop: NextHop) -> bool
	{
		let internet_protocol_address = networkAddress.network();
		let depth = networkAddress.maskBitsAsDepth();

		let result = unsafe { rte_lpm_add(self.0, *internet_protocol_address, depth, nextHop) };
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
			panic!("Unexpected positive value '{}' from rte_lpm_add()", result);
		}
	}

	#[inline(always)]
	fn hasRule(&self, networkAddress: &Self::IpNetworkAddress) -> Option<NextHop>
	{
		let internet_protocol_address = networkAddress.network();
		let depth = networkAddress.maskBitsAsDepth();

		let mut nextHop: NextHop = unsafe { uninitialized() };

		let result = unsafe { rte_lpm_is_rule_present(self.0, *internet_protocol_address, depth, &mut nextHop) };

		match result
		{
			1 => Some(nextHop),
			0 => None,
			negative if negative < 0 => panic!("Failure '{}' of some sort in rte_lpm_is_rule_present()", negative),
			unexpected @ _ => panic!("Unexpected positive value '{}' from rte_lpm_add()", unexpected),
		}
	}

	#[inline(always)]
	fn deleteRule(&mut self, networkAddress: &Self::IpNetworkAddress) -> bool
	{
		let internet_protocol_address = networkAddress.network();
		let depth = networkAddress.maskBitsAsDepth();

		let result = unsafe { rte_lpm_delete(self.0, *internet_protocol_address, depth) };
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
			panic!("Unexpected positive value '{}' from rte_lpm_add()", result);
		}
	}

	#[inline(always)]
	fn deleteAllRules(&mut self)
	{
		unsafe { rte_lpm_delete_all(self.0) }
	}
}

impl IpV4LongestPrefixMatchTable
{
	#[inline(always)]
	pub fn find(name: &str) -> Option<*mut rte_lpm>
	{
		let cName = CString::new(name).expect("IpV4LongestPrefixMatchTable.find() name contained an interior ASCII NUL and couldn't be converted to a CString");

		let result = unsafe { rte_lpm_find_existing(cName.as_ptr()) };
		if result.is_null()
		{
			None
		} else {
			Some(result)
		}
	}
}
