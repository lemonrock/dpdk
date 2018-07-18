// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use super::*;


/// Longest prefix matching, typically used to turn a destination internet protocol (IP) address into an index in a routing table.
pub mod longest_prefix_matching;

/// Masks.
pub mod mask_bits;

/// Packet reassembly from fragments.
pub mod packet_reassembly;


// TODO: suspended as requires changes in upstream crate to make this logic work.
// /// Combined routing and longest prefix matching.
// pub mod routing;


include!("DifferentiatedServiceCodePoint.rs");
include!("ExplicitCongestionNotification.rs");
include!("ExtensionHeaderType.rs");
include!("ExtensionHeaderTypeOrLayer4ProtocolNumber.rs");
include!("InternetHeaderLength.rs");
include!("InternetProtocolHostAddress.rs");
include!("InternetProtocolNetworkAddress.rs");
include!("InternetProtocolVersion4HostAddress.rs");
include!("InternetProtocolVersion4NetworkAddress.rs");
include!("InternetProtocolVersion4OrVersion6OrBoth.rs");
include!("InternetProtocolVersion4Packet.rs");
include!("InternetProtocolVersion4PacketHeader.rs");
include!("InternetProtocolVersion6HostAddress.rs");
include!("InternetProtocolVersion6MulticastAddressLifetime.rs");
include!("InternetProtocolVersion6MulticastAddressScope.rs");
include!("InternetProtocolVersion6NetworkAddress.rs");
include!("InternetProtocolVersion6Packet.rs");
include!("InternetProtocolVersion6PacketHeader.rs");
include!("Layer4ProtocolNumber.rs");
include!("TrafficClass.rs");
