// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub struct SourceEthernetAddressBlackList
{
	ethernetAddressAdministrationProhibition: Arc<RwLock<EthernetAddressAdministrationProhibition>>,
	organizationallyUniqueIdentifiersBlackListed: Arc<RwLock<HashSet<OrganizationallyUniqueIdentifier>>>,
	specificAddressesBlackListed: Arc<RwLock<HashSet<ether_addr>>>,
}

impl From<SourceEthernetAddressBlackListConfiguration> for SourceEthernetAddressBlackList
{
	#[inline(always)]
	fn from(sourceEthernetAddressBlackListConfiguration: SourceEthernetAddressBlackListConfiguration) -> Self
	{
		let x = sourceEthernetAddressBlackListConfiguration.specificAddressesBlackListed.iter().map(|value| (value.0).0).collect();
		
		Self
		{
			ethernetAddressAdministrationProhibition: Arc::new(RwLock::new(sourceEthernetAddressBlackListConfiguration.ethernetAddressAdministrationProhibition)),
			organizationallyUniqueIdentifiersBlackListed: Arc::new(RwLock::new(sourceEthernetAddressBlackListConfiguration.organizationallyUniqueIdentifiersBlackListed)),
			specificAddressesBlackListed: Arc::new(RwLock::new(x)),
		}
	}
}

impl SourceEthernetAddressBlackList
{
	#[inline(always)]
	pub fn isSourceEthernetAddressInvalidOrBlackListed(&self, sourceEthernetAddress: *const ether_addr) -> bool
	{
		MediaAccessControlAddress::ethernetAddressIsInvalid(sourceEthernetAddress) || self.isAdministeredAddressProhibited(sourceEthernetAddress) || self.isOrganizationallyUniqueIdentifiersBlackListed(sourceEthernetAddress) || self.isSpecificAddressesBlackListed(sourceEthernetAddress)
	}
	
	#[inline(always)]
	fn isAdministeredAddressProhibited(&self, sourceEthernetAddress: *const ether_addr) -> bool
	{
		let readLock = self.ethernetAddressAdministrationProhibition.read().unwrap();
		readLock.isProhibited(sourceEthernetAddress)
	}
	
	#[inline(always)]
	fn isOrganizationallyUniqueIdentifiersBlackListed(&self, sourceEthernetAddress: *const ether_addr) -> bool
	{
		let nastyHack: &OrganizationallyUniqueIdentifier = unsafe { transmute(sourceEthernetAddress) };
		
		let readLock = self.organizationallyUniqueIdentifiersBlackListed.read().unwrap();
		readLock.contains(nastyHack)
	}
	
	#[inline(always)]
	fn isSpecificAddressesBlackListed(&self, sourceEthernetAddress: *const ether_addr) -> bool
	{
		let readLock = self.specificAddressesBlackListed.read().unwrap();
		readLock.contains(unsafe { &*sourceEthernetAddress })
	}
}
