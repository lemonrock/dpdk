// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct SourceEthernetAddressBlackListConfiguration
{
	pub ethernetAddressAdministrationProhibition: EthernetAddressAdministrationProhibition,
	pub organizationallyUniqueIdentifiersBlackListed: HashSet<OrganizationallyUniqueIdentifier>,
	pub specificAddressesBlackListed: HashSet<UnicastEthernetAddress>,
}

impl Default for SourceEthernetAddressBlackListConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			ethernetAddressAdministrationProhibition: Default::default(),
			organizationallyUniqueIdentifiersBlackListed: Default::default(),
			specificAddressesBlackListed: Default::default(),
		}
	}
}
