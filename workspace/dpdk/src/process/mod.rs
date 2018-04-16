// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use super::*;
use super::devices::virtual_devices::*;
use super::devices::virtual_devices::cryptoVirtualDevices::*;
use super::devices::virtual_devices::net_virtual_devices::*;
use super::logicalCores::MaximumNumaSockets;
use super::logicalCores::discovery::NumaSockets;
use super::memory::*;
use super::bus::pci::DpdkPciDeviceAddress;


include!("DpdkProcess.rs");
include!("DpdkRteInitData.rs");
include!("MemoryChannels.rs");
include!("MemoryLimits.rs");
include!("MemoryRanks.rs");
include!("PciDeviceListColour.rs");
include!("ProcessType.rs");
include!("VirtualFunctionIoInterruptMode.rs");
