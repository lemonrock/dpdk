// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use ::rust_extra::u31;
use ::rust_extra::u4;
use ::rust_extra::u5;
use ::std::collections::HashMap;
use ::std::collections::HashSet;
use ::std::path::Path;
use ::devices::DeviceDriverName;
use ::devices::NetworkInterfaceName;
use ::devices::virtualDevices::VirtualDevice;
use ::devices::virtualDevices::VirtualDeviceName;
use ::ethernetPorts::BalanceBondingModeTransmitPolicy;
use ::ethernetPorts::BondingSlave;
use ::ethernetPorts::MediaAccessControlAddress;
use ::ethernetPorts::UsefulBondingMode;
use ::logicalCores::NumaSocketId;


include!("AfPacketNetVirtualDevice.rs");
include!("BondingNetVirtualDevice.rs");
include!("NetVirtualDevice.rs");
include!("NetVirtualDeviceDriverName.rs");
include!("NetVirtualDeviceName.rs");
include!("NullNetVirtualDevice.rs");
include!("PacketCaptureFileOrInterfacePrefix.rs");
include!("PacketCaptureFileOrInterface.rs");
include!("PacketCaptureNetVirtualDevice.rs");
include!("RingNodeAction.rs");
include!("RingNetVirtualDevice.rs");
include!("VirtIoForContainersNetVirtualDevice.rs");
include!("VirtualHostNetVirtualDevice.rs");
include!("XenNetVirtualDevice.rs");
