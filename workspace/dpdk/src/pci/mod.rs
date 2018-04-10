// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use ::const_cstr_fork::ConstCStr;
#[cfg(unix)] use ::std::os::unix::ffi::OsStrExt;
use ::dpdk_sys::rte_eth_dev;
use ::dpdk_sys::rte_pci_addr;
use ::dpdk_sys::rte_pci_device;
use ::dpdk_sys::rte_pci_driver;
use ::dpdk_sys::rte_pci_id;
use ::dpdk_sys::rte_pci_ioport;
use ::dpdk_sys::rust_RTE_DEV_TO_PCI;
use ::dpdk_unix::android_linux::pci::PciBusInformation;
use ::libc::c_void;
use ::libc::FILE;
use ::libc::off_t;
use ::libc::PATH_MAX;
use ::libc::strnlen;
use ::libc_extra::stderr;
use ::rust_extra::likely;
use ::rust_extra::u4;
use ::rust_extra::unlikely;
use ::std::ffi::CStr;
use ::std::ffi::CString;
use ::std::ffi::OsStr;
use ::std::mem::forget;
use ::std::mem::uninitialized;
use ::std::num::ParseIntError;
use ::std::path::PathBuf;
use ::std::slice::from_raw_parts;
use ::ethernetPorts::EthernetPort;
use ::devices::DeviceName;
use ::logicalCores::NumaSocketId;



#[cfg(any(target_os = "android", target_os = "linux"))] pub mod android_linux;


include!("Device.rs");
include!("DeviceAddress.rs");
include!("DeviceAddressStringParseError.rs");
include!("DeviceConfigurationMatch.rs");
include!("DeviceId.rs");
include!("DeviceListColour.rs");
include!("Driver.rs");
include!("DriverFlags.rs");
include!("DriverIdentifier.rs");
include!("InputOutputPort.rs");
include!("SupportedDevice.rs");
include!("VendorId.rs");
