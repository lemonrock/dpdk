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


use ::dpdk::pci::PciDeviceAddress;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::dpdk::dpdk_unix::android_linux::linux_kernel_modules::LinuxKernelModulesList;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::dpdk::dpdk_unix::android_linux::capabilities::*;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::dpdk::dpdk_unix::android_linux::process_control::*;
use ::dpdk::dpdk_unix::assert_effective_user_id_is_root;
use ::dpdk::dpdk_unix::signals::block_all_signals_on_current_thread_bar_child;
use ::dpdk::dpdk_unix::signals::block_all_signals_on_current_thread_bar_hang_up_and_terminate_and_child;
use ::dpdk::dpdk_sys::rte_eal_init;
use ::dpdk::devices::virtual_devices::cryptoVirtualDevices::AesNiGcmCryptoVirtualDevice;
use ::dpdk::devices::virtual_devices::cryptoVirtualDevices::AesNiMultiBufferCryptoVirtualDevice;
use ::dpdk::devices::virtual_devices::cryptoVirtualDevices::KasumiCryptoVirtualDevice;
use ::dpdk::devices::virtual_devices::cryptoVirtualDevices::NullCryptoVirtualDevice;
use ::dpdk::devices::virtual_devices::cryptoVirtualDevices::Snow3gCryptoVirtualDevice;
use ::dpdk::devices::virtual_devices::net_virtual_devices::AfPacketNetVirtualDevice;
use ::dpdk::devices::virtual_devices::net_virtual_devices::BondingNetVirtualDevice;
use ::dpdk::devices::virtual_devices::net_virtual_devices::NullNetVirtualDevice;
use ::dpdk::devices::virtual_devices::net_virtual_devices::PacketCaptureNetVirtualDevice;
use ::dpdk::devices::virtual_devices::net_virtual_devices::RingNetVirtualDevice;
use ::dpdk::devices::virtual_devices::net_virtual_devices::VirtIoNetVirtualDevice;
use ::dpdk::devices::virtual_devices::net_virtual_devices::VirtualHostNetVirtualDevice;
use ::dpdk::devices::virtual_devices::net_virtual_devices::XenNetVirtualDevice;
use ::dpdk::devices::virtual_devices::VirtualDeviceConfigurations;
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
use ::internet_protocol::*;
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
