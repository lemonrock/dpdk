// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use super::*;
use super::ipHostAddresses::InternetProcolVersion4HostAddress;
use super::ipHostAddresses::InternetProcolVersion6HostAddress;
use super::layer4::Layer4Protocol;


include!("DifferentiatedServiceCodePoint.rs");
include!("ExplicitCongestionNotification.rs");
include!("InternetProtocolVersion.rs");
include!("IpV4AndOrIpV6.rs");
include!("TrafficClass.rs");
