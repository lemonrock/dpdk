// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use ::dpdk_sys::ETHER_TYPE_VLAN;
use ::dpdk_sys::ETHER_TYPE_QINQ;
use ::domain::ethernet::MinimumSizeOfEthernetPacketSizeAssumingArp;
use ::domain::ethernet::SizeOfEthernetHeader;
use ::domain::ethernet::SizeOfEthernetHeaderLessEtherType;
use ::domain::ethernet::SizeOfEtherType;
use ::domain::ip::DifferentiatedServiceCodePoint;
use ::pointer::*;
use ::rust_extra::unlikely;
use ::serde::Deserialize;
use ::serde::Serialize;
use ::serde::de;
use ::serde::de::Deserializer;
use ::serde::de::Visitor;
use ::serde::ser::Serializer;
use ::std::cmp::Ord;
use ::std::cmp::Ordering;
use ::std::cmp::PartialOrd;
use ::std::fmt;
use ::std::fmt::Formatter;


include!("ClassOfService.rs");
include!("MinimumSizeOfQinQPacketAssumingArp.rs");
include!("MinimumSizeOfVlanPacketAssumingArp.rs");
include!("OffsetToJustAfterVlanHeaderEtherTypeIfVlanQinQ.rs");
include!("OffsetToVlanHeaderIfVlanQinQ.rs");
include!("OffsetToVlanQinQTciValue.rs");
include!("SizeOfQinQHeader.rs");
include!("SizeOfTagControlInformation.rs");
include!("SizeOfVlanHeader.rs");
include!("VirtualLanId.rs");
include!("VirtualLanKey.rs");
include!("VirtualLanTagging.rs");
include!("VirtualLanTrafficClassIndicator.rs");
include!("VirtualLanValue.rs");
