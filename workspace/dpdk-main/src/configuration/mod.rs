// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use ::configuration::ethernetPorts::*;
use ::configuration::ethernetPorts::EthernetPortConfiguration;
use ::dpdk::dpdk_unix::HugePageSize;
use ::dpdk::dpdk_unix::android_linux::memoryStatistics::MemoryStatistics;
use ::dpdk::dpdk_unix::android_linux::mounts::FileSystemType;
use ::dpdk::dpdk_unix::android_linux::mounts::Mount;
use ::dpdk::dpdk_unix::android_linux::mounts::Mounts;
use ::dpdk::dpdk_unix::android_linux::mounts::HugePageMountSettings;
use ::dpdk::dpdk_unix::android_linux::processControl::adjustTransparentHugePages;
use ::dpdk::dpdk_unix::android_linux::resourceLimits::ResourceLimitsSet;
use ::dpdk::dpdk_unix::android_linux::resourceLimits::ResourceLimit;
use ::dpdk::dpdk_unix::android_linux::resourceLimits::ResourceName;
use ::dpdk::dpdk_unix::helpers::makeFileReadWriteAll;
use ::dpdk::dpdk_unix::helpers::makeFolderReadableAndExecutable;
use ::dpdk::devices::DeviceName;
use ::dpdk::devices::virtualDevices::VirtualDevice;
use ::dpdk::devices::virtualDevices::netVirtualDevices::AfPacketNetVirtualDevice;
use ::dpdk::devices::virtualDevices::netVirtualDevices::PacketCaptureNetVirtualDevice;
use ::dpdk::devices::virtualDevices::netVirtualDevices::VirtIoForContainersNetVirtualDevice;
use ::dpdk::devices::virtualDevices::netVirtualDevices::VirtualHostNetVirtualDevice;
use ::dpdk::devices::virtualDevices::netVirtualDevices::XenNetVirtualDevice;
use ::dpdk::devices::virtualDevices::netVirtualDevices::BondingNetVirtualDevice;
use ::dpdk::logicalCores::discovery::NumaSockets;
use ::dpdk::logicalCores::HugePageAllocationStrategy;
use ::dpdk::logicalCores::NonNumaMemory;
use ::dpdk::logicalCores::NumaSocketId;
use ::dpdk::logicalCores::AnyNumaSocketId;
use ::dpdk::pci::android_linux::NetworkPortIdentifier;
use ::dpdk::pci::android_linux::PciDevice;
use ::dpdk::pci::android_linux::PciDriver;
use ::dpdk::pci::android_linux::Unbind;
use ::dpdk::process::MemoryChannels;
use ::dpdk::process::MemoryLimits;
use ::dpdk::process::MemoryRanks;
use ::dpdk::process::HugePageFilePathInformation;
use ::std::collections::HashMap;
use ::std::env::current_exe;
use ::std::fs::create_dir_all;
use ::std::fs::File;
use ::std::fs::read_dir;
use ::std::path::Path;
use ::std::path::PathBuf;
use ::std::sync::Arc;
use ::finishers::HugePageFinisher;
use ::finishers::PciDevicesFinisher;
use ::finishers::Finishers;
use ::programArguments::ProgramArguments;
use ::programArguments::ConfigurationAndProgramArguments;
use ::DpdkRteInitData;
use ::LinuxKernelModule;


pub mod ethernetPorts;


include!("Configuration.rs");
include!("HugePagesConfiguration.rs");
include!("loadAndConfigureLinuxKernelModulesAndVfio.rs");
include!("MemoryConfiguration.rs");
include!("NetworkInterfacesConfiguration.rs");
include!("SampleConfigurationAndProgramArguments.rs");
