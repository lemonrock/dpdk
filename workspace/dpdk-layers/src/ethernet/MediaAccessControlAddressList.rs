// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.



/// Blacklisted or Whitelisted MediaAccessControlAddresses.
///
/// Defaults to an empty `Blacklist`.
#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub enum MediaAccessControlAddressList
{
	/// Blacklist.
	///
	/// If the media access control address occurs in *either* of `organizationally_unique_identifiers_in_network_byte_order` or `other_media_access_control_addresses`, then it is blacklisted.
	Blacklist
	{
		/// Organizationally Unique Identifiers (OUIs), universally administered.
		#[serde(default)]
		organizationally_unique_identifiers_in_network_byte_order: HashSet<[u8; 3]>,
		
		/// Unicast media access control addresses.
		///
		/// There is no need to include the addresses of our ethernet interface.
		#[serde(default)]
		unicast_media_access_control_addresses: HashSet<MediaAccessControlAddress>
	},
	
	/// Whitelist.
	///
	/// If the media access control address occurs in *either* of `organizationally_unique_identifiers_in_network_byte_order` or `other_media_access_control_addresses`, then it is whitelisted.
	Whitelist
	{
		/// Organizationally Unique Identifiers (OUIs), universally administered.
		#[serde(default)]
		organizationally_unique_identifiers_in_network_byte_order: HashSet<[u8; 3]>,
		
		/// Unicast media access control addresses.
		///
		/// There is no need to include the addresses of our ethernet interface.
		#[serde(default)]
		unicast_media_access_control_addresses: HashSet<MediaAccessControlAddress>
	}
}

impl Default for MediaAccessControlAddressList
{
	#[inline(always)]
	fn default() -> Self
	{
		MediaAccessControlAddressList::Blacklist
		{
			organizationally_unique_identifiers_in_network_byte_order: Default::default(),
			unicast_media_access_control_addresses: Default::default(),
		}
	}
}

impl MediaAccessControlAddressList
{
	/// Is this `media_access_control_address` denied?
	#[inline(always)]
	pub fn is_denied(&self, media_access_control_address: &MediaAccessControlAddress) -> bool
	{
		use self::MediaAccessControlAddressList::*;
		
		if let Some((organizationally_unique_identifier, _)) = media_access_control_address.universally_administered_organizationally_unique_identifier()
		{
			match *self
			{
				Blacklist { ref organizationally_unique_identifiers_in_network_byte_order, ref unicast_media_access_control_addresses } => organizationally_unique_identifiers_in_network_byte_order.contains(organizationally_unique_identifier) || unicast_media_access_control_addresses.contains(media_access_control_address),
				Whitelist { ref organizationally_unique_identifiers_in_network_byte_order, ref unicast_media_access_control_addresses } => !(organizationally_unique_identifiers_in_network_byte_order.contains(organizationally_unique_identifier) || unicast_media_access_control_addresses.contains(media_access_control_address)),
			}
		}
		else
		{
			match *self
			{
				Blacklist { ref unicast_media_access_control_addresses, .. } => unicast_media_access_control_addresses.contains(media_access_control_address),
				Whitelist { ref unicast_media_access_control_addresses, .. } => !(unicast_media_access_control_addresses.contains(media_access_control_address)),
			}
		}
	}
}
