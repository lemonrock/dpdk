// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use super::*;
use super::ethernet::MinimumSizeOfEthernetPacketSizeAssumingArp;
use super::ethernet::SizeOfEthernetHeader;
use super::ethernet::SizeOfEthernetHeaderLessEtherType;
use super::ethernet::SizeOfEtherType;
use super::internet_protocol::DifferentiatedServiceCodePoint;


include!("ClassOfService.rs");
include!("MinimumSizeOfQinQPacketAssumingArp.rs");
include!("MinimumSizeOfVlanPacketAssumingArp.rs");
include!("NoVirtualLanKey.rs");
include!("OffsetToJustAfterVlanHeaderEtherTypeIfVlanQinQ.rs");
include!("OffsetToVlanHeaderIfVlanQinQ.rs");
include!("OffsetToVlanQinQTciValue.rs");
include!("SizeOfQinQHeader.rs");
include!("SizeOfTagControlInformation.rs");
include!("SizeOfVlanHeader.rs");
include!("VirtualLanIdentifier.rs");
include!("VirtualLanKey.rs");
include!("VirtualLanTagging.rs");
include!("VirtualLanTrafficClassIndicator.rs");
include!("VirtualLanValue.rs");
