// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Reason for finishing with a packet earlier than might be expected.
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum PacketProcessingDropReason
{
	// Re-use the packet in reply (temporary market reason until logic has been implemented).
	ReuseInReply,
	
	IsTooShortToBeAnEthernetPacket,
	
	/// Currently recognised and supported EtherTypes are Internet Protocol (IP) Version 4, Internet Protocol (IP) Version 6, Address Resolution Protocol, Virtual LAN tagging (801.1q) and QinQ Virtual LAN tagging (802.1ad).
	/// Ether frame lengths are entirely unsupported.
	UnsupportedEtherType,
	
	SourceEthernetAddressIsNotValidUnicast,
	SourceEthernetAddressIsOurUnicastEthernetAddress,
	DeniedSourceEthernetAddress,
	DestinationEthernetAddressIsZero,
	/// This can occur if a link has multiple ethernet addresses or is listening promiscuously.
	DestinationEthernetAddressIsNotOneOfOurs,
	
	IsTooShortToBeA8021QVirtualLanEthernetPacket,
	CouldNotParse8011QVirtualLanTag,
	DropEligibleFor8011QVirtualLan,
	NoConfigurationFor8011QVirtualLan,
	DropThisClassOfServiceFor8011QVirtualLan,
	
	IsTooShortToBeAQinQVirtualLanEthernetPacket,
	CouldNotParseOuterVirtualLanTag,
	DropEligibleForOuterVirtualLan,
	CouldNotParseInnerVirtualLanTag,
	DropEligibleForInnerVirtualLan,
	NoConfigurationForQinQVirtualLan,
	DropThisClassOfServiceForQinQVirtualLan,

	AddressResolutionProtocolPacketIsTooShort,
	AddressResolutionProtocolNotSupportedForAnythingOtherThanInternetProtocolVersion4,
	AddressResolutionProtocolDestinationEthernetAddressIsMulticast,
	AddressResolutionProtocolOperationIsUnsupported,
	AddressResolutionProtocolRequestIsMulticast,
	/// This is a violation of RFC 5227.
	AddressResolutionProtocolRequestTargetHardwareAddressIsZero,
	AddressResolutionProtocolHardwareAndPacketSourceEthernetAddressMismatch,
	AddressResolutionProtocolHardwareAndPacketDestinationEthernetAddressMismatch,
	AddressResolutionProtocolProbeIsNotForUs,
	AddressResolutionProtocolRequestIsNotAProbeAndIsNotBroadcast,
	AddressResolutionProtocolRequestIsNotAProbeAndSenderProtocolAddressIsNotUnicast,
	AddressResolutionProtocolGratuitousReplyIsNotValidUnicast,
	AddressResolutionProtocolReplyTargetHardwareAddressIsNotValidUnicast,
	AddressResolutionProtocolReplySourceAndTargetProtocolAddressesAreTheSame,
	AddressResolutionProtocolReplySenderProtocolAddressIsNotValidUnicast,
	AddressResolutionProtocolReplyTargetProtocolAddressIsNotValidUnicast,
	
	InternetProtocolVersion4PacketIsTooShort,
	InternetProtocolVersion4HeaderIsNot4,
	InternetProtocolVersion4TotalLengthInvalid,
	/// Affected by configuration.
	InternetProtocolVersion4InvalidFragmentationFlagsOrIdentification,
	InternetProtocolVersion4TotalLengthLessThanHeader,
	/// Only if configured.
	InternetProtocolVersion4HasOptions,
	InternetProtocolVersion4OptionLacksKind,
	InternetProtocolVersion4OptionLengthTooShort,
	InternetProtocolVersion4OptionLengthTooLong,
	InternetProtocolVersion4MulticastAddressDenied,
	InternetProtocolVersion4MulticastAddressWrong,
	
	InternetProtocolVersion6PacketIsTooShort,
	InternetProtocolVersion6HeaderIsNot6,
	InternetProtocolVersion6MulticastAddressDenied,
	InternetProtocolVersion6MulticastAddressWrong,
}
