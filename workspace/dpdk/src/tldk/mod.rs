// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use self::devices::*;
use super::*;
use super::domain::internet_protocol::InternetProtocolVersion4OrVersion6OrBoth;
use super::domain::internet_protocol::*::*;
use super::domain::layer4::Layer4Port;
use super::domain::layer4::Layer4Protocol;
use super::ethernetPorts::DeviceReceiveOffloadCapabilities;
use super::ethernetPorts::DeviceTransmitOffloadCapabilities;
use super::logicalCores::AnyNumaSocketId;
use super::logicalCores::NumaSocketId;


/// Devices.
pub mod devices;

// /// Streams.
// pub mod streams;


include!("AddressLookUpForSendCallback.rs");
include!("AddressWithListOfOpenLayer4Ports.rs");
include!("Context.rs");
include!("DeviceConfiguration.rs");
include!("TldkBlockedPortsList.rs");
include!("TcpContext.rs");
include!("UdpContext.rs");
