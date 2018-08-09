// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Specification for an `PacketMatcher::AddressResolutionProtocolForInternetProtocolVersion4OverEthernet`.
#[derive(Debug)]
#[derive(Serialize)]
pub struct AddressResolutionProtocolForInternetProtocolVersion4OverEthernetSpecification
{
	/// Source ethernet address.
	pub source_ethernet_address: MediaAccessControlAddress,
	
	/// Destination ethernet address.
	pub destination_ethernet_address: MediaAccessControlAddress,
	
	/// Source internet protocol version 4 address.
	pub source_internet_protocol_version_4_address: InternetProtocolVersion4HostAddress,
	
	/// Destination internet protocol version 4 address.
	pub destination_internet_protocol_version_4_address: InternetProtocolVersion4HostAddress,
	
	/// Operation; recommended to be either Request or Reply.
	pub operation: Operation,
	
	#[serde(skip_serializing)]
	cached: rte_flow_item_arp_eth_ipv4,
}

macro_rules! sequence_field
{
	($self: ident, $access: ident, $field_index: expr) =>
	{
		$access.next_element()?.ok_or_else(|| DeserializerError::invalid_length($field_index, &$self))?
	}
}

macro_rules! map_field
{
	($field_name: ident) =>
	{
		$field_name.ok_or_else(|| DeserializerError::missing_field(stringify!($field_name)))?
	}
}

macro_rules! custom_deserialize
{
	(
		$type: tt,
		$(
			$field_index: expr => $field_name: tt,
		)*
	) =>
	{
		impl<'deserialize> Deserialize<'deserialize> for $type
		{
			#[inline(always)]
			fn deserialize<D: Deserializer<'deserialize>>(deserializer: D) -> Result<Self, D::Error>
			{
				struct DeserializingVisitor;
				
				impl<'deserialize> Visitor<'deserialize> for DeserializingVisitor
				{
					type Value = $type;
					
					#[inline(always)]
					fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result
					{
						formatter.write_str(stringify!($type))
					}
					
					#[inline(always)]
					fn visit_seq<V: SeqAccess<'deserialize>>(self, mut access: V) -> Result<Self::Value, V::Error>
					{
						Ok
						(
							$type::new
							(
								$(
									sequence_field!(self, access, $field_index),
								)*
							)
						)
					}
					
					#[inline(always)]
					fn visit_map<V: MapAccess<'deserialize>>(self, mut access: V) -> Result<Self::Value, V::Error>
					{
						#[allow(non_camel_case_types)]
						#[derive(Deserialize)]
						enum Field
						{
							$(
								$field_name,
							)*
						}
						
						$(
							let mut $field_name = None;
						)*
						
						while let Some(key) = access.next_key()?
						{
							match key
							{
								$(
									Field::$field_name =>
									{
										if $field_name.is_some()
										{
											return Err(DeserializerError::duplicate_field(stringify!($field_name)));
										}
										$field_name = Some(access.next_value()?);
									}
								)*
							}
						}
						
						Ok
						(
							$type::new
							(
								$(
									map_field!($field_name),
								)*
							)
						)
					}
				}
				
				deserializer.deserialize_struct
				(
					stringify!($type),
					&[
						$(
							stringify!($field_name),
						)*
					],
					DeserializingVisitor
				)
			}
		}
	}
}

custom_deserialize!
{
	AddressResolutionProtocolForInternetProtocolVersion4OverEthernetSpecification,
	0 => source_ethernet_address,
	1 => destination_ethernet_address,
	2 => source_internet_protocol_version_4_address,
	3 => destination_internet_protocol_version_4_address,
	4 => operation,
}

impl AddressResolutionProtocolForInternetProtocolVersion4OverEthernetSpecification
{
	/// Creates a new instance.
	#[inline(always)]
	pub fn new(source_ethernet_address: MediaAccessControlAddress, destination_ethernet_address: MediaAccessControlAddress, source_internet_protocol_version_4_address: InternetProtocolVersion4HostAddress, destination_internet_protocol_version_4_address: InternetProtocolVersion4HostAddress, operation: Operation) -> Self
	{
		Self
		{
			source_ethernet_address,
			destination_ethernet_address,
			source_internet_protocol_version_4_address,
			destination_internet_protocol_version_4_address,
			operation,
			cached: rte_flow_item_arp_eth_ipv4
			{
				hrd: HardwareType::Ethernet2.to_network_endian(),
				pro: EtherType::AddressResolutionProtocol.to_network_endian(),
				hln: MediaAccessControlAddress::SizeU8,
				pln: InternetProtocolVersion4HostAddress::SizeU8,
				op: operation.to_network_endian(),
				sha: source_ethernet_address.to_ether_addr(),
				spa: source_internet_protocol_version_4_address.as_network_endian(),
				tha: destination_ethernet_address.to_ether_addr(),
				tpa: destination_internet_protocol_version_4_address.as_network_endian(),
			}
		}
	}
}
