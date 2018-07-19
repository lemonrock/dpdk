// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Captures the reason and salient data for dropping with a packet earlier than might be expected.
///
/// This reason is reported immediately before the underlying packet is dropped, at which point all referenced data will no longer exist.
///
/// Salient data is by its nature unlikely to always be completely valid, and should be used only as a source of raw bytes.
#[derive(Debug)]
pub enum PacketProcessingDropReason<'a>
{
	// Re-use the packet in reply (temporary reason until logic has been implemented).
	ReuseInReply,
	
	/// Occurs during Ethernet packet processing.
	IsTooShortToBeAnEthernetPacket,
	
	/// Occurs during Ethernet packet processing.
	SourceEthernetAddressIsNotValidUnicast
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'a EthernetAddresses,
	},
	
	/// Occurs during Ethernet packet processing.
	SourceEthernetAddressIsOurUnicastEthernetAddress
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'a EthernetAddresses,
	},
	
	/// Occurs during Ethernet packet processing.
	DeniedSourceEthernetAddress
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'a EthernetAddresses,
	},
	
	/// Occurs during Ethernet packet processing.
	DestinationEthernetAddressIsZero
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'a EthernetAddresses,
	},
	
	/// Occurs during Ethernet packet processing.
	///
	/// This can occur if a link has multiple ethernet addresses or is listening promiscuously.
	DestinationEthernetAddressIsNotOneOfOurs
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'a EthernetAddresses,
	},
	
	/// Occurs during Ethernet packet processing.
	///
	/// Currently recognised and supported EtherTypes are Internet Protocol (IP) Version 4, Internet Protocol (IP) Version 6, Address Resolution Protocol, Virtual LAN tagging (801.1q) and QinQ Virtual LAN tagging (802.1ad).
	/// Ether frame sizes are entirely unsupported.
	UnsupportedEtherType
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'a EthernetAddresses,
		
		/// EtherType or LegacyEthernetFrameSize.
		unsuspported_ether_type_or_legacy_ethernet_frame_size: EtherTypeOrLegacyEthernetFrameSize,
	},
	
	/// Occurs during Ethernet packet processing of an IEEE 802.1Q (Virtual LAN) packet.
	IsTooShortToBeA8021QVirtualLanEthernetPacket,
	
	/// Occurs during Ethernet packet processing of an IEEE 802.1Q (Virtual LAN) packet.
	CouldNotParse8011QVirtualLanTag
{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'a EthernetAddresses,

		/// Tag Control Information (TCI).
		tag_control_information: TagControlInformation,
	},
	
	/// Occurs during Ethernet packet processing of an IEEE 802.1Q (Virtual LAN) packet.
	NoConfigurationFor8011QVirtualLan
{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'a EthernetAddresses,

		/// Tag Control Information (TCI).
		tag_control_information: TagControlInformation,
	},
	
	/// Occurs during Ethernet packet processing of an IEEE 802.1Q (Virtual LAN) packet.
	DropEligibleFor8011QVirtualLan
{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'a EthernetAddresses,

		/// Tag Control Information (TCI).
		tag_control_information: TagControlInformation,
	},
	
	/// Occurs during Ethernet packet processing of an IEEE 802.1Q (Virtual LAN) packet.
	DropThisClassOfServiceFor8011QVirtualLan
{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'a EthernetAddresses,

		/// Tag Control Information (TCI).
		tag_control_information: TagControlInformation,
	},
	
	/// Occurs during Ethernet packet processing of an IEEE 802.1ad 'QinQ' (Virtual LAN) packet.
	IsTooShortToBeAQinQVirtualLanEthernetPacket,
	
	/// Occurs during Ethernet packet processing of an IEEE 802.1ad 'QinQ' (Virtual LAN) packet.
	CouldNotParseOuterVirtualLanTag
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'a EthernetAddresses,

		/// Outer Tag Control Information (TCI).
		outer_tag_control_information: TagControlInformation,

		/// Inner Tag Control Information (TCI).
		inner_tag_control_information: TagControlInformation,
	},
	
	/// Occurs during Ethernet packet processing of an IEEE 802.1ad 'QinQ' (Virtual LAN) packet.
	CouldNotParseInnerVirtualLanTag
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'a EthernetAddresses,

		/// Outer Tag Control Information (TCI).
		outer_tag_control_information: TagControlInformation,

		/// Inner Tag Control Information (TCI).
		inner_tag_control_information: TagControlInformation,
	},
	
	/// Occurs during Ethernet packet processing of an IEEE 802.1ad 'QinQ' (Virtual LAN) packet.
	NoConfigurationForQinQVirtualLan
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'a EthernetAddresses,

		/// Outer Tag Control Information (TCI).
		outer_tag_control_information: TagControlInformation,

		/// Inner Tag Control Information (TCI).
		inner_tag_control_information: TagControlInformation,
	},
	
	/// Occurs during Ethernet packet processing of an IEEE 802.1ad 'QinQ' (Virtual LAN) packet.
	DropEligibleForOuterVirtualLan
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'a EthernetAddresses,

		/// Outer Tag Control Information (TCI).
		outer_tag_control_information: TagControlInformation,

		/// Inner Tag Control Information (TCI).
		inner_tag_control_information: TagControlInformation,
	},
	
	/// Occurs during Ethernet packet processing of an IEEE 802.1ad 'QinQ' (Virtual LAN) packet.
	DropEligibleForInnerVirtualLan
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'a EthernetAddresses,

		/// Outer Tag Control Information (TCI).
		outer_tag_control_information: TagControlInformation,

		/// Inner Tag Control Information (TCI).
		inner_tag_control_information: TagControlInformation,
	},
	
	/// Occurs during Ethernet packet processing of an IEEE 802.1ad 'QinQ' (Virtual LAN) packet.
	DropThisClassOfServiceForOuterVirtualLan
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'a EthernetAddresses,

		/// Outer Tag Control Information (TCI).
		outer_tag_control_information: TagControlInformation,

		/// Inner Tag Control Information (TCI).
		inner_tag_control_information: TagControlInformation,
	},
	
	/// Occurs during Ethernet packet processing of an IEEE 802.1ad 'QinQ' (Virtual LAN) packet.
	DropThisClassOfServiceForInnerVirtualLan
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'a EthernetAddresses,

		/// Outer Tag Control Information (TCI).
		outer_tag_control_information: TagControlInformation,

		/// Inner Tag Control Information (TCI).
		inner_tag_control_information: TagControlInformation,
	},
	
	/// Occurs during Address Reolution Protocol (ARP) packet processing.
	AddressResolutionProtocolPacketIsTooShort
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'a EthernetAddresses,
	},
	
	/// Occurs during Address Reolution Protocol (ARP) packet processing.
	AddressResolutionProtocolNotSupportedForAnythingOtherThanInternetProtocolVersion4
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'a EthernetAddresses,
	},
	
	/// Occurs during Address Reolution Protocol (ARP) packet processing.
	AddressResolutionProtocolDestinationEthernetAddressIsMulticast
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'a EthernetAddresses,

		/// Address Reolution Protocol (ARP) packet header.
		header: &'a AddressResolutionProtocolPacketHeader,
	},
	
	/// Occurs during Address Reolution Protocol (ARP) packet processing.
	AddressResolutionProtocolOperationIsUnsupported
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'a EthernetAddresses,

		/// Address Reolution Protocol (ARP) packet header.
		header: &'a AddressResolutionProtocolPacketHeader,
	},
	
	/// Occurs during Address Reolution Protocol (ARP) packet processing.
	AddressResolutionProtocolRequestIsMulticast
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'a EthernetAddresses,

		/// Address Reolution Protocol (ARP) packet header.
		header: &'a AddressResolutionProtocolPacketHeader,
	},
	
	/// Occurs during Address Reolution Protocol (ARP) packet processing.
	///
	/// This is a violation of RFC 5227; it is only checked for if so configured.
	AddressResolutionProtocolRequestTargetHardwareAddressIsZero
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'a EthernetAddresses,

		/// Address Reolution Protocol (ARP) packet header.
		header: &'a AddressResolutionProtocolPacketHeader,
	},
	
	/// Occurs during Address Reolution Protocol (ARP) packet processing.
	AddressResolutionProtocolHardwareAndPacketSourceEthernetAddressMismatch
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'a EthernetAddresses,

		/// Address Reolution Protocol (ARP) packet header.
		header: &'a AddressResolutionProtocolPacketHeader,
	},
	
	/// Occurs during Address Reolution Protocol (ARP) packet processing.
	AddressResolutionProtocolHardwareAndPacketDestinationEthernetAddressMismatch
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'a EthernetAddresses,

		/// Address Reolution Protocol (ARP) packet header.
		header: &'a AddressResolutionProtocolPacketHeader,
	},
	
	/// Occurs during Address Reolution Protocol (ARP) packet processing.
	AddressResolutionProtocolProbeIsNotForUs
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'a EthernetAddresses,

		/// Address Reolution Protocol (ARP) packet header.
		header: &'a AddressResolutionProtocolPacketHeader,
	},
	
	/// Occurs during Address Reolution Protocol (ARP) packet processing.
	AddressResolutionProtocolRequestIsNotAProbeAndIsNotBroadcast
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'a EthernetAddresses,

		/// Address Reolution Protocol (ARP) packet header.
		header: &'a AddressResolutionProtocolPacketHeader,
	},
	
	/// Occurs during Address Reolution Protocol (ARP) packet processing.
	AddressResolutionProtocolRequestIsNotAProbeAndSenderProtocolAddressIsNotUnicast
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'a EthernetAddresses,

		/// Address Reolution Protocol (ARP) packet header.
		header: &'a AddressResolutionProtocolPacketHeader,
	},
	
	/// Occurs during Address Reolution Protocol (ARP) packet processing.
	AddressResolutionProtocolGratuitousReplyIsNotValidUnicast
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'a EthernetAddresses,

		/// Address Reolution Protocol (ARP) packet header.
		header: &'a AddressResolutionProtocolPacketHeader,
	},
	
	/// Occurs during Address Reolution Protocol (ARP) packet processing.
	AddressResolutionProtocolReplyTargetHardwareAddressIsNotValidUnicast
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'a EthernetAddresses,

		/// Address Reolution Protocol (ARP) packet header.
		header: &'a AddressResolutionProtocolPacketHeader,
	},
	
	/// Occurs during Address Reolution Protocol (ARP) packet processing.
	AddressResolutionProtocolReplySourceAndTargetProtocolAddressesAreTheSame
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'a EthernetAddresses,

		/// Address Reolution Protocol (ARP) packet header.
		header: &'a AddressResolutionProtocolPacketHeader,
	},
	
	/// Occurs during Address Reolution Protocol (ARP) packet processing.
	AddressResolutionProtocolReplySenderProtocolAddressIsNotValidUnicast
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'a EthernetAddresses,

		/// Address Reolution Protocol (ARP) packet header.
		header: &'a AddressResolutionProtocolPacketHeader,
	},
	
	/// Occurs during Address Reolution Protocol (ARP) packet processing.
	AddressResolutionProtocolReplyTargetProtocolAddressIsNotValidUnicast
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'a EthernetAddresses,

		/// Address Reolution Protocol (ARP) packet header.
		header: &'a AddressResolutionProtocolPacketHeader,
	},
	
	/// Occurs during Internet Protocol (IP) version 4 packet processing.
	InternetProtocolVersion4PacketIsTooShort
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'a EthernetAddresses,
	},
	
	/// Occurs during Internet Protocol (IP) version 4 packet processing.
	InternetProtocolVersion4HeaderIsNot4
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'a EthernetAddresses,

		/// Internet Protocol (IP) version 4 packet header.
		header: &'a InternetProtocolVersion4PacketHeader,
	},
	
	/// Occurs during Internet Protocol (IP) version 4 packet processing.
	InternetProtocolVersion4TotalLengthInvalid
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'a EthernetAddresses,

		/// Internet Protocol (IP) version 4 packet header.
		header: &'a InternetProtocolVersion4PacketHeader,
	},
	
	/// Occurs during Internet Protocol (IP) version 4 packet processing.
	///
	/// Affected by configuration.
	InternetProtocolVersion4InvalidFragmentationFlagsOrIdentification
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'a EthernetAddresses,

		/// Internet Protocol (IP) version 4 packet header.
		header: &'a InternetProtocolVersion4PacketHeader,
	},
	
	/// Occurs during Internet Protocol (IP) version 4 packet processing.
	InternetProtocolVersion4TotalLengthLessThanHeader
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'a EthernetAddresses,

		/// Internet Protocol (IP) version 4 packet header.
		header: &'a InternetProtocolVersion4PacketHeader,
	},
	
	/// Occurs during Internet Protocol (IP) version 4 packet processing.
	///
	/// Only if configured.
	InternetProtocolVersion4HasOptions
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'a EthernetAddresses,

		/// Internet Protocol (IP) version 4 packet header.
		header: &'a InternetProtocolVersion4PacketHeader,
	},
	
	/// Occurs during Internet Protocol (IP) version 4 packet processing.
	InternetProtocolVersion4OptionLacksKind
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'a EthernetAddresses,

		/// Internet Protocol (IP) version 4 packet header.
		header: &'a InternetProtocolVersion4PacketHeader,
	},
	
	/// Occurs during Internet Protocol (IP) version 4 packet processing.
	InternetProtocolVersion4OptionLengthTooShort
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'a EthernetAddresses,

		/// Internet Protocol (IP) version 4 packet header.
		header: &'a InternetProtocolVersion4PacketHeader,
	},
	
	/// Occurs during Internet Protocol (IP) version 4 packet processing.
	InternetProtocolVersion4OptionLengthTooLong
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'a EthernetAddresses,

		/// Internet Protocol (IP) version 4 packet header.
		header: &'a InternetProtocolVersion4PacketHeader,
	},
	
	/// Occurs during Internet Protocol (IP) version 4 packet processing.
	InternetProtocolVersion4MulticastAddressDenied
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'a EthernetAddresses,

		/// Internet Protocol (IP) version 4 packet header.
		header: &'a InternetProtocolVersion4PacketHeader,
	},
	
	/// Occurs during Internet Protocol (IP) version 4 packet processing.
	InternetProtocolVersion4MulticastAddressWrong
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'a EthernetAddresses,

		/// Internet Protocol (IP) version 4 packet header.
		header: &'a InternetProtocolVersion4PacketHeader,
	},
	
	/// Occurs during Internet Protocol (IP) version 6 packet processing.
	InternetProtocolVersion6PacketIsTooShort
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'a EthernetAddresses,
	},
	
	/// Occurs during Internet Protocol (IP) version 6 packet processing.
	InternetProtocolVersion6HeaderIsNot6
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'a EthernetAddresses,

		/// Internet Protocol (IP) version 6 packet header.
		header: &'a InternetProtocolVersion6PacketHeader,
	},
	
	/// Occurs during Internet Protocol (IP) version 6 packet processing.
	InternetProtocolVersion6MulticastAddressDenied
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'a EthernetAddresses,

		/// Internet Protocol (IP) version 6 packet header.
		header: &'a InternetProtocolVersion6PacketHeader,
	},
	
	/// Occurs during Internet Protocol (IP) version 6 packet processing.
	InternetProtocolVersion6MulticastAddressWrong
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'a EthernetAddresses,

		/// Internet Protocol (IP) version 6 packet header.
		header: &'a InternetProtocolVersion6PacketHeader,
	},
}
