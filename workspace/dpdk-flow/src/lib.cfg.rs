// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern crate arrayvec;
extern crate dpdk_sys;
extern crate network_address_resolution_protocol;
extern crate network_endian;
extern crate network_ethernet;
extern crate network_internet_protocol;
extern crate serde;
#[macro_use] extern crate serde_derive;


use ::arrayvec::ArrayVec;
use ::dpdk_sys::*;
use ::network_address_resolution_protocol::*;
use ::network_endian::*;
use ::network_ethernet::*;
use ::network_ethernet::virtual_lans::*;
#[allow(unused_imports)] use ::network_internet_protocol::InternetProtocolHostAddress;
use ::network_internet_protocol::version_4::*;
use ::network_internet_protocol::version_6::*;
use ::serde::Deserialize;
use ::serde::Deserializer;
use ::serde::de::Error as DeserializerError;
use ::serde::de::MapAccess;
use ::serde::de::SeqAccess;
use ::serde::de::Visitor;
use ::std::fmt;
use ::std::mem::transmute;
use ::std::ptr::null_mut;


include!("custom_deserialize.rs");


include!("PatternItem.rs");
include!("AddressResolutionProtocolForInternetProtocolVersion4OverEthernetMask.rs");
include!("AddressResolutionProtocolForInternetProtocolVersion4OverEthernetSpecification.rs");
include!("EthernetAddress.rs");
include!("EthernetAddressesMask.rs");
include!("EthernetHeaderMask.rs");
include!("EthernetHeaderSpecification.rs");
include!("MediaAccessControlAddressMask.rs");
