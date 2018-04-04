// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#![feature(associated_consts)]
#![feature(const_fn)]
#![feature(never_type)]
#![feature(stmt_expr_attributes)]


// The very long #cfg expressions ensure that dpdk's dependencies and code are only compiled on supported targets
extern crate arrayvec;
#[macro_use] extern crate const_cstr_fork;
extern crate docopt;
#[cfg(any(all(any(target_os = "android", target_os = "linux"), any(target_arch = "aarch64", target_arch = "arm", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64")), all(target_os = "freebsd", target_arch = "x86_64")))] #[macro_use] pub extern crate dpdk;
#[cfg(unix)] extern crate libc;
#[cfg(unix)] extern crate libc_extra;
#[macro_use] extern crate log;
extern crate lru_time_cache;
extern crate ordermap;
#[macro_use] extern crate quick_error;
#[macro_use] extern crate rust_extra;
extern crate rustc_serialize;  // Needed only for docopt
#[cfg(unix)] extern crate stderr_logging;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
#[cfg(unix)] extern crate syscall_alt;


use ::dpdk::pci::DeviceListColour;
use ::dpdk::pci::DeviceAddress;
use ::dpdk::dpdk_unix::strings::pathToCString;
use ::dpdk::dpdk_unix::strings::osStrToCString;
use ::dpdk::dpdk_unix::android_linux::modules::ModulesList;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::dpdk::dpdk_unix::android_linux::capabilities::*;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::dpdk::dpdk_unix::android_linux::processControl::*;
use ::dpdk::dpdk_unix::helpers::blockAllSignalsBarChild;
use ::dpdk::dpdk_unix::helpers::assertEffectiveUserIsRoot;
use ::dpdk::dpdk_unix::helpers::blockNearlyAllSignals;
use ::dpdk::devices::virtualDevices::cryptoVirtualDevices::AesNiGcmCryptoVirtualDevice;
use ::dpdk::devices::virtualDevices::cryptoVirtualDevices::AesNiMultiBufferCryptoVirtualDevice;
use ::dpdk::devices::virtualDevices::cryptoVirtualDevices::KasumiCryptoVirtualDevice;
use ::dpdk::devices::virtualDevices::cryptoVirtualDevices::NullCryptoVirtualDevice;
use ::dpdk::devices::virtualDevices::cryptoVirtualDevices::Snow3gCryptoVirtualDevice;
use ::dpdk::devices::virtualDevices::netVirtualDevices::AfPacketNetVirtualDevice;
use ::dpdk::devices::virtualDevices::netVirtualDevices::BondingNetVirtualDevice;
use ::dpdk::devices::virtualDevices::netVirtualDevices::NullNetVirtualDevice;
use ::dpdk::devices::virtualDevices::netVirtualDevices::PacketCaptureNetVirtualDevice;
use ::dpdk::devices::virtualDevices::netVirtualDevices::RingNetVirtualDevice;
use ::dpdk::devices::virtualDevices::netVirtualDevices::VirtIoForContainersNetVirtualDevice;
use ::dpdk::devices::virtualDevices::netVirtualDevices::VirtualHostNetVirtualDevice;
use ::dpdk::devices::virtualDevices::netVirtualDevices::XenNetVirtualDevice;
use ::dpdk::devices::virtualDevices::VirtualDeviceConfigurations;
use ::dpdk::ethernetPorts::EthernetPort;
use ::dpdk::ethernetPorts::EthernetPortInformation;
use ::dpdk::ethernetPorts::LinkStatus;
use ::dpdk::ethernetPorts::EthernetPortConfigurationResult;
use ::dpdk::logicalCores::discovery::NumaSockets;
use ::dpdk::logicalCores::discovery::LogicalCoreUser;
use ::dpdk::logicalCores::receiveTransmitQueuePair::ExecutionRoutineCreator;
use ::dpdk::logicalCores::receiveTransmitQueuePair::ExecutionRoutineCreatorCreator;
use ::dpdk::logicalCores::receiveTransmitQueuePair::ReceiveTransmitQueuePairSlaveLogicalCoreTask;
use ::dpdk::logicalCores::receiveTransmitQueuePair::ExecutionRoutineGroup;
use ::dpdk::process::HugePageFilePathInformation;
use ::dpdk::process::MemoryChannels;
use ::dpdk::process::MemoryRanks;
use ::dpdk::process::MemoryLimits;
use ::dpdk::process::ProcessType;
use ::dpdk::process::VfioInterruptMode;
use ::programArguments::ConfigurationAndProgramArguments;
use ::configuration::*;
use ::configuration::ethernetPorts::*;
use ::finishers::*;
use ::ip::*;
use ::libc::c_char;
use ::libc::c_int;
use ::libc::setpgid;
use ::libc::umask;
use ::libc_extra::ffi::arguments::VecArguments;
use ::rust_extra::likely;
use ::rust_extra::u31;
use ::std::collections::HashSet;
use ::std::ffi::CString;
use ::std::ffi::OsStr;
use ::std::panic::catch_unwind;
use ::std::panic::AssertUnwindSafe;
use ::std::panic::resume_unwind;
use ::std::path::Path;
use ::std::path::PathBuf;
use ::std::process::exit;
use ::std::ptr::null_mut;
use ::std::sync::Arc;
use ::std::sync::Mutex;


#[cfg(any(all(any(target_os = "android", target_os = "linux"), any(target_arch = "aarch64", target_arch = "arm", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64")), all(target_os = "freebsd", target_arch = "x86_64")))] pub mod configuration;
#[cfg(any(all(any(target_os = "android", target_os = "linux"), any(target_arch = "aarch64", target_arch = "arm", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64")), all(target_os = "freebsd", target_arch = "x86_64")))] pub mod finishers;
#[cfg(any(all(any(target_os = "android", target_os = "linux"), any(target_arch = "aarch64", target_arch = "arm", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64")), all(target_os = "freebsd", target_arch = "x86_64")))] pub mod ip;
#[cfg(any(all(any(target_os = "android", target_os = "linux"), any(target_arch = "aarch64", target_arch = "arm", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64")), all(target_os = "freebsd", target_arch = "x86_64")))] pub mod programArguments;


include!("DpdkRteInitData.rs");
include!("LinuxKernelModule.rs");
include!("Networking.rs");
include!("MainLogic.rs");
