// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[cfg(any(target_os = "android", target_os = "linux"))] use ::dpdk_ioctl::android_linux::rawPciBusAddressForNetworkInterfaceIndex;
use ::libc::IF_NAMESIZE;
use ::libc::if_nametoindex;
use ::std::ffi::CString;
use ::std::fmt::Debug;
use ::std::hash::Hash;
use ::pci::DeviceAddress;
use ::pci::DeviceAddressStringParseError;


pub mod physicalDevices;
pub mod virtualDevices;


include!("DeviceDriverName.rs");
include!("DeviceName.rs");
include!("NetworkInterfaceName.rs");
