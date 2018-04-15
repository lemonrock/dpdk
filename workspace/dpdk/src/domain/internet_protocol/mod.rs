// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use super::*;
use super::internet_protocol::*;
use super::layer4::Layer4Protocol;
use self::mask_bits::*;


/// Masks.
pub mod mask_bits;


include!("DifferentiatedServiceCodePoint.rs");
include!("ExplicitCongestionNotification.rs");
include!("InternetProtocolNetworkAddress.rs");
include!("InternetProtocolVersion.rs");
include!("InternetProtocolVersion4HostAddress.rs");
include!("InternetProtocolVersion4NetworkAddress.rs");
include!("InternetProtocolVersion6HostAddress.rs");
include!("InternetProtocolVersion6NetworkAddress.rs");
include!("InternetProtocolVersion6MulticastAddressLifetime.rs");
include!("InternetProtocolVersion6MulticastAddressScope.rs");
include!("InternetProtocolNetworkAddress.rs");
include!("IpV4AndOrIpV6.rs");
include!("TrafficClass.rs");
