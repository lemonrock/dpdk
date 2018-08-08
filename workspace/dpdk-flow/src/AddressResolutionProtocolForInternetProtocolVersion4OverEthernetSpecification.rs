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

macro_rules! expecting
{
	($type: tt, $formatter: ident) =>
	{
		$formatter.write_str(stringify!($type))
	}
}


macro_rules! sequence_field
{
	($self: ident, $access: ident, $length: expr) =>
	{
		$access.next_element()?.ok_or_else(|| DeserializerError::invalid_length($length, &$self))?
	}
}

macro_rules! decode_from_sequence
{
	(
		$type: tt,
		$self: ident,
        $access: ident
        $(
            ,$length: expr
        )*
	) =>
	{
		{
			Ok
			(
				$type::new
				(
					$(sequence_field!($self, $access, $length),)*
				)
			)
		}
	}
}

macro_rules! decode_from_map
{
	(
		$type: tt,
        $access: expr,
        $(
            $field_name: tt,
        )*
	) =>
	{
		{
			#[allow(non_camel_case_types)]
			#[derive(Deserialize)]
			enum Field
			{
				$($field_name,)*
			}
			
			$(
			let mut $field_name = None;
			)*
			
			decode_loop!($access, $($field_name => $field_name,)*);
			
			Ok($type::new($(map_field!($field_name),)*))
		}
	}
}

macro_rules! decode_loop
{
	(
        $access: expr,
        $(
            $field_name: tt => $field_value: ident,
        )*
	) =>
	{
		while let Some(key) = $access.next_key()?
		{
			decode_match!
			{
				key,
				$access,
				$($field_name => $field_value,)*
			}
		}
	}
}

macro_rules! decode_match
{
    (
    	$key: expr,
        $access: expr,
        $(
            $field_name: tt => $field_value: ident,
        )*
    ) =>
	{
        match $key
        {
            $(
                Field::$field_name =>
                {
					if $field_value.is_some()
					{
						return Err(DeserializerError::duplicate_field(stringify!($field_value)));
					}
					$field_value = Some($access.next_value()?);
                }
            )*
        }
    }
}

macro_rules! map_field
{
	($field_value: ident) =>
	{
		$field_value.ok_or_else(|| DeserializerError::missing_field(stringify!($field_value)))?
	}
}

impl<'deserialize> Deserialize<'deserialize> for AddressResolutionProtocolForInternetProtocolVersion4OverEthernetSpecification
{
	#[inline(always)]
	fn deserialize<D: Deserializer<'deserialize>>(deserializer: D) -> Result<Self, D::Error>
	{
		struct DeserializingVisitor;
		
		impl<'deserialize> Visitor<'deserialize> for DeserializingVisitor
		{
			type Value = AddressResolutionProtocolForInternetProtocolVersion4OverEthernetSpecification;
			
			#[inline(always)]
			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result
			{
				expecting!(AddressResolutionProtocolForInternetProtocolVersion4OverEthernetSpecification, formatter)
			}
			
			#[inline(always)]
			fn visit_seq<V: SeqAccess<'deserialize>>(self, mut access: V) -> Result<Self::Value, V::Error>
			{
				decode_from_sequence!(AddressResolutionProtocolForInternetProtocolVersion4OverEthernetSpecification, self, access, 0, 1, 2, 3, 4)
			}
			
			#[inline(always)]
			fn visit_map<V: MapAccess<'deserialize>>(self, mut access: V) -> Result<Self::Value, V::Error>
			{
				decode_from_map!
				(
					AddressResolutionProtocolForInternetProtocolVersion4OverEthernetSpecification,
					access,
					source_ethernet_address,
					destination_ethernet_address,
					source_internet_protocol_version_4_address,
					destination_internet_protocol_version_4_address,
					operation,
				)
			}
		}
		
		deserializer.deserialize_struct("AddressResolutionProtocolForInternetProtocolVersion4OverEthernetSpecification", &["source_ethernet_address", "destination_ethernet_address", "source_internet_protocol_version_4_address", "destination_internet_protocol_version_4_address", "operation"], DeserializingVisitor)
	}
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
