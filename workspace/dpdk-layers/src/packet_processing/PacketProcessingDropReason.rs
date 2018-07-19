// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// The source and destination ethernet addresses (MACs) of a packet.
///
/// Depending on the PacketProcessingDropReason, these may be invalid, inappropriate, not for our interface, etc.
#[repr(C, packed)]
#[derive(Debug)]
pub struct EthernetAddresses<'a>
{
	/// Source ethernet address.
	pub source: &'a MediaAccessControlAddress,
	
	/// Destination ethernet address.
	pub destination: &'a MediaAccessControlAddress,
}



/// Reason for finishing with a packet earlier than might be expected.
#[allow(missing_docs)]
#[derive(Debug)]
pub enum PacketProcessingDropReason<'a>
{
	// Re-use the packet in reply (temporary market reason until logic has been implemented).
	ReuseInReply,
	
	/// Occurs during EthernetPacket processing.
	IsTooShortToBeAnEthernetPacket,
	
	/// Occurs during EthernetPacket processing.
	SourceEthernetAddressIsNotValidUnicast { ethernet_addresses: &'a EthernetAddresses<'a> },
	
	/// Occurs during EthernetPacket processing.
	SourceEthernetAddressIsOurUnicastEthernetAddress { ethernet_addresses: &'a EthernetAddresses<'a> },
	
	/// Occurs during EthernetPacket processing.
	DeniedSourceEthernetAddress { ethernet_addresses: &'a EthernetAddresses<'a> },
	
	/// Occurs during EthernetPacket processing.
	DestinationEthernetAddressIsZero { ethernet_addresses: &'a EthernetAddresses<'a> },
	
	/// Occurs during EthernetPacket processing.
	///
	/// This can occur if a link has multiple ethernet addresses or is listening promiscuously.
	DestinationEthernetAddressIsNotOneOfOurs { ethernet_addresses: &'a EthernetAddresses<'a> },
	
	/// Occurs during EthernetPacket processing.
	///
	/// Currently recognised and supported EtherTypes are Internet Protocol (IP) Version 4, Internet Protocol (IP) Version 6, Address Resolution Protocol, Virtual LAN tagging (801.1q) and QinQ Virtual LAN tagging (802.1ad).
	/// Ether frame sizes are entirely unsupported.
	UnsupportedEtherType
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'a EthernetAddresses<'a>,
		
		/// EtherType or LegacyEthernetFrameSize.
		unsuspported_ether_type_or_legacy_ethernet_frame_size: EtherTypeOrLegacyEthernetFrameSize,
	},
	
	/// Occurs during EthernetPacket processing.
	IsTooShortToBeA8021QVirtualLanEthernetPacket,
	CouldNotParse8011QVirtualLanTag(VirtualLanPacketTagControlInformation),
	NoConfigurationFor8011QVirtualLan(VirtualLanPacketTagControlInformation),
	DropEligibleFor8011QVirtualLan(VirtualLanPacketTagControlInformation),
	DropThisClassOfServiceFor8011QVirtualLan(VirtualLanPacketTagControlInformation),
	
	IsTooShortToBeAQinQVirtualLanEthernetPacket,
	CouldNotParseOuterVirtualLanTag(VirtualLanPacketTagControlInformation),
	CouldNotParseInnerVirtualLanTag(VirtualLanPacketTagControlInformation, VirtualLanPacketTagControlInformation),
	NoConfigurationForQinQVirtualLan(VirtualLanPacketTagControlInformation, VirtualLanPacketTagControlInformation),
	DropEligibleForOuterVirtualLan(VirtualLanPacketTagControlInformation),
	DropEligibleForInnerVirtualLan(VirtualLanPacketTagControlInformation, VirtualLanPacketTagControlInformation),
	DropThisClassOfServiceForOuterVirtualLan(VirtualLanPacketTagControlInformation),
	DropThisClassOfServiceForInnerVirtualLan(VirtualLanPacketTagControlInformation, VirtualLanPacketTagControlInformation),

	AddressResolutionProtocolPacketIsTooShort { ethernet_addresses: &'a EthernetAddresses<'a> },
	AddressResolutionProtocolNotSupportedForAnythingOtherThanInternetProtocolVersion4 { ethernet_addresses: &'a EthernetAddresses<'a> },
	AddressResolutionProtocolDestinationEthernetAddressIsMulticast { ethernet_addresses: &'a EthernetAddresses<'a>, header: &'a AddressResolutionProtocolPacketHeader },
	AddressResolutionProtocolOperationIsUnsupported { ethernet_addresses: &'a EthernetAddresses<'a>, header: &'a AddressResolutionProtocolPacketHeader },
	AddressResolutionProtocolRequestIsMulticast { ethernet_addresses: &'a EthernetAddresses<'a>, header: &'a AddressResolutionProtocolPacketHeader },
	/// This is a violation of RFC 5227.
	AddressResolutionProtocolRequestTargetHardwareAddressIsZero { ethernet_addresses: &'a EthernetAddresses<'a>, header: &'a AddressResolutionProtocolPacketHeader },
	AddressResolutionProtocolHardwareAndPacketSourceEthernetAddressMismatch { ethernet_addresses: &'a EthernetAddresses<'a>, header: &'a AddressResolutionProtocolPacketHeader },
	AddressResolutionProtocolHardwareAndPacketDestinationEthernetAddressMismatch { ethernet_addresses: &'a EthernetAddresses<'a>, header: &'a AddressResolutionProtocolPacketHeader },
	AddressResolutionProtocolProbeIsNotForUs { ethernet_addresses: &'a EthernetAddresses<'a>, header: &'a AddressResolutionProtocolPacketHeader },
	AddressResolutionProtocolRequestIsNotAProbeAndIsNotBroadcast { ethernet_addresses: &'a EthernetAddresses<'a>, header: &'a AddressResolutionProtocolPacketHeader },
	AddressResolutionProtocolRequestIsNotAProbeAndSenderProtocolAddressIsNotUnicast { ethernet_addresses: &'a EthernetAddresses<'a>, header: &'a AddressResolutionProtocolPacketHeader },
	AddressResolutionProtocolGratuitousReplyIsNotValidUnicast { ethernet_addresses: &'a EthernetAddresses<'a>, header: &'a AddressResolutionProtocolPacketHeader },
	AddressResolutionProtocolReplyTargetHardwareAddressIsNotValidUnicast { ethernet_addresses: &'a EthernetAddresses<'a>, header: &'a AddressResolutionProtocolPacketHeader },
	AddressResolutionProtocolReplySourceAndTargetProtocolAddressesAreTheSame { ethernet_addresses: &'a EthernetAddresses<'a>, header: &'a AddressResolutionProtocolPacketHeader },
	AddressResolutionProtocolReplySenderProtocolAddressIsNotValidUnicast { ethernet_addresses: &'a EthernetAddresses<'a>, header: &'a AddressResolutionProtocolPacketHeader },
	AddressResolutionProtocolReplyTargetProtocolAddressIsNotValidUnicast { ethernet_addresses: &'a EthernetAddresses<'a>, header: &'a AddressResolutionProtocolPacketHeader },
	
	InternetProtocolVersion4PacketIsTooShort { ethernet_addresses: &'a EthernetAddresses<'a> },
	InternetProtocolVersion4HeaderIsNot4 { ethernet_addresses: &'a EthernetAddresses<'a>, header: &'a InternetProtocolVersion4PacketHeader },
	InternetProtocolVersion4TotalLengthInvalid { ethernet_addresses: &'a EthernetAddresses<'a>, header: &'a InternetProtocolVersion4PacketHeader },
	/// Affected by configuration.
	InternetProtocolVersion4InvalidFragmentationFlagsOrIdentification { ethernet_addresses: &'a EthernetAddresses<'a>, header: &'a InternetProtocolVersion4PacketHeader },
	InternetProtocolVersion4TotalLengthLessThanHeader { ethernet_addresses: &'a EthernetAddresses<'a>, header: &'a InternetProtocolVersion4PacketHeader },
	/// Only if configured.
	InternetProtocolVersion4HasOptions { ethernet_addresses: &'a EthernetAddresses<'a>, header: &'a InternetProtocolVersion4PacketHeader },
	InternetProtocolVersion4OptionLacksKind { ethernet_addresses: &'a EthernetAddresses<'a>, header: &'a InternetProtocolVersion4PacketHeader },
	InternetProtocolVersion4OptionLengthTooShort { ethernet_addresses: &'a EthernetAddresses<'a>, header: &'a InternetProtocolVersion4PacketHeader },
	InternetProtocolVersion4OptionLengthTooLong { ethernet_addresses: &'a EthernetAddresses<'a>, header: &'a InternetProtocolVersion4PacketHeader },
	InternetProtocolVersion4MulticastAddressDenied { ethernet_addresses: &'a EthernetAddresses<'a>, header: &'a InternetProtocolVersion4PacketHeader },
	InternetProtocolVersion4MulticastAddressWrong { ethernet_addresses: &'a EthernetAddresses<'a>, header: &'a InternetProtocolVersion4PacketHeader },
	
	InternetProtocolVersion6PacketIsTooShort { ethernet_addresses: &'a EthernetAddresses<'a> },
	InternetProtocolVersion6HeaderIsNot6 { ethernet_addresses: &'a EthernetAddresses<'a>, header: &'a InternetProtocolVersion6PacketHeader },
	InternetProtocolVersion6MulticastAddressDenied { ethernet_addresses: &'a EthernetAddresses<'a>, header: &'a InternetProtocolVersion6PacketHeader },
	InternetProtocolVersion6MulticastAddressWrong { ethernet_addresses: &'a EthernetAddresses<'a>, header: &'a InternetProtocolVersion6PacketHeader },
}
