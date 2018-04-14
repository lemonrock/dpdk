// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use super::*;
use super::super::super::ethernetPorts::BalanceBondingModeTransmitPolicy;
use super::super::super::ethernetPorts::BondingSlave;
use super::super::super::ethernetPorts::MediaAccessControlAddress;
use super::super::super::ethernetPorts::UsefulBondingMode;
use super::super::super::logicalCores::NumaSocketId;


include!("AfPacketNetVirtualDevice.rs");
include!("BondingNetVirtualDevice.rs");
include!("NetVirtualDevice.rs");
include!("NetVirtualDeviceDriverName.rs");
include!("NetVirtualDeviceName.rs");
include!("PacketCaptureFileOrInterfacePrefix.rs");
include!("PacketCaptureFileOrInterface.rs");
include!("PacketCaptureNetVirtualDevice.rs");
include!("VirtIoNetVirtualDevice.rs");
include!("VirtualHostNetVirtualDevice.rs");
include!("XenNetVirtualDevice.rs");
