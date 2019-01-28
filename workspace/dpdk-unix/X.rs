#![feature(prelude_import)]
#![no_std]
// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#![allow(non_upper_case_globals)]
#![allow(renamed_and_removed_lints)]
#![deny(missing_docs)]
#![feature(core_intrinsics)]


//! #dpdk-unix
//!
//! This crate proves additional mid-level functionality for Unix-like Operating Systems which wraps functionality found in low-level FFI bindings for libc.
//!
//! It also provides a very small modicum of Windows support to get the current program name.
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std;


#[macro_use]
extern crate const_cstr_fork;
extern crate errno;
extern crate libc;
extern crate libc_extra;
#[macro_use]
extern crate likely;
#[cfg(unix)]
#[macro_use]
extern crate maplit;
#[macro_use]
extern crate serde_derive;
extern crate rust_extra;
#[cfg(unix)]
extern crate syscall_alt;


use self::memory_information::*;
use self::process_status::*;
use self::strings::*;
use ::const_cstr_fork::ConstCStr;
use ::errno::errno;
use ::libc::*;
use ::libc::c_void;
use ::libc::FILE;
use ::libc::gid_t;
use ::libc::mode_t;
use ::libc::mount;
use ::libc::uid_t;
use ::libc::umount2;
#[cfg(unix)]
use ::libc_extra::unix::stdio::stderr;
#[cfg(unix)]
use ::libc_extra::unix::stdio::stdout;
#[cfg(unix)]
use ::libc_extra::unix::unistd::setegid;
use ::std::collections::BTreeSet;
use ::std::collections::HashMap;
use ::std::collections::HashSet;
use ::std::env::set_var;
use ::std::env::var_os;
use ::std::error;
use ::std::ffi::CStr;
use ::std::ffi::CString;
use ::std::ffi::NulError;
use ::std::ffi::OsStr;
use ::std::ffi::OsString;
use ::std::fmt;
use ::std::fmt::Debug;
use ::std::fmt::Display;
use ::std::fmt::Formatter;
use ::std::fs::create_dir_all;
use ::std::fs::File;
use ::std::fs::OpenOptions;
use ::std::fs::metadata;
use ::std::fs::Permissions;
use ::std::fs::read_to_string;
use ::std::fs::remove_file;
use ::std::fs::set_permissions;
use ::std::io;
use ::std::io::BufRead;
use ::std::io::BufReader;
use ::std::io::ErrorKind;
#[allow(unused_imports)]
use ::std::io::Read;
#[allow(unused_imports)]
use ::std::io::Seek;
use ::std::io::SeekFrom;
use ::std::io::Write;
use ::std::mem::size_of;
use ::std::mem::transmute;
use ::std::mem::uninitialized;
use ::std::num::ParseIntError;
use ::std::ops::Add;
use ::std::ops::AddAssign;
use ::std::ops::Sub;
use ::std::ops::SubAssign;
#[cfg(unix)]
use ::std::os::unix::io::RawFd;
#[cfg(unix)]
use ::std::os::unix::io::AsRawFd;
#[cfg(unix)]
use ::std::os::unix::ffi::OsStrExt;
#[cfg(unix)]
#[allow(unused_imports)]
use ::std::os::unix::ffi::OsStringExt;
#[cfg(unix)]
use ::std::os::unix::fs::PermissionsExt;
use ::std::process;
use ::std::process::Command;
use ::std::process::Stdio;
use ::std::path::Path;
use ::std::path::PathBuf;
use ::std::ptr::NonNull;
use ::std::ptr::null_mut;
use ::std::ptr::write;
use ::std::str::from_utf8;
use ::std::str::FromStr;
use ::std::str::Utf8Error;
#[cfg(unix)]
use ::syscall_alt::constants::E;




/// Memory Information.
pub mod memory_information {








    use super::*;
    /// Memory information names for a process.
    ///
    /// Some old help here: <https://www.centos.org/docs/5/html/5.1/Deployment_Guide/s2-proc-meminfo.html>.
    #[allow(missing_docs)]
    #[structural_match]
    pub enum MemoryInformationName {
        TotalPhysicalRam,
        FreePhysicalRam,

        /// An estimate of physical ram available for starting new applications.
        /// Always larger than `FreePhysicalRam`; see <https://git.kernel.org/cgit/linux/kernel/git/torvalds/linux.git/commit/?id=34e431b0ae398fc54ea69ff85ec700722c9da773>.
        AvailablePhysicalRam,
        UsedAsFileBuffersPhysicalRam,
        UsedAsCachePhysicalRam,
        TotalSwap,
        FreeSwap,
        UsedAsCacheSwap,
        ActiveFileBufferAndCacheInUse,
        InactiveFileBufferAndCacheAvailable,
        AnonymousActive,
        AnonymousInactive,
        FileActive,
        FileInactive,
        Unevictable,

        /// aka 'Dirty'.
        WaitingToBeWrittenBackToDisks,

        /// aka 'Writeback'.
        CurrentlyBeingWrittenBackToDisks,
        UsedForBlockDeviceBounceBuffers,
        NetworkFileSystemUnstablePagesSentToServerButNotYetCommittedToStableStorage,
        MemoryUsedByFuseForTemporaryWritebackBuffers,
        AnonymousMemoryMappedUsingMmap,
        FilesMappedUsingMmap,
        Shmem,
        LockedByMlock,
        Slab,
        SlabReclaimable,
        SlabUnreclaimable,
        KernelStack,
        MemoryDedicatedToLowestPageTableLevel,
        CommitLimit,
        WorstCaseScenarioMemoryRequiredToCompleteWorkloadIncludingSwapMemory,
        TotalVirtualAddressSpaceEgByMalloc,
        UsedVirtualAddressSpaceEgByMalloc,
        LargestContiguousChunkInVirtualAddressSpaceEgByMalloc,

        /// The number is derived by dividing `SizeOfAHugePage` by the megabytes set aside for `hugepages` specified in `/proc/sys/vm/hugetlb_pool`.
        TotalNumberOfHugePages,
        FreeNumberOfHugePages,
        ReservedNumberOfHugePages,
        SurplusNumberOfHugePages,
        SizeOfAHugePage,

        /// Not mapped using `tlbfs`.
        TransparentHugePagesMemoryUsage,
        DirectMap4k,
        DirectMap2M,
        HardwareCorrupted,
        TotalHighNotDirectlyMappedIntoKernelSpace,
        FreeHighNotDirectlyMappedIntoKernelSpace,
        TotalLowDirectlyMappedIntoKernelSpace,
        FreeLowDirectlyMappedIntoKernelSpace,
        ShmemHugePageUsage,
        ShmemMemoryMappedIntoUserSpaceUsingHugePages,
        Unknown(Box<[u8]>),
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    #[allow(missing_docs)]
    impl $crate::fmt::Debug for MemoryInformationName {
        fn fmt(&self, f: &mut $crate::fmt::Formatter) -> $crate::fmt::Result {
            match (&*self,) {
                (&MemoryInformationName::TotalPhysicalRam,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("TotalPhysicalRam");
                    debug_trait_builder.finish()
                }
                (&MemoryInformationName::FreePhysicalRam,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("FreePhysicalRam");
                    debug_trait_builder.finish()
                }
                (&MemoryInformationName::AvailablePhysicalRam,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("AvailablePhysicalRam");
                    debug_trait_builder.finish()
                }
                (&MemoryInformationName::UsedAsFileBuffersPhysicalRam,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("UsedAsFileBuffersPhysicalRam");
                    debug_trait_builder.finish()
                }
                (&MemoryInformationName::UsedAsCachePhysicalRam,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("UsedAsCachePhysicalRam");
                    debug_trait_builder.finish()
                }
                (&MemoryInformationName::TotalSwap,) => {
                    let mut debug_trait_builder = f.debug_tuple("TotalSwap");
                    debug_trait_builder.finish()
                }
                (&MemoryInformationName::FreeSwap,) => {
                    let mut debug_trait_builder = f.debug_tuple("FreeSwap");
                    debug_trait_builder.finish()
                }
                (&MemoryInformationName::UsedAsCacheSwap,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("UsedAsCacheSwap");
                    debug_trait_builder.finish()
                }
                (&MemoryInformationName::ActiveFileBufferAndCacheInUse,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("ActiveFileBufferAndCacheInUse");
                    debug_trait_builder.finish()
                }
                (&MemoryInformationName::InactiveFileBufferAndCacheAvailable,)
                => {
                    let mut debug_trait_builder =
                        f.debug_tuple("InactiveFileBufferAndCacheAvailable");
                    debug_trait_builder.finish()
                }
                (&MemoryInformationName::AnonymousActive,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("AnonymousActive");
                    debug_trait_builder.finish()
                }
                (&MemoryInformationName::AnonymousInactive,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("AnonymousInactive");
                    debug_trait_builder.finish()
                }
                (&MemoryInformationName::FileActive,) => {
                    let mut debug_trait_builder = f.debug_tuple("FileActive");
                    debug_trait_builder.finish()
                }
                (&MemoryInformationName::FileInactive,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("FileInactive");
                    debug_trait_builder.finish()
                }
                (&MemoryInformationName::Unevictable,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("Unevictable");
                    debug_trait_builder.finish()
                }
                (&MemoryInformationName::WaitingToBeWrittenBackToDisks,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("WaitingToBeWrittenBackToDisks");
                    debug_trait_builder.finish()
                }
                (&MemoryInformationName::CurrentlyBeingWrittenBackToDisks,) =>
                {
                    let mut debug_trait_builder =
                        f.debug_tuple("CurrentlyBeingWrittenBackToDisks");
                    debug_trait_builder.finish()
                }
                (&MemoryInformationName::UsedForBlockDeviceBounceBuffers,) =>
                {
                    let mut debug_trait_builder =
                        f.debug_tuple("UsedForBlockDeviceBounceBuffers");
                    debug_trait_builder.finish()
                }
                (&MemoryInformationName::NetworkFileSystemUnstablePagesSentToServerButNotYetCommittedToStableStorage,)
                => {
                    let mut debug_trait_builder =
                        f.debug_tuple("NetworkFileSystemUnstablePagesSentToServerButNotYetCommittedToStableStorage");
                    debug_trait_builder.finish()
                }
                (&MemoryInformationName::MemoryUsedByFuseForTemporaryWritebackBuffers,)
                => {
                    let mut debug_trait_builder =
                        f.debug_tuple("MemoryUsedByFuseForTemporaryWritebackBuffers");
                    debug_trait_builder.finish()
                }
                (&MemoryInformationName::AnonymousMemoryMappedUsingMmap,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("AnonymousMemoryMappedUsingMmap");
                    debug_trait_builder.finish()
                }
                (&MemoryInformationName::FilesMappedUsingMmap,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("FilesMappedUsingMmap");
                    debug_trait_builder.finish()
                }
                (&MemoryInformationName::Shmem,) => {
                    let mut debug_trait_builder = f.debug_tuple("Shmem");
                    debug_trait_builder.finish()
                }
                (&MemoryInformationName::LockedByMlock,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("LockedByMlock");
                    debug_trait_builder.finish()
                }
                (&MemoryInformationName::Slab,) => {
                    let mut debug_trait_builder = f.debug_tuple("Slab");
                    debug_trait_builder.finish()
                }
                (&MemoryInformationName::SlabReclaimable,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("SlabReclaimable");
                    debug_trait_builder.finish()
                }
                (&MemoryInformationName::SlabUnreclaimable,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("SlabUnreclaimable");
                    debug_trait_builder.finish()
                }
                (&MemoryInformationName::KernelStack,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("KernelStack");
                    debug_trait_builder.finish()
                }
                (&MemoryInformationName::MemoryDedicatedToLowestPageTableLevel,)
                => {
                    let mut debug_trait_builder =
                        f.debug_tuple("MemoryDedicatedToLowestPageTableLevel");
                    debug_trait_builder.finish()
                }
                (&MemoryInformationName::CommitLimit,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("CommitLimit");
                    debug_trait_builder.finish()
                }
                (&MemoryInformationName::WorstCaseScenarioMemoryRequiredToCompleteWorkloadIncludingSwapMemory,)
                => {
                    let mut debug_trait_builder =
                        f.debug_tuple("WorstCaseScenarioMemoryRequiredToCompleteWorkloadIncludingSwapMemory");
                    debug_trait_builder.finish()
                }
                (&MemoryInformationName::TotalVirtualAddressSpaceEgByMalloc,)
                => {
                    let mut debug_trait_builder =
                        f.debug_tuple("TotalVirtualAddressSpaceEgByMalloc");
                    debug_trait_builder.finish()
                }
                (&MemoryInformationName::UsedVirtualAddressSpaceEgByMalloc,)
                => {
                    let mut debug_trait_builder =
                        f.debug_tuple("UsedVirtualAddressSpaceEgByMalloc");
                    debug_trait_builder.finish()
                }
                (&MemoryInformationName::LargestContiguousChunkInVirtualAddressSpaceEgByMalloc,)
                => {
                    let mut debug_trait_builder =
                        f.debug_tuple("LargestContiguousChunkInVirtualAddressSpaceEgByMalloc");
                    debug_trait_builder.finish()
                }
                (&MemoryInformationName::TotalNumberOfHugePages,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("TotalNumberOfHugePages");
                    debug_trait_builder.finish()
                }
                (&MemoryInformationName::FreeNumberOfHugePages,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("FreeNumberOfHugePages");
                    debug_trait_builder.finish()
                }
                (&MemoryInformationName::ReservedNumberOfHugePages,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("ReservedNumberOfHugePages");
                    debug_trait_builder.finish()
                }
                (&MemoryInformationName::SurplusNumberOfHugePages,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("SurplusNumberOfHugePages");
                    debug_trait_builder.finish()
                }
                (&MemoryInformationName::SizeOfAHugePage,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("SizeOfAHugePage");
                    debug_trait_builder.finish()
                }
                (&MemoryInformationName::TransparentHugePagesMemoryUsage,) =>
                {
                    let mut debug_trait_builder =
                        f.debug_tuple("TransparentHugePagesMemoryUsage");
                    debug_trait_builder.finish()
                }
                (&MemoryInformationName::DirectMap4k,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("DirectMap4k");
                    debug_trait_builder.finish()
                }
                (&MemoryInformationName::DirectMap2M,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("DirectMap2M");
                    debug_trait_builder.finish()
                }
                (&MemoryInformationName::HardwareCorrupted,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("HardwareCorrupted");
                    debug_trait_builder.finish()
                }
                (&MemoryInformationName::TotalHighNotDirectlyMappedIntoKernelSpace,)
                => {
                    let mut debug_trait_builder =
                        f.debug_tuple("TotalHighNotDirectlyMappedIntoKernelSpace");
                    debug_trait_builder.finish()
                }
                (&MemoryInformationName::FreeHighNotDirectlyMappedIntoKernelSpace,)
                => {
                    let mut debug_trait_builder =
                        f.debug_tuple("FreeHighNotDirectlyMappedIntoKernelSpace");
                    debug_trait_builder.finish()
                }
                (&MemoryInformationName::TotalLowDirectlyMappedIntoKernelSpace,)
                => {
                    let mut debug_trait_builder =
                        f.debug_tuple("TotalLowDirectlyMappedIntoKernelSpace");
                    debug_trait_builder.finish()
                }
                (&MemoryInformationName::FreeLowDirectlyMappedIntoKernelSpace,)
                => {
                    let mut debug_trait_builder =
                        f.debug_tuple("FreeLowDirectlyMappedIntoKernelSpace");
                    debug_trait_builder.finish()
                }
                (&MemoryInformationName::ShmemHugePageUsage,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("ShmemHugePageUsage");
                    debug_trait_builder.finish()
                }
                (&MemoryInformationName::ShmemMemoryMappedIntoUserSpaceUsingHugePages,)
                => {
                    let mut debug_trait_builder =
                        f.debug_tuple("ShmemMemoryMappedIntoUserSpaceUsingHugePages");
                    debug_trait_builder.finish()
                }
                (&MemoryInformationName::Unknown(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Unknown");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    #[allow(missing_docs)]
    impl $crate::clone::Clone for MemoryInformationName {
        #[inline]
        fn clone(&self) -> MemoryInformationName {
            match (&*self,) {
                (&MemoryInformationName::TotalPhysicalRam,) =>
                MemoryInformationName::TotalPhysicalRam,
                (&MemoryInformationName::FreePhysicalRam,) =>
                MemoryInformationName::FreePhysicalRam,
                (&MemoryInformationName::AvailablePhysicalRam,) =>
                MemoryInformationName::AvailablePhysicalRam,
                (&MemoryInformationName::UsedAsFileBuffersPhysicalRam,) =>
                MemoryInformationName::UsedAsFileBuffersPhysicalRam,
                (&MemoryInformationName::UsedAsCachePhysicalRam,) =>
                MemoryInformationName::UsedAsCachePhysicalRam,
                (&MemoryInformationName::TotalSwap,) =>
                MemoryInformationName::TotalSwap,
                (&MemoryInformationName::FreeSwap,) =>
                MemoryInformationName::FreeSwap,
                (&MemoryInformationName::UsedAsCacheSwap,) =>
                MemoryInformationName::UsedAsCacheSwap,
                (&MemoryInformationName::ActiveFileBufferAndCacheInUse,) =>
                MemoryInformationName::ActiveFileBufferAndCacheInUse,
                (&MemoryInformationName::InactiveFileBufferAndCacheAvailable,)
                => MemoryInformationName::InactiveFileBufferAndCacheAvailable,
                (&MemoryInformationName::AnonymousActive,) =>
                MemoryInformationName::AnonymousActive,
                (&MemoryInformationName::AnonymousInactive,) =>
                MemoryInformationName::AnonymousInactive,
                (&MemoryInformationName::FileActive,) =>
                MemoryInformationName::FileActive,
                (&MemoryInformationName::FileInactive,) =>
                MemoryInformationName::FileInactive,
                (&MemoryInformationName::Unevictable,) =>
                MemoryInformationName::Unevictable,
                (&MemoryInformationName::WaitingToBeWrittenBackToDisks,) =>
                MemoryInformationName::WaitingToBeWrittenBackToDisks,
                (&MemoryInformationName::CurrentlyBeingWrittenBackToDisks,) =>
                MemoryInformationName::CurrentlyBeingWrittenBackToDisks,
                (&MemoryInformationName::UsedForBlockDeviceBounceBuffers,) =>
                MemoryInformationName::UsedForBlockDeviceBounceBuffers,
                (&MemoryInformationName::NetworkFileSystemUnstablePagesSentToServerButNotYetCommittedToStableStorage,)
                =>
                MemoryInformationName::NetworkFileSystemUnstablePagesSentToServerButNotYetCommittedToStableStorage,
                (&MemoryInformationName::MemoryUsedByFuseForTemporaryWritebackBuffers,)
                =>
                MemoryInformationName::MemoryUsedByFuseForTemporaryWritebackBuffers,
                (&MemoryInformationName::AnonymousMemoryMappedUsingMmap,) =>
                MemoryInformationName::AnonymousMemoryMappedUsingMmap,
                (&MemoryInformationName::FilesMappedUsingMmap,) =>
                MemoryInformationName::FilesMappedUsingMmap,
                (&MemoryInformationName::Shmem,) =>
                MemoryInformationName::Shmem,
                (&MemoryInformationName::LockedByMlock,) =>
                MemoryInformationName::LockedByMlock,
                (&MemoryInformationName::Slab,) =>
                MemoryInformationName::Slab,
                (&MemoryInformationName::SlabReclaimable,) =>
                MemoryInformationName::SlabReclaimable,
                (&MemoryInformationName::SlabUnreclaimable,) =>
                MemoryInformationName::SlabUnreclaimable,
                (&MemoryInformationName::KernelStack,) =>
                MemoryInformationName::KernelStack,
                (&MemoryInformationName::MemoryDedicatedToLowestPageTableLevel,)
                =>
                MemoryInformationName::MemoryDedicatedToLowestPageTableLevel,
                (&MemoryInformationName::CommitLimit,) =>
                MemoryInformationName::CommitLimit,
                (&MemoryInformationName::WorstCaseScenarioMemoryRequiredToCompleteWorkloadIncludingSwapMemory,)
                =>
                MemoryInformationName::WorstCaseScenarioMemoryRequiredToCompleteWorkloadIncludingSwapMemory,
                (&MemoryInformationName::TotalVirtualAddressSpaceEgByMalloc,)
                => MemoryInformationName::TotalVirtualAddressSpaceEgByMalloc,
                (&MemoryInformationName::UsedVirtualAddressSpaceEgByMalloc,)
                => MemoryInformationName::UsedVirtualAddressSpaceEgByMalloc,
                (&MemoryInformationName::LargestContiguousChunkInVirtualAddressSpaceEgByMalloc,)
                =>
                MemoryInformationName::LargestContiguousChunkInVirtualAddressSpaceEgByMalloc,
                (&MemoryInformationName::TotalNumberOfHugePages,) =>
                MemoryInformationName::TotalNumberOfHugePages,
                (&MemoryInformationName::FreeNumberOfHugePages,) =>
                MemoryInformationName::FreeNumberOfHugePages,
                (&MemoryInformationName::ReservedNumberOfHugePages,) =>
                MemoryInformationName::ReservedNumberOfHugePages,
                (&MemoryInformationName::SurplusNumberOfHugePages,) =>
                MemoryInformationName::SurplusNumberOfHugePages,
                (&MemoryInformationName::SizeOfAHugePage,) =>
                MemoryInformationName::SizeOfAHugePage,
                (&MemoryInformationName::TransparentHugePagesMemoryUsage,) =>
                MemoryInformationName::TransparentHugePagesMemoryUsage,
                (&MemoryInformationName::DirectMap4k,) =>
                MemoryInformationName::DirectMap4k,
                (&MemoryInformationName::DirectMap2M,) =>
                MemoryInformationName::DirectMap2M,
                (&MemoryInformationName::HardwareCorrupted,) =>
                MemoryInformationName::HardwareCorrupted,
                (&MemoryInformationName::TotalHighNotDirectlyMappedIntoKernelSpace,)
                =>
                MemoryInformationName::TotalHighNotDirectlyMappedIntoKernelSpace,
                (&MemoryInformationName::FreeHighNotDirectlyMappedIntoKernelSpace,)
                =>
                MemoryInformationName::FreeHighNotDirectlyMappedIntoKernelSpace,
                (&MemoryInformationName::TotalLowDirectlyMappedIntoKernelSpace,)
                =>
                MemoryInformationName::TotalLowDirectlyMappedIntoKernelSpace,
                (&MemoryInformationName::FreeLowDirectlyMappedIntoKernelSpace,)
                =>
                MemoryInformationName::FreeLowDirectlyMappedIntoKernelSpace,
                (&MemoryInformationName::ShmemHugePageUsage,) =>
                MemoryInformationName::ShmemHugePageUsage,
                (&MemoryInformationName::ShmemMemoryMappedIntoUserSpaceUsingHugePages,)
                =>
                MemoryInformationName::ShmemMemoryMappedIntoUserSpaceUsingHugePages,
                (&MemoryInformationName::Unknown(ref __self_0),) =>
                MemoryInformationName::Unknown($crate::clone::Clone::clone(&(*__self_0))),
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    #[allow(missing_docs)]
    impl $crate::cmp::PartialEq for MemoryInformationName {
        #[inline]
        fn eq(&self, other: &MemoryInformationName) -> bool {
            {
                let __self_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&MemoryInformationName::Unknown(ref __self_0),
                         &MemoryInformationName::Unknown(ref __arg_1_0)) =>
                        (*__self_0) == (*__arg_1_0),
                        _ => true,
                    }
                } else { false }
            }
        }
        #[inline]
        fn ne(&self, other: &MemoryInformationName) -> bool {
            {
                let __self_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&MemoryInformationName::Unknown(ref __self_0),
                         &MemoryInformationName::Unknown(ref __arg_1_0)) =>
                        (*__self_0) != (*__arg_1_0),
                        _ => false,
                    }
                } else { true }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    #[allow(missing_docs)]
    impl $crate::cmp::Eq for MemoryInformationName {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            { let _: $crate::cmp::AssertParamIsEq<Box<[u8]>>; }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    #[allow(missing_docs)]
    impl $crate::cmp::PartialOrd for MemoryInformationName {
        #[inline]
        fn partial_cmp(&self, other: &MemoryInformationName)
         -> $crate::option::Option<$crate::cmp::Ordering> {
            {
                let __self_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&MemoryInformationName::Unknown(ref __self_0),
                         &MemoryInformationName::Unknown(ref __arg_1_0)) =>
                        match $crate::cmp::PartialOrd::partial_cmp(&(*__self_0),
                                                                   &(*__arg_1_0))
                            {
                            $crate::option::Option::Some($crate::cmp::Ordering::Equal)
                            =>
                            $crate::option::Option::Some($crate::cmp::Ordering::Equal),
                            cmp => cmp,
                        },
                        _ =>
                        $crate::option::Option::Some($crate::cmp::Ordering::Equal),
                    }
                } else { __self_vi.partial_cmp(&__arg_1_vi) }
            }
        }
        #[inline]
        fn lt(&self, other: &MemoryInformationName) -> bool {
            {
                let __self_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&MemoryInformationName::Unknown(ref __self_0),
                         &MemoryInformationName::Unknown(ref __arg_1_0)) =>
                        $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0),
                                                                                               &(*__arg_1_0)),
                                                          $crate::cmp::Ordering::Greater)
                            == $crate::cmp::Ordering::Less,
                        _ => false,
                    }
                } else { __self_vi.lt(&__arg_1_vi) }
            }
        }
        #[inline]
        fn le(&self, other: &MemoryInformationName) -> bool {
            {
                let __self_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&MemoryInformationName::Unknown(ref __self_0),
                         &MemoryInformationName::Unknown(ref __arg_1_0)) =>
                        $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0),
                                                                                               &(*__arg_1_0)),
                                                          $crate::cmp::Ordering::Greater)
                            != $crate::cmp::Ordering::Greater,
                        _ => true,
                    }
                } else { __self_vi.le(&__arg_1_vi) }
            }
        }
        #[inline]
        fn gt(&self, other: &MemoryInformationName) -> bool {
            {
                let __self_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&MemoryInformationName::Unknown(ref __self_0),
                         &MemoryInformationName::Unknown(ref __arg_1_0)) =>
                        $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0),
                                                                                               &(*__arg_1_0)),
                                                          $crate::cmp::Ordering::Less)
                            == $crate::cmp::Ordering::Greater,
                        _ => false,
                    }
                } else { __self_vi.gt(&__arg_1_vi) }
            }
        }
        #[inline]
        fn ge(&self, other: &MemoryInformationName) -> bool {
            {
                let __self_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&MemoryInformationName::Unknown(ref __self_0),
                         &MemoryInformationName::Unknown(ref __arg_1_0)) =>
                        $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0),
                                                                                               &(*__arg_1_0)),
                                                          $crate::cmp::Ordering::Less)
                            != $crate::cmp::Ordering::Less,
                        _ => true,
                    }
                } else { __self_vi.ge(&__arg_1_vi) }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    #[allow(missing_docs)]
    impl $crate::cmp::Ord for MemoryInformationName {
        #[inline]
        fn cmp(&self, other: &MemoryInformationName)
         -> $crate::cmp::Ordering {
            {
                let __self_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&MemoryInformationName::Unknown(ref __self_0),
                         &MemoryInformationName::Unknown(ref __arg_1_0)) =>
                        match $crate::cmp::Ord::cmp(&(*__self_0),
                                                    &(*__arg_1_0)) {
                            $crate::cmp::Ordering::Equal =>
                            $crate::cmp::Ordering::Equal,
                            cmp => cmp,
                        },
                        _ => $crate::cmp::Ordering::Equal,
                    }
                } else { __self_vi.cmp(&__arg_1_vi) }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    #[allow(missing_docs)]
    impl $crate::hash::Hash for MemoryInformationName {
        fn hash<__H: $crate::hash::Hasher>(&self, state: &mut __H) -> () {
            match (&*self,) {
                (&MemoryInformationName::Unknown(ref __self_0),) => {
                    $crate::hash::Hash::hash(&unsafe {
                                                  $crate::intrinsics::discriminant_value(self)
                                              }, state);
                    $crate::hash::Hash::hash(&(*__self_0), state)
                }
                _ => {
                    $crate::hash::Hash::hash(&unsafe {
                                                  $crate::intrinsics::discriminant_value(self)
                                              }, state)
                }
            }
        }
    }
    impl MemoryInformationName {
        /// Parse a memory statistic name.
        ///
        /// This list is NOT definitive; names come and go.
        #[inline(always)]
        pub(crate) fn parse(value: &[u8],
                            memory_information_name_prefix: &[u8])
         -> MemoryInformationName {
            use self::MemoryInformationName::*;
            if !value.starts_with(memory_information_name_prefix) {
                return Unknown(value.to_vec().into_boxed_slice())
            }
            match &value[memory_information_name_prefix.len()..] {
                b"MemTotal" => TotalPhysicalRam,
                b"MemFree" => FreePhysicalRam,
                b"MemAvailable" => AvailablePhysicalRam,
                b"Buffers" => UsedAsFileBuffersPhysicalRam,
                b"Cached" => UsedAsCachePhysicalRam,
                b"SwapTotal" => TotalSwap,
                b"SwapFree" => FreeSwap,
                b"SwapCached" => UsedAsCacheSwap,
                b"Active" => ActiveFileBufferAndCacheInUse,
                b"Inactive" => InactiveFileBufferAndCacheAvailable,
                b"Active(anon" => AnonymousActive,
                b"Inactive(anon" => AnonymousInactive,
                b"Active(file" => FileActive,
                b"Inactive(file" => FileInactive,
                b"Unevictable" => Unevictable,
                b"Dirty" => WaitingToBeWrittenBackToDisks,
                b"Writeback" => CurrentlyBeingWrittenBackToDisks,
                b"Bounce" => UsedForBlockDeviceBounceBuffers,
                b"NFS_Unstable" =>
                NetworkFileSystemUnstablePagesSentToServerButNotYetCommittedToStableStorage,
                b"WritebackTmp" =>
                MemoryUsedByFuseForTemporaryWritebackBuffers,
                b"AnonPages" => AnonymousMemoryMappedUsingMmap,
                b"Mapped" => FilesMappedUsingMmap,
                b"Shmem" => Shmem,
                b"Mlocked" => LockedByMlock,
                b"Slab" => Slab,
                b"SReclaimable" => SlabReclaimable,
                b"SUnreclaim" => SlabUnreclaimable,
                b"KernelStack" => KernelStack,
                b"PageTables" => MemoryDedicatedToLowestPageTableLevel,
                b"CommitLimit" => CommitLimit,
                b"Committed_AS" =>
                WorstCaseScenarioMemoryRequiredToCompleteWorkloadIncludingSwapMemory,
                b"VmallocTotal" => TotalVirtualAddressSpaceEgByMalloc,
                b"VmallocUsed" => UsedVirtualAddressSpaceEgByMalloc,
                b"VmallocChunk" =>
                LargestContiguousChunkInVirtualAddressSpaceEgByMalloc,
                b"HugePages_Total" => TotalNumberOfHugePages,
                b"HugePages_Free" => FreeNumberOfHugePages,
                b"HugePages_Rsvd" => ReservedNumberOfHugePages,
                b"HugePages_Surp" => SurplusNumberOfHugePages,
                b"Hugepagesize" => SizeOfAHugePage,
                b"AnonHugePages" => TransparentHugePagesMemoryUsage,
                b"DirectMap4k" => DirectMap4k,
                b"DirectMap2M" => DirectMap2M,
                b"HardwareCorrupted" => HardwareCorrupted,
                b"HighTotal" => TotalHighNotDirectlyMappedIntoKernelSpace,
                b"HighFree" => FreeHighNotDirectlyMappedIntoKernelSpace,
                b"LowTotal" => TotalLowDirectlyMappedIntoKernelSpace,
                b"LowFree" => FreeLowDirectlyMappedIntoKernelSpace,
                b"ShmemHugePages" => ShmemHugePageUsage,
                b"ShmemPmdMapped" =>
                ShmemMemoryMappedIntoUserSpaceUsingHugePages,
                name@_ => Unknown(name.to_vec().into_boxed_slice()),
            }
        }
        /// Deprecated or not understood memory statistic names.
        #[inline(always)]
        pub fn is_deprecated_or_not_understood(&self) -> bool {
            use self::MemoryInformationName::*;
            match *self {
                HardwareCorrupted => true,
                TotalHighNotDirectlyMappedIntoKernelSpace => true,
                FreeHighNotDirectlyMappedIntoKernelSpace => true,
                TotalLowDirectlyMappedIntoKernelSpace => true,
                FreeLowDirectlyMappedIntoKernelSpace => true,
                ShmemHugePageUsage => true,
                ShmemMemoryMappedIntoUserSpaceUsingHugePages => true,
                _ => false,
            }
        }
        /// Associated memory statistic unit.
        #[inline(always)]
        pub fn unit(&self) -> MemoryInformationUnit {
            use self::MemoryInformationName::*;
            use self::MemoryInformationUnit::*;
            match *self {
                TotalNumberOfHugePages => Count,
                FreeNumberOfHugePages => Count,
                ReservedNumberOfHugePages => Count,
                SurplusNumberOfHugePages => Count,
                _ => KiloByte,
            }
        }
    }
    /// Memory statistic unit.
    #[structural_match]
    #[rustc_copy_clone_marker]
    pub enum MemoryInformationUnit {

        /// KiloByte.
        KiloByte,

        /// Numeric count.
        Count,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::fmt::Debug for MemoryInformationUnit {
        fn fmt(&self, f: &mut $crate::fmt::Formatter) -> $crate::fmt::Result {
            match (&*self,) {
                (&MemoryInformationUnit::KiloByte,) => {
                    let mut debug_trait_builder = f.debug_tuple("KiloByte");
                    debug_trait_builder.finish()
                }
                (&MemoryInformationUnit::Count,) => {
                    let mut debug_trait_builder = f.debug_tuple("Count");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::marker::Copy for MemoryInformationUnit { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::clone::Clone for MemoryInformationUnit {
        #[inline]
        fn clone(&self) -> MemoryInformationUnit { { *self } }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::cmp::PartialEq for MemoryInformationUnit {
        #[inline]
        fn eq(&self, other: &MemoryInformationUnit) -> bool {
            {
                let __self_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) { _ => true, }
                } else { false }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::cmp::Eq for MemoryInformationUnit {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () { { } }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::cmp::PartialOrd for MemoryInformationUnit {
        #[inline]
        fn partial_cmp(&self, other: &MemoryInformationUnit)
         -> $crate::option::Option<$crate::cmp::Ordering> {
            {
                let __self_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        _ =>
                        $crate::option::Option::Some($crate::cmp::Ordering::Equal),
                    }
                } else { __self_vi.partial_cmp(&__arg_1_vi) }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::cmp::Ord for MemoryInformationUnit {
        #[inline]
        fn cmp(&self, other: &MemoryInformationUnit)
         -> $crate::cmp::Ordering {
            {
                let __self_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        _ => $crate::cmp::Ordering::Equal,
                    }
                } else { __self_vi.cmp(&__arg_1_vi) }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::hash::Hash for MemoryInformationUnit {
        fn hash<__H: $crate::hash::Hasher>(&self, state: &mut __H) -> () {
            match (&*self,) {
                _ => {
                    $crate::hash::Hash::hash(&unsafe {
                                                  $crate::intrinsics::discriminant_value(self)
                                              }, state)
                }
            }
        }
    }
    impl MemoryInformationUnit {
        #[inline(always)]
        pub(crate) fn ends_with(&self) -> &'static str {
            match *self {
                MemoryInformationUnit::KiloByte => " kB",
                MemoryInformationUnit::Count => "",
            }
        }
    }
    /// A set of memory statistics.
    ///
    /// Super-detailed information (hard to parse, too) is in `/proc/zoneinfo`.
    /// This is broken down into DMA, DMA33 and Normal sub-zones and then by CPU for each Numa Node (aka 'zone').
    /// A sort of detailed version of `/proc/vmstat`.
    #[structural_match]
    pub struct MemoryInformation(pub(crate) HashMap<MemoryInformationName,
                                                    u64>);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::fmt::Debug for MemoryInformation {
        fn fmt(&self, f: &mut $crate::fmt::Formatter) -> $crate::fmt::Result {
            match *self {
                MemoryInformation(ref __self_0_0) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("MemoryInformation");
                    let _ = debug_trait_builder.field(&&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::clone::Clone for MemoryInformation {
        #[inline]
        fn clone(&self) -> MemoryInformation {
            match *self {
                MemoryInformation(ref __self_0_0) =>
                MemoryInformation($crate::clone::Clone::clone(&(*__self_0_0))),
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::cmp::PartialEq for MemoryInformation {
        #[inline]
        fn eq(&self, other: &MemoryInformation) -> bool {
            match *other {
                MemoryInformation(ref __self_1_0) =>
                match *self {
                    MemoryInformation(ref __self_0_0) =>
                    (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &MemoryInformation) -> bool {
            match *other {
                MemoryInformation(ref __self_1_0) =>
                match *self {
                    MemoryInformation(ref __self_0_0) =>
                    (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::cmp::Eq for MemoryInformation {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _:
                        $crate::cmp::AssertParamIsEq<HashMap<MemoryInformationName,
                                                             u64>>;
            }
        }
    }
    impl MemoryInformation {
        /// Get a statistic.
        #[inline]
        pub fn get_statistic(&self,
                             memory_information_name: &MemoryInformationName)
         -> Option<u64> {
            match self.0.get(memory_information_name) {
                None => None,
                Some(value) => Some(*value),
            }
        }
        /// Free physical RAM in Kilobytes.
        #[inline(always)]
        pub fn free_physical_ram(&self) -> Option<u64> {
            self.get_statistic(&MemoryInformationName::FreePhysicalRam)
        }
        /// Default huge page size.
        pub fn default_huge_page_size(&self) -> Option<HugePageSize> {
            if let Some(size_in_bytes) =
                   self.get_statistic(&MemoryInformationName::SizeOfAHugePage)
                   {
                HugePageSize::from_proc_mem_info_value(size_in_bytes)
            } else { None }
        }
        /// Used physical RAM in bytes.
        #[inline]
        pub fn used_physical_ram(&self) -> Option<u64> {
            if let Some(total_physical_ram) =
                   self.get_statistic(&MemoryInformationName::TotalPhysicalRam)
                   {
                if let Some(free_physical_ram) =
                       self.get_statistic(&MemoryInformationName::FreePhysicalRam)
                       {
                    Some(total_physical_ram - free_physical_ram)
                } else { None }
            } else { None }
        }
        /// Used swap RAM in bytes.
        #[inline]
        pub fn used_swap(&self) -> Option<u64> {
            if let Some(total_swap) =
                   self.get_statistic(&MemoryInformationName::TotalSwap) {
                if let Some(free_swap) =
                       self.get_statistic(&MemoryInformationName::FreeSwap) {
                    Some(total_swap - free_swap)
                } else { None }
            } else { None }
        }
    }
    /// Errors possible when parsing memory statistics.
    pub enum MemoryInformationParseError {

        /// Could not open a file of memory statistics.
        CouldNotOpenFile(io::Error),

        /// Could not parse a memory statistic.
        CouldNotParseMemoryInformationValue {
            /// Zero-based line number in the file the error occurred at.
            zero_based_line_number: usize,
            /// Memory item it occurred for.
            memory_information_name: MemoryInformationName,
        },

        /// Could not parse a memory statistic as a UTF-8 string.
        CouldNotParseAsUtf8 {
            /// Zero-based line number in the file the error occurred at.
            zero_based_line_number: usize,
            /// Memory item it occurred for.
            memory_information_name: MemoryInformationName,
            /// Bad value.
            bad_value: Box<[u8]>,
            /// Cause.
            cause: Utf8Error,
        },

        /// Could not parse a memory statistic (trimmed).
        CouldNotParseMemoryInformationValueTrimmed {
            /// Zero-based line number in the file the error occurred at.
            zero_based_line_number: usize,
            /// Memory item it occurred for.
            memory_information_name: MemoryInformationName,
            /// Bad value.
            bad_value: String,
        },

        /// Could not parse a memory statistic as a u64 value.
        CouldNotParseMemoryInformationValueAsU64 {
            /// Zero-based line number in the file the error occurred at.
            zero_based_line_number: usize,
            /// Memory item it occurred for.
            memory_information_name: MemoryInformationName,
            /// Bad value.
            bad_value: String,
            /// Underlying parse error.
            cause: ParseIntError,
        },

        /// Could not parse a memory statistic because it was a duplicate.
        DuplicateMemoryInformation {
            /// Zero-based line number in the file the error occurred at.
            zero_based_line_number: usize,
            /// Memory item it occurred for.
            memory_information_name: MemoryInformationName,
            /// New value.
            new_value: u64,
        },
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::fmt::Debug for MemoryInformationParseError {
        fn fmt(&self, f: &mut $crate::fmt::Formatter) -> $crate::fmt::Result {
            match (&*self,) {
                (&MemoryInformationParseError::CouldNotOpenFile(ref __self_0),)
                => {
                    let mut debug_trait_builder =
                        f.debug_tuple("CouldNotOpenFile");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&MemoryInformationParseError::CouldNotParseMemoryInformationValue {
                 zero_based_line_number: ref __self_0,
                 memory_information_name: ref __self_1 },) => {
                    let mut debug_trait_builder =
                        f.debug_struct("CouldNotParseMemoryInformationValue");
                    let _ =
                        debug_trait_builder.field("zero_based_line_number",
                                                  &&(*__self_0));
                    let _ =
                        debug_trait_builder.field("memory_information_name",
                                                  &&(*__self_1));
                    debug_trait_builder.finish()
                }
                (&MemoryInformationParseError::CouldNotParseAsUtf8 {
                 zero_based_line_number: ref __self_0,
                 memory_information_name: ref __self_1,
                 bad_value: ref __self_2,
                 cause: ref __self_3 },) => {
                    let mut debug_trait_builder =
                        f.debug_struct("CouldNotParseAsUtf8");
                    let _ =
                        debug_trait_builder.field("zero_based_line_number",
                                                  &&(*__self_0));
                    let _ =
                        debug_trait_builder.field("memory_information_name",
                                                  &&(*__self_1));
                    let _ =
                        debug_trait_builder.field("bad_value", &&(*__self_2));
                    let _ = debug_trait_builder.field("cause", &&(*__self_3));
                    debug_trait_builder.finish()
                }
                (&MemoryInformationParseError::CouldNotParseMemoryInformationValueTrimmed {
                 zero_based_line_number: ref __self_0,
                 memory_information_name: ref __self_1,
                 bad_value: ref __self_2 },) => {
                    let mut debug_trait_builder =
                        f.debug_struct("CouldNotParseMemoryInformationValueTrimmed");
                    let _ =
                        debug_trait_builder.field("zero_based_line_number",
                                                  &&(*__self_0));
                    let _ =
                        debug_trait_builder.field("memory_information_name",
                                                  &&(*__self_1));
                    let _ =
                        debug_trait_builder.field("bad_value", &&(*__self_2));
                    debug_trait_builder.finish()
                }
                (&MemoryInformationParseError::CouldNotParseMemoryInformationValueAsU64 {
                 zero_based_line_number: ref __self_0,
                 memory_information_name: ref __self_1,
                 bad_value: ref __self_2,
                 cause: ref __self_3 },) => {
                    let mut debug_trait_builder =
                        f.debug_struct("CouldNotParseMemoryInformationValueAsU64");
                    let _ =
                        debug_trait_builder.field("zero_based_line_number",
                                                  &&(*__self_0));
                    let _ =
                        debug_trait_builder.field("memory_information_name",
                                                  &&(*__self_1));
                    let _ =
                        debug_trait_builder.field("bad_value", &&(*__self_2));
                    let _ = debug_trait_builder.field("cause", &&(*__self_3));
                    debug_trait_builder.finish()
                }
                (&MemoryInformationParseError::DuplicateMemoryInformation {
                 zero_based_line_number: ref __self_0,
                 memory_information_name: ref __self_1,
                 new_value: ref __self_2 },) => {
                    let mut debug_trait_builder =
                        f.debug_struct("DuplicateMemoryInformation");
                    let _ =
                        debug_trait_builder.field("zero_based_line_number",
                                                  &&(*__self_0));
                    let _ =
                        debug_trait_builder.field("memory_information_name",
                                                  &&(*__self_1));
                    let _ =
                        debug_trait_builder.field("new_value", &&(*__self_2));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl Display for MemoryInformationParseError {
        #[inline(always)]
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            <MemoryInformationParseError as Debug>::fmt(self, f)
        }
    }
    impl error::Error for MemoryInformationParseError {
        #[inline(always)]
        fn source(&self) -> Option<&(error::Error + 'static)> {
            use self::MemoryInformationParseError::*;
            match self {
                &CouldNotOpenFile(ref error) => Some(error),
                &CouldNotParseMemoryInformationValue { .. } => None,
                &CouldNotParseAsUtf8 { ref cause, .. } => Some(cause),
                &CouldNotParseMemoryInformationValueTrimmed { .. } => None,
                &CouldNotParseMemoryInformationValueAsU64 { ref cause, .. } =>
                Some(cause),
                &DuplicateMemoryInformation { .. } => None,
            }
        }
    }
    impl From<io::Error> for MemoryInformationParseError {
        #[inline(always)]
        fn from(error: io::Error) -> Self {
            MemoryInformationParseError::CouldNotOpenFile(error)
        }
    }
    /// Physical Address.
    #[structural_match]
    #[rustc_copy_clone_marker]
    pub struct PhysicalAddress(u64);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::default::Default for PhysicalAddress {
        #[inline]
        fn default() -> PhysicalAddress {
            PhysicalAddress($crate::default::Default::default())
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::fmt::Debug for PhysicalAddress {
        fn fmt(&self, f: &mut $crate::fmt::Formatter) -> $crate::fmt::Result {
            match *self {
                PhysicalAddress(ref __self_0_0) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("PhysicalAddress");
                    let _ = debug_trait_builder.field(&&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::marker::Copy for PhysicalAddress { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::clone::Clone for PhysicalAddress {
        #[inline]
        fn clone(&self) -> PhysicalAddress {
            { let _: $crate::clone::AssertParamIsClone<u64>; *self }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::cmp::Ord for PhysicalAddress {
        #[inline]
        fn cmp(&self, other: &PhysicalAddress) -> $crate::cmp::Ordering {
            match *other {
                PhysicalAddress(ref __self_1_0) =>
                match *self {
                    PhysicalAddress(ref __self_0_0) =>
                    match $crate::cmp::Ord::cmp(&(*__self_0_0),
                                                &(*__self_1_0)) {
                        $crate::cmp::Ordering::Equal =>
                        $crate::cmp::Ordering::Equal,
                        cmp => cmp,
                    },
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::cmp::PartialOrd for PhysicalAddress {
        #[inline]
        fn partial_cmp(&self, other: &PhysicalAddress)
         -> $crate::option::Option<$crate::cmp::Ordering> {
            match *other {
                PhysicalAddress(ref __self_1_0) =>
                match *self {
                    PhysicalAddress(ref __self_0_0) =>
                    match $crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                               &(*__self_1_0))
                        {
                        $crate::option::Option::Some($crate::cmp::Ordering::Equal)
                        =>
                        $crate::option::Option::Some($crate::cmp::Ordering::Equal),
                        cmp => cmp,
                    },
                },
            }
        }
        #[inline]
        fn lt(&self, other: &PhysicalAddress) -> bool {
            match *other {
                PhysicalAddress(ref __self_1_0) =>
                match *self {
                    PhysicalAddress(ref __self_0_0) =>
                    $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                           &(*__self_1_0)),
                                                      $crate::cmp::Ordering::Greater)
                        == $crate::cmp::Ordering::Less,
                },
            }
        }
        #[inline]
        fn le(&self, other: &PhysicalAddress) -> bool {
            match *other {
                PhysicalAddress(ref __self_1_0) =>
                match *self {
                    PhysicalAddress(ref __self_0_0) =>
                    $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                           &(*__self_1_0)),
                                                      $crate::cmp::Ordering::Greater)
                        != $crate::cmp::Ordering::Greater,
                },
            }
        }
        #[inline]
        fn gt(&self, other: &PhysicalAddress) -> bool {
            match *other {
                PhysicalAddress(ref __self_1_0) =>
                match *self {
                    PhysicalAddress(ref __self_0_0) =>
                    $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                           &(*__self_1_0)),
                                                      $crate::cmp::Ordering::Less)
                        == $crate::cmp::Ordering::Greater,
                },
            }
        }
        #[inline]
        fn ge(&self, other: &PhysicalAddress) -> bool {
            match *other {
                PhysicalAddress(ref __self_1_0) =>
                match *self {
                    PhysicalAddress(ref __self_0_0) =>
                    $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                           &(*__self_1_0)),
                                                      $crate::cmp::Ordering::Less)
                        != $crate::cmp::Ordering::Less,
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::cmp::Eq for PhysicalAddress {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            { let _: $crate::cmp::AssertParamIsEq<u64>; }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::cmp::PartialEq for PhysicalAddress {
        #[inline]
        fn eq(&self, other: &PhysicalAddress) -> bool {
            match *other {
                PhysicalAddress(ref __self_1_0) =>
                match *self {
                    PhysicalAddress(ref __self_0_0) =>
                    (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &PhysicalAddress) -> bool {
            match *other {
                PhysicalAddress(ref __self_1_0) =>
                match *self {
                    PhysicalAddress(ref __self_0_0) =>
                    (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::hash::Hash for PhysicalAddress {
        fn hash<__H: $crate::hash::Hasher>(&self, state: &mut __H) -> () {
            match *self {
                PhysicalAddress(ref __self_0_0) => {
                    $crate::hash::Hash::hash(&(*__self_0_0), state)
                }
            }
        }
    }
    impl Into<u64> for PhysicalAddress {
        #[inline(always)]
        fn into(self) -> u64 { self.0 }
    }
    impl Add<u64> for PhysicalAddress {
        type
        Output
        =
        Self;
        #[inline(always)]
        fn add(self, rhs: u64) -> Self::Output {
            PhysicalAddress(self.0 + rhs)
        }
    }
    impl AddAssign<u64> for PhysicalAddress {
        #[inline(always)]
        fn add_assign(&mut self, rhs: u64) { self.0 += rhs }
    }
    impl Add<usize> for PhysicalAddress {
        type
        Output
        =
        Self;
        #[inline(always)]
        fn add(self, rhs: usize) -> Self::Output {
            PhysicalAddress(self.0 + (rhs as u64))
        }
    }
    impl AddAssign<usize> for PhysicalAddress {
        #[inline(always)]
        fn add_assign(&mut self, rhs: usize) { self.0 += rhs as u64 }
    }
    impl PhysicalAddress {
        /// Relative offset from the start of the system page containing this physical address.
        ///
        /// May be zero, but will always be less than the system page size.
        #[inline(always)]
        pub fn offset_from_start_of_page(self) -> u64 {
            self.0 % (page_size() as u64)
        }
        /// The address of the page which contains this physical address.
        /// May be the same value.
        #[inline(always)]
        pub fn first_address_in_page(self) -> Self {
            PhysicalAddress(self.0 & !((page_size() as u64) - 1))
        }
    }
    /// Physical Page Frame Number (PFN).
    #[structural_match]
    #[rustc_copy_clone_marker]
    pub struct PhysicalPageFrameNumber(pub(crate) u64);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::default::Default for PhysicalPageFrameNumber {
        #[inline]
        fn default() -> PhysicalPageFrameNumber {
            PhysicalPageFrameNumber($crate::default::Default::default())
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::fmt::Debug for PhysicalPageFrameNumber {
        fn fmt(&self, f: &mut $crate::fmt::Formatter) -> $crate::fmt::Result {
            match *self {
                PhysicalPageFrameNumber(ref __self_0_0) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("PhysicalPageFrameNumber");
                    let _ = debug_trait_builder.field(&&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::marker::Copy for PhysicalPageFrameNumber { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::clone::Clone for PhysicalPageFrameNumber {
        #[inline]
        fn clone(&self) -> PhysicalPageFrameNumber {
            { let _: $crate::clone::AssertParamIsClone<u64>; *self }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::cmp::Ord for PhysicalPageFrameNumber {
        #[inline]
        fn cmp(&self, other: &PhysicalPageFrameNumber)
         -> $crate::cmp::Ordering {
            match *other {
                PhysicalPageFrameNumber(ref __self_1_0) =>
                match *self {
                    PhysicalPageFrameNumber(ref __self_0_0) =>
                    match $crate::cmp::Ord::cmp(&(*__self_0_0),
                                                &(*__self_1_0)) {
                        $crate::cmp::Ordering::Equal =>
                        $crate::cmp::Ordering::Equal,
                        cmp => cmp,
                    },
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::cmp::PartialOrd for PhysicalPageFrameNumber {
        #[inline]
        fn partial_cmp(&self, other: &PhysicalPageFrameNumber)
         -> $crate::option::Option<$crate::cmp::Ordering> {
            match *other {
                PhysicalPageFrameNumber(ref __self_1_0) =>
                match *self {
                    PhysicalPageFrameNumber(ref __self_0_0) =>
                    match $crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                               &(*__self_1_0))
                        {
                        $crate::option::Option::Some($crate::cmp::Ordering::Equal)
                        =>
                        $crate::option::Option::Some($crate::cmp::Ordering::Equal),
                        cmp => cmp,
                    },
                },
            }
        }
        #[inline]
        fn lt(&self, other: &PhysicalPageFrameNumber) -> bool {
            match *other {
                PhysicalPageFrameNumber(ref __self_1_0) =>
                match *self {
                    PhysicalPageFrameNumber(ref __self_0_0) =>
                    $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                           &(*__self_1_0)),
                                                      $crate::cmp::Ordering::Greater)
                        == $crate::cmp::Ordering::Less,
                },
            }
        }
        #[inline]
        fn le(&self, other: &PhysicalPageFrameNumber) -> bool {
            match *other {
                PhysicalPageFrameNumber(ref __self_1_0) =>
                match *self {
                    PhysicalPageFrameNumber(ref __self_0_0) =>
                    $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                           &(*__self_1_0)),
                                                      $crate::cmp::Ordering::Greater)
                        != $crate::cmp::Ordering::Greater,
                },
            }
        }
        #[inline]
        fn gt(&self, other: &PhysicalPageFrameNumber) -> bool {
            match *other {
                PhysicalPageFrameNumber(ref __self_1_0) =>
                match *self {
                    PhysicalPageFrameNumber(ref __self_0_0) =>
                    $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                           &(*__self_1_0)),
                                                      $crate::cmp::Ordering::Less)
                        == $crate::cmp::Ordering::Greater,
                },
            }
        }
        #[inline]
        fn ge(&self, other: &PhysicalPageFrameNumber) -> bool {
            match *other {
                PhysicalPageFrameNumber(ref __self_1_0) =>
                match *self {
                    PhysicalPageFrameNumber(ref __self_0_0) =>
                    $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                           &(*__self_1_0)),
                                                      $crate::cmp::Ordering::Less)
                        != $crate::cmp::Ordering::Less,
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::cmp::Eq for PhysicalPageFrameNumber {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            { let _: $crate::cmp::AssertParamIsEq<u64>; }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::cmp::PartialEq for PhysicalPageFrameNumber {
        #[inline]
        fn eq(&self, other: &PhysicalPageFrameNumber) -> bool {
            match *other {
                PhysicalPageFrameNumber(ref __self_1_0) =>
                match *self {
                    PhysicalPageFrameNumber(ref __self_0_0) =>
                    (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &PhysicalPageFrameNumber) -> bool {
            match *other {
                PhysicalPageFrameNumber(ref __self_1_0) =>
                match *self {
                    PhysicalPageFrameNumber(ref __self_0_0) =>
                    (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::hash::Hash for PhysicalPageFrameNumber {
        fn hash<__H: $crate::hash::Hasher>(&self, state: &mut __H) -> () {
            match *self {
                PhysicalPageFrameNumber(ref __self_0_0) => {
                    $crate::hash::Hash::hash(&(*__self_0_0), state)
                }
            }
        }
    }
    impl Into<u64> for PhysicalPageFrameNumber {
        #[inline(always)]
        fn into(self) -> u64 { self.0 }
    }
    impl Into<PhysicalAddress> for PhysicalPageFrameNumber {
        #[inline(always)]
        fn into(self) -> PhysicalAddress {
            PhysicalAddress(self.0 * (page_size() as u64))
        }
    }
    /// A virtual address.
    ///
    /// This is the same as the value returned from `malloc()`, a `*mut T` pointer, a `&T` reference, etc.
    ///
    /// No checks are made for its validity.
    #[structural_match]
    #[rustc_copy_clone_marker]
    pub struct VirtualAddress(usize);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::default::Default for VirtualAddress {
        #[inline]
        fn default() -> VirtualAddress {
            VirtualAddress($crate::default::Default::default())
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::fmt::Debug for VirtualAddress {
        fn fmt(&self, f: &mut $crate::fmt::Formatter) -> $crate::fmt::Result {
            match *self {
                VirtualAddress(ref __self_0_0) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("VirtualAddress");
                    let _ = debug_trait_builder.field(&&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::marker::Copy for VirtualAddress { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::clone::Clone for VirtualAddress {
        #[inline]
        fn clone(&self) -> VirtualAddress {
            { let _: $crate::clone::AssertParamIsClone<usize>; *self }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::cmp::Ord for VirtualAddress {
        #[inline]
        fn cmp(&self, other: &VirtualAddress) -> $crate::cmp::Ordering {
            match *other {
                VirtualAddress(ref __self_1_0) =>
                match *self {
                    VirtualAddress(ref __self_0_0) =>
                    match $crate::cmp::Ord::cmp(&(*__self_0_0),
                                                &(*__self_1_0)) {
                        $crate::cmp::Ordering::Equal =>
                        $crate::cmp::Ordering::Equal,
                        cmp => cmp,
                    },
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::cmp::PartialOrd for VirtualAddress {
        #[inline]
        fn partial_cmp(&self, other: &VirtualAddress)
         -> $crate::option::Option<$crate::cmp::Ordering> {
            match *other {
                VirtualAddress(ref __self_1_0) =>
                match *self {
                    VirtualAddress(ref __self_0_0) =>
                    match $crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                               &(*__self_1_0))
                        {
                        $crate::option::Option::Some($crate::cmp::Ordering::Equal)
                        =>
                        $crate::option::Option::Some($crate::cmp::Ordering::Equal),
                        cmp => cmp,
                    },
                },
            }
        }
        #[inline]
        fn lt(&self, other: &VirtualAddress) -> bool {
            match *other {
                VirtualAddress(ref __self_1_0) =>
                match *self {
                    VirtualAddress(ref __self_0_0) =>
                    $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                           &(*__self_1_0)),
                                                      $crate::cmp::Ordering::Greater)
                        == $crate::cmp::Ordering::Less,
                },
            }
        }
        #[inline]
        fn le(&self, other: &VirtualAddress) -> bool {
            match *other {
                VirtualAddress(ref __self_1_0) =>
                match *self {
                    VirtualAddress(ref __self_0_0) =>
                    $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                           &(*__self_1_0)),
                                                      $crate::cmp::Ordering::Greater)
                        != $crate::cmp::Ordering::Greater,
                },
            }
        }
        #[inline]
        fn gt(&self, other: &VirtualAddress) -> bool {
            match *other {
                VirtualAddress(ref __self_1_0) =>
                match *self {
                    VirtualAddress(ref __self_0_0) =>
                    $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                           &(*__self_1_0)),
                                                      $crate::cmp::Ordering::Less)
                        == $crate::cmp::Ordering::Greater,
                },
            }
        }
        #[inline]
        fn ge(&self, other: &VirtualAddress) -> bool {
            match *other {
                VirtualAddress(ref __self_1_0) =>
                match *self {
                    VirtualAddress(ref __self_0_0) =>
                    $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                           &(*__self_1_0)),
                                                      $crate::cmp::Ordering::Less)
                        != $crate::cmp::Ordering::Less,
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::cmp::Eq for VirtualAddress {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            { let _: $crate::cmp::AssertParamIsEq<usize>; }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::cmp::PartialEq for VirtualAddress {
        #[inline]
        fn eq(&self, other: &VirtualAddress) -> bool {
            match *other {
                VirtualAddress(ref __self_1_0) =>
                match *self {
                    VirtualAddress(ref __self_0_0) =>
                    (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &VirtualAddress) -> bool {
            match *other {
                VirtualAddress(ref __self_1_0) =>
                match *self {
                    VirtualAddress(ref __self_0_0) =>
                    (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::hash::Hash for VirtualAddress {
        fn hash<__H: $crate::hash::Hasher>(&self, state: &mut __H) -> () {
            match *self {
                VirtualAddress(ref __self_0_0) => {
                    $crate::hash::Hash::hash(&(*__self_0_0), state)
                }
            }
        }
    }
    impl <T> From<NonNull<T>> for VirtualAddress {
        #[inline(always)]
        fn from(value: NonNull<T>) -> Self {
            VirtualAddress(value.as_ptr() as usize)
        }
    }
    impl <T> From<Option<NonNull<T>>> for VirtualAddress {
        #[inline(always)]
        fn from(value: Option<NonNull<T>>) -> Self {
            let address =
                match value {
                    None => 0,
                    Some(value) => value.as_ptr() as usize,
                };
            VirtualAddress(address)
        }
    }
    impl <T> Into<Option<NonNull<T>>> for VirtualAddress {
        #[inline(always)]
        fn into(self) -> Option<NonNull<T>> { NonNull::new(self.0 as *mut T) }
    }
    impl <'a, T: 'a> From<&'a T> for VirtualAddress {
        #[inline(always)]
        fn from(value: &'a T) -> Self {
            VirtualAddress(value as *const T as usize)
        }
    }
    impl <'a, T: 'a> From<&'a mut T> for VirtualAddress {
        #[inline(always)]
        fn from(value: &'a mut T) -> Self {
            VirtualAddress(value as *mut T as usize)
        }
    }
    impl <T> From<*const T> for VirtualAddress {
        #[inline(always)]
        fn from(value: *const T) -> Self { VirtualAddress(value as usize) }
    }
    impl <T> Into<*const T> for VirtualAddress {
        #[inline(always)]
        fn into(self) -> *const T { self.0 as *const T }
    }
    impl <T> From<*mut T> for VirtualAddress {
        #[inline(always)]
        fn from(value: *mut T) -> Self { VirtualAddress(value as usize) }
    }
    impl <T> Into<*mut T> for VirtualAddress {
        #[inline(always)]
        fn into(self) -> *mut T { self.0 as *mut T }
    }
    impl From<usize> for VirtualAddress {
        #[inline(always)]
        fn from(value: usize) -> Self { VirtualAddress(value) }
    }
    impl Into<usize> for VirtualAddress {
        #[inline(always)]
        fn into(self) -> usize { self.0 }
    }
    impl Add<usize> for VirtualAddress {
        type
        Output
        =
        Self;
        #[inline(always)]
        fn add(self, rhs: usize) -> Self::Output {
            VirtualAddress(self.0 + rhs)
        }
    }
    impl AddAssign<usize> for VirtualAddress {
        #[inline(always)]
        fn add_assign(&mut self, rhs: usize) { self.0 += rhs }
    }
    impl Sub<usize> for VirtualAddress {
        type
        Output
        =
        Self;
        #[inline(always)]
        fn sub(self, rhs: usize) -> Self::Output {
            VirtualAddress(self.0 - rhs)
        }
    }
    impl SubAssign<usize> for VirtualAddress {
        #[inline(always)]
        fn sub_assign(&mut self, rhs: usize) { self.0 -= rhs }
    }
    impl VirtualAddress {
        /// Relative offset from the start of the system page containing this virtual address.
        ///
        /// May be zero, but will always be less than the system page size.
        #[inline(always)]
        pub fn offset_from_start_of_page(self) -> usize {
            self.0 % page_size()
        }
        /// The address of the page which contains this physical address.
        /// May be the same value.
        #[inline(always)]
        pub fn first_address_in_page(self) -> Self {
            VirtualAddress(self.0 & !(page_size() - 1))
        }
    }
    /// A Virtual Page Frame Number (PFN).
    #[structural_match]
    #[rustc_copy_clone_marker]
    pub struct VirtualPageFrameNumber(usize);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::default::Default for VirtualPageFrameNumber {
        #[inline]
        fn default() -> VirtualPageFrameNumber {
            VirtualPageFrameNumber($crate::default::Default::default())
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::fmt::Debug for VirtualPageFrameNumber {
        fn fmt(&self, f: &mut $crate::fmt::Formatter) -> $crate::fmt::Result {
            match *self {
                VirtualPageFrameNumber(ref __self_0_0) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("VirtualPageFrameNumber");
                    let _ = debug_trait_builder.field(&&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::marker::Copy for VirtualPageFrameNumber { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::clone::Clone for VirtualPageFrameNumber {
        #[inline]
        fn clone(&self) -> VirtualPageFrameNumber {
            { let _: $crate::clone::AssertParamIsClone<usize>; *self }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::cmp::Ord for VirtualPageFrameNumber {
        #[inline]
        fn cmp(&self, other: &VirtualPageFrameNumber)
         -> $crate::cmp::Ordering {
            match *other {
                VirtualPageFrameNumber(ref __self_1_0) =>
                match *self {
                    VirtualPageFrameNumber(ref __self_0_0) =>
                    match $crate::cmp::Ord::cmp(&(*__self_0_0),
                                                &(*__self_1_0)) {
                        $crate::cmp::Ordering::Equal =>
                        $crate::cmp::Ordering::Equal,
                        cmp => cmp,
                    },
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::cmp::PartialOrd for VirtualPageFrameNumber {
        #[inline]
        fn partial_cmp(&self, other: &VirtualPageFrameNumber)
         -> $crate::option::Option<$crate::cmp::Ordering> {
            match *other {
                VirtualPageFrameNumber(ref __self_1_0) =>
                match *self {
                    VirtualPageFrameNumber(ref __self_0_0) =>
                    match $crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                               &(*__self_1_0))
                        {
                        $crate::option::Option::Some($crate::cmp::Ordering::Equal)
                        =>
                        $crate::option::Option::Some($crate::cmp::Ordering::Equal),
                        cmp => cmp,
                    },
                },
            }
        }
        #[inline]
        fn lt(&self, other: &VirtualPageFrameNumber) -> bool {
            match *other {
                VirtualPageFrameNumber(ref __self_1_0) =>
                match *self {
                    VirtualPageFrameNumber(ref __self_0_0) =>
                    $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                           &(*__self_1_0)),
                                                      $crate::cmp::Ordering::Greater)
                        == $crate::cmp::Ordering::Less,
                },
            }
        }
        #[inline]
        fn le(&self, other: &VirtualPageFrameNumber) -> bool {
            match *other {
                VirtualPageFrameNumber(ref __self_1_0) =>
                match *self {
                    VirtualPageFrameNumber(ref __self_0_0) =>
                    $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                           &(*__self_1_0)),
                                                      $crate::cmp::Ordering::Greater)
                        != $crate::cmp::Ordering::Greater,
                },
            }
        }
        #[inline]
        fn gt(&self, other: &VirtualPageFrameNumber) -> bool {
            match *other {
                VirtualPageFrameNumber(ref __self_1_0) =>
                match *self {
                    VirtualPageFrameNumber(ref __self_0_0) =>
                    $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                           &(*__self_1_0)),
                                                      $crate::cmp::Ordering::Less)
                        == $crate::cmp::Ordering::Greater,
                },
            }
        }
        #[inline]
        fn ge(&self, other: &VirtualPageFrameNumber) -> bool {
            match *other {
                VirtualPageFrameNumber(ref __self_1_0) =>
                match *self {
                    VirtualPageFrameNumber(ref __self_0_0) =>
                    $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                           &(*__self_1_0)),
                                                      $crate::cmp::Ordering::Less)
                        != $crate::cmp::Ordering::Less,
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::cmp::Eq for VirtualPageFrameNumber {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            { let _: $crate::cmp::AssertParamIsEq<usize>; }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::cmp::PartialEq for VirtualPageFrameNumber {
        #[inline]
        fn eq(&self, other: &VirtualPageFrameNumber) -> bool {
            match *other {
                VirtualPageFrameNumber(ref __self_1_0) =>
                match *self {
                    VirtualPageFrameNumber(ref __self_0_0) =>
                    (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &VirtualPageFrameNumber) -> bool {
            match *other {
                VirtualPageFrameNumber(ref __self_1_0) =>
                match *self {
                    VirtualPageFrameNumber(ref __self_0_0) =>
                    (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::hash::Hash for VirtualPageFrameNumber {
        fn hash<__H: $crate::hash::Hasher>(&self, state: &mut __H) -> () {
            match *self {
                VirtualPageFrameNumber(ref __self_0_0) => {
                    $crate::hash::Hash::hash(&(*__self_0_0), state)
                }
            }
        }
    }
    impl From<VirtualAddress> for VirtualPageFrameNumber {
        #[inline(always)]
        fn from(value: VirtualAddress) -> Self {
            let into: usize = value.into();
            VirtualPageFrameNumber(into / page_size())
        }
    }
    impl Into<usize> for VirtualPageFrameNumber {
        #[inline(always)]
        fn into(self) -> usize { self.0 }
    }
    impl Into<u64> for VirtualPageFrameNumber {
        #[inline(always)]
        fn into(self) -> u64 { self.0 as u64 }
    }
}
/// Process status.
pub mod process_status {
    use super::*;
    /// A bitmask.
    pub type Bitmask = u64;
    /// A kilobyte.
    pub type Kilobyte = u64;
    /// Group identifiers (GIDs).
    #[structural_match]
    pub struct ProcessGroupIdentifiers {
        /// Real group identifier (GID).
        pub real: uid_t,
        /// Effective group identifier (GID).
        pub effective: uid_t,
        /// Saved set group identifier (GID).
        pub saved_set: uid_t,
        /// File system group identifier (GID).
        pub file_system: uid_t,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::default::Default for ProcessGroupIdentifiers {
        #[inline]
        fn default() -> ProcessGroupIdentifiers {
            ProcessGroupIdentifiers{real: $crate::default::Default::default(),
                                    effective:
                                        $crate::default::Default::default(),
                                    saved_set:
                                        $crate::default::Default::default(),
                                    file_system:
                                        $crate::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::fmt::Debug for ProcessGroupIdentifiers {
        fn fmt(&self, f: &mut $crate::fmt::Formatter) -> $crate::fmt::Result {
            match *self {
                ProcessGroupIdentifiers {
                real: ref __self_0_0,
                effective: ref __self_0_1,
                saved_set: ref __self_0_2,
                file_system: ref __self_0_3 } => {
                    let mut debug_trait_builder =
                        f.debug_struct("ProcessGroupIdentifiers");
                    let _ =
                        debug_trait_builder.field("real", &&(*__self_0_0));
                    let _ =
                        debug_trait_builder.field("effective",
                                                  &&(*__self_0_1));
                    let _ =
                        debug_trait_builder.field("saved_set",
                                                  &&(*__self_0_2));
                    let _ =
                        debug_trait_builder.field("file_system",
                                                  &&(*__self_0_3));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::clone::Clone for ProcessGroupIdentifiers {
        #[inline]
        fn clone(&self) -> ProcessGroupIdentifiers {
            match *self {
                ProcessGroupIdentifiers {
                real: ref __self_0_0,
                effective: ref __self_0_1,
                saved_set: ref __self_0_2,
                file_system: ref __self_0_3 } =>
                ProcessGroupIdentifiers{real:
                                            $crate::clone::Clone::clone(&(*__self_0_0)),
                                        effective:
                                            $crate::clone::Clone::clone(&(*__self_0_1)),
                                        saved_set:
                                            $crate::clone::Clone::clone(&(*__self_0_2)),
                                        file_system:
                                            $crate::clone::Clone::clone(&(*__self_0_3)),},
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::cmp::PartialEq for ProcessGroupIdentifiers {
        #[inline]
        fn eq(&self, other: &ProcessGroupIdentifiers) -> bool {
            match *other {
                ProcessGroupIdentifiers {
                real: ref __self_1_0,
                effective: ref __self_1_1,
                saved_set: ref __self_1_2,
                file_system: ref __self_1_3 } =>
                match *self {
                    ProcessGroupIdentifiers {
                    real: ref __self_0_0,
                    effective: ref __self_0_1,
                    saved_set: ref __self_0_2,
                    file_system: ref __self_0_3 } =>
                    (*__self_0_0) == (*__self_1_0) &&
                        (*__self_0_1) == (*__self_1_1) &&
                        (*__self_0_2) == (*__self_1_2) &&
                        (*__self_0_3) == (*__self_1_3),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &ProcessGroupIdentifiers) -> bool {
            match *other {
                ProcessGroupIdentifiers {
                real: ref __self_1_0,
                effective: ref __self_1_1,
                saved_set: ref __self_1_2,
                file_system: ref __self_1_3 } =>
                match *self {
                    ProcessGroupIdentifiers {
                    real: ref __self_0_0,
                    effective: ref __self_0_1,
                    saved_set: ref __self_0_2,
                    file_system: ref __self_0_3 } =>
                    (*__self_0_0) != (*__self_1_0) ||
                        (*__self_0_1) != (*__self_1_1) ||
                        (*__self_0_2) != (*__self_1_2) ||
                        (*__self_0_3) != (*__self_1_3),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::cmp::Eq for ProcessGroupIdentifiers {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: $crate::cmp::AssertParamIsEq<uid_t>;
                let _: $crate::cmp::AssertParamIsEq<uid_t>;
                let _: $crate::cmp::AssertParamIsEq<uid_t>;
                let _: $crate::cmp::AssertParamIsEq<uid_t>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::cmp::PartialOrd for ProcessGroupIdentifiers {
        #[inline]
        fn partial_cmp(&self, other: &ProcessGroupIdentifiers)
         -> $crate::option::Option<$crate::cmp::Ordering> {
            match *other {
                ProcessGroupIdentifiers {
                real: ref __self_1_0,
                effective: ref __self_1_1,
                saved_set: ref __self_1_2,
                file_system: ref __self_1_3 } =>
                match *self {
                    ProcessGroupIdentifiers {
                    real: ref __self_0_0,
                    effective: ref __self_0_1,
                    saved_set: ref __self_0_2,
                    file_system: ref __self_0_3 } =>
                    match $crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                               &(*__self_1_0))
                        {
                        $crate::option::Option::Some($crate::cmp::Ordering::Equal)
                        =>
                        match $crate::cmp::PartialOrd::partial_cmp(&(*__self_0_1),
                                                                   &(*__self_1_1))
                            {
                            $crate::option::Option::Some($crate::cmp::Ordering::Equal)
                            =>
                            match $crate::cmp::PartialOrd::partial_cmp(&(*__self_0_2),
                                                                       &(*__self_1_2))
                                {
                                $crate::option::Option::Some($crate::cmp::Ordering::Equal)
                                =>
                                match $crate::cmp::PartialOrd::partial_cmp(&(*__self_0_3),
                                                                           &(*__self_1_3))
                                    {
                                    $crate::option::Option::Some($crate::cmp::Ordering::Equal)
                                    =>
                                    $crate::option::Option::Some($crate::cmp::Ordering::Equal),
                                    cmp => cmp,
                                },
                                cmp => cmp,
                            },
                            cmp => cmp,
                        },
                        cmp => cmp,
                    },
                },
            }
        }
        #[inline]
        fn lt(&self, other: &ProcessGroupIdentifiers) -> bool {
            match *other {
                ProcessGroupIdentifiers {
                real: ref __self_1_0,
                effective: ref __self_1_1,
                saved_set: ref __self_1_2,
                file_system: ref __self_1_3 } =>
                match *self {
                    ProcessGroupIdentifiers {
                    real: ref __self_0_0,
                    effective: ref __self_0_1,
                    saved_set: ref __self_0_2,
                    file_system: ref __self_0_3 } =>
                    $crate::cmp::Ordering::then_with($crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                                                            &(*__self_1_0)),
                                                                                       $crate::cmp::Ordering::Equal),
                                                     ||
                                                         $crate::cmp::Ordering::then_with($crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_1),
                                                                                                                                                                 &(*__self_1_1)),
                                                                                                                            $crate::cmp::Ordering::Equal),
                                                                                          ||
                                                                                              $crate::cmp::Ordering::then_with($crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_2),
                                                                                                                                                                                                      &(*__self_1_2)),
                                                                                                                                                                 $crate::cmp::Ordering::Equal),
                                                                                                                               ||
                                                                                                                                   $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_3),
                                                                                                                                                                                                          &(*__self_1_3)),
                                                                                                                                                                     $crate::cmp::Ordering::Greater))))
                        == $crate::cmp::Ordering::Less,
                },
            }
        }
        #[inline]
        fn le(&self, other: &ProcessGroupIdentifiers) -> bool {
            match *other {
                ProcessGroupIdentifiers {
                real: ref __self_1_0,
                effective: ref __self_1_1,
                saved_set: ref __self_1_2,
                file_system: ref __self_1_3 } =>
                match *self {
                    ProcessGroupIdentifiers {
                    real: ref __self_0_0,
                    effective: ref __self_0_1,
                    saved_set: ref __self_0_2,
                    file_system: ref __self_0_3 } =>
                    $crate::cmp::Ordering::then_with($crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                                                            &(*__self_1_0)),
                                                                                       $crate::cmp::Ordering::Equal),
                                                     ||
                                                         $crate::cmp::Ordering::then_with($crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_1),
                                                                                                                                                                 &(*__self_1_1)),
                                                                                                                            $crate::cmp::Ordering::Equal),
                                                                                          ||
                                                                                              $crate::cmp::Ordering::then_with($crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_2),
                                                                                                                                                                                                      &(*__self_1_2)),
                                                                                                                                                                 $crate::cmp::Ordering::Equal),
                                                                                                                               ||
                                                                                                                                   $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_3),
                                                                                                                                                                                                          &(*__self_1_3)),
                                                                                                                                                                     $crate::cmp::Ordering::Greater))))
                        != $crate::cmp::Ordering::Greater,
                },
            }
        }
        #[inline]
        fn gt(&self, other: &ProcessGroupIdentifiers) -> bool {
            match *other {
                ProcessGroupIdentifiers {
                real: ref __self_1_0,
                effective: ref __self_1_1,
                saved_set: ref __self_1_2,
                file_system: ref __self_1_3 } =>
                match *self {
                    ProcessGroupIdentifiers {
                    real: ref __self_0_0,
                    effective: ref __self_0_1,
                    saved_set: ref __self_0_2,
                    file_system: ref __self_0_3 } =>
                    $crate::cmp::Ordering::then_with($crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                                                            &(*__self_1_0)),
                                                                                       $crate::cmp::Ordering::Equal),
                                                     ||
                                                         $crate::cmp::Ordering::then_with($crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_1),
                                                                                                                                                                 &(*__self_1_1)),
                                                                                                                            $crate::cmp::Ordering::Equal),
                                                                                          ||
                                                                                              $crate::cmp::Ordering::then_with($crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_2),
                                                                                                                                                                                                      &(*__self_1_2)),
                                                                                                                                                                 $crate::cmp::Ordering::Equal),
                                                                                                                               ||
                                                                                                                                   $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_3),
                                                                                                                                                                                                          &(*__self_1_3)),
                                                                                                                                                                     $crate::cmp::Ordering::Less))))
                        == $crate::cmp::Ordering::Greater,
                },
            }
        }
        #[inline]
        fn ge(&self, other: &ProcessGroupIdentifiers) -> bool {
            match *other {
                ProcessGroupIdentifiers {
                real: ref __self_1_0,
                effective: ref __self_1_1,
                saved_set: ref __self_1_2,
                file_system: ref __self_1_3 } =>
                match *self {
                    ProcessGroupIdentifiers {
                    real: ref __self_0_0,
                    effective: ref __self_0_1,
                    saved_set: ref __self_0_2,
                    file_system: ref __self_0_3 } =>
                    $crate::cmp::Ordering::then_with($crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                                                            &(*__self_1_0)),
                                                                                       $crate::cmp::Ordering::Equal),
                                                     ||
                                                         $crate::cmp::Ordering::then_with($crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_1),
                                                                                                                                                                 &(*__self_1_1)),
                                                                                                                            $crate::cmp::Ordering::Equal),
                                                                                          ||
                                                                                              $crate::cmp::Ordering::then_with($crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_2),
                                                                                                                                                                                                      &(*__self_1_2)),
                                                                                                                                                                 $crate::cmp::Ordering::Equal),
                                                                                                                               ||
                                                                                                                                   $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_3),
                                                                                                                                                                                                          &(*__self_1_3)),
                                                                                                                                                                     $crate::cmp::Ordering::Less))))
                        != $crate::cmp::Ordering::Less,
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::cmp::Ord for ProcessGroupIdentifiers {
        #[inline]
        fn cmp(&self, other: &ProcessGroupIdentifiers)
         -> $crate::cmp::Ordering {
            match *other {
                ProcessGroupIdentifiers {
                real: ref __self_1_0,
                effective: ref __self_1_1,
                saved_set: ref __self_1_2,
                file_system: ref __self_1_3 } =>
                match *self {
                    ProcessGroupIdentifiers {
                    real: ref __self_0_0,
                    effective: ref __self_0_1,
                    saved_set: ref __self_0_2,
                    file_system: ref __self_0_3 } =>
                    match $crate::cmp::Ord::cmp(&(*__self_0_0),
                                                &(*__self_1_0)) {
                        $crate::cmp::Ordering::Equal =>
                        match $crate::cmp::Ord::cmp(&(*__self_0_1),
                                                    &(*__self_1_1)) {
                            $crate::cmp::Ordering::Equal =>
                            match $crate::cmp::Ord::cmp(&(*__self_0_2),
                                                        &(*__self_1_2)) {
                                $crate::cmp::Ordering::Equal =>
                                match $crate::cmp::Ord::cmp(&(*__self_0_3),
                                                            &(*__self_1_3)) {
                                    $crate::cmp::Ordering::Equal =>
                                    $crate::cmp::Ordering::Equal,
                                    cmp => cmp,
                                },
                                cmp => cmp,
                            },
                            cmp => cmp,
                        },
                        cmp => cmp,
                    },
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::hash::Hash for ProcessGroupIdentifiers {
        fn hash<__H: $crate::hash::Hasher>(&self, state: &mut __H) -> () {
            match *self {
                ProcessGroupIdentifiers {
                real: ref __self_0_0,
                effective: ref __self_0_1,
                saved_set: ref __self_0_2,
                file_system: ref __self_0_3 } => {
                    $crate::hash::Hash::hash(&(*__self_0_0), state);
                    $crate::hash::Hash::hash(&(*__self_0_1), state);
                    $crate::hash::Hash::hash(&(*__self_0_2), state);
                    $crate::hash::Hash::hash(&(*__self_0_3), state)
                }
            }
        }
    }
    /// Process state.
    #[structural_match]
    #[rustc_copy_clone_marker]
    pub enum ProcessState {

        /// Also known as `R (running)`.
        Running,

        /// Also known as `S (sleeping)`.
        Sleeping,

        /// Also known as `D (disk sleep)`, or disk sleep.
        SleepingInAnUninterruptibleWait,

        /// Also known as `T (stopped)`.
        TracedOrStopped,

        /// Also known as `t (tracing stop)`.
        TracingStop,

        /// Also known as `X (dead)`.
        Dead,

        /// Also known as `Z (zombie)`.
        Zombie,

        /// Also known as `P (parked)`.
        Parked,

        /// Also known as `I (idle)`.
        Idle,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::fmt::Debug for ProcessState {
        fn fmt(&self, f: &mut $crate::fmt::Formatter) -> $crate::fmt::Result {
            match (&*self,) {
                (&ProcessState::Running,) => {
                    let mut debug_trait_builder = f.debug_tuple("Running");
                    debug_trait_builder.finish()
                }
                (&ProcessState::Sleeping,) => {
                    let mut debug_trait_builder = f.debug_tuple("Sleeping");
                    debug_trait_builder.finish()
                }
                (&ProcessState::SleepingInAnUninterruptibleWait,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("SleepingInAnUninterruptibleWait");
                    debug_trait_builder.finish()
                }
                (&ProcessState::TracedOrStopped,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("TracedOrStopped");
                    debug_trait_builder.finish()
                }
                (&ProcessState::TracingStop,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("TracingStop");
                    debug_trait_builder.finish()
                }
                (&ProcessState::Dead,) => {
                    let mut debug_trait_builder = f.debug_tuple("Dead");
                    debug_trait_builder.finish()
                }
                (&ProcessState::Zombie,) => {
                    let mut debug_trait_builder = f.debug_tuple("Zombie");
                    debug_trait_builder.finish()
                }
                (&ProcessState::Parked,) => {
                    let mut debug_trait_builder = f.debug_tuple("Parked");
                    debug_trait_builder.finish()
                }
                (&ProcessState::Idle,) => {
                    let mut debug_trait_builder = f.debug_tuple("Idle");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::marker::Copy for ProcessState { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::clone::Clone for ProcessState {
        #[inline]
        fn clone(&self) -> ProcessState { { *self } }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::cmp::PartialEq for ProcessState {
        #[inline]
        fn eq(&self, other: &ProcessState) -> bool {
            {
                let __self_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) { _ => true, }
                } else { false }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::cmp::Eq for ProcessState {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () { { } }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::cmp::PartialOrd for ProcessState {
        #[inline]
        fn partial_cmp(&self, other: &ProcessState)
         -> $crate::option::Option<$crate::cmp::Ordering> {
            {
                let __self_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        _ =>
                        $crate::option::Option::Some($crate::cmp::Ordering::Equal),
                    }
                } else { __self_vi.partial_cmp(&__arg_1_vi) }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::cmp::Ord for ProcessState {
        #[inline]
        fn cmp(&self, other: &ProcessState) -> $crate::cmp::Ordering {
            {
                let __self_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        _ => $crate::cmp::Ordering::Equal,
                    }
                } else { __self_vi.cmp(&__arg_1_vi) }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::hash::Hash for ProcessState {
        fn hash<__H: $crate::hash::Hasher>(&self, state: &mut __H) -> () {
            match (&*self,) {
                _ => {
                    $crate::hash::Hash::hash(&unsafe {
                                                  $crate::intrinsics::discriminant_value(self)
                                              }, state)
                }
            }
        }
    }
    /// Parsing of a process status file failed.
    pub enum ProcessStatusFileParseError {

        /// Could not open a file.
        CouldNotOpenFile(io::Error),

        /// Could not read a line of data.
        CouldNotReadLine {
            /// Zero-based line number.
            zero_based_line_number: usize,
            /// Cause.
            cause: io::Error,
        },

        /// Could not parse a line of data.
        CouldNotParseLine {
            /// Zero-based line number.
            zero_based_line_number: usize,
            /// Cause.
            cause: ProcessStatusStatisticParseError,
        },
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::fmt::Debug for ProcessStatusFileParseError {
        fn fmt(&self, f: &mut $crate::fmt::Formatter) -> $crate::fmt::Result {
            match (&*self,) {
                (&ProcessStatusFileParseError::CouldNotOpenFile(ref __self_0),)
                => {
                    let mut debug_trait_builder =
                        f.debug_tuple("CouldNotOpenFile");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&ProcessStatusFileParseError::CouldNotReadLine {
                 zero_based_line_number: ref __self_0, cause: ref __self_1 },)
                => {
                    let mut debug_trait_builder =
                        f.debug_struct("CouldNotReadLine");
                    let _ =
                        debug_trait_builder.field("zero_based_line_number",
                                                  &&(*__self_0));
                    let _ = debug_trait_builder.field("cause", &&(*__self_1));
                    debug_trait_builder.finish()
                }
                (&ProcessStatusFileParseError::CouldNotParseLine {
                 zero_based_line_number: ref __self_0, cause: ref __self_1 },)
                => {
                    let mut debug_trait_builder =
                        f.debug_struct("CouldNotParseLine");
                    let _ =
                        debug_trait_builder.field("zero_based_line_number",
                                                  &&(*__self_0));
                    let _ = debug_trait_builder.field("cause", &&(*__self_1));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl Display for ProcessStatusFileParseError {
        #[inline(always)]
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            <ProcessStatusFileParseError as Debug>::fmt(self, f)
        }
    }
    impl error::Error for ProcessStatusFileParseError {
        #[inline(always)]
        fn source(&self) -> Option<&(error::Error + 'static)> {
            use self::ProcessStatusFileParseError::*;
            match self {
                &CouldNotOpenFile(ref error) => Some(error),
                &CouldNotReadLine { ref cause, .. } => Some(cause),
                &CouldNotParseLine { ref cause, .. } => Some(cause),
            }
        }
    }
    impl From<io::Error> for ProcessStatusFileParseError {
        #[inline(always)]
        fn from(error: io::Error) -> Self {
            ProcessStatusFileParseError::CouldNotOpenFile(error)
        }
    }
    /// A parse error.
    pub enum ProcessStatusStatisticParseError {

        /// No value.
        NoValue,

        /// Value was not preceeded by a horizontal tab.
        ValueNotPreceededByHorizontalTab,

        /// Length was invalid.
        InvalidLength,

        /// Ending was invalid.
        InvalidEnding,

        /// Separator of components of value was invalid in some way; either not present, the wrong kind or too few or too many.
        InvalidSeparator,

        /// Value was out-of-range, eg `2` for a `bool`.
        OutOfRange,

        /// Statistic was present more than once.
        DuplicatedStatistic,

        /// Statistic value sub-set had a duplicated entry.
        DuplicatedStatisticValue,

        /// Value was not a valid UTF-8 string.
        NotAUtf8String(Utf8Error),

        /// Value was not a valid integer.
        NotAValidInteger(ParseIntError),

        /// Value was not a valid CPU or NUMA node list.
        NotAValidListOfCpusOrNumaNodes(ListParseError),
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::fmt::Debug for ProcessStatusStatisticParseError {
        fn fmt(&self, f: &mut $crate::fmt::Formatter) -> $crate::fmt::Result {
            match (&*self,) {
                (&ProcessStatusStatisticParseError::NoValue,) => {
                    let mut debug_trait_builder = f.debug_tuple("NoValue");
                    debug_trait_builder.finish()
                }
                (&ProcessStatusStatisticParseError::ValueNotPreceededByHorizontalTab,)
                => {
                    let mut debug_trait_builder =
                        f.debug_tuple("ValueNotPreceededByHorizontalTab");
                    debug_trait_builder.finish()
                }
                (&ProcessStatusStatisticParseError::InvalidLength,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("InvalidLength");
                    debug_trait_builder.finish()
                }
                (&ProcessStatusStatisticParseError::InvalidEnding,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("InvalidEnding");
                    debug_trait_builder.finish()
                }
                (&ProcessStatusStatisticParseError::InvalidSeparator,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("InvalidSeparator");
                    debug_trait_builder.finish()
                }
                (&ProcessStatusStatisticParseError::OutOfRange,) => {
                    let mut debug_trait_builder = f.debug_tuple("OutOfRange");
                    debug_trait_builder.finish()
                }
                (&ProcessStatusStatisticParseError::DuplicatedStatistic,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("DuplicatedStatistic");
                    debug_trait_builder.finish()
                }
                (&ProcessStatusStatisticParseError::DuplicatedStatisticValue,)
                => {
                    let mut debug_trait_builder =
                        f.debug_tuple("DuplicatedStatisticValue");
                    debug_trait_builder.finish()
                }
                (&ProcessStatusStatisticParseError::NotAUtf8String(ref __self_0),)
                => {
                    let mut debug_trait_builder =
                        f.debug_tuple("NotAUtf8String");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&ProcessStatusStatisticParseError::NotAValidInteger(ref __self_0),)
                => {
                    let mut debug_trait_builder =
                        f.debug_tuple("NotAValidInteger");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&ProcessStatusStatisticParseError::NotAValidListOfCpusOrNumaNodes(ref __self_0),)
                => {
                    let mut debug_trait_builder =
                        f.debug_tuple("NotAValidListOfCpusOrNumaNodes");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl Display for ProcessStatusStatisticParseError {
        #[inline(always)]
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            <ProcessStatusStatisticParseError as Debug>::fmt(self, f)
        }
    }
    impl error::Error for ProcessStatusStatisticParseError {
        #[inline(always)]
        fn source(&self) -> Option<&(error::Error + 'static)> {
            use self::ProcessStatusStatisticParseError::*;
            match self {
                &NoValue => None,
                &ValueNotPreceededByHorizontalTab => None,
                &InvalidLength => None,
                &InvalidEnding => None,
                &InvalidSeparator => None,
                &OutOfRange => None,
                &DuplicatedStatistic => None,
                &DuplicatedStatisticValue => None,
                &NotAUtf8String(ref error) => Some(error),
                &NotAValidInteger(ref error) => Some(error),
                &NotAValidListOfCpusOrNumaNodes(ref error) => Some(error),
            }
        }
    }
    impl From<Utf8Error> for ProcessStatusStatisticParseError {
        #[inline(always)]
        fn from(error: Utf8Error) -> Self {
            ProcessStatusStatisticParseError::NotAUtf8String(error)
        }
    }
    impl From<ParseIntError> for ProcessStatusStatisticParseError {
        #[inline(always)]
        fn from(error: ParseIntError) -> Self {
            ProcessStatusStatisticParseError::NotAValidInteger(error)
        }
    }
    impl From<ListParseError> for ProcessStatusStatisticParseError {
        #[inline(always)]
        fn from(error: ListParseError) -> Self {
            ProcessStatusStatisticParseError::NotAValidListOfCpusOrNumaNodes(error)
        }
    }
    /// Status statistics.
    #[structural_match]
    pub struct ProcessStatusStatistics {
        /// Process name.
        ///
        /// Known as `Name`.
        pub process_name: Option<Box<[u8]>>,
        /// File creation mode mask (`umask`).
        ///
        /// Known as `Umask`.
        pub file_mode_creation_mask: Option<mode_t>,
        /// State.
        ///
        /// Known as `State`.
        ///
        /// Note that <> ***does not*** document all possible states.
        pub state: Option<ProcessState>,
        /// Thread group identifier.
        ///
        /// Known as `Tgid`.
        pub thread_group_identifier: Option<pid_t>,
        /// NUMA group identifier.
        ///
        /// Known as `Ngid`.
        ///
        /// Zero if no NUMA is not supported.
        pub numa_group_identifier: Option<NumaNode>,
        /// Process identifier.
        ///
        /// Known as `Pid`.
        pub process_identifier: Option<pid_t>,
        /// Parent process identifier.
        ///
        /// Known as `PPid`.
        pub parent_process_identifier: Option<pid_t>,
        /// Usually zero, implying no tracer process.
        ///
        /// Known as `TracerPid`.
        pub tracer_process_identifier: Option<pid_t>,
        /// User identifiers.
        ///
        /// Known as `Uid`.
        pub user_identifiers: Option<ProcessUserIdentifiers>,
        /// Group identifiers.
        ///
        /// Known as `Gid`.
        pub group_identifiers: Option<ProcessGroupIdentifiers>,
        /// Number of file descriptor slots currently allocated.
        ///
        /// Known as `FDSize`.
        ///
        /// eg `64`.
        pub number_of_file_descriptor_slots_currently_allocated: Option<u64>,
        /// Other group memberships.
        ///
        /// Known as `Groups`.
        ///
        /// Seems to always contain at least one member, which is the same as the primary group of the user.
        pub groups: Option<BTreeSet<gid_t>>,
        /// Descendant namespace thread group identifiers.
        ///
        /// Known as `NStgid`.
        pub descendant_namespace_thread_group_identifier: Option<BTreeSet<pid_t>>,
        /// Descendant namespace process identifiers.
        ///
        /// Known as `NSpid`.
        pub descendant_namespace_process_identifier: Option<BTreeSet<pid_t>>,
        /// Descendant namespace process group identifiers.
        ///
        /// Known as `NSpgid`.
        pub descendant_namespace_process_group_identifier: Option<BTreeSet<pid_t>>,
        /// Descendant namespace session identifiers.
        ///
        /// Known as `NSsid`.
        pub descendant_namespace_session_identifier: Option<BTreeSet<pid_t>>,
        /// Peak virtual memory size.
        ///
        /// Known as `VmPeak`.
        pub peak_virtual_memory_size: Option<Kilobyte>,
        /// Total program size.
        ///
        /// Known as `VmSize`.
        pub total_program_size: Option<Kilobyte>,
        /// Locked memory size.
        ///
        /// Known as `VmLck`.
        ///
        /// See `man 3 lock`.
        pub locked_memory_size: Option<Kilobyte>,
        /// Pinned memory size (since Linux 3.2).
        ///
        /// Known as `VmPin`.
        ///
        /// These are pages that can't be moved because something needs to directly access physical memory.
        pub pinned_memory_size: Option<Kilobyte>,
        /// Peak resident set size ("High Water Mark").
        ///
        /// Known as `VmHWM`.
        pub peak_resident_set_size: Option<Kilobyte>,
        /// The sum of `anonymous_resident_set_memory_size`, `resident_set_file_mappings_memory_size` and `resident_set_shared_memory_size`.
        ///
        /// Known as `VmRSS`.
        pub resident_set_memory_size: Option<Kilobyte>,
        /// Size of resident set anonymous memory (since Linux 4.5).
        ///
        /// Known as `RssAnon`.
        pub anonymous_resident_set_memory_size: Option<Kilobyte>,
        /// Size of resident set file mappings (since Linux 4.5).
        ///
        /// Known as `RssFile`.
        pub resident_set_file_mappings_memory_size: Option<Kilobyte>,
        /// Size of resident set shared memory (`shmem`) (since Linux 4.5).
        ///
        /// Known as `RssShmem`.
        ///
        /// Includes Sys_v `shm`, any mappings from `tmpfs` and shared anonymous mappings.
        pub resident_set_shared_memory_size: Option<Kilobyte>,
        /// Size of private data segments.
        ///
        /// Known as `VmData`.
        pub private_data_segments_size: Option<Kilobyte>,
        /// Size of stack segments.
        ///
        /// Known as `VmStk`.
        pub stack_segments_size: Option<Kilobyte>,
        /// Size of text segment.
        ///
        /// Known as `VmExe`.
        pub text_segment_size: Option<Kilobyte>,
        /// Size of shared library code.
        ///
        /// Known as `VmLib`.
        pub dynamically_loaded_shared_library_size: Option<Kilobyte>,
        /// Size of page table entries (since Linux 2.6.10).
        ///
        /// Known as `VmPTE`.
        pub page_table_entries_size: Option<Kilobyte>,
        /// Size of second-level page tables (since Linux 4.0).
        ///
        /// Known as `VmPMD`.
        ///
        /// Undocumented in <https://github.com/torvalds/linux/blob/master/Documentation/filesystems/proc.txt>.
        pub vm_pmd: Option<Kilobyte>,
        /// The amount of swap used by anonymous private data (since Linux 2.6.34).
        ///
        /// Known as `VmSwap`.
        ///
        /// Shared memory `shmem` swap usage is not included.
        pub swap_memory_size: Option<Kilobyte>,
        /// Size of `hugetlb` memory portions.
        ///
        /// Known as `HugetlbPages`.
        pub huge_tlb_pages_memory_size: Option<Kilobyte>,
        /// Number of threads.
        ///
        /// Known as `Threads`.
        pub threads: Option<u64>,
        /// Signal queue status.
        ///
        /// Known as `SigQ`.
        pub signal_queue: Option<SignalQueueStatus>,
        /// Pending signals for the thread.
        ///
        /// Known as `SigPnd`.
        pub thread_pending_signals: Option<Bitmask>,
        /// Shared pending signals for the process.
        ///
        /// Known as `ShdPnd`.
        pub process_shared_pending_signals: Option<Bitmask>,
        /// Blocked signals.
        ///
        /// Known as `SigBlk`.
        pub blocked_signals: Option<Bitmask>,
        /// Ignored signals.
        ///
        /// Known as `SigIgn`.
        pub ignored_signals: Option<Bitmask>,
        /// Caught signals.
        ///
        /// Known as `SigCgt`.
        pub caught_signals: Option<Bitmask>,
        /// Inheritable capabilities.
        ///
        /// Known as `CapInh`.
        pub inheritable_capabilities: Option<Bitmask>,
        /// Permitted capabilities.
        ///
        /// Known as `CapPrm`.
        pub permitted_capabilities: Option<Bitmask>,
        /// Effective capabilities.
        ///
        /// Known as `CapEff`.
        pub effective_capabilities: Option<Bitmask>,
        /// Capabilities bounding set.
        ///
        /// Known as `CapBnd`.
        pub capabilities_bounding_set: Option<Bitmask>,
        /// Ambient capabilities.
        ///
        /// Known as `CapAmb`.
        pub ambient_capabilities: Option<Bitmask>,
        /// Thread's `no_new_privs` bit (see `man 2 prctl` description for `PR_GET_NO_NEW_PRIVS`).
        ///
        /// Known as `NoNewPrivs`.
        pub thread_no_new_privileges_bit: Option<Bitmask>,
        /// Seccomp mode.
        ///
        /// Known as `Seccomp`.
        pub seccomp_mode: Option<SeccompMode>,
        /// Speculation store ('Spectre' vulnerability) bypass status.
        ///
        /// Known as `Speculation_Store_Bypass`.
        pub speculation_store_bypass: Option<SpeculationStoreBypassStatus>,
        /// CPUs (actually, hyper threaded cores) allowed for the current process.
        ///
        /// Known as `Cpus_allowed`.
        ///
        /// May have bits set well beyond those than the number of cores on the system.
        pub cpus_allowed_bitmask: Option<HyperThreadBitmask>,
        /// CPUs (actually, hyper threaded cores) allowed for the current process.
        ///
        /// Known as `Cpus_allowed_list`.
        ///
        /// May have cores available beyond those than the number of cores on the system, but usually a much more restricted list than `cpus_allowed_bitmask`.
        pub cpus_allowed_list: Option<BTreeSet<HyperThread>>,
        /// NUMA nodes allowed for the current process.
        ///
        /// Known as `Mems_allowed`.
        ///
        /// Linux defines the config option `NODES_SHIFT` (aka `CONFIG_NODES_SHIFT`) to be 1 to 10 if defined and 0 if not defined, giving a maximum of 2^10 (1024) NUMA nodes, if defaults to 6 (ie 64 NUMA nodes) on x86-64.
        pub numa_nodes_allowed_bitmask: Option<NumaNodeBitmask>,
        /// NUMA nodes allowed for the current process.
        ///
        /// Known as `Mems_allowed_list`.
        ///
        /// On a non-NUMA system, defaults to 0.
        pub numa_nodes_allowed_list: Option<BTreeSet<NumaNode>>,
        /// Voluntary context switches.
        ///
        /// Known as `voluntary_ctxt_switches`.
        pub voluntary_context_switches: Option<u64>,
        /// Involuntary context switches.
        ///
        /// Known as `nonvoluntary_ctxt_switches`.
        pub involuntary_context_switches: Option<u64>,
        unrecognised: HashMap<Box<[u8]>, Box<[u8]>>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::default::Default for ProcessStatusStatistics {
        #[inline]
        fn default() -> ProcessStatusStatistics {
            ProcessStatusStatistics{process_name:
                                        $crate::default::Default::default(),
                                    file_mode_creation_mask:
                                        $crate::default::Default::default(),
                                    state:
                                        $crate::default::Default::default(),
                                    thread_group_identifier:
                                        $crate::default::Default::default(),
                                    numa_group_identifier:
                                        $crate::default::Default::default(),
                                    process_identifier:
                                        $crate::default::Default::default(),
                                    parent_process_identifier:
                                        $crate::default::Default::default(),
                                    tracer_process_identifier:
                                        $crate::default::Default::default(),
                                    user_identifiers:
                                        $crate::default::Default::default(),
                                    group_identifiers:
                                        $crate::default::Default::default(),
                                    number_of_file_descriptor_slots_currently_allocated:
                                        $crate::default::Default::default(),
                                    groups:
                                        $crate::default::Default::default(),
                                    descendant_namespace_thread_group_identifier:
                                        $crate::default::Default::default(),
                                    descendant_namespace_process_identifier:
                                        $crate::default::Default::default(),
                                    descendant_namespace_process_group_identifier:
                                        $crate::default::Default::default(),
                                    descendant_namespace_session_identifier:
                                        $crate::default::Default::default(),
                                    peak_virtual_memory_size:
                                        $crate::default::Default::default(),
                                    total_program_size:
                                        $crate::default::Default::default(),
                                    locked_memory_size:
                                        $crate::default::Default::default(),
                                    pinned_memory_size:
                                        $crate::default::Default::default(),
                                    peak_resident_set_size:
                                        $crate::default::Default::default(),
                                    resident_set_memory_size:
                                        $crate::default::Default::default(),
                                    anonymous_resident_set_memory_size:
                                        $crate::default::Default::default(),
                                    resident_set_file_mappings_memory_size:
                                        $crate::default::Default::default(),
                                    resident_set_shared_memory_size:
                                        $crate::default::Default::default(),
                                    private_data_segments_size:
                                        $crate::default::Default::default(),
                                    stack_segments_size:
                                        $crate::default::Default::default(),
                                    text_segment_size:
                                        $crate::default::Default::default(),
                                    dynamically_loaded_shared_library_size:
                                        $crate::default::Default::default(),
                                    page_table_entries_size:
                                        $crate::default::Default::default(),
                                    vm_pmd:
                                        $crate::default::Default::default(),
                                    swap_memory_size:
                                        $crate::default::Default::default(),
                                    huge_tlb_pages_memory_size:
                                        $crate::default::Default::default(),
                                    threads:
                                        $crate::default::Default::default(),
                                    signal_queue:
                                        $crate::default::Default::default(),
                                    thread_pending_signals:
                                        $crate::default::Default::default(),
                                    process_shared_pending_signals:
                                        $crate::default::Default::default(),
                                    blocked_signals:
                                        $crate::default::Default::default(),
                                    ignored_signals:
                                        $crate::default::Default::default(),
                                    caught_signals:
                                        $crate::default::Default::default(),
                                    inheritable_capabilities:
                                        $crate::default::Default::default(),
                                    permitted_capabilities:
                                        $crate::default::Default::default(),
                                    effective_capabilities:
                                        $crate::default::Default::default(),
                                    capabilities_bounding_set:
                                        $crate::default::Default::default(),
                                    ambient_capabilities:
                                        $crate::default::Default::default(),
                                    thread_no_new_privileges_bit:
                                        $crate::default::Default::default(),
                                    seccomp_mode:
                                        $crate::default::Default::default(),
                                    speculation_store_bypass:
                                        $crate::default::Default::default(),
                                    cpus_allowed_bitmask:
                                        $crate::default::Default::default(),
                                    cpus_allowed_list:
                                        $crate::default::Default::default(),
                                    numa_nodes_allowed_bitmask:
                                        $crate::default::Default::default(),
                                    numa_nodes_allowed_list:
                                        $crate::default::Default::default(),
                                    voluntary_context_switches:
                                        $crate::default::Default::default(),
                                    involuntary_context_switches:
                                        $crate::default::Default::default(),
                                    unrecognised:
                                        $crate::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::fmt::Debug for ProcessStatusStatistics {
        fn fmt(&self, f: &mut $crate::fmt::Formatter) -> $crate::fmt::Result {
            match *self {
                ProcessStatusStatistics {
                process_name: ref __self_0_0,
                file_mode_creation_mask: ref __self_0_1,
                state: ref __self_0_2,
                thread_group_identifier: ref __self_0_3,
                numa_group_identifier: ref __self_0_4,
                process_identifier: ref __self_0_5,
                parent_process_identifier: ref __self_0_6,
                tracer_process_identifier: ref __self_0_7,
                user_identifiers: ref __self_0_8,
                group_identifiers: ref __self_0_9,
                number_of_file_descriptor_slots_currently_allocated: ref __self_0_10,
                groups: ref __self_0_11,
                descendant_namespace_thread_group_identifier: ref __self_0_12,
                descendant_namespace_process_identifier: ref __self_0_13,
                descendant_namespace_process_group_identifier: ref __self_0_14,
                descendant_namespace_session_identifier: ref __self_0_15,
                peak_virtual_memory_size: ref __self_0_16,
                total_program_size: ref __self_0_17,
                locked_memory_size: ref __self_0_18,
                pinned_memory_size: ref __self_0_19,
                peak_resident_set_size: ref __self_0_20,
                resident_set_memory_size: ref __self_0_21,
                anonymous_resident_set_memory_size: ref __self_0_22,
                resident_set_file_mappings_memory_size: ref __self_0_23,
                resident_set_shared_memory_size: ref __self_0_24,
                private_data_segments_size: ref __self_0_25,
                stack_segments_size: ref __self_0_26,
                text_segment_size: ref __self_0_27,
                dynamically_loaded_shared_library_size: ref __self_0_28,
                page_table_entries_size: ref __self_0_29,
                vm_pmd: ref __self_0_30,
                swap_memory_size: ref __self_0_31,
                huge_tlb_pages_memory_size: ref __self_0_32,
                threads: ref __self_0_33,
                signal_queue: ref __self_0_34,
                thread_pending_signals: ref __self_0_35,
                process_shared_pending_signals: ref __self_0_36,
                blocked_signals: ref __self_0_37,
                ignored_signals: ref __self_0_38,
                caught_signals: ref __self_0_39,
                inheritable_capabilities: ref __self_0_40,
                permitted_capabilities: ref __self_0_41,
                effective_capabilities: ref __self_0_42,
                capabilities_bounding_set: ref __self_0_43,
                ambient_capabilities: ref __self_0_44,
                thread_no_new_privileges_bit: ref __self_0_45,
                seccomp_mode: ref __self_0_46,
                speculation_store_bypass: ref __self_0_47,
                cpus_allowed_bitmask: ref __self_0_48,
                cpus_allowed_list: ref __self_0_49,
                numa_nodes_allowed_bitmask: ref __self_0_50,
                numa_nodes_allowed_list: ref __self_0_51,
                voluntary_context_switches: ref __self_0_52,
                involuntary_context_switches: ref __self_0_53,
                unrecognised: ref __self_0_54 } => {
                    let mut debug_trait_builder =
                        f.debug_struct("ProcessStatusStatistics");
                    let _ =
                        debug_trait_builder.field("process_name",
                                                  &&(*__self_0_0));
                    let _ =
                        debug_trait_builder.field("file_mode_creation_mask",
                                                  &&(*__self_0_1));
                    let _ =
                        debug_trait_builder.field("state", &&(*__self_0_2));
                    let _ =
                        debug_trait_builder.field("thread_group_identifier",
                                                  &&(*__self_0_3));
                    let _ =
                        debug_trait_builder.field("numa_group_identifier",
                                                  &&(*__self_0_4));
                    let _ =
                        debug_trait_builder.field("process_identifier",
                                                  &&(*__self_0_5));
                    let _ =
                        debug_trait_builder.field("parent_process_identifier",
                                                  &&(*__self_0_6));
                    let _ =
                        debug_trait_builder.field("tracer_process_identifier",
                                                  &&(*__self_0_7));
                    let _ =
                        debug_trait_builder.field("user_identifiers",
                                                  &&(*__self_0_8));
                    let _ =
                        debug_trait_builder.field("group_identifiers",
                                                  &&(*__self_0_9));
                    let _ =
                        debug_trait_builder.field("number_of_file_descriptor_slots_currently_allocated",
                                                  &&(*__self_0_10));
                    let _ =
                        debug_trait_builder.field("groups", &&(*__self_0_11));
                    let _ =
                        debug_trait_builder.field("descendant_namespace_thread_group_identifier",
                                                  &&(*__self_0_12));
                    let _ =
                        debug_trait_builder.field("descendant_namespace_process_identifier",
                                                  &&(*__self_0_13));
                    let _ =
                        debug_trait_builder.field("descendant_namespace_process_group_identifier",
                                                  &&(*__self_0_14));
                    let _ =
                        debug_trait_builder.field("descendant_namespace_session_identifier",
                                                  &&(*__self_0_15));
                    let _ =
                        debug_trait_builder.field("peak_virtual_memory_size",
                                                  &&(*__self_0_16));
                    let _ =
                        debug_trait_builder.field("total_program_size",
                                                  &&(*__self_0_17));
                    let _ =
                        debug_trait_builder.field("locked_memory_size",
                                                  &&(*__self_0_18));
                    let _ =
                        debug_trait_builder.field("pinned_memory_size",
                                                  &&(*__self_0_19));
                    let _ =
                        debug_trait_builder.field("peak_resident_set_size",
                                                  &&(*__self_0_20));
                    let _ =
                        debug_trait_builder.field("resident_set_memory_size",
                                                  &&(*__self_0_21));
                    let _ =
                        debug_trait_builder.field("anonymous_resident_set_memory_size",
                                                  &&(*__self_0_22));
                    let _ =
                        debug_trait_builder.field("resident_set_file_mappings_memory_size",
                                                  &&(*__self_0_23));
                    let _ =
                        debug_trait_builder.field("resident_set_shared_memory_size",
                                                  &&(*__self_0_24));
                    let _ =
                        debug_trait_builder.field("private_data_segments_size",
                                                  &&(*__self_0_25));
                    let _ =
                        debug_trait_builder.field("stack_segments_size",
                                                  &&(*__self_0_26));
                    let _ =
                        debug_trait_builder.field("text_segment_size",
                                                  &&(*__self_0_27));
                    let _ =
                        debug_trait_builder.field("dynamically_loaded_shared_library_size",
                                                  &&(*__self_0_28));
                    let _ =
                        debug_trait_builder.field("page_table_entries_size",
                                                  &&(*__self_0_29));
                    let _ =
                        debug_trait_builder.field("vm_pmd", &&(*__self_0_30));
                    let _ =
                        debug_trait_builder.field("swap_memory_size",
                                                  &&(*__self_0_31));
                    let _ =
                        debug_trait_builder.field("huge_tlb_pages_memory_size",
                                                  &&(*__self_0_32));
                    let _ =
                        debug_trait_builder.field("threads",
                                                  &&(*__self_0_33));
                    let _ =
                        debug_trait_builder.field("signal_queue",
                                                  &&(*__self_0_34));
                    let _ =
                        debug_trait_builder.field("thread_pending_signals",
                                                  &&(*__self_0_35));
                    let _ =
                        debug_trait_builder.field("process_shared_pending_signals",
                                                  &&(*__self_0_36));
                    let _ =
                        debug_trait_builder.field("blocked_signals",
                                                  &&(*__self_0_37));
                    let _ =
                        debug_trait_builder.field("ignored_signals",
                                                  &&(*__self_0_38));
                    let _ =
                        debug_trait_builder.field("caught_signals",
                                                  &&(*__self_0_39));
                    let _ =
                        debug_trait_builder.field("inheritable_capabilities",
                                                  &&(*__self_0_40));
                    let _ =
                        debug_trait_builder.field("permitted_capabilities",
                                                  &&(*__self_0_41));
                    let _ =
                        debug_trait_builder.field("effective_capabilities",
                                                  &&(*__self_0_42));
                    let _ =
                        debug_trait_builder.field("capabilities_bounding_set",
                                                  &&(*__self_0_43));
                    let _ =
                        debug_trait_builder.field("ambient_capabilities",
                                                  &&(*__self_0_44));
                    let _ =
                        debug_trait_builder.field("thread_no_new_privileges_bit",
                                                  &&(*__self_0_45));
                    let _ =
                        debug_trait_builder.field("seccomp_mode",
                                                  &&(*__self_0_46));
                    let _ =
                        debug_trait_builder.field("speculation_store_bypass",
                                                  &&(*__self_0_47));
                    let _ =
                        debug_trait_builder.field("cpus_allowed_bitmask",
                                                  &&(*__self_0_48));
                    let _ =
                        debug_trait_builder.field("cpus_allowed_list",
                                                  &&(*__self_0_49));
                    let _ =
                        debug_trait_builder.field("numa_nodes_allowed_bitmask",
                                                  &&(*__self_0_50));
                    let _ =
                        debug_trait_builder.field("numa_nodes_allowed_list",
                                                  &&(*__self_0_51));
                    let _ =
                        debug_trait_builder.field("voluntary_context_switches",
                                                  &&(*__self_0_52));
                    let _ =
                        debug_trait_builder.field("involuntary_context_switches",
                                                  &&(*__self_0_53));
                    let _ =
                        debug_trait_builder.field("unrecognised",
                                                  &&(*__self_0_54));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::clone::Clone for ProcessStatusStatistics {
        #[inline]
        fn clone(&self) -> ProcessStatusStatistics {
            match *self {
                ProcessStatusStatistics {
                process_name: ref __self_0_0,
                file_mode_creation_mask: ref __self_0_1,
                state: ref __self_0_2,
                thread_group_identifier: ref __self_0_3,
                numa_group_identifier: ref __self_0_4,
                process_identifier: ref __self_0_5,
                parent_process_identifier: ref __self_0_6,
                tracer_process_identifier: ref __self_0_7,
                user_identifiers: ref __self_0_8,
                group_identifiers: ref __self_0_9,
                number_of_file_descriptor_slots_currently_allocated: ref __self_0_10,
                groups: ref __self_0_11,
                descendant_namespace_thread_group_identifier: ref __self_0_12,
                descendant_namespace_process_identifier: ref __self_0_13,
                descendant_namespace_process_group_identifier: ref __self_0_14,
                descendant_namespace_session_identifier: ref __self_0_15,
                peak_virtual_memory_size: ref __self_0_16,
                total_program_size: ref __self_0_17,
                locked_memory_size: ref __self_0_18,
                pinned_memory_size: ref __self_0_19,
                peak_resident_set_size: ref __self_0_20,
                resident_set_memory_size: ref __self_0_21,
                anonymous_resident_set_memory_size: ref __self_0_22,
                resident_set_file_mappings_memory_size: ref __self_0_23,
                resident_set_shared_memory_size: ref __self_0_24,
                private_data_segments_size: ref __self_0_25,
                stack_segments_size: ref __self_0_26,
                text_segment_size: ref __self_0_27,
                dynamically_loaded_shared_library_size: ref __self_0_28,
                page_table_entries_size: ref __self_0_29,
                vm_pmd: ref __self_0_30,
                swap_memory_size: ref __self_0_31,
                huge_tlb_pages_memory_size: ref __self_0_32,
                threads: ref __self_0_33,
                signal_queue: ref __self_0_34,
                thread_pending_signals: ref __self_0_35,
                process_shared_pending_signals: ref __self_0_36,
                blocked_signals: ref __self_0_37,
                ignored_signals: ref __self_0_38,
                caught_signals: ref __self_0_39,
                inheritable_capabilities: ref __self_0_40,
                permitted_capabilities: ref __self_0_41,
                effective_capabilities: ref __self_0_42,
                capabilities_bounding_set: ref __self_0_43,
                ambient_capabilities: ref __self_0_44,
                thread_no_new_privileges_bit: ref __self_0_45,
                seccomp_mode: ref __self_0_46,
                speculation_store_bypass: ref __self_0_47,
                cpus_allowed_bitmask: ref __self_0_48,
                cpus_allowed_list: ref __self_0_49,
                numa_nodes_allowed_bitmask: ref __self_0_50,
                numa_nodes_allowed_list: ref __self_0_51,
                voluntary_context_switches: ref __self_0_52,
                involuntary_context_switches: ref __self_0_53,
                unrecognised: ref __self_0_54 } =>
                ProcessStatusStatistics{process_name:
                                            $crate::clone::Clone::clone(&(*__self_0_0)),
                                        file_mode_creation_mask:
                                            $crate::clone::Clone::clone(&(*__self_0_1)),
                                        state:
                                            $crate::clone::Clone::clone(&(*__self_0_2)),
                                        thread_group_identifier:
                                            $crate::clone::Clone::clone(&(*__self_0_3)),
                                        numa_group_identifier:
                                            $crate::clone::Clone::clone(&(*__self_0_4)),
                                        process_identifier:
                                            $crate::clone::Clone::clone(&(*__self_0_5)),
                                        parent_process_identifier:
                                            $crate::clone::Clone::clone(&(*__self_0_6)),
                                        tracer_process_identifier:
                                            $crate::clone::Clone::clone(&(*__self_0_7)),
                                        user_identifiers:
                                            $crate::clone::Clone::clone(&(*__self_0_8)),
                                        group_identifiers:
                                            $crate::clone::Clone::clone(&(*__self_0_9)),
                                        number_of_file_descriptor_slots_currently_allocated:
                                            $crate::clone::Clone::clone(&(*__self_0_10)),
                                        groups:
                                            $crate::clone::Clone::clone(&(*__self_0_11)),
                                        descendant_namespace_thread_group_identifier:
                                            $crate::clone::Clone::clone(&(*__self_0_12)),
                                        descendant_namespace_process_identifier:
                                            $crate::clone::Clone::clone(&(*__self_0_13)),
                                        descendant_namespace_process_group_identifier:
                                            $crate::clone::Clone::clone(&(*__self_0_14)),
                                        descendant_namespace_session_identifier:
                                            $crate::clone::Clone::clone(&(*__self_0_15)),
                                        peak_virtual_memory_size:
                                            $crate::clone::Clone::clone(&(*__self_0_16)),
                                        total_program_size:
                                            $crate::clone::Clone::clone(&(*__self_0_17)),
                                        locked_memory_size:
                                            $crate::clone::Clone::clone(&(*__self_0_18)),
                                        pinned_memory_size:
                                            $crate::clone::Clone::clone(&(*__self_0_19)),
                                        peak_resident_set_size:
                                            $crate::clone::Clone::clone(&(*__self_0_20)),
                                        resident_set_memory_size:
                                            $crate::clone::Clone::clone(&(*__self_0_21)),
                                        anonymous_resident_set_memory_size:
                                            $crate::clone::Clone::clone(&(*__self_0_22)),
                                        resident_set_file_mappings_memory_size:
                                            $crate::clone::Clone::clone(&(*__self_0_23)),
                                        resident_set_shared_memory_size:
                                            $crate::clone::Clone::clone(&(*__self_0_24)),
                                        private_data_segments_size:
                                            $crate::clone::Clone::clone(&(*__self_0_25)),
                                        stack_segments_size:
                                            $crate::clone::Clone::clone(&(*__self_0_26)),
                                        text_segment_size:
                                            $crate::clone::Clone::clone(&(*__self_0_27)),
                                        dynamically_loaded_shared_library_size:
                                            $crate::clone::Clone::clone(&(*__self_0_28)),
                                        page_table_entries_size:
                                            $crate::clone::Clone::clone(&(*__self_0_29)),
                                        vm_pmd:
                                            $crate::clone::Clone::clone(&(*__self_0_30)),
                                        swap_memory_size:
                                            $crate::clone::Clone::clone(&(*__self_0_31)),
                                        huge_tlb_pages_memory_size:
                                            $crate::clone::Clone::clone(&(*__self_0_32)),
                                        threads:
                                            $crate::clone::Clone::clone(&(*__self_0_33)),
                                        signal_queue:
                                            $crate::clone::Clone::clone(&(*__self_0_34)),
                                        thread_pending_signals:
                                            $crate::clone::Clone::clone(&(*__self_0_35)),
                                        process_shared_pending_signals:
                                            $crate::clone::Clone::clone(&(*__self_0_36)),
                                        blocked_signals:
                                            $crate::clone::Clone::clone(&(*__self_0_37)),
                                        ignored_signals:
                                            $crate::clone::Clone::clone(&(*__self_0_38)),
                                        caught_signals:
                                            $crate::clone::Clone::clone(&(*__self_0_39)),
                                        inheritable_capabilities:
                                            $crate::clone::Clone::clone(&(*__self_0_40)),
                                        permitted_capabilities:
                                            $crate::clone::Clone::clone(&(*__self_0_41)),
                                        effective_capabilities:
                                            $crate::clone::Clone::clone(&(*__self_0_42)),
                                        capabilities_bounding_set:
                                            $crate::clone::Clone::clone(&(*__self_0_43)),
                                        ambient_capabilities:
                                            $crate::clone::Clone::clone(&(*__self_0_44)),
                                        thread_no_new_privileges_bit:
                                            $crate::clone::Clone::clone(&(*__self_0_45)),
                                        seccomp_mode:
                                            $crate::clone::Clone::clone(&(*__self_0_46)),
                                        speculation_store_bypass:
                                            $crate::clone::Clone::clone(&(*__self_0_47)),
                                        cpus_allowed_bitmask:
                                            $crate::clone::Clone::clone(&(*__self_0_48)),
                                        cpus_allowed_list:
                                            $crate::clone::Clone::clone(&(*__self_0_49)),
                                        numa_nodes_allowed_bitmask:
                                            $crate::clone::Clone::clone(&(*__self_0_50)),
                                        numa_nodes_allowed_list:
                                            $crate::clone::Clone::clone(&(*__self_0_51)),
                                        voluntary_context_switches:
                                            $crate::clone::Clone::clone(&(*__self_0_52)),
                                        involuntary_context_switches:
                                            $crate::clone::Clone::clone(&(*__self_0_53)),
                                        unrecognised:
                                            $crate::clone::Clone::clone(&(*__self_0_54)),},
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::cmp::PartialEq for ProcessStatusStatistics {
        #[inline]
        fn eq(&self, other: &ProcessStatusStatistics) -> bool {
            match *other {
                ProcessStatusStatistics {
                process_name: ref __self_1_0,
                file_mode_creation_mask: ref __self_1_1,
                state: ref __self_1_2,
                thread_group_identifier: ref __self_1_3,
                numa_group_identifier: ref __self_1_4,
                process_identifier: ref __self_1_5,
                parent_process_identifier: ref __self_1_6,
                tracer_process_identifier: ref __self_1_7,
                user_identifiers: ref __self_1_8,
                group_identifiers: ref __self_1_9,
                number_of_file_descriptor_slots_currently_allocated: ref __self_1_10,
                groups: ref __self_1_11,
                descendant_namespace_thread_group_identifier: ref __self_1_12,
                descendant_namespace_process_identifier: ref __self_1_13,
                descendant_namespace_process_group_identifier: ref __self_1_14,
                descendant_namespace_session_identifier: ref __self_1_15,
                peak_virtual_memory_size: ref __self_1_16,
                total_program_size: ref __self_1_17,
                locked_memory_size: ref __self_1_18,
                pinned_memory_size: ref __self_1_19,
                peak_resident_set_size: ref __self_1_20,
                resident_set_memory_size: ref __self_1_21,
                anonymous_resident_set_memory_size: ref __self_1_22,
                resident_set_file_mappings_memory_size: ref __self_1_23,
                resident_set_shared_memory_size: ref __self_1_24,
                private_data_segments_size: ref __self_1_25,
                stack_segments_size: ref __self_1_26,
                text_segment_size: ref __self_1_27,
                dynamically_loaded_shared_library_size: ref __self_1_28,
                page_table_entries_size: ref __self_1_29,
                vm_pmd: ref __self_1_30,
                swap_memory_size: ref __self_1_31,
                huge_tlb_pages_memory_size: ref __self_1_32,
                threads: ref __self_1_33,
                signal_queue: ref __self_1_34,
                thread_pending_signals: ref __self_1_35,
                process_shared_pending_signals: ref __self_1_36,
                blocked_signals: ref __self_1_37,
                ignored_signals: ref __self_1_38,
                caught_signals: ref __self_1_39,
                inheritable_capabilities: ref __self_1_40,
                permitted_capabilities: ref __self_1_41,
                effective_capabilities: ref __self_1_42,
                capabilities_bounding_set: ref __self_1_43,
                ambient_capabilities: ref __self_1_44,
                thread_no_new_privileges_bit: ref __self_1_45,
                seccomp_mode: ref __self_1_46,
                speculation_store_bypass: ref __self_1_47,
                cpus_allowed_bitmask: ref __self_1_48,
                cpus_allowed_list: ref __self_1_49,
                numa_nodes_allowed_bitmask: ref __self_1_50,
                numa_nodes_allowed_list: ref __self_1_51,
                voluntary_context_switches: ref __self_1_52,
                involuntary_context_switches: ref __self_1_53,
                unrecognised: ref __self_1_54 } =>
                match *self {
                    ProcessStatusStatistics {
                    process_name: ref __self_0_0,
                    file_mode_creation_mask: ref __self_0_1,
                    state: ref __self_0_2,
                    thread_group_identifier: ref __self_0_3,
                    numa_group_identifier: ref __self_0_4,
                    process_identifier: ref __self_0_5,
                    parent_process_identifier: ref __self_0_6,
                    tracer_process_identifier: ref __self_0_7,
                    user_identifiers: ref __self_0_8,
                    group_identifiers: ref __self_0_9,
                    number_of_file_descriptor_slots_currently_allocated: ref __self_0_10,
                    groups: ref __self_0_11,
                    descendant_namespace_thread_group_identifier: ref __self_0_12,
                    descendant_namespace_process_identifier: ref __self_0_13,
                    descendant_namespace_process_group_identifier: ref __self_0_14,
                    descendant_namespace_session_identifier: ref __self_0_15,
                    peak_virtual_memory_size: ref __self_0_16,
                    total_program_size: ref __self_0_17,
                    locked_memory_size: ref __self_0_18,
                    pinned_memory_size: ref __self_0_19,
                    peak_resident_set_size: ref __self_0_20,
                    resident_set_memory_size: ref __self_0_21,
                    anonymous_resident_set_memory_size: ref __self_0_22,
                    resident_set_file_mappings_memory_size: ref __self_0_23,
                    resident_set_shared_memory_size: ref __self_0_24,
                    private_data_segments_size: ref __self_0_25,
                    stack_segments_size: ref __self_0_26,
                    text_segment_size: ref __self_0_27,
                    dynamically_loaded_shared_library_size: ref __self_0_28,
                    page_table_entries_size: ref __self_0_29,
                    vm_pmd: ref __self_0_30,
                    swap_memory_size: ref __self_0_31,
                    huge_tlb_pages_memory_size: ref __self_0_32,
                    threads: ref __self_0_33,
                    signal_queue: ref __self_0_34,
                    thread_pending_signals: ref __self_0_35,
                    process_shared_pending_signals: ref __self_0_36,
                    blocked_signals: ref __self_0_37,
                    ignored_signals: ref __self_0_38,
                    caught_signals: ref __self_0_39,
                    inheritable_capabilities: ref __self_0_40,
                    permitted_capabilities: ref __self_0_41,
                    effective_capabilities: ref __self_0_42,
                    capabilities_bounding_set: ref __self_0_43,
                    ambient_capabilities: ref __self_0_44,
                    thread_no_new_privileges_bit: ref __self_0_45,
                    seccomp_mode: ref __self_0_46,
                    speculation_store_bypass: ref __self_0_47,
                    cpus_allowed_bitmask: ref __self_0_48,
                    cpus_allowed_list: ref __self_0_49,
                    numa_nodes_allowed_bitmask: ref __self_0_50,
                    numa_nodes_allowed_list: ref __self_0_51,
                    voluntary_context_switches: ref __self_0_52,
                    involuntary_context_switches: ref __self_0_53,
                    unrecognised: ref __self_0_54 } =>
                    (*__self_0_0) == (*__self_1_0) &&
                        (*__self_0_1) == (*__self_1_1) &&
                        (*__self_0_2) == (*__self_1_2) &&
                        (*__self_0_3) == (*__self_1_3) &&
                        (*__self_0_4) == (*__self_1_4) &&
                        (*__self_0_5) == (*__self_1_5) &&
                        (*__self_0_6) == (*__self_1_6) &&
                        (*__self_0_7) == (*__self_1_7) &&
                        (*__self_0_8) == (*__self_1_8) &&
                        (*__self_0_9) == (*__self_1_9) &&
                        (*__self_0_10) == (*__self_1_10) &&
                        (*__self_0_11) == (*__self_1_11) &&
                        (*__self_0_12) == (*__self_1_12) &&
                        (*__self_0_13) == (*__self_1_13) &&
                        (*__self_0_14) == (*__self_1_14) &&
                        (*__self_0_15) == (*__self_1_15) &&
                        (*__self_0_16) == (*__self_1_16) &&
                        (*__self_0_17) == (*__self_1_17) &&
                        (*__self_0_18) == (*__self_1_18) &&
                        (*__self_0_19) == (*__self_1_19) &&
                        (*__self_0_20) == (*__self_1_20) &&
                        (*__self_0_21) == (*__self_1_21) &&
                        (*__self_0_22) == (*__self_1_22) &&
                        (*__self_0_23) == (*__self_1_23) &&
                        (*__self_0_24) == (*__self_1_24) &&
                        (*__self_0_25) == (*__self_1_25) &&
                        (*__self_0_26) == (*__self_1_26) &&
                        (*__self_0_27) == (*__self_1_27) &&
                        (*__self_0_28) == (*__self_1_28) &&
                        (*__self_0_29) == (*__self_1_29) &&
                        (*__self_0_30) == (*__self_1_30) &&
                        (*__self_0_31) == (*__self_1_31) &&
                        (*__self_0_32) == (*__self_1_32) &&
                        (*__self_0_33) == (*__self_1_33) &&
                        (*__self_0_34) == (*__self_1_34) &&
                        (*__self_0_35) == (*__self_1_35) &&
                        (*__self_0_36) == (*__self_1_36) &&
                        (*__self_0_37) == (*__self_1_37) &&
                        (*__self_0_38) == (*__self_1_38) &&
                        (*__self_0_39) == (*__self_1_39) &&
                        (*__self_0_40) == (*__self_1_40) &&
                        (*__self_0_41) == (*__self_1_41) &&
                        (*__self_0_42) == (*__self_1_42) &&
                        (*__self_0_43) == (*__self_1_43) &&
                        (*__self_0_44) == (*__self_1_44) &&
                        (*__self_0_45) == (*__self_1_45) &&
                        (*__self_0_46) == (*__self_1_46) &&
                        (*__self_0_47) == (*__self_1_47) &&
                        (*__self_0_48) == (*__self_1_48) &&
                        (*__self_0_49) == (*__self_1_49) &&
                        (*__self_0_50) == (*__self_1_50) &&
                        (*__self_0_51) == (*__self_1_51) &&
                        (*__self_0_52) == (*__self_1_52) &&
                        (*__self_0_53) == (*__self_1_53) &&
                        (*__self_0_54) == (*__self_1_54),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &ProcessStatusStatistics) -> bool {
            match *other {
                ProcessStatusStatistics {
                process_name: ref __self_1_0,
                file_mode_creation_mask: ref __self_1_1,
                state: ref __self_1_2,
                thread_group_identifier: ref __self_1_3,
                numa_group_identifier: ref __self_1_4,
                process_identifier: ref __self_1_5,
                parent_process_identifier: ref __self_1_6,
                tracer_process_identifier: ref __self_1_7,
                user_identifiers: ref __self_1_8,
                group_identifiers: ref __self_1_9,
                number_of_file_descriptor_slots_currently_allocated: ref __self_1_10,
                groups: ref __self_1_11,
                descendant_namespace_thread_group_identifier: ref __self_1_12,
                descendant_namespace_process_identifier: ref __self_1_13,
                descendant_namespace_process_group_identifier: ref __self_1_14,
                descendant_namespace_session_identifier: ref __self_1_15,
                peak_virtual_memory_size: ref __self_1_16,
                total_program_size: ref __self_1_17,
                locked_memory_size: ref __self_1_18,
                pinned_memory_size: ref __self_1_19,
                peak_resident_set_size: ref __self_1_20,
                resident_set_memory_size: ref __self_1_21,
                anonymous_resident_set_memory_size: ref __self_1_22,
                resident_set_file_mappings_memory_size: ref __self_1_23,
                resident_set_shared_memory_size: ref __self_1_24,
                private_data_segments_size: ref __self_1_25,
                stack_segments_size: ref __self_1_26,
                text_segment_size: ref __self_1_27,
                dynamically_loaded_shared_library_size: ref __self_1_28,
                page_table_entries_size: ref __self_1_29,
                vm_pmd: ref __self_1_30,
                swap_memory_size: ref __self_1_31,
                huge_tlb_pages_memory_size: ref __self_1_32,
                threads: ref __self_1_33,
                signal_queue: ref __self_1_34,
                thread_pending_signals: ref __self_1_35,
                process_shared_pending_signals: ref __self_1_36,
                blocked_signals: ref __self_1_37,
                ignored_signals: ref __self_1_38,
                caught_signals: ref __self_1_39,
                inheritable_capabilities: ref __self_1_40,
                permitted_capabilities: ref __self_1_41,
                effective_capabilities: ref __self_1_42,
                capabilities_bounding_set: ref __self_1_43,
                ambient_capabilities: ref __self_1_44,
                thread_no_new_privileges_bit: ref __self_1_45,
                seccomp_mode: ref __self_1_46,
                speculation_store_bypass: ref __self_1_47,
                cpus_allowed_bitmask: ref __self_1_48,
                cpus_allowed_list: ref __self_1_49,
                numa_nodes_allowed_bitmask: ref __self_1_50,
                numa_nodes_allowed_list: ref __self_1_51,
                voluntary_context_switches: ref __self_1_52,
                involuntary_context_switches: ref __self_1_53,
                unrecognised: ref __self_1_54 } =>
                match *self {
                    ProcessStatusStatistics {
                    process_name: ref __self_0_0,
                    file_mode_creation_mask: ref __self_0_1,
                    state: ref __self_0_2,
                    thread_group_identifier: ref __self_0_3,
                    numa_group_identifier: ref __self_0_4,
                    process_identifier: ref __self_0_5,
                    parent_process_identifier: ref __self_0_6,
                    tracer_process_identifier: ref __self_0_7,
                    user_identifiers: ref __self_0_8,
                    group_identifiers: ref __self_0_9,
                    number_of_file_descriptor_slots_currently_allocated: ref __self_0_10,
                    groups: ref __self_0_11,
                    descendant_namespace_thread_group_identifier: ref __self_0_12,
                    descendant_namespace_process_identifier: ref __self_0_13,
                    descendant_namespace_process_group_identifier: ref __self_0_14,
                    descendant_namespace_session_identifier: ref __self_0_15,
                    peak_virtual_memory_size: ref __self_0_16,
                    total_program_size: ref __self_0_17,
                    locked_memory_size: ref __self_0_18,
                    pinned_memory_size: ref __self_0_19,
                    peak_resident_set_size: ref __self_0_20,
                    resident_set_memory_size: ref __self_0_21,
                    anonymous_resident_set_memory_size: ref __self_0_22,
                    resident_set_file_mappings_memory_size: ref __self_0_23,
                    resident_set_shared_memory_size: ref __self_0_24,
                    private_data_segments_size: ref __self_0_25,
                    stack_segments_size: ref __self_0_26,
                    text_segment_size: ref __self_0_27,
                    dynamically_loaded_shared_library_size: ref __self_0_28,
                    page_table_entries_size: ref __self_0_29,
                    vm_pmd: ref __self_0_30,
                    swap_memory_size: ref __self_0_31,
                    huge_tlb_pages_memory_size: ref __self_0_32,
                    threads: ref __self_0_33,
                    signal_queue: ref __self_0_34,
                    thread_pending_signals: ref __self_0_35,
                    process_shared_pending_signals: ref __self_0_36,
                    blocked_signals: ref __self_0_37,
                    ignored_signals: ref __self_0_38,
                    caught_signals: ref __self_0_39,
                    inheritable_capabilities: ref __self_0_40,
                    permitted_capabilities: ref __self_0_41,
                    effective_capabilities: ref __self_0_42,
                    capabilities_bounding_set: ref __self_0_43,
                    ambient_capabilities: ref __self_0_44,
                    thread_no_new_privileges_bit: ref __self_0_45,
                    seccomp_mode: ref __self_0_46,
                    speculation_store_bypass: ref __self_0_47,
                    cpus_allowed_bitmask: ref __self_0_48,
                    cpus_allowed_list: ref __self_0_49,
                    numa_nodes_allowed_bitmask: ref __self_0_50,
                    numa_nodes_allowed_list: ref __self_0_51,
                    voluntary_context_switches: ref __self_0_52,
                    involuntary_context_switches: ref __self_0_53,
                    unrecognised: ref __self_0_54 } =>
                    (*__self_0_0) != (*__self_1_0) ||
                        (*__self_0_1) != (*__self_1_1) ||
                        (*__self_0_2) != (*__self_1_2) ||
                        (*__self_0_3) != (*__self_1_3) ||
                        (*__self_0_4) != (*__self_1_4) ||
                        (*__self_0_5) != (*__self_1_5) ||
                        (*__self_0_6) != (*__self_1_6) ||
                        (*__self_0_7) != (*__self_1_7) ||
                        (*__self_0_8) != (*__self_1_8) ||
                        (*__self_0_9) != (*__self_1_9) ||
                        (*__self_0_10) != (*__self_1_10) ||
                        (*__self_0_11) != (*__self_1_11) ||
                        (*__self_0_12) != (*__self_1_12) ||
                        (*__self_0_13) != (*__self_1_13) ||
                        (*__self_0_14) != (*__self_1_14) ||
                        (*__self_0_15) != (*__self_1_15) ||
                        (*__self_0_16) != (*__self_1_16) ||
                        (*__self_0_17) != (*__self_1_17) ||
                        (*__self_0_18) != (*__self_1_18) ||
                        (*__self_0_19) != (*__self_1_19) ||
                        (*__self_0_20) != (*__self_1_20) ||
                        (*__self_0_21) != (*__self_1_21) ||
                        (*__self_0_22) != (*__self_1_22) ||
                        (*__self_0_23) != (*__self_1_23) ||
                        (*__self_0_24) != (*__self_1_24) ||
                        (*__self_0_25) != (*__self_1_25) ||
                        (*__self_0_26) != (*__self_1_26) ||
                        (*__self_0_27) != (*__self_1_27) ||
                        (*__self_0_28) != (*__self_1_28) ||
                        (*__self_0_29) != (*__self_1_29) ||
                        (*__self_0_30) != (*__self_1_30) ||
                        (*__self_0_31) != (*__self_1_31) ||
                        (*__self_0_32) != (*__self_1_32) ||
                        (*__self_0_33) != (*__self_1_33) ||
                        (*__self_0_34) != (*__self_1_34) ||
                        (*__self_0_35) != (*__self_1_35) ||
                        (*__self_0_36) != (*__self_1_36) ||
                        (*__self_0_37) != (*__self_1_37) ||
                        (*__self_0_38) != (*__self_1_38) ||
                        (*__self_0_39) != (*__self_1_39) ||
                        (*__self_0_40) != (*__self_1_40) ||
                        (*__self_0_41) != (*__self_1_41) ||
                        (*__self_0_42) != (*__self_1_42) ||
                        (*__self_0_43) != (*__self_1_43) ||
                        (*__self_0_44) != (*__self_1_44) ||
                        (*__self_0_45) != (*__self_1_45) ||
                        (*__self_0_46) != (*__self_1_46) ||
                        (*__self_0_47) != (*__self_1_47) ||
                        (*__self_0_48) != (*__self_1_48) ||
                        (*__self_0_49) != (*__self_1_49) ||
                        (*__self_0_50) != (*__self_1_50) ||
                        (*__self_0_51) != (*__self_1_51) ||
                        (*__self_0_52) != (*__self_1_52) ||
                        (*__self_0_53) != (*__self_1_53) ||
                        (*__self_0_54) != (*__self_1_54),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::cmp::Eq for ProcessStatusStatistics {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: $crate::cmp::AssertParamIsEq<Option<Box<[u8]>>>;
                let _: $crate::cmp::AssertParamIsEq<Option<mode_t>>;
                let _: $crate::cmp::AssertParamIsEq<Option<ProcessState>>;
                let _: $crate::cmp::AssertParamIsEq<Option<pid_t>>;
                let _: $crate::cmp::AssertParamIsEq<Option<NumaNode>>;
                let _: $crate::cmp::AssertParamIsEq<Option<pid_t>>;
                let _: $crate::cmp::AssertParamIsEq<Option<pid_t>>;
                let _: $crate::cmp::AssertParamIsEq<Option<pid_t>>;
                let _:
                        $crate::cmp::AssertParamIsEq<Option<ProcessUserIdentifiers>>;
                let _:
                        $crate::cmp::AssertParamIsEq<Option<ProcessGroupIdentifiers>>;
                let _: $crate::cmp::AssertParamIsEq<Option<u64>>;
                let _: $crate::cmp::AssertParamIsEq<Option<BTreeSet<gid_t>>>;
                let _: $crate::cmp::AssertParamIsEq<Option<BTreeSet<pid_t>>>;
                let _: $crate::cmp::AssertParamIsEq<Option<BTreeSet<pid_t>>>;
                let _: $crate::cmp::AssertParamIsEq<Option<BTreeSet<pid_t>>>;
                let _: $crate::cmp::AssertParamIsEq<Option<BTreeSet<pid_t>>>;
                let _: $crate::cmp::AssertParamIsEq<Option<Kilobyte>>;
                let _: $crate::cmp::AssertParamIsEq<Option<Kilobyte>>;
                let _: $crate::cmp::AssertParamIsEq<Option<Kilobyte>>;
                let _: $crate::cmp::AssertParamIsEq<Option<Kilobyte>>;
                let _: $crate::cmp::AssertParamIsEq<Option<Kilobyte>>;
                let _: $crate::cmp::AssertParamIsEq<Option<Kilobyte>>;
                let _: $crate::cmp::AssertParamIsEq<Option<Kilobyte>>;
                let _: $crate::cmp::AssertParamIsEq<Option<Kilobyte>>;
                let _: $crate::cmp::AssertParamIsEq<Option<Kilobyte>>;
                let _: $crate::cmp::AssertParamIsEq<Option<Kilobyte>>;
                let _: $crate::cmp::AssertParamIsEq<Option<Kilobyte>>;
                let _: $crate::cmp::AssertParamIsEq<Option<Kilobyte>>;
                let _: $crate::cmp::AssertParamIsEq<Option<Kilobyte>>;
                let _: $crate::cmp::AssertParamIsEq<Option<Kilobyte>>;
                let _: $crate::cmp::AssertParamIsEq<Option<Kilobyte>>;
                let _: $crate::cmp::AssertParamIsEq<Option<Kilobyte>>;
                let _: $crate::cmp::AssertParamIsEq<Option<Kilobyte>>;
                let _: $crate::cmp::AssertParamIsEq<Option<u64>>;
                let _:
                        $crate::cmp::AssertParamIsEq<Option<SignalQueueStatus>>;
                let _: $crate::cmp::AssertParamIsEq<Option<Bitmask>>;
                let _: $crate::cmp::AssertParamIsEq<Option<Bitmask>>;
                let _: $crate::cmp::AssertParamIsEq<Option<Bitmask>>;
                let _: $crate::cmp::AssertParamIsEq<Option<Bitmask>>;
                let _: $crate::cmp::AssertParamIsEq<Option<Bitmask>>;
                let _: $crate::cmp::AssertParamIsEq<Option<Bitmask>>;
                let _: $crate::cmp::AssertParamIsEq<Option<Bitmask>>;
                let _: $crate::cmp::AssertParamIsEq<Option<Bitmask>>;
                let _: $crate::cmp::AssertParamIsEq<Option<Bitmask>>;
                let _: $crate::cmp::AssertParamIsEq<Option<Bitmask>>;
                let _: $crate::cmp::AssertParamIsEq<Option<Bitmask>>;
                let _: $crate::cmp::AssertParamIsEq<Option<SeccompMode>>;
                let _:
                        $crate::cmp::AssertParamIsEq<Option<SpeculationStoreBypassStatus>>;
                let _:
                        $crate::cmp::AssertParamIsEq<Option<HyperThreadBitmask>>;
                let _:
                        $crate::cmp::AssertParamIsEq<Option<BTreeSet<HyperThread>>>;
                let _: $crate::cmp::AssertParamIsEq<Option<NumaNodeBitmask>>;
                let _:
                        $crate::cmp::AssertParamIsEq<Option<BTreeSet<NumaNode>>>;
                let _: $crate::cmp::AssertParamIsEq<Option<u64>>;
                let _: $crate::cmp::AssertParamIsEq<Option<u64>>;
                let _:
                        $crate::cmp::AssertParamIsEq<HashMap<Box<[u8]>,
                                                             Box<[u8]>>>;
            }
        }
    }
    impl ProcessStatusStatistics {
        /// Get an unrecognised static's value using a `statistic_name` byte string.
        #[inline(always)]
        pub fn unrecognised_statistic(&self, statistic_name: &[u8])
         -> Option<&Box<[u8]>> {
            self.unrecognised.get(statistic_name)
        }
        /// Parses; returns a zero-based line number and parse error if it fails.
        pub fn parse(reader: BufReader<File>)
         -> Result<Self, ProcessStatusFileParseError> {
            use self::ProcessStatusFileParseError::*;
            let mut this = Self::default();
            let mut zero_based_line_number = 0;
            for line in reader.split(b'\n') {
                let mut line =
                    match line {
                        Err(cause) =>
                        return Err(CouldNotReadLine{zero_based_line_number,
                                                    cause,}),
                        Ok(line) => line,
                    };
                {
                    let mut split = splitn(&line, 2, b':');
                    let statistic_name = split.next().unwrap();
                    match split.next() {
                        None =>
                        return Err(CouldNotParseLine{zero_based_line_number,
                                                     cause:
                                                         ProcessStatusStatisticParseError::NoValue,}),
                        Some(tab_then_statistic_value) => {
                            if unsafe {
                                   ::std::intrinsics::unlikely(!tab_then_statistic_value.starts_with(b"\t"))
                               } {
                                return Err(CouldNotParseLine{zero_based_line_number,
                                                             cause:
                                                                 ProcessStatusStatisticParseError::ValueNotPreceededByHorizontalTab,})
                            }
                            let statistic_value =
                                &tab_then_statistic_value[1..];
                            match this.parse_line(statistic_name,
                                                  statistic_value) {
                                Err(cause) =>
                                return Err(CouldNotParseLine{zero_based_line_number,
                                                             cause,}),
                                Ok(()) => (),
                            }
                        }
                    };
                }
                zero_based_line_number += 1;
            }
            this.unrecognised.shrink_to_fit();
            Ok(this)
        }
        /// When in doubt, check the source code for status files at <https://github.com/torvalds/linux/blob/f346b0becb1bc62e45495f9cdbae3eef35d0b635/fs/proc/array.c>.
        #[inline]
        fn parse_line(&mut self, statistic_name: &[u8],
                      statistic_value: &[u8])
         -> Result<(), ProcessStatusStatisticParseError> {
            #[inline(always)]
            fn to_box(value: &[u8]) -> Box<[u8]> {
                value.to_vec().into_boxed_slice()
            }
            #[inline(always)]
            fn parse_token(value: &[u8])
             -> Result<Box<[u8]>, ProcessStatusStatisticParseError> {
                Ok(to_box(value))
            }
            #[inline(always)]
            fn parse_mode(value: &[u8])
             -> Result<mode_t, ProcessStatusStatisticParseError> {
                if unsafe { ::std::intrinsics::likely(value.len() == 4) } {
                    Ok(mode_t::from_str_radix(from_utf8(value)?, 8)?)
                } else {
                    Err(ProcessStatusStatisticParseError::InvalidLength)
                }
            }
            #[inline(always)]
            fn parse_process_state(value: &[u8])
             -> Result<ProcessState, ProcessStatusStatisticParseError> {
                if unsafe { ::std::intrinsics::unlikely(value.len() == 0) } {
                    return Err(ProcessStatusStatisticParseError::InvalidLength)
                }
                use self::ProcessState::*;
                let value =
                    match value[0] {
                        b'R' => Running,
                        b'S' => Sleeping,
                        b'D' => SleepingInAnUninterruptibleWait,
                        b'T' => TracedOrStopped,
                        b't' => TracingStop,
                        b'X' => Dead,
                        b'Z' => Zombie,
                        b'P' => Parked,
                        b'I' => Idle,
                        _ =>
                        return Err(ProcessStatusStatisticParseError::OutOfRange),
                    };
                Ok(value)
            }
            #[inline(always)]
            fn parse_pid(value: &[u8])
             -> Result<pid_t, ProcessStatusStatisticParseError> {
                Ok(pid_t::from_str_radix(from_utf8(value)?, 10)?)
            }
            #[inline(always)]
            fn parse_numa_node(value: &[u8])
             -> Result<NumaNode, ProcessStatusStatisticParseError> {
                Ok(NumaNode(u16::from_str_radix(from_utf8(value)?, 10)?))
            }
            #[inline(always)]
            fn parse_uid(value: &[u8])
             -> Result<uid_t, ProcessStatusStatisticParseError> {
                Ok(uid_t::from_str_radix(from_utf8(value)?, 10)?)
            }
            #[inline(always)]
            fn parse_gid(value: &[u8])
             -> Result<gid_t, ProcessStatusStatisticParseError> {
                Ok(gid_t::from_str_radix(from_utf8(value)?, 10)?)
            }
            #[inline(always)]
            fn parse_user_identifiers(value: &[u8])
             ->
                 Result<ProcessUserIdentifiers,
                        ProcessStatusStatisticParseError> {
                #[inline(always)]
                fn parse_subsequent<'a>(iterator:
                                            &mut impl Iterator<Item =
                                                 &'a [u8]>)
                 -> Result<uid_t, ProcessStatusStatisticParseError> {
                    if let Some(effective) = iterator.next() {
                        parse_uid(effective)
                    } else {
                        Err(ProcessStatusStatisticParseError::InvalidSeparator)
                    }
                }
                let mut iterator = splitn(value, 4, b'\t');
                Ok(ProcessUserIdentifiers{real:
                                              parse_uid(iterator.next().unwrap())?,
                                          effective:
                                              parse_subsequent(&mut iterator)?,
                                          saved_set:
                                              parse_subsequent(&mut iterator)?,
                                          file_system:
                                              parse_subsequent(&mut iterator)?,})
            }
            #[inline(always)]
            fn parse_group_identifiers(value: &[u8])
             ->
                 Result<ProcessGroupIdentifiers,
                        ProcessStatusStatisticParseError> {
                #[inline(always)]
                fn parse_subsequent<'a>(iterator:
                                            &mut impl Iterator<Item =
                                                 &'a [u8]>)
                 -> Result<gid_t, ProcessStatusStatisticParseError> {
                    if let Some(effective) = iterator.next() {
                        parse_gid(effective)
                    } else {
                        Err(ProcessStatusStatisticParseError::InvalidSeparator)
                    }
                }
                let mut iterator = splitn(value, 4, b'\t');
                Ok(ProcessGroupIdentifiers{real:
                                               parse_gid(iterator.next().unwrap())?,
                                           effective:
                                               parse_subsequent(&mut iterator)?,
                                           saved_set:
                                               parse_subsequent(&mut iterator)?,
                                           file_system:
                                               parse_subsequent(&mut iterator)?,})
            }
            #[inline(always)]
            fn parse_groups(value: &[u8])
             -> Result<BTreeSet<gid_t>, ProcessStatusStatisticParseError> {
                let mut groups = BTreeSet::new();
                for value in split(value, b' ') {
                    let was_added_for_the_first_time =
                        groups.insert(parse_gid(value)?);
                    if unsafe {
                           ::std::intrinsics::unlikely(!was_added_for_the_first_time)
                       } {
                        return Err(ProcessStatusStatisticParseError::DuplicatedStatisticValue)
                    }
                }
                Ok(groups)
            }
            #[inline(always)]
            fn parse_pids(value: &[u8])
             -> Result<BTreeSet<pid_t>, ProcessStatusStatisticParseError> {
                let mut pids = BTreeSet::new();
                for value in split(value, b'\t') {
                    let was_added_for_the_first_time =
                        pids.insert(parse_pid(value)?);
                    if unsafe {
                           ::std::intrinsics::unlikely(!was_added_for_the_first_time)
                       } {
                        return Err(ProcessStatusStatisticParseError::DuplicatedStatisticValue)
                    }
                }
                Ok(pids)
            }
            #[inline(always)]
            fn parse_u64(value: &[u8])
             -> Result<u64, ProcessStatusStatisticParseError> {
                Ok(u64::from_str_radix(from_utf8(value)?, 10)?)
            }
            #[inline(always)]
            fn parse_kb(value: &[u8])
             -> Result<Kilobyte, ProcessStatusStatisticParseError> {
                const Ending: &'static [u8] = b" kB";
                if unsafe {
                       ::std::intrinsics::likely(value.ends_with(b" kB"))
                   } {
                    parse_u64(&value[0..value.len() - Ending.len()])
                } else {
                    Err(ProcessStatusStatisticParseError::InvalidEnding)
                }
            }
            #[inline(always)]
            fn parse_signal_queue(value: &[u8])
             -> Result<SignalQueueStatus, ProcessStatusStatisticParseError> {
                let mut iterator = splitn(value, 2, b'/');
                let number_of_signals_queued =
                    parse_u64(iterator.next().unwrap())?;
                let maximum_number_of_signals_that_can_be_queued =
                    match iterator.next() {
                        None =>
                        return Err(ProcessStatusStatisticParseError::InvalidSeparator),
                        Some(maximum_number_of_signals_that_can_be_queued) =>
                        parse_u64(maximum_number_of_signals_that_can_be_queued)?,
                    };
                Ok(SignalQueueStatus{number_of_signals_queued,
                                     maximum_number_of_signals_that_can_be_queued,})
            }
            #[inline(always)]
            fn parse_bitmask(value: &[u8])
             -> Result<Bitmask, ProcessStatusStatisticParseError> {
                if unsafe { ::std::intrinsics::likely(value.len() == 16) } {
                    Ok(u64::from_str_radix(from_utf8(value)?, 16)?)
                } else {
                    Err(ProcessStatusStatisticParseError::InvalidLength)
                }
            }
            #[inline(always)]
            fn parse_bool(value: &[u8])
             -> Result<bool, ProcessStatusStatisticParseError> {
                if unsafe { ::std::intrinsics::likely(value.len() == 1) } {
                    match value[0] {
                        b'0' => Ok(false),
                        b'1' => Ok(true),
                        _ =>
                        Err(ProcessStatusStatisticParseError::OutOfRange),
                    }
                } else {
                    Err(ProcessStatusStatisticParseError::InvalidLength)
                }
            }
            #[inline(always)]
            fn parse_seccomp_mode(value: &[u8])
             -> Result<SeccompMode, ProcessStatusStatisticParseError> {
                if unsafe { ::std::intrinsics::likely(value.len() == 1) } {
                    use self::SeccompMode::*;
                    match value[0] {
                        b'0' => Ok(Off),
                        b'1' => Ok(Strict),
                        b'2' => Ok(Filter),
                        _ =>
                        Err(ProcessStatusStatisticParseError::OutOfRange),
                    }
                } else {
                    Err(ProcessStatusStatisticParseError::InvalidLength)
                }
            }
            #[inline(always)]
            fn parse_speculation_store_bypass(value: &[u8])
             ->
                 Result<SpeculationStoreBypassStatus,
                        ProcessStatusStatisticParseError> {
                use self::SpeculationStoreBypassStatus::*;
                let value =
                    match value {
                        b"unknown" => SpeculationStoreBypassStatus::Unknown,
                        b"not vulnerable" => NotVulnerable,
                        b"thread force mitigated" => ThreadForceMitigated,
                        b"thread mitigated" => ThreadMitigated,
                        b"thread vulnerable" => ThreadVulnerable,
                        b"globally mitigated" => GloballyMitigated,
                        b"vulnerable" => Vulnerable,
                        _ =>
                        return Err(ProcessStatusStatisticParseError::OutOfRange),
                    };
                Ok(value)
            }
            #[inline(always)]
            fn parse_cpus_or_numa_nodes_allowed_bitmask(value: &[u8])
             -> Result<u32, ProcessStatusStatisticParseError> {
                if unsafe {
                       ::std::intrinsics::likely(value.len() <= 8 &&
                                                     value.len() != 0)
                   } {
                    Ok(u32::from_str_radix(from_utf8(value)?, 16)?)
                } else {
                    Err(ProcessStatusStatisticParseError::InvalidLength)
                }
            }
            #[inline(always)]
            fn parse_cpus_allowed_list(value: &[u8])
             ->
                 Result<BTreeSet<HyperThread>,
                        ProcessStatusStatisticParseError> {
                Ok(ListParseError::parse_linux_list_string(value,
                                                           HyperThread)?)
            }
            #[inline(always)]
            fn parse_numa_nodes_allowed_list(value: &[u8])
             -> Result<BTreeSet<NumaNode>, ProcessStatusStatisticParseError> {
                Ok(ListParseError::parse_linux_list_string(value, NumaNode)?)
            }
            macro_rules! parse((
                               $ statistic_name : ident , $ statistic_value :
                               ident , $ (
                               $ proc_status_name : literal => $ struct_field
                               : ident @ $ parse_expr : ident , ) * ) => {
                               match $ statistic_name {
                               $ (
                               $ proc_status_name => if unlikely ! (
                               self . $ struct_field . is_some (  ) ) {
                               Err (
                               ProcessStatusStatisticParseError ::
                               DuplicatedStatistic ) } else {
                               let result = $ parse_expr ( $ statistic_value )
                               ; let parsed_value = result ? ; let some = Some
                               ( parsed_value ) ; self . $ struct_field = some
                               ; Ok ( (  ) ) } , ) * _ => {
                               let previous = self . unrecognised . insert (
                               to_box ( $ statistic_name ) , to_box (
                               $ statistic_value ) ) ; return if unlikely ! (
                               previous . is_some (  ) ) {
                               Err (
                               ProcessStatusStatisticParseError ::
                               DuplicatedStatistic ) } else { Ok ( (  ) ) } }
                               } });
            match statistic_name {
                b"Name" =>
                if unsafe {
                       ::std::intrinsics::unlikely(self.process_name.is_some())
                   } {
                    Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
                } else {
                    let result = parse_token(statistic_value);
                    let parsed_value = result?;
                    let some = Some(parsed_value);
                    self.process_name = some;
                    Ok(())
                },
                b"Umask" =>
                if unsafe {
                       ::std::intrinsics::unlikely(self.file_mode_creation_mask.is_some())
                   } {
                    Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
                } else {
                    let result = parse_mode(statistic_value);
                    let parsed_value = result?;
                    let some = Some(parsed_value);
                    self.file_mode_creation_mask = some;
                    Ok(())
                },
                b"Tgid" =>
                if unsafe {
                       ::std::intrinsics::unlikely(self.thread_group_identifier.is_some())
                   } {
                    Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
                } else {
                    let result = parse_pid(statistic_value);
                    let parsed_value = result?;
                    let some = Some(parsed_value);
                    self.thread_group_identifier = some;
                    Ok(())
                },
                b"Ngid" =>
                if unsafe {
                       ::std::intrinsics::unlikely(self.numa_group_identifier.is_some())
                   } {
                    Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
                } else {
                    let result = parse_numa_node(statistic_value);
                    let parsed_value = result?;
                    let some = Some(parsed_value);
                    self.numa_group_identifier = some;
                    Ok(())
                },
                b"Pid" =>
                if unsafe {
                       ::std::intrinsics::unlikely(self.process_identifier.is_some())
                   } {
                    Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
                } else {
                    let result = parse_pid(statistic_value);
                    let parsed_value = result?;
                    let some = Some(parsed_value);
                    self.process_identifier = some;
                    Ok(())
                },
                b"PPid" =>
                if unsafe {
                       ::std::intrinsics::unlikely(self.parent_process_identifier.is_some())
                   } {
                    Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
                } else {
                    let result = parse_pid(statistic_value);
                    let parsed_value = result?;
                    let some = Some(parsed_value);
                    self.parent_process_identifier = some;
                    Ok(())
                },
                b"TracerPid" =>
                if unsafe {
                       ::std::intrinsics::unlikely(self.tracer_process_identifier.is_some())
                   } {
                    Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
                } else {
                    let result = parse_pid(statistic_value);
                    let parsed_value = result?;
                    let some = Some(parsed_value);
                    self.tracer_process_identifier = some;
                    Ok(())
                },
                b"Uid" =>
                if unsafe {
                       ::std::intrinsics::unlikely(self.user_identifiers.is_some())
                   } {
                    Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
                } else {
                    let result = parse_user_identifiers(statistic_value);
                    let parsed_value = result?;
                    let some = Some(parsed_value);
                    self.user_identifiers = some;
                    Ok(())
                },
                b"Gid" =>
                if unsafe {
                       ::std::intrinsics::unlikely(self.group_identifiers.is_some())
                   } {
                    Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
                } else {
                    let result = parse_group_identifiers(statistic_value);
                    let parsed_value = result?;
                    let some = Some(parsed_value);
                    self.group_identifiers = some;
                    Ok(())
                },
                b"FDSize" =>
                if unsafe {
                       ::std::intrinsics::unlikely(self.number_of_file_descriptor_slots_currently_allocated.is_some())
                   } {
                    Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
                } else {
                    let result = parse_u64(statistic_value);
                    let parsed_value = result?;
                    let some = Some(parsed_value);
                    self.number_of_file_descriptor_slots_currently_allocated =
                        some;
                    Ok(())
                },
                b"Groups" =>
                if unsafe {
                       ::std::intrinsics::unlikely(self.groups.is_some())
                   } {
                    Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
                } else {
                    let result = parse_groups(statistic_value);
                    let parsed_value = result?;
                    let some = Some(parsed_value);
                    self.groups = some;
                    Ok(())
                },
                b"NStgid" =>
                if unsafe {
                       ::std::intrinsics::unlikely(self.descendant_namespace_thread_group_identifier.is_some())
                   } {
                    Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
                } else {
                    let result = parse_pids(statistic_value);
                    let parsed_value = result?;
                    let some = Some(parsed_value);
                    self.descendant_namespace_thread_group_identifier = some;
                    Ok(())
                },
                b"NSpid" =>
                if unsafe {
                       ::std::intrinsics::unlikely(self.descendant_namespace_process_identifier.is_some())
                   } {
                    Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
                } else {
                    let result = parse_pids(statistic_value);
                    let parsed_value = result?;
                    let some = Some(parsed_value);
                    self.descendant_namespace_process_identifier = some;
                    Ok(())
                },
                b"NSpgid" =>
                if unsafe {
                       ::std::intrinsics::unlikely(self.descendant_namespace_process_group_identifier.is_some())
                   } {
                    Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
                } else {
                    let result = parse_pids(statistic_value);
                    let parsed_value = result?;
                    let some = Some(parsed_value);
                    self.descendant_namespace_process_group_identifier = some;
                    Ok(())
                },
                b"NSsid" =>
                if unsafe {
                       ::std::intrinsics::unlikely(self.descendant_namespace_session_identifier.is_some())
                   } {
                    Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
                } else {
                    let result = parse_pids(statistic_value);
                    let parsed_value = result?;
                    let some = Some(parsed_value);
                    self.descendant_namespace_session_identifier = some;
                    Ok(())
                },
                b"VmPeak" =>
                if unsafe {
                       ::std::intrinsics::unlikely(self.peak_virtual_memory_size.is_some())
                   } {
                    Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
                } else {
                    let result = parse_kb(statistic_value);
                    let parsed_value = result?;
                    let some = Some(parsed_value);
                    self.peak_virtual_memory_size = some;
                    Ok(())
                },
                b"VmSize" =>
                if unsafe {
                       ::std::intrinsics::unlikely(self.total_program_size.is_some())
                   } {
                    Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
                } else {
                    let result = parse_kb(statistic_value);
                    let parsed_value = result?;
                    let some = Some(parsed_value);
                    self.total_program_size = some;
                    Ok(())
                },
                b"VmLck" =>
                if unsafe {
                       ::std::intrinsics::unlikely(self.locked_memory_size.is_some())
                   } {
                    Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
                } else {
                    let result = parse_kb(statistic_value);
                    let parsed_value = result?;
                    let some = Some(parsed_value);
                    self.locked_memory_size = some;
                    Ok(())
                },
                b"VmPin" =>
                if unsafe {
                       ::std::intrinsics::unlikely(self.pinned_memory_size.is_some())
                   } {
                    Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
                } else {
                    let result = parse_kb(statistic_value);
                    let parsed_value = result?;
                    let some = Some(parsed_value);
                    self.pinned_memory_size = some;
                    Ok(())
                },
                b"VmHWM" =>
                if unsafe {
                       ::std::intrinsics::unlikely(self.peak_resident_set_size.is_some())
                   } {
                    Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
                } else {
                    let result = parse_kb(statistic_value);
                    let parsed_value = result?;
                    let some = Some(parsed_value);
                    self.peak_resident_set_size = some;
                    Ok(())
                },
                b"VmRSS" =>
                if unsafe {
                       ::std::intrinsics::unlikely(self.resident_set_memory_size.is_some())
                   } {
                    Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
                } else {
                    let result = parse_kb(statistic_value);
                    let parsed_value = result?;
                    let some = Some(parsed_value);
                    self.resident_set_memory_size = some;
                    Ok(())
                },
                b"RssAnon" =>
                if unsafe {
                       ::std::intrinsics::unlikely(self.anonymous_resident_set_memory_size.is_some())
                   } {
                    Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
                } else {
                    let result = parse_kb(statistic_value);
                    let parsed_value = result?;
                    let some = Some(parsed_value);
                    self.anonymous_resident_set_memory_size = some;
                    Ok(())
                },
                b"RssFile" =>
                if unsafe {
                       ::std::intrinsics::unlikely(self.resident_set_file_mappings_memory_size.is_some())
                   } {
                    Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
                } else {
                    let result = parse_kb(statistic_value);
                    let parsed_value = result?;
                    let some = Some(parsed_value);
                    self.resident_set_file_mappings_memory_size = some;
                    Ok(())
                },
                b"RssShmem" =>
                if unsafe {
                       ::std::intrinsics::unlikely(self.resident_set_shared_memory_size.is_some())
                   } {
                    Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
                } else {
                    let result = parse_kb(statistic_value);
                    let parsed_value = result?;
                    let some = Some(parsed_value);
                    self.resident_set_shared_memory_size = some;
                    Ok(())
                },
                b"VmData" =>
                if unsafe {
                       ::std::intrinsics::unlikely(self.private_data_segments_size.is_some())
                   } {
                    Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
                } else {
                    let result = parse_kb(statistic_value);
                    let parsed_value = result?;
                    let some = Some(parsed_value);
                    self.private_data_segments_size = some;
                    Ok(())
                },
                b"VmStk" =>
                if unsafe {
                       ::std::intrinsics::unlikely(self.stack_segments_size.is_some())
                   } {
                    Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
                } else {
                    let result = parse_kb(statistic_value);
                    let parsed_value = result?;
                    let some = Some(parsed_value);
                    self.stack_segments_size = some;
                    Ok(())
                },
                b"VmExe" =>
                if unsafe {
                       ::std::intrinsics::unlikely(self.text_segment_size.is_some())
                   } {
                    Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
                } else {
                    let result = parse_kb(statistic_value);
                    let parsed_value = result?;
                    let some = Some(parsed_value);
                    self.text_segment_size = some;
                    Ok(())
                },
                b"VmLi" =>
                if unsafe {
                       ::std::intrinsics::unlikely(self.dynamically_loaded_shared_library_size.is_some())
                   } {
                    Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
                } else {
                    let result = parse_kb(statistic_value);
                    let parsed_value = result?;
                    let some = Some(parsed_value);
                    self.dynamically_loaded_shared_library_size = some;
                    Ok(())
                },
                b"VmPTE" =>
                if unsafe {
                       ::std::intrinsics::unlikely(self.page_table_entries_size.is_some())
                   } {
                    Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
                } else {
                    let result = parse_kb(statistic_value);
                    let parsed_value = result?;
                    let some = Some(parsed_value);
                    self.page_table_entries_size = some;
                    Ok(())
                },
                b"VmPMD" =>
                if unsafe {
                       ::std::intrinsics::unlikely(self.vm_pmd.is_some())
                   } {
                    Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
                } else {
                    let result = parse_kb(statistic_value);
                    let parsed_value = result?;
                    let some = Some(parsed_value);
                    self.vm_pmd = some;
                    Ok(())
                },
                b"VmSwap" =>
                if unsafe {
                       ::std::intrinsics::unlikely(self.swap_memory_size.is_some())
                   } {
                    Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
                } else {
                    let result = parse_kb(statistic_value);
                    let parsed_value = result?;
                    let some = Some(parsed_value);
                    self.swap_memory_size = some;
                    Ok(())
                },
                b"HugetlbPages" =>
                if unsafe {
                       ::std::intrinsics::unlikely(self.huge_tlb_pages_memory_size.is_some())
                   } {
                    Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
                } else {
                    let result = parse_kb(statistic_value);
                    let parsed_value = result?;
                    let some = Some(parsed_value);
                    self.huge_tlb_pages_memory_size = some;
                    Ok(())
                },
                b"Threads" =>
                if unsafe {
                       ::std::intrinsics::unlikely(self.threads.is_some())
                   } {
                    Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
                } else {
                    let result = parse_u64(statistic_value);
                    let parsed_value = result?;
                    let some = Some(parsed_value);
                    self.threads = some;
                    Ok(())
                },
                b"SigQ" =>
                if unsafe {
                       ::std::intrinsics::unlikely(self.signal_queue.is_some())
                   } {
                    Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
                } else {
                    let result = parse_signal_queue(statistic_value);
                    let parsed_value = result?;
                    let some = Some(parsed_value);
                    self.signal_queue = some;
                    Ok(())
                },
                b"SigPnd" =>
                if unsafe {
                       ::std::intrinsics::unlikely(self.thread_pending_signals.is_some())
                   } {
                    Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
                } else {
                    let result = parse_bitmask(statistic_value);
                    let parsed_value = result?;
                    let some = Some(parsed_value);
                    self.thread_pending_signals = some;
                    Ok(())
                },
                b"ShdPnd" =>
                if unsafe {
                       ::std::intrinsics::unlikely(self.process_shared_pending_signals.is_some())
                   } {
                    Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
                } else {
                    let result = parse_bitmask(statistic_value);
                    let parsed_value = result?;
                    let some = Some(parsed_value);
                    self.process_shared_pending_signals = some;
                    Ok(())
                },
                b"SigBlk" =>
                if unsafe {
                       ::std::intrinsics::unlikely(self.blocked_signals.is_some())
                   } {
                    Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
                } else {
                    let result = parse_bitmask(statistic_value);
                    let parsed_value = result?;
                    let some = Some(parsed_value);
                    self.blocked_signals = some;
                    Ok(())
                },
                b"SigIgn" =>
                if unsafe {
                       ::std::intrinsics::unlikely(self.ignored_signals.is_some())
                   } {
                    Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
                } else {
                    let result = parse_bitmask(statistic_value);
                    let parsed_value = result?;
                    let some = Some(parsed_value);
                    self.ignored_signals = some;
                    Ok(())
                },
                b"SigCgt" =>
                if unsafe {
                       ::std::intrinsics::unlikely(self.caught_signals.is_some())
                   } {
                    Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
                } else {
                    let result = parse_bitmask(statistic_value);
                    let parsed_value = result?;
                    let some = Some(parsed_value);
                    self.caught_signals = some;
                    Ok(())
                },
                b"CapInh" =>
                if unsafe {
                       ::std::intrinsics::unlikely(self.inheritable_capabilities.is_some())
                   } {
                    Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
                } else {
                    let result = parse_bitmask(statistic_value);
                    let parsed_value = result?;
                    let some = Some(parsed_value);
                    self.inheritable_capabilities = some;
                    Ok(())
                },
                b"CapPrm" =>
                if unsafe {
                       ::std::intrinsics::unlikely(self.permitted_capabilities.is_some())
                   } {
                    Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
                } else {
                    let result = parse_bitmask(statistic_value);
                    let parsed_value = result?;
                    let some = Some(parsed_value);
                    self.permitted_capabilities = some;
                    Ok(())
                },
                b"CapEff" =>
                if unsafe {
                       ::std::intrinsics::unlikely(self.effective_capabilities.is_some())
                   } {
                    Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
                } else {
                    let result = parse_bitmask(statistic_value);
                    let parsed_value = result?;
                    let some = Some(parsed_value);
                    self.effective_capabilities = some;
                    Ok(())
                },
                b"CapBnd" =>
                if unsafe {
                       ::std::intrinsics::unlikely(self.capabilities_bounding_set.is_some())
                   } {
                    Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
                } else {
                    let result = parse_bitmask(statistic_value);
                    let parsed_value = result?;
                    let some = Some(parsed_value);
                    self.capabilities_bounding_set = some;
                    Ok(())
                },
                b"CapAm" =>
                if unsafe {
                       ::std::intrinsics::unlikely(self.ambient_capabilities.is_some())
                   } {
                    Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
                } else {
                    let result = parse_bitmask(statistic_value);
                    let parsed_value = result?;
                    let some = Some(parsed_value);
                    self.ambient_capabilities = some;
                    Ok(())
                },
                b"NoNewPrivs" =>
                if unsafe {
                       ::std::intrinsics::unlikely(self.thread_no_new_privileges_bit.is_some())
                   } {
                    Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
                } else {
                    let result = parse_bool(statistic_value);
                    let parsed_value = result?;
                    let some = Some(parsed_value);
                    self.thread_no_new_privileges_bit = some;
                    Ok(())
                },
                b"Seccomp" =>
                if unsafe {
                       ::std::intrinsics::unlikely(self.seccomp_mode.is_some())
                   } {
                    Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
                } else {
                    let result = parse_seccomp_mode(statistic_value);
                    let parsed_value = result?;
                    let some = Some(parsed_value);
                    self.seccomp_mode = some;
                    Ok(())
                },
                b"Speculation_Store_Bypass" =>
                if unsafe {
                       ::std::intrinsics::unlikely(self.speculation_store_bypass.is_some())
                   } {
                    Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
                } else {
                    let result =
                        parse_speculation_store_bypass(statistic_value);
                    let parsed_value = result?;
                    let some = Some(parsed_value);
                    self.speculation_store_bypass = some;
                    Ok(())
                },
                b"Cpus_allowed" =>
                if unsafe {
                       ::std::intrinsics::unlikely(self.cpus_allowed_bitmask.is_some())
                   } {
                    Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
                } else {
                    let result =
                        parse_cpus_or_numa_nodes_allowed_bitmask(statistic_value);
                    let parsed_value = result?;
                    let some = Some(parsed_value);
                    self.cpus_allowed_bitmask = some;
                    Ok(())
                },
                b"Cpus_allowed_list" =>
                if unsafe {
                       ::std::intrinsics::unlikely(self.cpus_allowed_list.is_some())
                   } {
                    Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
                } else {
                    let result = parse_cpus_allowed_list(statistic_value);
                    let parsed_value = result?;
                    let some = Some(parsed_value);
                    self.cpus_allowed_list = some;
                    Ok(())
                },
                b"Mems_allowed" =>
                if unsafe {
                       ::std::intrinsics::unlikely(self.numa_nodes_allowed_bitmask.is_some())
                   } {
                    Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
                } else {
                    let result =
                        parse_cpus_or_numa_nodes_allowed_bitmask(statistic_value);
                    let parsed_value = result?;
                    let some = Some(parsed_value);
                    self.numa_nodes_allowed_bitmask = some;
                    Ok(())
                },
                b"Mems_allowed_list" =>
                if unsafe {
                       ::std::intrinsics::unlikely(self.numa_nodes_allowed_list.is_some())
                   } {
                    Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
                } else {
                    let result =
                        parse_numa_nodes_allowed_list(statistic_value);
                    let parsed_value = result?;
                    let some = Some(parsed_value);
                    self.numa_nodes_allowed_list = some;
                    Ok(())
                },
                b"voluntary_ctxt_switches" =>
                if unsafe {
                       ::std::intrinsics::unlikely(self.voluntary_context_switches.is_some())
                   } {
                    Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
                } else {
                    let result = parse_u64(statistic_value);
                    let parsed_value = result?;
                    let some = Some(parsed_value);
                    self.voluntary_context_switches = some;
                    Ok(())
                },
                b"nonvoluntary_ctxt_switches" =>
                if unsafe {
                       ::std::intrinsics::unlikely(self.involuntary_context_switches.is_some())
                   } {
                    Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
                } else {
                    let result = parse_u64(statistic_value);
                    let parsed_value = result?;
                    let some = Some(parsed_value);
                    self.involuntary_context_switches = some;
                    Ok(())
                },
                _ => {
                    let previous =
                        self.unrecognised.insert(to_box(statistic_name),
                                                 to_box(statistic_value));
                    return if unsafe {
                                  ::std::intrinsics::unlikely(previous.is_some())
                              } {
                               Err(ProcessStatusStatisticParseError::DuplicatedStatistic)
                           } else { Ok(()) }
                }
            }
        }
    }
    /// User identifiers (UIDs).
    #[structural_match]
    pub struct ProcessUserIdentifiers {
        /// Real user identifier (UID).
        pub real: uid_t,
        /// Effective user identifier (UID).
        pub effective: uid_t,
        /// Saved set user identifier (UID).
        pub saved_set: uid_t,
        /// File system user identifier (UID).
        pub file_system: uid_t,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::default::Default for ProcessUserIdentifiers {
        #[inline]
        fn default() -> ProcessUserIdentifiers {
            ProcessUserIdentifiers{real: $crate::default::Default::default(),
                                   effective:
                                       $crate::default::Default::default(),
                                   saved_set:
                                       $crate::default::Default::default(),
                                   file_system:
                                       $crate::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::fmt::Debug for ProcessUserIdentifiers {
        fn fmt(&self, f: &mut $crate::fmt::Formatter) -> $crate::fmt::Result {
            match *self {
                ProcessUserIdentifiers {
                real: ref __self_0_0,
                effective: ref __self_0_1,
                saved_set: ref __self_0_2,
                file_system: ref __self_0_3 } => {
                    let mut debug_trait_builder =
                        f.debug_struct("ProcessUserIdentifiers");
                    let _ =
                        debug_trait_builder.field("real", &&(*__self_0_0));
                    let _ =
                        debug_trait_builder.field("effective",
                                                  &&(*__self_0_1));
                    let _ =
                        debug_trait_builder.field("saved_set",
                                                  &&(*__self_0_2));
                    let _ =
                        debug_trait_builder.field("file_system",
                                                  &&(*__self_0_3));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::clone::Clone for ProcessUserIdentifiers {
        #[inline]
        fn clone(&self) -> ProcessUserIdentifiers {
            match *self {
                ProcessUserIdentifiers {
                real: ref __self_0_0,
                effective: ref __self_0_1,
                saved_set: ref __self_0_2,
                file_system: ref __self_0_3 } =>
                ProcessUserIdentifiers{real:
                                           $crate::clone::Clone::clone(&(*__self_0_0)),
                                       effective:
                                           $crate::clone::Clone::clone(&(*__self_0_1)),
                                       saved_set:
                                           $crate::clone::Clone::clone(&(*__self_0_2)),
                                       file_system:
                                           $crate::clone::Clone::clone(&(*__self_0_3)),},
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::cmp::PartialEq for ProcessUserIdentifiers {
        #[inline]
        fn eq(&self, other: &ProcessUserIdentifiers) -> bool {
            match *other {
                ProcessUserIdentifiers {
                real: ref __self_1_0,
                effective: ref __self_1_1,
                saved_set: ref __self_1_2,
                file_system: ref __self_1_3 } =>
                match *self {
                    ProcessUserIdentifiers {
                    real: ref __self_0_0,
                    effective: ref __self_0_1,
                    saved_set: ref __self_0_2,
                    file_system: ref __self_0_3 } =>
                    (*__self_0_0) == (*__self_1_0) &&
                        (*__self_0_1) == (*__self_1_1) &&
                        (*__self_0_2) == (*__self_1_2) &&
                        (*__self_0_3) == (*__self_1_3),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &ProcessUserIdentifiers) -> bool {
            match *other {
                ProcessUserIdentifiers {
                real: ref __self_1_0,
                effective: ref __self_1_1,
                saved_set: ref __self_1_2,
                file_system: ref __self_1_3 } =>
                match *self {
                    ProcessUserIdentifiers {
                    real: ref __self_0_0,
                    effective: ref __self_0_1,
                    saved_set: ref __self_0_2,
                    file_system: ref __self_0_3 } =>
                    (*__self_0_0) != (*__self_1_0) ||
                        (*__self_0_1) != (*__self_1_1) ||
                        (*__self_0_2) != (*__self_1_2) ||
                        (*__self_0_3) != (*__self_1_3),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::cmp::Eq for ProcessUserIdentifiers {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: $crate::cmp::AssertParamIsEq<uid_t>;
                let _: $crate::cmp::AssertParamIsEq<uid_t>;
                let _: $crate::cmp::AssertParamIsEq<uid_t>;
                let _: $crate::cmp::AssertParamIsEq<uid_t>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::cmp::PartialOrd for ProcessUserIdentifiers {
        #[inline]
        fn partial_cmp(&self, other: &ProcessUserIdentifiers)
         -> $crate::option::Option<$crate::cmp::Ordering> {
            match *other {
                ProcessUserIdentifiers {
                real: ref __self_1_0,
                effective: ref __self_1_1,
                saved_set: ref __self_1_2,
                file_system: ref __self_1_3 } =>
                match *self {
                    ProcessUserIdentifiers {
                    real: ref __self_0_0,
                    effective: ref __self_0_1,
                    saved_set: ref __self_0_2,
                    file_system: ref __self_0_3 } =>
                    match $crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                               &(*__self_1_0))
                        {
                        $crate::option::Option::Some($crate::cmp::Ordering::Equal)
                        =>
                        match $crate::cmp::PartialOrd::partial_cmp(&(*__self_0_1),
                                                                   &(*__self_1_1))
                            {
                            $crate::option::Option::Some($crate::cmp::Ordering::Equal)
                            =>
                            match $crate::cmp::PartialOrd::partial_cmp(&(*__self_0_2),
                                                                       &(*__self_1_2))
                                {
                                $crate::option::Option::Some($crate::cmp::Ordering::Equal)
                                =>
                                match $crate::cmp::PartialOrd::partial_cmp(&(*__self_0_3),
                                                                           &(*__self_1_3))
                                    {
                                    $crate::option::Option::Some($crate::cmp::Ordering::Equal)
                                    =>
                                    $crate::option::Option::Some($crate::cmp::Ordering::Equal),
                                    cmp => cmp,
                                },
                                cmp => cmp,
                            },
                            cmp => cmp,
                        },
                        cmp => cmp,
                    },
                },
            }
        }
        #[inline]
        fn lt(&self, other: &ProcessUserIdentifiers) -> bool {
            match *other {
                ProcessUserIdentifiers {
                real: ref __self_1_0,
                effective: ref __self_1_1,
                saved_set: ref __self_1_2,
                file_system: ref __self_1_3 } =>
                match *self {
                    ProcessUserIdentifiers {
                    real: ref __self_0_0,
                    effective: ref __self_0_1,
                    saved_set: ref __self_0_2,
                    file_system: ref __self_0_3 } =>
                    $crate::cmp::Ordering::then_with($crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                                                            &(*__self_1_0)),
                                                                                       $crate::cmp::Ordering::Equal),
                                                     ||
                                                         $crate::cmp::Ordering::then_with($crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_1),
                                                                                                                                                                 &(*__self_1_1)),
                                                                                                                            $crate::cmp::Ordering::Equal),
                                                                                          ||
                                                                                              $crate::cmp::Ordering::then_with($crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_2),
                                                                                                                                                                                                      &(*__self_1_2)),
                                                                                                                                                                 $crate::cmp::Ordering::Equal),
                                                                                                                               ||
                                                                                                                                   $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_3),
                                                                                                                                                                                                          &(*__self_1_3)),
                                                                                                                                                                     $crate::cmp::Ordering::Greater))))
                        == $crate::cmp::Ordering::Less,
                },
            }
        }
        #[inline]
        fn le(&self, other: &ProcessUserIdentifiers) -> bool {
            match *other {
                ProcessUserIdentifiers {
                real: ref __self_1_0,
                effective: ref __self_1_1,
                saved_set: ref __self_1_2,
                file_system: ref __self_1_3 } =>
                match *self {
                    ProcessUserIdentifiers {
                    real: ref __self_0_0,
                    effective: ref __self_0_1,
                    saved_set: ref __self_0_2,
                    file_system: ref __self_0_3 } =>
                    $crate::cmp::Ordering::then_with($crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                                                            &(*__self_1_0)),
                                                                                       $crate::cmp::Ordering::Equal),
                                                     ||
                                                         $crate::cmp::Ordering::then_with($crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_1),
                                                                                                                                                                 &(*__self_1_1)),
                                                                                                                            $crate::cmp::Ordering::Equal),
                                                                                          ||
                                                                                              $crate::cmp::Ordering::then_with($crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_2),
                                                                                                                                                                                                      &(*__self_1_2)),
                                                                                                                                                                 $crate::cmp::Ordering::Equal),
                                                                                                                               ||
                                                                                                                                   $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_3),
                                                                                                                                                                                                          &(*__self_1_3)),
                                                                                                                                                                     $crate::cmp::Ordering::Greater))))
                        != $crate::cmp::Ordering::Greater,
                },
            }
        }
        #[inline]
        fn gt(&self, other: &ProcessUserIdentifiers) -> bool {
            match *other {
                ProcessUserIdentifiers {
                real: ref __self_1_0,
                effective: ref __self_1_1,
                saved_set: ref __self_1_2,
                file_system: ref __self_1_3 } =>
                match *self {
                    ProcessUserIdentifiers {
                    real: ref __self_0_0,
                    effective: ref __self_0_1,
                    saved_set: ref __self_0_2,
                    file_system: ref __self_0_3 } =>
                    $crate::cmp::Ordering::then_with($crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                                                            &(*__self_1_0)),
                                                                                       $crate::cmp::Ordering::Equal),
                                                     ||
                                                         $crate::cmp::Ordering::then_with($crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_1),
                                                                                                                                                                 &(*__self_1_1)),
                                                                                                                            $crate::cmp::Ordering::Equal),
                                                                                          ||
                                                                                              $crate::cmp::Ordering::then_with($crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_2),
                                                                                                                                                                                                      &(*__self_1_2)),
                                                                                                                                                                 $crate::cmp::Ordering::Equal),
                                                                                                                               ||
                                                                                                                                   $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_3),
                                                                                                                                                                                                          &(*__self_1_3)),
                                                                                                                                                                     $crate::cmp::Ordering::Less))))
                        == $crate::cmp::Ordering::Greater,
                },
            }
        }
        #[inline]
        fn ge(&self, other: &ProcessUserIdentifiers) -> bool {
            match *other {
                ProcessUserIdentifiers {
                real: ref __self_1_0,
                effective: ref __self_1_1,
                saved_set: ref __self_1_2,
                file_system: ref __self_1_3 } =>
                match *self {
                    ProcessUserIdentifiers {
                    real: ref __self_0_0,
                    effective: ref __self_0_1,
                    saved_set: ref __self_0_2,
                    file_system: ref __self_0_3 } =>
                    $crate::cmp::Ordering::then_with($crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                                                            &(*__self_1_0)),
                                                                                       $crate::cmp::Ordering::Equal),
                                                     ||
                                                         $crate::cmp::Ordering::then_with($crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_1),
                                                                                                                                                                 &(*__self_1_1)),
                                                                                                                            $crate::cmp::Ordering::Equal),
                                                                                          ||
                                                                                              $crate::cmp::Ordering::then_with($crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_2),
                                                                                                                                                                                                      &(*__self_1_2)),
                                                                                                                                                                 $crate::cmp::Ordering::Equal),
                                                                                                                               ||
                                                                                                                                   $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_3),
                                                                                                                                                                                                          &(*__self_1_3)),
                                                                                                                                                                     $crate::cmp::Ordering::Less))))
                        != $crate::cmp::Ordering::Less,
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::cmp::Ord for ProcessUserIdentifiers {
        #[inline]
        fn cmp(&self, other: &ProcessUserIdentifiers)
         -> $crate::cmp::Ordering {
            match *other {
                ProcessUserIdentifiers {
                real: ref __self_1_0,
                effective: ref __self_1_1,
                saved_set: ref __self_1_2,
                file_system: ref __self_1_3 } =>
                match *self {
                    ProcessUserIdentifiers {
                    real: ref __self_0_0,
                    effective: ref __self_0_1,
                    saved_set: ref __self_0_2,
                    file_system: ref __self_0_3 } =>
                    match $crate::cmp::Ord::cmp(&(*__self_0_0),
                                                &(*__self_1_0)) {
                        $crate::cmp::Ordering::Equal =>
                        match $crate::cmp::Ord::cmp(&(*__self_0_1),
                                                    &(*__self_1_1)) {
                            $crate::cmp::Ordering::Equal =>
                            match $crate::cmp::Ord::cmp(&(*__self_0_2),
                                                        &(*__self_1_2)) {
                                $crate::cmp::Ordering::Equal =>
                                match $crate::cmp::Ord::cmp(&(*__self_0_3),
                                                            &(*__self_1_3)) {
                                    $crate::cmp::Ordering::Equal =>
                                    $crate::cmp::Ordering::Equal,
                                    cmp => cmp,
                                },
                                cmp => cmp,
                            },
                            cmp => cmp,
                        },
                        cmp => cmp,
                    },
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::hash::Hash for ProcessUserIdentifiers {
        fn hash<__H: $crate::hash::Hasher>(&self, state: &mut __H) -> () {
            match *self {
                ProcessUserIdentifiers {
                real: ref __self_0_0,
                effective: ref __self_0_1,
                saved_set: ref __self_0_2,
                file_system: ref __self_0_3 } => {
                    $crate::hash::Hash::hash(&(*__self_0_0), state);
                    $crate::hash::Hash::hash(&(*__self_0_1), state);
                    $crate::hash::Hash::hash(&(*__self_0_2), state);
                    $crate::hash::Hash::hash(&(*__self_0_3), state)
                }
            }
        }
    }
    /// `seccomp` mode.
    #[repr(u32)]
    #[structural_match]
    #[rustc_copy_clone_marker]
    pub enum SeccompMode {

        /// Off.
        Off = 0,

        /// Strict.
        Strict = 1,

        /// Filter.
        Filter = 2,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::fmt::Debug for SeccompMode {
        fn fmt(&self, f: &mut $crate::fmt::Formatter) -> $crate::fmt::Result {
            match (&*self,) {
                (&SeccompMode::Off,) => {
                    let mut debug_trait_builder = f.debug_tuple("Off");
                    debug_trait_builder.finish()
                }
                (&SeccompMode::Strict,) => {
                    let mut debug_trait_builder = f.debug_tuple("Strict");
                    debug_trait_builder.finish()
                }
                (&SeccompMode::Filter,) => {
                    let mut debug_trait_builder = f.debug_tuple("Filter");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::marker::Copy for SeccompMode { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::clone::Clone for SeccompMode {
        #[inline]
        fn clone(&self) -> SeccompMode { { *self } }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::cmp::PartialEq for SeccompMode {
        #[inline]
        fn eq(&self, other: &SeccompMode) -> bool {
            {
                let __self_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*self) }
                        as u32;
                let __arg_1_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*other) }
                        as u32;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) { _ => true, }
                } else { false }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::cmp::Eq for SeccompMode {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () { { } }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::cmp::PartialOrd for SeccompMode {
        #[inline]
        fn partial_cmp(&self, other: &SeccompMode)
         -> $crate::option::Option<$crate::cmp::Ordering> {
            {
                let __self_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*self) }
                        as u32;
                let __arg_1_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*other) }
                        as u32;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        _ =>
                        $crate::option::Option::Some($crate::cmp::Ordering::Equal),
                    }
                } else { __self_vi.partial_cmp(&__arg_1_vi) }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::cmp::Ord for SeccompMode {
        #[inline]
        fn cmp(&self, other: &SeccompMode) -> $crate::cmp::Ordering {
            {
                let __self_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*self) }
                        as u32;
                let __arg_1_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*other) }
                        as u32;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        _ => $crate::cmp::Ordering::Equal,
                    }
                } else { __self_vi.cmp(&__arg_1_vi) }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::hash::Hash for SeccompMode {
        fn hash<__H: $crate::hash::Hasher>(&self, state: &mut __H) -> () {
            match (&*self,) {
                _ => {
                    $crate::hash::Hash::hash(&unsafe {
                                                  $crate::intrinsics::discriminant_value(self)
                                              }, state)
                }
            }
        }
    }
    /// Signal queue status.
    #[structural_match]
    #[rustc_copy_clone_marker]
    pub struct SignalQueueStatus {
        /// Number of signals queued.
        pub number_of_signals_queued: u64,
        /// Maximum number of signals that can be queued (maximum queue depth).
        pub maximum_number_of_signals_that_can_be_queued: u64,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::default::Default for SignalQueueStatus {
        #[inline]
        fn default() -> SignalQueueStatus {
            SignalQueueStatus{number_of_signals_queued:
                                  $crate::default::Default::default(),
                              maximum_number_of_signals_that_can_be_queued:
                                  $crate::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::fmt::Debug for SignalQueueStatus {
        fn fmt(&self, f: &mut $crate::fmt::Formatter) -> $crate::fmt::Result {
            match *self {
                SignalQueueStatus {
                number_of_signals_queued: ref __self_0_0,
                maximum_number_of_signals_that_can_be_queued: ref __self_0_1 }
                => {
                    let mut debug_trait_builder =
                        f.debug_struct("SignalQueueStatus");
                    let _ =
                        debug_trait_builder.field("number_of_signals_queued",
                                                  &&(*__self_0_0));
                    let _ =
                        debug_trait_builder.field("maximum_number_of_signals_that_can_be_queued",
                                                  &&(*__self_0_1));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::marker::Copy for SignalQueueStatus { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::clone::Clone for SignalQueueStatus {
        #[inline]
        fn clone(&self) -> SignalQueueStatus {
            {
                let _: $crate::clone::AssertParamIsClone<u64>;
                let _: $crate::clone::AssertParamIsClone<u64>;
                *self
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::cmp::PartialEq for SignalQueueStatus {
        #[inline]
        fn eq(&self, other: &SignalQueueStatus) -> bool {
            match *other {
                SignalQueueStatus {
                number_of_signals_queued: ref __self_1_0,
                maximum_number_of_signals_that_can_be_queued: ref __self_1_1 }
                =>
                match *self {
                    SignalQueueStatus {
                    number_of_signals_queued: ref __self_0_0,
                    maximum_number_of_signals_that_can_be_queued: ref __self_0_1
                    } =>
                    (*__self_0_0) == (*__self_1_0) &&
                        (*__self_0_1) == (*__self_1_1),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &SignalQueueStatus) -> bool {
            match *other {
                SignalQueueStatus {
                number_of_signals_queued: ref __self_1_0,
                maximum_number_of_signals_that_can_be_queued: ref __self_1_1 }
                =>
                match *self {
                    SignalQueueStatus {
                    number_of_signals_queued: ref __self_0_0,
                    maximum_number_of_signals_that_can_be_queued: ref __self_0_1
                    } =>
                    (*__self_0_0) != (*__self_1_0) ||
                        (*__self_0_1) != (*__self_1_1),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::cmp::Eq for SignalQueueStatus {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: $crate::cmp::AssertParamIsEq<u64>;
                let _: $crate::cmp::AssertParamIsEq<u64>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::cmp::PartialOrd for SignalQueueStatus {
        #[inline]
        fn partial_cmp(&self, other: &SignalQueueStatus)
         -> $crate::option::Option<$crate::cmp::Ordering> {
            match *other {
                SignalQueueStatus {
                number_of_signals_queued: ref __self_1_0,
                maximum_number_of_signals_that_can_be_queued: ref __self_1_1 }
                =>
                match *self {
                    SignalQueueStatus {
                    number_of_signals_queued: ref __self_0_0,
                    maximum_number_of_signals_that_can_be_queued: ref __self_0_1
                    } =>
                    match $crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                               &(*__self_1_0))
                        {
                        $crate::option::Option::Some($crate::cmp::Ordering::Equal)
                        =>
                        match $crate::cmp::PartialOrd::partial_cmp(&(*__self_0_1),
                                                                   &(*__self_1_1))
                            {
                            $crate::option::Option::Some($crate::cmp::Ordering::Equal)
                            =>
                            $crate::option::Option::Some($crate::cmp::Ordering::Equal),
                            cmp => cmp,
                        },
                        cmp => cmp,
                    },
                },
            }
        }
        #[inline]
        fn lt(&self, other: &SignalQueueStatus) -> bool {
            match *other {
                SignalQueueStatus {
                number_of_signals_queued: ref __self_1_0,
                maximum_number_of_signals_that_can_be_queued: ref __self_1_1 }
                =>
                match *self {
                    SignalQueueStatus {
                    number_of_signals_queued: ref __self_0_0,
                    maximum_number_of_signals_that_can_be_queued: ref __self_0_1
                    } =>
                    $crate::cmp::Ordering::then_with($crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                                                            &(*__self_1_0)),
                                                                                       $crate::cmp::Ordering::Equal),
                                                     ||
                                                         $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_1),
                                                                                                                                &(*__self_1_1)),
                                                                                           $crate::cmp::Ordering::Greater))
                        == $crate::cmp::Ordering::Less,
                },
            }
        }
        #[inline]
        fn le(&self, other: &SignalQueueStatus) -> bool {
            match *other {
                SignalQueueStatus {
                number_of_signals_queued: ref __self_1_0,
                maximum_number_of_signals_that_can_be_queued: ref __self_1_1 }
                =>
                match *self {
                    SignalQueueStatus {
                    number_of_signals_queued: ref __self_0_0,
                    maximum_number_of_signals_that_can_be_queued: ref __self_0_1
                    } =>
                    $crate::cmp::Ordering::then_with($crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                                                            &(*__self_1_0)),
                                                                                       $crate::cmp::Ordering::Equal),
                                                     ||
                                                         $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_1),
                                                                                                                                &(*__self_1_1)),
                                                                                           $crate::cmp::Ordering::Greater))
                        != $crate::cmp::Ordering::Greater,
                },
            }
        }
        #[inline]
        fn gt(&self, other: &SignalQueueStatus) -> bool {
            match *other {
                SignalQueueStatus {
                number_of_signals_queued: ref __self_1_0,
                maximum_number_of_signals_that_can_be_queued: ref __self_1_1 }
                =>
                match *self {
                    SignalQueueStatus {
                    number_of_signals_queued: ref __self_0_0,
                    maximum_number_of_signals_that_can_be_queued: ref __self_0_1
                    } =>
                    $crate::cmp::Ordering::then_with($crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                                                            &(*__self_1_0)),
                                                                                       $crate::cmp::Ordering::Equal),
                                                     ||
                                                         $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_1),
                                                                                                                                &(*__self_1_1)),
                                                                                           $crate::cmp::Ordering::Less))
                        == $crate::cmp::Ordering::Greater,
                },
            }
        }
        #[inline]
        fn ge(&self, other: &SignalQueueStatus) -> bool {
            match *other {
                SignalQueueStatus {
                number_of_signals_queued: ref __self_1_0,
                maximum_number_of_signals_that_can_be_queued: ref __self_1_1 }
                =>
                match *self {
                    SignalQueueStatus {
                    number_of_signals_queued: ref __self_0_0,
                    maximum_number_of_signals_that_can_be_queued: ref __self_0_1
                    } =>
                    $crate::cmp::Ordering::then_with($crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                                                            &(*__self_1_0)),
                                                                                       $crate::cmp::Ordering::Equal),
                                                     ||
                                                         $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_1),
                                                                                                                                &(*__self_1_1)),
                                                                                           $crate::cmp::Ordering::Less))
                        != $crate::cmp::Ordering::Less,
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::cmp::Ord for SignalQueueStatus {
        #[inline]
        fn cmp(&self, other: &SignalQueueStatus) -> $crate::cmp::Ordering {
            match *other {
                SignalQueueStatus {
                number_of_signals_queued: ref __self_1_0,
                maximum_number_of_signals_that_can_be_queued: ref __self_1_1 }
                =>
                match *self {
                    SignalQueueStatus {
                    number_of_signals_queued: ref __self_0_0,
                    maximum_number_of_signals_that_can_be_queued: ref __self_0_1
                    } =>
                    match $crate::cmp::Ord::cmp(&(*__self_0_0),
                                                &(*__self_1_0)) {
                        $crate::cmp::Ordering::Equal =>
                        match $crate::cmp::Ord::cmp(&(*__self_0_1),
                                                    &(*__self_1_1)) {
                            $crate::cmp::Ordering::Equal =>
                            $crate::cmp::Ordering::Equal,
                            cmp => cmp,
                        },
                        cmp => cmp,
                    },
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::hash::Hash for SignalQueueStatus {
        fn hash<__H: $crate::hash::Hasher>(&self, state: &mut __H) -> () {
            match *self {
                SignalQueueStatus {
                number_of_signals_queued: ref __self_0_0,
                maximum_number_of_signals_that_can_be_queued: ref __self_0_1 }
                => {
                    $crate::hash::Hash::hash(&(*__self_0_0), state);
                    $crate::hash::Hash::hash(&(*__self_0_1), state)
                }
            }
        }
    }
    /// Speculation store ('Spectre' vulnerability) bypass status.
    #[structural_match]
    #[rustc_copy_clone_marker]
    pub enum SpeculationStoreBypassStatus {

        /// Linux errored internally with `EINVAL`!
        Unknown,

        /// `prctl(PR_SPEC_STORE_BYPASS)` is `PR_SPEC_NOT_AFFECTED`.
        NotVulnerable,

        /// `prctl(PR_SPEC_STORE_BYPASS)` is `PR_SPEC_PRCTL | PR_SPEC_FORCE_DISABLE`.
        ThreadForceMitigated,

        /// `prctl(PR_SPEC_STORE_BYPASS)` is `PR_SPEC_PRCTL | PR_SPEC_DISABLE`.
        ThreadMitigated,

        /// `prctl(PR_SPEC_STORE_BYPASS)` is `PR_SPEC_PRCTL | PR_SPEC_ENABLE`.
        ThreadVulnerable,

        /// `prctl(PR_SPEC_STORE_BYPASS)` is `PR_SPEC_DISABLE`.
        GloballyMitigated,

        /// `prctl(PR_SPEC_STORE_BYPASS)` is any other value to those above.
        Vulnerable,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::fmt::Debug for SpeculationStoreBypassStatus {
        fn fmt(&self, f: &mut $crate::fmt::Formatter) -> $crate::fmt::Result {
            match (&*self,) {
                (&SpeculationStoreBypassStatus::Unknown,) => {
                    let mut debug_trait_builder = f.debug_tuple("Unknown");
                    debug_trait_builder.finish()
                }
                (&SpeculationStoreBypassStatus::NotVulnerable,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("NotVulnerable");
                    debug_trait_builder.finish()
                }
                (&SpeculationStoreBypassStatus::ThreadForceMitigated,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("ThreadForceMitigated");
                    debug_trait_builder.finish()
                }
                (&SpeculationStoreBypassStatus::ThreadMitigated,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("ThreadMitigated");
                    debug_trait_builder.finish()
                }
                (&SpeculationStoreBypassStatus::ThreadVulnerable,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("ThreadVulnerable");
                    debug_trait_builder.finish()
                }
                (&SpeculationStoreBypassStatus::GloballyMitigated,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("GloballyMitigated");
                    debug_trait_builder.finish()
                }
                (&SpeculationStoreBypassStatus::Vulnerable,) => {
                    let mut debug_trait_builder = f.debug_tuple("Vulnerable");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::marker::Copy for SpeculationStoreBypassStatus { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::clone::Clone for SpeculationStoreBypassStatus {
        #[inline]
        fn clone(&self) -> SpeculationStoreBypassStatus { { *self } }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::cmp::PartialEq for SpeculationStoreBypassStatus {
        #[inline]
        fn eq(&self, other: &SpeculationStoreBypassStatus) -> bool {
            {
                let __self_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) { _ => true, }
                } else { false }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::cmp::Eq for SpeculationStoreBypassStatus {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () { { } }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::cmp::PartialOrd for SpeculationStoreBypassStatus {
        #[inline]
        fn partial_cmp(&self, other: &SpeculationStoreBypassStatus)
         -> $crate::option::Option<$crate::cmp::Ordering> {
            {
                let __self_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        _ =>
                        $crate::option::Option::Some($crate::cmp::Ordering::Equal),
                    }
                } else { __self_vi.partial_cmp(&__arg_1_vi) }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::cmp::Ord for SpeculationStoreBypassStatus {
        #[inline]
        fn cmp(&self, other: &SpeculationStoreBypassStatus)
         -> $crate::cmp::Ordering {
            {
                let __self_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        _ => $crate::cmp::Ordering::Equal,
                    }
                } else { __self_vi.cmp(&__arg_1_vi) }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::hash::Hash for SpeculationStoreBypassStatus {
        fn hash<__H: $crate::hash::Hasher>(&self, state: &mut __H) -> () {
            match (&*self,) {
                _ => {
                    $crate::hash::Hash::hash(&unsafe {
                                                  $crate::intrinsics::discriminant_value(self)
                                              }, state)
                }
            }
        }
    }
}
/// Support for signals.
#[cfg(unix)]
pub mod signals {
    use super::*;
    /// Block all signals on the current thread.
    #[inline(always)]
    pub fn block_all_signals_on_current_thread() {
        block_all_signals_on_current_thread_bar(&HashSet::default())
    }
    /// Block all signals specified the current thread.
    #[inline(always)]
    pub fn block_all_signals_on_current_thread_bar(signals:
                                                       &HashSet<SignalNumber>) {
        let result =
            unsafe {
                let mut set = uninitialized();
                sigfillset(&mut set);
                for signal in signals.iter() { sigdelset(&mut set, *signal); }
                pthread_sigmask(SIG_SETMASK, &set, null_mut())
            };
        match result {
            0 => (),
            -1 => {
                $crate::rt::begin_panic("pthread_sigmask returned an error",
                                        &("dpdk-unix/src/signals/block_all_signals_on_current_thread_bar.rs",
                                          23u32, 9u32))
            }
            _ => {
                $crate::rt::begin_panic_fmt(&$crate::fmt::Arguments::new_v1_formatted(&["pthread_sigmask returned an invalid result \'",
                                                                                        "\'"],
                                                                                      &match (&result,)
                                                                                           {
                                                                                           (arg0,)
                                                                                           =>
                                                                                           [$crate::fmt::ArgumentV1::new(arg0,
                                                                                                                         $crate::fmt::Display::fmt)],
                                                                                       },
                                                                                      &[$crate::fmt::rt::v1::Argument{position:
                                                                                                                          $crate::fmt::rt::v1::Position::At(0usize),
                                                                                                                      format:
                                                                                                                          $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                              ' ',
                                                                                                                                                          align:
                                                                                                                                                              $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                          flags:
                                                                                                                                                              0u32,
                                                                                                                                                          precision:
                                                                                                                                                              $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                          width:
                                                                                                                                                              $crate::fmt::rt::v1::Count::Implied,},}]),
                                            &("dpdk-unix/src/signals/block_all_signals_on_current_thread_bar.rs",
                                              24u32, 8u32))
            }
        }
    }
    /// Block all signals on the current thread bar `SIGCHLD` (ie a child process has exited).
    #[inline(always)]
    pub fn block_all_signals_on_current_thread_bar_child() {
        let signals =
            {
                let _cap = <[()]>::len(&[()]);
                let mut _set =
                    ::std::collections::HashSet::with_capacity(_cap);
                let _ = _set.insert(SIGCHLD);
                _set
            };
        block_all_signals_on_current_thread_bar(&signals)
    }
    /// Block all signals on the current thread bar `SIGUP`, `SIGTERM` and `SIGCHLD` (ie a child process has exited).
    #[inline(always)]
    pub fn block_all_signals_on_current_thread_bar_hang_up_and_terminate_and_child() {
        let signals =
            {
                let _cap = <[()]>::len(&[(), (), ()]);
                let mut _set =
                    ::std::collections::HashSet::with_capacity(_cap);
                let _ = _set.insert(SIGHUP);
                let _ = _set.insert(SIGTERM);
                let _ = _set.insert(SIGCHLD);
                _set
            };
        block_all_signals_on_current_thread_bar(&signals)
    }
    /// Converts a hash set of signals to a libc `sigset_t`.
    #[inline(always)]
    pub fn hash_set_to_signal_set(signals: &HashSet<i32>) -> sigset_t {
        unsafe {
            let mut signal_set: sigset_t = uninitialized();
            sigemptyset(&mut signal_set);
            for signal in signals.iter() {
                sigaddset(&mut signal_set, *signal);
            }
            signal_set
        }
    }
    /// A signal number.
    pub type SignalNumber = i32;
    /// Represents the result of waiting for a set of signals.
    #[structural_match]
    #[rustc_copy_clone_marker]
    pub enum TimedSignalWait {

        /// Timed out.
        TimedOut,

        /// Signalled.
        Signalled(SignalNumber),

        /// Other signal interrupted.
        OtherSignalInterrupted,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::fmt::Debug for TimedSignalWait {
        fn fmt(&self, f: &mut $crate::fmt::Formatter) -> $crate::fmt::Result {
            match (&*self,) {
                (&TimedSignalWait::TimedOut,) => {
                    let mut debug_trait_builder = f.debug_tuple("TimedOut");
                    debug_trait_builder.finish()
                }
                (&TimedSignalWait::Signalled(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Signalled");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&TimedSignalWait::OtherSignalInterrupted,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("OtherSignalInterrupted");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::marker::Copy for TimedSignalWait { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::clone::Clone for TimedSignalWait {
        #[inline]
        fn clone(&self) -> TimedSignalWait {
            { let _: $crate::clone::AssertParamIsClone<SignalNumber>; *self }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::cmp::Ord for TimedSignalWait {
        #[inline]
        fn cmp(&self, other: &TimedSignalWait) -> $crate::cmp::Ordering {
            {
                let __self_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&TimedSignalWait::Signalled(ref __self_0),
                         &TimedSignalWait::Signalled(ref __arg_1_0)) =>
                        match $crate::cmp::Ord::cmp(&(*__self_0),
                                                    &(*__arg_1_0)) {
                            $crate::cmp::Ordering::Equal =>
                            $crate::cmp::Ordering::Equal,
                            cmp => cmp,
                        },
                        _ => $crate::cmp::Ordering::Equal,
                    }
                } else { __self_vi.cmp(&__arg_1_vi) }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::cmp::PartialOrd for TimedSignalWait {
        #[inline]
        fn partial_cmp(&self, other: &TimedSignalWait)
         -> $crate::option::Option<$crate::cmp::Ordering> {
            {
                let __self_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&TimedSignalWait::Signalled(ref __self_0),
                         &TimedSignalWait::Signalled(ref __arg_1_0)) =>
                        match $crate::cmp::PartialOrd::partial_cmp(&(*__self_0),
                                                                   &(*__arg_1_0))
                            {
                            $crate::option::Option::Some($crate::cmp::Ordering::Equal)
                            =>
                            $crate::option::Option::Some($crate::cmp::Ordering::Equal),
                            cmp => cmp,
                        },
                        _ =>
                        $crate::option::Option::Some($crate::cmp::Ordering::Equal),
                    }
                } else { __self_vi.partial_cmp(&__arg_1_vi) }
            }
        }
        #[inline]
        fn lt(&self, other: &TimedSignalWait) -> bool {
            {
                let __self_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&TimedSignalWait::Signalled(ref __self_0),
                         &TimedSignalWait::Signalled(ref __arg_1_0)) =>
                        $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0),
                                                                                               &(*__arg_1_0)),
                                                          $crate::cmp::Ordering::Greater)
                            == $crate::cmp::Ordering::Less,
                        _ => false,
                    }
                } else { __self_vi.lt(&__arg_1_vi) }
            }
        }
        #[inline]
        fn le(&self, other: &TimedSignalWait) -> bool {
            {
                let __self_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&TimedSignalWait::Signalled(ref __self_0),
                         &TimedSignalWait::Signalled(ref __arg_1_0)) =>
                        $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0),
                                                                                               &(*__arg_1_0)),
                                                          $crate::cmp::Ordering::Greater)
                            != $crate::cmp::Ordering::Greater,
                        _ => true,
                    }
                } else { __self_vi.le(&__arg_1_vi) }
            }
        }
        #[inline]
        fn gt(&self, other: &TimedSignalWait) -> bool {
            {
                let __self_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&TimedSignalWait::Signalled(ref __self_0),
                         &TimedSignalWait::Signalled(ref __arg_1_0)) =>
                        $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0),
                                                                                               &(*__arg_1_0)),
                                                          $crate::cmp::Ordering::Less)
                            == $crate::cmp::Ordering::Greater,
                        _ => false,
                    }
                } else { __self_vi.gt(&__arg_1_vi) }
            }
        }
        #[inline]
        fn ge(&self, other: &TimedSignalWait) -> bool {
            {
                let __self_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&TimedSignalWait::Signalled(ref __self_0),
                         &TimedSignalWait::Signalled(ref __arg_1_0)) =>
                        $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0),
                                                                                               &(*__arg_1_0)),
                                                          $crate::cmp::Ordering::Less)
                            != $crate::cmp::Ordering::Less,
                        _ => true,
                    }
                } else { __self_vi.ge(&__arg_1_vi) }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::cmp::Eq for TimedSignalWait {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            { let _: $crate::cmp::AssertParamIsEq<SignalNumber>; }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::cmp::PartialEq for TimedSignalWait {
        #[inline]
        fn eq(&self, other: &TimedSignalWait) -> bool {
            {
                let __self_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&TimedSignalWait::Signalled(ref __self_0),
                         &TimedSignalWait::Signalled(ref __arg_1_0)) =>
                        (*__self_0) == (*__arg_1_0),
                        _ => true,
                    }
                } else { false }
            }
        }
        #[inline]
        fn ne(&self, other: &TimedSignalWait) -> bool {
            {
                let __self_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { $crate::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&TimedSignalWait::Signalled(ref __self_0),
                         &TimedSignalWait::Signalled(ref __arg_1_0)) =>
                        (*__self_0) != (*__arg_1_0),
                        _ => false,
                    }
                } else { true }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::hash::Hash for TimedSignalWait {
        fn hash<__H: $crate::hash::Hasher>(&self, state: &mut __H) -> () {
            match (&*self,) {
                (&TimedSignalWait::Signalled(ref __self_0),) => {
                    $crate::hash::Hash::hash(&unsafe {
                                                  $crate::intrinsics::discriminant_value(self)
                                              }, state);
                    $crate::hash::Hash::hash(&(*__self_0), state)
                }
                _ => {
                    $crate::hash::Hash::hash(&unsafe {
                                                  $crate::intrinsics::discriminant_value(self)
                                              }, state)
                }
            }
        }
    }
}
pub(crate) mod strings {
    use super::*;
    #[cfg(unix)]
    #[inline(always)]
    pub(crate) fn c_string_pointer_to_path_buf(nul_terminated: *mut c_char)
     -> Result<Option<PathBuf>, ()> {
        if unsafe { ::std::intrinsics::unlikely(nul_terminated.is_null()) } {
            return Ok(None);
        }
        let c_str = unsafe { CStr::from_ptr(nul_terminated) };
        let bytes = c_str.to_bytes();
        if bytes.len() == 0 {
            Err(())
        } else {
            let os_str: &OsStr = OsStrExt::from_bytes(bytes);
            Ok(Some(PathBuf::from(os_str)))
        }
    }
    #[inline(always)]
    pub(crate) fn replace(extant: &[u8], from: u8, to: u8) -> Box<[u8]> {
        let mut result = Vec::with_capacity(extant.len());
        for byte in extant.iter() {
            let byte = *byte;
            let byte_to_push =
                if unsafe { ::std::intrinsics::unlikely(byte == from) } {
                    to
                } else { byte };
            result.push(byte_to_push);
        }
        result.into_boxed_slice()
    }
    #[inline(always)]
    pub(crate) fn split<'a>(slice: &'a [u8], predicate: u8)
     -> ::std::slice::Split<'a, u8, impl FnMut(&u8) -> bool> {
        slice.split(move |value| *value == predicate)
    }
    #[inline(always)]
    pub(crate) fn splitn<'a>(slice: &'a [u8], n: usize, predicate: u8)
     -> ::std::slice::SplitN<'a, u8, impl FnMut(&u8) -> bool> {
        slice.splitn(n, move |value| *value == predicate)
    }
}
/// Asserts that the effective user id (`uid`) is root.
///
/// Takes a necessity to explain why the user must be root.
#[inline(always)]
pub fn assert_effective_user_id_is_root(necessity: &str) {
    let effective_user_id = unsafe { geteuid() };
    {
        match (&(effective_user_id), &(0)) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    {
                        $crate::rt::begin_panic_fmt(&$crate::fmt::Arguments::new_v1_formatted(&["assertion failed: `(left == right)`\n  left: `",
                                                                                                "`,\n right: `",
                                                                                                "`: "],
                                                                                              &match (&left_val,
                                                                                                      &right_val,
                                                                                                      &$crate::fmt::Arguments::new_v1_formatted(&["Effective User Id (euid) is not root (0). Necessity: "],
                                                                                                                                                &match (&necessity,)
                                                                                                                                                     {
                                                                                                                                                     (arg0,)
                                                                                                                                                     =>
                                                                                                                                                     [$crate::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                   $crate::fmt::Display::fmt)],
                                                                                                                                                 },
                                                                                                                                                &[$crate::fmt::rt::v1::Argument{position:
                                                                                                                                                                                    $crate::fmt::rt::v1::Position::At(0usize),
                                                                                                                                                                                format:
                                                                                                                                                                                    $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                                                                        ' ',
                                                                                                                                                                                                                    align:
                                                                                                                                                                                                                        $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                                                                    flags:
                                                                                                                                                                                                                        0u32,
                                                                                                                                                                                                                    precision:
                                                                                                                                                                                                                        $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                                                                    width:
                                                                                                                                                                                                                        $crate::fmt::rt::v1::Count::Implied,},}]))
                                                                                                   {
                                                                                                   (arg0,
                                                                                                    arg1,
                                                                                                    arg2)
                                                                                                   =>
                                                                                                   [$crate::fmt::ArgumentV1::new(arg0,
                                                                                                                                 $crate::fmt::Debug::fmt),
                                                                                                    $crate::fmt::ArgumentV1::new(arg1,
                                                                                                                                 $crate::fmt::Debug::fmt),
                                                                                                    $crate::fmt::ArgumentV1::new(arg2,
                                                                                                                                 $crate::fmt::Display::fmt)],
                                                                                               },
                                                                                              &[$crate::fmt::rt::v1::Argument{position:
                                                                                                                                  $crate::fmt::rt::v1::Position::At(0usize),
                                                                                                                              format:
                                                                                                                                  $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                      ' ',
                                                                                                                                                                  align:
                                                                                                                                                                      $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                  flags:
                                                                                                                                                                      0u32,
                                                                                                                                                                  precision:
                                                                                                                                                                      $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                  width:
                                                                                                                                                                      $crate::fmt::rt::v1::Count::Implied,},},
                                                                                                $crate::fmt::rt::v1::Argument{position:
                                                                                                                                  $crate::fmt::rt::v1::Position::At(1usize),
                                                                                                                              format:
                                                                                                                                  $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                      ' ',
                                                                                                                                                                  align:
                                                                                                                                                                      $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                  flags:
                                                                                                                                                                      0u32,
                                                                                                                                                                  precision:
                                                                                                                                                                      $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                  width:
                                                                                                                                                                      $crate::fmt::rt::v1::Count::Implied,},},
                                                                                                $crate::fmt::rt::v1::Argument{position:
                                                                                                                                  $crate::fmt::rt::v1::Position::At(2usize),
                                                                                                                              format:
                                                                                                                                  $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                      ' ',
                                                                                                                                                                  align:
                                                                                                                                                                      $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                  flags:
                                                                                                                                                                      0u32,
                                                                                                                                                                  precision:
                                                                                                                                                                      $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                  width:
                                                                                                                                                                      $crate::fmt::rt::v1::Count::Implied,},}]),
                                                    &("dpdk-unix/src/assert_effective_user_id_is_root.rs",
                                                      12u32, 2u32))
                    }
                }
            }
        }
    };
}
/// An object that can be used with a configuration file (eg via Serde) to configure a daemon.
///
/// The following are done:-
///
/// * umask is set to just the current user
/// * Checks are made to check the program is not running with the set uid bit set ('setuid' or 'suid').
/// * A PID file is created
/// * standard in is redirected to `/dev/null`.
/// * standard out and error are redirected to `/dev/null`.
/// * `fprintf` and friends using the `FILE` API are redirected to syslog on Linux (this is probably also possible to implement for FreeBSD - see <https://mischasan.wordpress.com/2011/05/25/redirecting-stderr-to-syslog/>).
/// * Double forking and a new session are created.
/// * Real and effective user and group ids are changed.
/// * Additional groups from `/etc/group`, if any, are assigned.
/// * Environment variables are populated if missing (`IFS`, `PATH`)
/// * User environment variables are overwritten (`HOME`, `LOGNAME`, `USER`).
#[structural_match]
pub struct Daemonize {
    /// The folder path to use as the 'current working directory' (CWD).
    ///
    /// Equivalent functionality to the shell command `chdir`.
    ///
    /// Defaults to `/`.
    #[serde(default = "Daemonize::working_folder_path_default")]
    pub working_folder_path: PathBuf,
    /// A folder path in which to put a PID file.
    ///
    /// This uses the processes' name for the actual file base name.
    ///
    /// Defaults to `/var/run`.
    #[serde(default = "Daemonize::pid_folder_path_default")]
    pub pid_folder_path: PathBuf,
    /// An user name that must exist in `/etc/passwd` (or the local equivalent).
    ///
    /// Use to discover runtime user and groups to change to and the home folder of the running user.
    #[serde(default = "Daemonize::user_name_default")]
    pub user_name: CString,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::fmt::Debug for Daemonize {
    fn fmt(&self, f: &mut $crate::fmt::Formatter) -> $crate::fmt::Result {
        match *self {
            Daemonize {
            working_folder_path: ref __self_0_0,
            pid_folder_path: ref __self_0_1,
            user_name: ref __self_0_2 } => {
                let mut debug_trait_builder = f.debug_struct("Daemonize");
                let _ =
                    debug_trait_builder.field("working_folder_path",
                                              &&(*__self_0_0));
                let _ =
                    debug_trait_builder.field("pid_folder_path",
                                              &&(*__self_0_1));
                let _ =
                    debug_trait_builder.field("user_name", &&(*__self_0_2));
                debug_trait_builder.finish()
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::clone::Clone for Daemonize {
    #[inline]
    fn clone(&self) -> Daemonize {
        match *self {
            Daemonize {
            working_folder_path: ref __self_0_0,
            pid_folder_path: ref __self_0_1,
            user_name: ref __self_0_2 } =>
            Daemonize{working_folder_path:
                          $crate::clone::Clone::clone(&(*__self_0_0)),
                      pid_folder_path:
                          $crate::clone::Clone::clone(&(*__self_0_1)),
                      user_name:
                          $crate::clone::Clone::clone(&(*__self_0_2)),},
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::cmp::Ord for Daemonize {
    #[inline]
    fn cmp(&self, other: &Daemonize) -> $crate::cmp::Ordering {
        match *other {
            Daemonize {
            working_folder_path: ref __self_1_0,
            pid_folder_path: ref __self_1_1,
            user_name: ref __self_1_2 } =>
            match *self {
                Daemonize {
                working_folder_path: ref __self_0_0,
                pid_folder_path: ref __self_0_1,
                user_name: ref __self_0_2 } =>
                match $crate::cmp::Ord::cmp(&(*__self_0_0), &(*__self_1_0)) {
                    $crate::cmp::Ordering::Equal =>
                    match $crate::cmp::Ord::cmp(&(*__self_0_1),
                                                &(*__self_1_1)) {
                        $crate::cmp::Ordering::Equal =>
                        match $crate::cmp::Ord::cmp(&(*__self_0_2),
                                                    &(*__self_1_2)) {
                            $crate::cmp::Ordering::Equal =>
                            $crate::cmp::Ordering::Equal,
                            cmp => cmp,
                        },
                        cmp => cmp,
                    },
                    cmp => cmp,
                },
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::cmp::PartialOrd for Daemonize {
    #[inline]
    fn partial_cmp(&self, other: &Daemonize)
     -> $crate::option::Option<$crate::cmp::Ordering> {
        match *other {
            Daemonize {
            working_folder_path: ref __self_1_0,
            pid_folder_path: ref __self_1_1,
            user_name: ref __self_1_2 } =>
            match *self {
                Daemonize {
                working_folder_path: ref __self_0_0,
                pid_folder_path: ref __self_0_1,
                user_name: ref __self_0_2 } =>
                match $crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                           &(*__self_1_0)) {
                    $crate::option::Option::Some($crate::cmp::Ordering::Equal)
                    =>
                    match $crate::cmp::PartialOrd::partial_cmp(&(*__self_0_1),
                                                               &(*__self_1_1))
                        {
                        $crate::option::Option::Some($crate::cmp::Ordering::Equal)
                        =>
                        match $crate::cmp::PartialOrd::partial_cmp(&(*__self_0_2),
                                                                   &(*__self_1_2))
                            {
                            $crate::option::Option::Some($crate::cmp::Ordering::Equal)
                            =>
                            $crate::option::Option::Some($crate::cmp::Ordering::Equal),
                            cmp => cmp,
                        },
                        cmp => cmp,
                    },
                    cmp => cmp,
                },
            },
        }
    }
    #[inline]
    fn lt(&self, other: &Daemonize) -> bool {
        match *other {
            Daemonize {
            working_folder_path: ref __self_1_0,
            pid_folder_path: ref __self_1_1,
            user_name: ref __self_1_2 } =>
            match *self {
                Daemonize {
                working_folder_path: ref __self_0_0,
                pid_folder_path: ref __self_0_1,
                user_name: ref __self_0_2 } =>
                $crate::cmp::Ordering::then_with($crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                                                        &(*__self_1_0)),
                                                                                   $crate::cmp::Ordering::Equal),
                                                 ||
                                                     $crate::cmp::Ordering::then_with($crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_1),
                                                                                                                                                             &(*__self_1_1)),
                                                                                                                        $crate::cmp::Ordering::Equal),
                                                                                      ||
                                                                                          $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_2),
                                                                                                                                                                 &(*__self_1_2)),
                                                                                                                            $crate::cmp::Ordering::Greater)))
                    == $crate::cmp::Ordering::Less,
            },
        }
    }
    #[inline]
    fn le(&self, other: &Daemonize) -> bool {
        match *other {
            Daemonize {
            working_folder_path: ref __self_1_0,
            pid_folder_path: ref __self_1_1,
            user_name: ref __self_1_2 } =>
            match *self {
                Daemonize {
                working_folder_path: ref __self_0_0,
                pid_folder_path: ref __self_0_1,
                user_name: ref __self_0_2 } =>
                $crate::cmp::Ordering::then_with($crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                                                        &(*__self_1_0)),
                                                                                   $crate::cmp::Ordering::Equal),
                                                 ||
                                                     $crate::cmp::Ordering::then_with($crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_1),
                                                                                                                                                             &(*__self_1_1)),
                                                                                                                        $crate::cmp::Ordering::Equal),
                                                                                      ||
                                                                                          $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_2),
                                                                                                                                                                 &(*__self_1_2)),
                                                                                                                            $crate::cmp::Ordering::Greater)))
                    != $crate::cmp::Ordering::Greater,
            },
        }
    }
    #[inline]
    fn gt(&self, other: &Daemonize) -> bool {
        match *other {
            Daemonize {
            working_folder_path: ref __self_1_0,
            pid_folder_path: ref __self_1_1,
            user_name: ref __self_1_2 } =>
            match *self {
                Daemonize {
                working_folder_path: ref __self_0_0,
                pid_folder_path: ref __self_0_1,
                user_name: ref __self_0_2 } =>
                $crate::cmp::Ordering::then_with($crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                                                        &(*__self_1_0)),
                                                                                   $crate::cmp::Ordering::Equal),
                                                 ||
                                                     $crate::cmp::Ordering::then_with($crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_1),
                                                                                                                                                             &(*__self_1_1)),
                                                                                                                        $crate::cmp::Ordering::Equal),
                                                                                      ||
                                                                                          $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_2),
                                                                                                                                                                 &(*__self_1_2)),
                                                                                                                            $crate::cmp::Ordering::Less)))
                    == $crate::cmp::Ordering::Greater,
            },
        }
    }
    #[inline]
    fn ge(&self, other: &Daemonize) -> bool {
        match *other {
            Daemonize {
            working_folder_path: ref __self_1_0,
            pid_folder_path: ref __self_1_1,
            user_name: ref __self_1_2 } =>
            match *self {
                Daemonize {
                working_folder_path: ref __self_0_0,
                pid_folder_path: ref __self_0_1,
                user_name: ref __self_0_2 } =>
                $crate::cmp::Ordering::then_with($crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                                                        &(*__self_1_0)),
                                                                                   $crate::cmp::Ordering::Equal),
                                                 ||
                                                     $crate::cmp::Ordering::then_with($crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_1),
                                                                                                                                                             &(*__self_1_1)),
                                                                                                                        $crate::cmp::Ordering::Equal),
                                                                                      ||
                                                                                          $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_2),
                                                                                                                                                                 &(*__self_1_2)),
                                                                                                                            $crate::cmp::Ordering::Less)))
                    != $crate::cmp::Ordering::Less,
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::cmp::Eq for Daemonize {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: $crate::cmp::AssertParamIsEq<PathBuf>;
            let _: $crate::cmp::AssertParamIsEq<PathBuf>;
            let _: $crate::cmp::AssertParamIsEq<CString>;
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::cmp::PartialEq for Daemonize {
    #[inline]
    fn eq(&self, other: &Daemonize) -> bool {
        match *other {
            Daemonize {
            working_folder_path: ref __self_1_0,
            pid_folder_path: ref __self_1_1,
            user_name: ref __self_1_2 } =>
            match *self {
                Daemonize {
                working_folder_path: ref __self_0_0,
                pid_folder_path: ref __self_0_1,
                user_name: ref __self_0_2 } =>
                (*__self_0_0) == (*__self_1_0) &&
                    (*__self_0_1) == (*__self_1_1) &&
                    (*__self_0_2) == (*__self_1_2),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &Daemonize) -> bool {
        match *other {
            Daemonize {
            working_folder_path: ref __self_1_0,
            pid_folder_path: ref __self_1_1,
            user_name: ref __self_1_2 } =>
            match *self {
                Daemonize {
                working_folder_path: ref __self_0_0,
                pid_folder_path: ref __self_0_1,
                user_name: ref __self_0_2 } =>
                (*__self_0_0) != (*__self_1_0) ||
                    (*__self_0_1) != (*__self_1_1) ||
                    (*__self_0_2) != (*__self_1_2),
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::hash::Hash for Daemonize {
    fn hash<__H: $crate::hash::Hasher>(&self, state: &mut __H) -> () {
        match *self {
            Daemonize {
            working_folder_path: ref __self_0_0,
            pid_folder_path: ref __self_0_1,
            user_name: ref __self_0_2 } => {
                $crate::hash::Hash::hash(&(*__self_0_0), state);
                $crate::hash::Hash::hash(&(*__self_0_1), state);
                $crate::hash::Hash::hash(&(*__self_0_2), state)
            }
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_Daemonize: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[allow(unused_macros)]
        macro_rules! try(( $ __expr : expr ) => {
                         match $ __expr {
                         _serde :: export :: Ok ( __val ) => __val , _serde ::
                         export :: Err ( __err ) => {
                         return _serde :: export :: Err ( __err ) ; } } });
        #[automatically_derived]
        impl <'de> _serde::Deserialize<'de> for Daemonize {
            fn deserialize<__D>(__deserializer: __D)
             -> _serde::export::Result<Self, __D::Error> where
             __D: _serde::Deserializer<'de> {
                #[allow(non_camel_case_types)]
                enum __Field { __field0, __field1, __field2, __ignore, }
                struct __FieldVisitor;
                impl <'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type
                    Value
                    =
                    __Field;
                    fn expecting(&self,
                                 __formatter: &mut _serde::export::Formatter)
                     -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter,
                                                             "field identifier")
                    }
                    fn visit_u64<__E>(self, __value: u64)
                     -> _serde::export::Result<Self::Value, __E> where
                     __E: _serde::de::Error {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            2u64 => _serde::export::Ok(__Field::__field2),
                            _ =>
                            _serde::export::Err(_serde::de::Error::invalid_value(_serde::de::Unexpected::Unsigned(__value),
                                                                                 &"field index 0 <= i < 3")),
                        }
                    }
                    fn visit_str<__E>(self, __value: &str)
                     -> _serde::export::Result<Self::Value, __E> where
                     __E: _serde::de::Error {
                        match __value {
                            "working_folder_path" =>
                            _serde::export::Ok(__Field::__field0),
                            "pid_folder_path" =>
                            _serde::export::Ok(__Field::__field1),
                            "user_name" =>
                            _serde::export::Ok(__Field::__field2),
                            _ => { _serde::export::Ok(__Field::__ignore) }
                        }
                    }
                    fn visit_bytes<__E>(self, __value: &[u8])
                     -> _serde::export::Result<Self::Value, __E> where
                     __E: _serde::de::Error {
                        match __value {
                            b"working_folder_path" =>
                            _serde::export::Ok(__Field::__field0),
                            b"pid_folder_path" =>
                            _serde::export::Ok(__Field::__field1),
                            b"user_name" =>
                            _serde::export::Ok(__Field::__field2),
                            _ => { _serde::export::Ok(__Field::__ignore) }
                        }
                    }
                }
                impl <'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(__deserializer: __D)
                     -> _serde::export::Result<Self, __D::Error> where
                     __D: _serde::Deserializer<'de> {
                        _serde::Deserializer::deserialize_identifier(__deserializer,
                                                                     __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<Daemonize>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl <'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type
                    Value
                    =
                    Daemonize;
                    fn expecting(&self,
                                 __formatter: &mut _serde::export::Formatter)
                     -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter,
                                                             "struct Daemonize")
                    }
                    #[inline]
                    fn visit_seq<__A>(self, mut __seq: __A)
                     -> _serde::export::Result<Self::Value, __A::Error> where
                     __A: _serde::de::SeqAccess<'de> {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<PathBuf>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    Daemonize::working_folder_path_default()
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<PathBuf>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    Daemonize::pid_folder_path_default()
                                }
                            };
                        let __field2 =
                            match match _serde::de::SeqAccess::next_element::<CString>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    Daemonize::user_name_default()
                                }
                            };
                        _serde::export::Ok(Daemonize{working_folder_path:
                                                         __field0,
                                                     pid_folder_path:
                                                         __field1,
                                                     user_name: __field2,})
                    }
                    #[inline]
                    fn visit_map<__A>(self, mut __map: __A)
                     -> _serde::export::Result<Self::Value, __A::Error> where
                     __A: _serde::de::MapAccess<'de> {
                        let mut __field0: _serde::export::Option<PathBuf> =
                            _serde::export::None;
                        let mut __field1: _serde::export::Option<PathBuf> =
                            _serde::export::None;
                        let mut __field2: _serde::export::Option<CString> =
                            _serde::export::None;
                        while let _serde::export::Some(__key) =
                                  match _serde::de::MapAccess::next_key::<__Field>(&mut __map)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("working_folder_path"));
                                    }
                                    __field0 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<PathBuf>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 => __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("pid_folder_path"));
                                    }
                                    __field1 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<PathBuf>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 => __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field2 => {
                                    if _serde::export::Option::is_some(&__field2)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("user_name"));
                                    }
                                    __field2 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<CString>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 => __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                _ => {
                                    let _ =
                                        match _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)
                                            {
                                            _serde::export::Ok(__val) =>
                                            __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        };
                                }
                            }
                        }
                        let __field0 =
                            match __field0 {
                                _serde::export::Some(__field0) => __field0,
                                _serde::export::None =>
                                Daemonize::working_folder_path_default(),
                            };
                        let __field1 =
                            match __field1 {
                                _serde::export::Some(__field1) => __field1,
                                _serde::export::None =>
                                Daemonize::pid_folder_path_default(),
                            };
                        let __field2 =
                            match __field2 {
                                _serde::export::Some(__field2) => __field2,
                                _serde::export::None =>
                                Daemonize::user_name_default(),
                            };
                        _serde::export::Ok(Daemonize{working_folder_path:
                                                         __field0,
                                                     pid_folder_path:
                                                         __field1,
                                                     user_name: __field2,})
                    }
                }
                const FIELDS: &'static [&'static str] =
                    &["working_folder_path", "pid_folder_path", "user_name"];
                _serde::Deserializer::deserialize_struct(__deserializer,
                                                         "Daemonize", FIELDS,
                                                         __Visitor{marker:
                                                                       _serde::export::PhantomData::<Daemonize>,
                                                                   lifetime:
                                                                       _serde::export::PhantomData,})
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_Daemonize: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[allow(unused_macros)]
        macro_rules! try(( $ __expr : expr ) => {
                         match $ __expr {
                         _serde :: export :: Ok ( __val ) => __val , _serde ::
                         export :: Err ( __err ) => {
                         return _serde :: export :: Err ( __err ) ; } } });
        #[automatically_derived]
        impl _serde::Serialize for Daemonize {
            fn serialize<__S>(&self, __serializer: __S)
             -> _serde::export::Result<__S::Ok, __S::Error> where
             __S: _serde::Serializer {
                let mut __serde_state =
                    match _serde::Serializer::serialize_struct(__serializer,
                                                               "Daemonize",
                                                               0usize + 1 + 1
                                                                   + 1) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "working_folder_path",
                                                                    &self.working_folder_path)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "pid_folder_path",
                                                                    &self.pid_folder_path)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "user_name",
                                                                    &self.user_name)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
impl Default for Daemonize {
    #[inline(always)]
    fn default() -> Self {
        Self{working_folder_path: Self::working_folder_path_default(),
             pid_folder_path: Self::pid_folder_path_default(),
             user_name: Self::user_name_default(),}
    }
}
impl Daemonize {
    /// Daemonizes the current process.
    ///
    /// Returns an object that needs to have `clean_up()` called on it just before process exit.
    #[inline(always)]
    pub fn daemonize(self) -> DaemonizeCleanUpOnExit {
        Self::verify_not_running_with_set_uid_bit_set();
        Self::initial_umask();
        let pid_file_path = self.switch_user();
        self.change_current_working_directory();
        self.redirect_standard_in_out_and_error();
        Self::fork();
        Self::create_a_new_progress_group_and_session_detach_controlling_terminal();
        Self::fork();
        self.populate_pid_file_when_running(&pid_file_path);
        Self::ensure_environment_variable_is_set("IFS", "\t\n");
        Self::ensure_environment_variable_is_set("PATH",
                                                 "/usr/local/bin:/usr/bin");
        DaemonizeCleanUpOnExit{pid_file_path,}
    }
    #[inline(always)]
    fn verify_not_running_with_set_uid_bit_set() {
        {
            match (&(unsafe { geteuid() }), &(unsafe { getuid() })) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        {
                            $crate::rt::begin_panic_fmt(&$crate::fmt::Arguments::new_v1_formatted(&["assertion failed: `(left == right)`\n  left: `",
                                                                                                    "`,\n right: `",
                                                                                                    "`: "],
                                                                                                  &match (&left_val,
                                                                                                          &right_val,
                                                                                                          &$crate::fmt::Arguments::new_v1(&["Can not be run with set uid bit set (\'setuid\')"],
                                                                                                                                          &match ()
                                                                                                                                               {
                                                                                                                                               ()
                                                                                                                                               =>
                                                                                                                                               [],
                                                                                                                                           }))
                                                                                                       {
                                                                                                       (arg0,
                                                                                                        arg1,
                                                                                                        arg2)
                                                                                                       =>
                                                                                                       [$crate::fmt::ArgumentV1::new(arg0,
                                                                                                                                     $crate::fmt::Debug::fmt),
                                                                                                        $crate::fmt::ArgumentV1::new(arg1,
                                                                                                                                     $crate::fmt::Debug::fmt),
                                                                                                        $crate::fmt::ArgumentV1::new(arg2,
                                                                                                                                     $crate::fmt::Display::fmt)],
                                                                                                   },
                                                                                                  &[$crate::fmt::rt::v1::Argument{position:
                                                                                                                                      $crate::fmt::rt::v1::Position::At(0usize),
                                                                                                                                  format:
                                                                                                                                      $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                          ' ',
                                                                                                                                                                      align:
                                                                                                                                                                          $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                      flags:
                                                                                                                                                                          0u32,
                                                                                                                                                                      precision:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                      width:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,},},
                                                                                                    $crate::fmt::rt::v1::Argument{position:
                                                                                                                                      $crate::fmt::rt::v1::Position::At(1usize),
                                                                                                                                  format:
                                                                                                                                      $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                          ' ',
                                                                                                                                                                      align:
                                                                                                                                                                          $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                      flags:
                                                                                                                                                                          0u32,
                                                                                                                                                                      precision:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                      width:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,},},
                                                                                                    $crate::fmt::rt::v1::Argument{position:
                                                                                                                                      $crate::fmt::rt::v1::Position::At(2usize),
                                                                                                                                  format:
                                                                                                                                      $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                          ' ',
                                                                                                                                                                      align:
                                                                                                                                                                          $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                      flags:
                                                                                                                                                                          0u32,
                                                                                                                                                                      precision:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                      width:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,},}]),
                                                        &("dpdk-unix/src/Daemonize.rs",
                                                          100u32, 3u32))
                        }
                    }
                }
            }
        };
    }
    #[inline(always)]
    fn initial_umask() { unsafe { umask(0) }; }
    #[inline(always)]
    fn get_user_entry(&self) -> NonNull<passwd> {
        let entry = unsafe { getpwnam(self.user_name.as_ptr()) };
        if !!entry.is_null() {
            {
                $crate::rt::begin_panic_fmt(&$crate::fmt::Arguments::new_v1_formatted(&["user name \'",
                                                                                        " does not exist in /etc/passwd"],
                                                                                      &match (&&self.user_name,)
                                                                                           {
                                                                                           (arg0,)
                                                                                           =>
                                                                                           [$crate::fmt::ArgumentV1::new(arg0,
                                                                                                                         $crate::fmt::Debug::fmt)],
                                                                                       },
                                                                                      &[$crate::fmt::rt::v1::Argument{position:
                                                                                                                          $crate::fmt::rt::v1::Position::At(0usize),
                                                                                                                      format:
                                                                                                                          $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                              ' ',
                                                                                                                                                          align:
                                                                                                                                                              $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                          flags:
                                                                                                                                                              0u32,
                                                                                                                                                          precision:
                                                                                                                                                              $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                          width:
                                                                                                                                                              $crate::fmt::rt::v1::Count::Implied,},}]),
                                            &("dpdk-unix/src/Daemonize.rs",
                                              113u32, 3u32))
            }
        };
        unsafe { NonNull::new_unchecked(entry) }
    }
    #[inline(always)]
    fn switch_user(&self) -> PathBuf {
        Self::guard_we_are_root();
        let entry = self.get_user_entry();
        let (uid, gid, user_name, home_folder_path) =
            {
                let entry = unsafe { entry.as_ref() };
                (entry.pw_uid, entry.pw_gid,
                 NonNull::new(entry.pw_name).expect("pw_name was null"),
                 NonNull::new(entry.pw_dir).expect("pw_dir was null"))
            };
        let pid_file_path =
            self.create_pid_file_before_switching_user(uid, gid);
        {
            match (&(unsafe { setgid(gid) }), &(0)) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        {
                            $crate::rt::begin_panic_fmt(&$crate::fmt::Arguments::new_v1_formatted(&["assertion failed: `(left == right)`\n  left: `",
                                                                                                    "`,\n right: `",
                                                                                                    "`: "],
                                                                                                  &match (&left_val,
                                                                                                          &right_val,
                                                                                                          &$crate::fmt::Arguments::new_v1_formatted(&["Could not set group identifier to \'",
                                                                                                                                                      "\' because \'",
                                                                                                                                                      "\'"],
                                                                                                                                                    &match (&gid,
                                                                                                                                                            &Self::os_error())
                                                                                                                                                         {
                                                                                                                                                         (arg0,
                                                                                                                                                          arg1)
                                                                                                                                                         =>
                                                                                                                                                         [$crate::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                       $crate::fmt::Display::fmt),
                                                                                                                                                          $crate::fmt::ArgumentV1::new(arg1,
                                                                                                                                                                                       $crate::fmt::Display::fmt)],
                                                                                                                                                     },
                                                                                                                                                    &[$crate::fmt::rt::v1::Argument{position:
                                                                                                                                                                                        $crate::fmt::rt::v1::Position::At(0usize),
                                                                                                                                                                                    format:
                                                                                                                                                                                        $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                                                                            ' ',
                                                                                                                                                                                                                        align:
                                                                                                                                                                                                                            $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                                                                        flags:
                                                                                                                                                                                                                            0u32,
                                                                                                                                                                                                                        precision:
                                                                                                                                                                                                                            $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                                                                        width:
                                                                                                                                                                                                                            $crate::fmt::rt::v1::Count::Implied,},},
                                                                                                                                                      $crate::fmt::rt::v1::Argument{position:
                                                                                                                                                                                        $crate::fmt::rt::v1::Position::At(1usize),
                                                                                                                                                                                    format:
                                                                                                                                                                                        $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                                                                            ' ',
                                                                                                                                                                                                                        align:
                                                                                                                                                                                                                            $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                                                                        flags:
                                                                                                                                                                                                                            0u32,
                                                                                                                                                                                                                        precision:
                                                                                                                                                                                                                            $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                                                                        width:
                                                                                                                                                                                                                            $crate::fmt::rt::v1::Count::Implied,},}]))
                                                                                                       {
                                                                                                       (arg0,
                                                                                                        arg1,
                                                                                                        arg2)
                                                                                                       =>
                                                                                                       [$crate::fmt::ArgumentV1::new(arg0,
                                                                                                                                     $crate::fmt::Debug::fmt),
                                                                                                        $crate::fmt::ArgumentV1::new(arg1,
                                                                                                                                     $crate::fmt::Debug::fmt),
                                                                                                        $crate::fmt::ArgumentV1::new(arg2,
                                                                                                                                     $crate::fmt::Display::fmt)],
                                                                                                   },
                                                                                                  &[$crate::fmt::rt::v1::Argument{position:
                                                                                                                                      $crate::fmt::rt::v1::Position::At(0usize),
                                                                                                                                  format:
                                                                                                                                      $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                          ' ',
                                                                                                                                                                      align:
                                                                                                                                                                          $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                      flags:
                                                                                                                                                                          0u32,
                                                                                                                                                                      precision:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                      width:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,},},
                                                                                                    $crate::fmt::rt::v1::Argument{position:
                                                                                                                                      $crate::fmt::rt::v1::Position::At(1usize),
                                                                                                                                  format:
                                                                                                                                      $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                          ' ',
                                                                                                                                                                      align:
                                                                                                                                                                          $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                      flags:
                                                                                                                                                                          0u32,
                                                                                                                                                                      precision:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                      width:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,},},
                                                                                                    $crate::fmt::rt::v1::Argument{position:
                                                                                                                                      $crate::fmt::rt::v1::Position::At(2usize),
                                                                                                                                  format:
                                                                                                                                      $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                          ' ',
                                                                                                                                                                      align:
                                                                                                                                                                          $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                      flags:
                                                                                                                                                                          0u32,
                                                                                                                                                                      precision:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                      width:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,},}]),
                                                        &("dpdk-unix/src/Daemonize.rs",
                                                          137u32, 3u32))
                        }
                    }
                }
            }
        };
        {
            match (&(unsafe { initgroups(user_name.as_ptr(), gid as i32) }),
                   &(0)) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        {
                            $crate::rt::begin_panic_fmt(&$crate::fmt::Arguments::new_v1_formatted(&["assertion failed: `(left == right)`\n  left: `",
                                                                                                    "`,\n right: `",
                                                                                                    "`: "],
                                                                                                  &match (&left_val,
                                                                                                          &right_val,
                                                                                                          &$crate::fmt::Arguments::new_v1_formatted(&["Could not initialize additional groups for \'",
                                                                                                                                                      "\' because \'",
                                                                                                                                                      "\'"],
                                                                                                                                                    &match (&gid,
                                                                                                                                                            &Self::os_error())
                                                                                                                                                         {
                                                                                                                                                         (arg0,
                                                                                                                                                          arg1)
                                                                                                                                                         =>
                                                                                                                                                         [$crate::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                       $crate::fmt::Display::fmt),
                                                                                                                                                          $crate::fmt::ArgumentV1::new(arg1,
                                                                                                                                                                                       $crate::fmt::Display::fmt)],
                                                                                                                                                     },
                                                                                                                                                    &[$crate::fmt::rt::v1::Argument{position:
                                                                                                                                                                                        $crate::fmt::rt::v1::Position::At(0usize),
                                                                                                                                                                                    format:
                                                                                                                                                                                        $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                                                                            ' ',
                                                                                                                                                                                                                        align:
                                                                                                                                                                                                                            $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                                                                        flags:
                                                                                                                                                                                                                            0u32,
                                                                                                                                                                                                                        precision:
                                                                                                                                                                                                                            $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                                                                        width:
                                                                                                                                                                                                                            $crate::fmt::rt::v1::Count::Implied,},},
                                                                                                                                                      $crate::fmt::rt::v1::Argument{position:
                                                                                                                                                                                        $crate::fmt::rt::v1::Position::At(1usize),
                                                                                                                                                                                    format:
                                                                                                                                                                                        $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                                                                            ' ',
                                                                                                                                                                                                                        align:
                                                                                                                                                                                                                            $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                                                                        flags:
                                                                                                                                                                                                                            0u32,
                                                                                                                                                                                                                        precision:
                                                                                                                                                                                                                            $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                                                                        width:
                                                                                                                                                                                                                            $crate::fmt::rt::v1::Count::Implied,},}]))
                                                                                                       {
                                                                                                       (arg0,
                                                                                                        arg1,
                                                                                                        arg2)
                                                                                                       =>
                                                                                                       [$crate::fmt::ArgumentV1::new(arg0,
                                                                                                                                     $crate::fmt::Debug::fmt),
                                                                                                        $crate::fmt::ArgumentV1::new(arg1,
                                                                                                                                     $crate::fmt::Debug::fmt),
                                                                                                        $crate::fmt::ArgumentV1::new(arg2,
                                                                                                                                     $crate::fmt::Display::fmt)],
                                                                                                   },
                                                                                                  &[$crate::fmt::rt::v1::Argument{position:
                                                                                                                                      $crate::fmt::rt::v1::Position::At(0usize),
                                                                                                                                  format:
                                                                                                                                      $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                          ' ',
                                                                                                                                                                      align:
                                                                                                                                                                          $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                      flags:
                                                                                                                                                                          0u32,
                                                                                                                                                                      precision:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                      width:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,},},
                                                                                                    $crate::fmt::rt::v1::Argument{position:
                                                                                                                                      $crate::fmt::rt::v1::Position::At(1usize),
                                                                                                                                  format:
                                                                                                                                      $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                          ' ',
                                                                                                                                                                      align:
                                                                                                                                                                          $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                      flags:
                                                                                                                                                                          0u32,
                                                                                                                                                                      precision:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                      width:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,},},
                                                                                                    $crate::fmt::rt::v1::Argument{position:
                                                                                                                                      $crate::fmt::rt::v1::Position::At(2usize),
                                                                                                                                  format:
                                                                                                                                      $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                          ' ',
                                                                                                                                                                      align:
                                                                                                                                                                          $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                      flags:
                                                                                                                                                                          0u32,
                                                                                                                                                                      precision:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                      width:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,},}]),
                                                        &("dpdk-unix/src/Daemonize.rs",
                                                          139u32, 55u32))
                        }
                    }
                }
            }
        };
        Self::restrict_umask_to_current_user();
        {
            match (&(unsafe { setegid(gid) }), &(0)) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        {
                            $crate::rt::begin_panic_fmt(&$crate::fmt::Arguments::new_v1_formatted(&["assertion failed: `(left == right)`\n  left: `",
                                                                                                    "`,\n right: `",
                                                                                                    "`: "],
                                                                                                  &match (&left_val,
                                                                                                          &right_val,
                                                                                                          &$crate::fmt::Arguments::new_v1_formatted(&["Could not set effective group id to \'",
                                                                                                                                                      "\' because \'"],
                                                                                                                                                    &match (&gid,
                                                                                                                                                            &Self::os_error())
                                                                                                                                                         {
                                                                                                                                                         (arg0,
                                                                                                                                                          arg1)
                                                                                                                                                         =>
                                                                                                                                                         [$crate::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                       $crate::fmt::Display::fmt),
                                                                                                                                                          $crate::fmt::ArgumentV1::new(arg1,
                                                                                                                                                                                       $crate::fmt::Display::fmt)],
                                                                                                                                                     },
                                                                                                                                                    &[$crate::fmt::rt::v1::Argument{position:
                                                                                                                                                                                        $crate::fmt::rt::v1::Position::At(0usize),
                                                                                                                                                                                    format:
                                                                                                                                                                                        $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                                                                            ' ',
                                                                                                                                                                                                                        align:
                                                                                                                                                                                                                            $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                                                                        flags:
                                                                                                                                                                                                                            0u32,
                                                                                                                                                                                                                        precision:
                                                                                                                                                                                                                            $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                                                                        width:
                                                                                                                                                                                                                            $crate::fmt::rt::v1::Count::Implied,},},
                                                                                                                                                      $crate::fmt::rt::v1::Argument{position:
                                                                                                                                                                                        $crate::fmt::rt::v1::Position::At(1usize),
                                                                                                                                                                                    format:
                                                                                                                                                                                        $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                                                                            ' ',
                                                                                                                                                                                                                        align:
                                                                                                                                                                                                                            $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                                                                        flags:
                                                                                                                                                                                                                            0u32,
                                                                                                                                                                                                                        precision:
                                                                                                                                                                                                                            $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                                                                        width:
                                                                                                                                                                                                                            $crate::fmt::rt::v1::Count::Implied,},}]))
                                                                                                       {
                                                                                                       (arg0,
                                                                                                        arg1,
                                                                                                        arg2)
                                                                                                       =>
                                                                                                       [$crate::fmt::ArgumentV1::new(arg0,
                                                                                                                                     $crate::fmt::Debug::fmt),
                                                                                                        $crate::fmt::ArgumentV1::new(arg1,
                                                                                                                                     $crate::fmt::Debug::fmt),
                                                                                                        $crate::fmt::ArgumentV1::new(arg2,
                                                                                                                                     $crate::fmt::Display::fmt)],
                                                                                                   },
                                                                                                  &[$crate::fmt::rt::v1::Argument{position:
                                                                                                                                      $crate::fmt::rt::v1::Position::At(0usize),
                                                                                                                                  format:
                                                                                                                                      $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                          ' ',
                                                                                                                                                                      align:
                                                                                                                                                                          $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                      flags:
                                                                                                                                                                          0u32,
                                                                                                                                                                      precision:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                      width:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,},},
                                                                                                    $crate::fmt::rt::v1::Argument{position:
                                                                                                                                      $crate::fmt::rt::v1::Position::At(1usize),
                                                                                                                                  format:
                                                                                                                                      $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                          ' ',
                                                                                                                                                                      align:
                                                                                                                                                                          $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                      flags:
                                                                                                                                                                          0u32,
                                                                                                                                                                      precision:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                      width:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,},},
                                                                                                    $crate::fmt::rt::v1::Argument{position:
                                                                                                                                      $crate::fmt::rt::v1::Position::At(2usize),
                                                                                                                                  format:
                                                                                                                                      $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                          ' ',
                                                                                                                                                                      align:
                                                                                                                                                                          $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                      flags:
                                                                                                                                                                          0u32,
                                                                                                                                                                      precision:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                      width:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,},}]),
                                                        &("dpdk-unix/src/Daemonize.rs",
                                                          143u32, 3u32))
                        }
                    }
                }
            }
        };
        {
            match (&(unsafe { setuid(uid) }), &(0)) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        {
                            $crate::rt::begin_panic_fmt(&$crate::fmt::Arguments::new_v1_formatted(&["assertion failed: `(left == right)`\n  left: `",
                                                                                                    "`,\n right: `",
                                                                                                    "`: "],
                                                                                                  &match (&left_val,
                                                                                                          &right_val,
                                                                                                          &$crate::fmt::Arguments::new_v1_formatted(&["Could not set user id to \'",
                                                                                                                                                      "\' because \'"],
                                                                                                                                                    &match (&uid,
                                                                                                                                                            &Self::os_error())
                                                                                                                                                         {
                                                                                                                                                         (arg0,
                                                                                                                                                          arg1)
                                                                                                                                                         =>
                                                                                                                                                         [$crate::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                       $crate::fmt::Display::fmt),
                                                                                                                                                          $crate::fmt::ArgumentV1::new(arg1,
                                                                                                                                                                                       $crate::fmt::Display::fmt)],
                                                                                                                                                     },
                                                                                                                                                    &[$crate::fmt::rt::v1::Argument{position:
                                                                                                                                                                                        $crate::fmt::rt::v1::Position::At(0usize),
                                                                                                                                                                                    format:
                                                                                                                                                                                        $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                                                                            ' ',
                                                                                                                                                                                                                        align:
                                                                                                                                                                                                                            $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                                                                        flags:
                                                                                                                                                                                                                            0u32,
                                                                                                                                                                                                                        precision:
                                                                                                                                                                                                                            $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                                                                        width:
                                                                                                                                                                                                                            $crate::fmt::rt::v1::Count::Implied,},},
                                                                                                                                                      $crate::fmt::rt::v1::Argument{position:
                                                                                                                                                                                        $crate::fmt::rt::v1::Position::At(1usize),
                                                                                                                                                                                    format:
                                                                                                                                                                                        $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                                                                            ' ',
                                                                                                                                                                                                                        align:
                                                                                                                                                                                                                            $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                                                                        flags:
                                                                                                                                                                                                                            0u32,
                                                                                                                                                                                                                        precision:
                                                                                                                                                                                                                            $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                                                                        width:
                                                                                                                                                                                                                            $crate::fmt::rt::v1::Count::Implied,},}]))
                                                                                                       {
                                                                                                       (arg0,
                                                                                                        arg1,
                                                                                                        arg2)
                                                                                                       =>
                                                                                                       [$crate::fmt::ArgumentV1::new(arg0,
                                                                                                                                     $crate::fmt::Debug::fmt),
                                                                                                        $crate::fmt::ArgumentV1::new(arg1,
                                                                                                                                     $crate::fmt::Debug::fmt),
                                                                                                        $crate::fmt::ArgumentV1::new(arg2,
                                                                                                                                     $crate::fmt::Display::fmt)],
                                                                                                   },
                                                                                                  &[$crate::fmt::rt::v1::Argument{position:
                                                                                                                                      $crate::fmt::rt::v1::Position::At(0usize),
                                                                                                                                  format:
                                                                                                                                      $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                          ' ',
                                                                                                                                                                      align:
                                                                                                                                                                          $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                      flags:
                                                                                                                                                                          0u32,
                                                                                                                                                                      precision:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                      width:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,},},
                                                                                                    $crate::fmt::rt::v1::Argument{position:
                                                                                                                                      $crate::fmt::rt::v1::Position::At(1usize),
                                                                                                                                  format:
                                                                                                                                      $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                          ' ',
                                                                                                                                                                      align:
                                                                                                                                                                          $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                      flags:
                                                                                                                                                                          0u32,
                                                                                                                                                                      precision:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                      width:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,},},
                                                                                                    $crate::fmt::rt::v1::Argument{position:
                                                                                                                                      $crate::fmt::rt::v1::Position::At(2usize),
                                                                                                                                  format:
                                                                                                                                      $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                          ' ',
                                                                                                                                                                      align:
                                                                                                                                                                          $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                      flags:
                                                                                                                                                                          0u32,
                                                                                                                                                                      precision:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                      width:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,},}]),
                                                        &("dpdk-unix/src/Daemonize.rs",
                                                          144u32, 3u32))
                        }
                    }
                }
            }
        };
        {
            match (&(unsafe { seteuid(uid) }), &(0)) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        {
                            $crate::rt::begin_panic_fmt(&$crate::fmt::Arguments::new_v1_formatted(&["assertion failed: `(left == right)`\n  left: `",
                                                                                                    "`,\n right: `",
                                                                                                    "`: "],
                                                                                                  &match (&left_val,
                                                                                                          &right_val,
                                                                                                          &$crate::fmt::Arguments::new_v1_formatted(&["Could not set effective user id to \'",
                                                                                                                                                      "\' because \'"],
                                                                                                                                                    &match (&uid,
                                                                                                                                                            &Self::os_error())
                                                                                                                                                         {
                                                                                                                                                         (arg0,
                                                                                                                                                          arg1)
                                                                                                                                                         =>
                                                                                                                                                         [$crate::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                       $crate::fmt::Display::fmt),
                                                                                                                                                          $crate::fmt::ArgumentV1::new(arg1,
                                                                                                                                                                                       $crate::fmt::Display::fmt)],
                                                                                                                                                     },
                                                                                                                                                    &[$crate::fmt::rt::v1::Argument{position:
                                                                                                                                                                                        $crate::fmt::rt::v1::Position::At(0usize),
                                                                                                                                                                                    format:
                                                                                                                                                                                        $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                                                                            ' ',
                                                                                                                                                                                                                        align:
                                                                                                                                                                                                                            $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                                                                        flags:
                                                                                                                                                                                                                            0u32,
                                                                                                                                                                                                                        precision:
                                                                                                                                                                                                                            $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                                                                        width:
                                                                                                                                                                                                                            $crate::fmt::rt::v1::Count::Implied,},},
                                                                                                                                                      $crate::fmt::rt::v1::Argument{position:
                                                                                                                                                                                        $crate::fmt::rt::v1::Position::At(1usize),
                                                                                                                                                                                    format:
                                                                                                                                                                                        $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                                                                            ' ',
                                                                                                                                                                                                                        align:
                                                                                                                                                                                                                            $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                                                                        flags:
                                                                                                                                                                                                                            0u32,
                                                                                                                                                                                                                        precision:
                                                                                                                                                                                                                            $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                                                                        width:
                                                                                                                                                                                                                            $crate::fmt::rt::v1::Count::Implied,},}]))
                                                                                                       {
                                                                                                       (arg0,
                                                                                                        arg1,
                                                                                                        arg2)
                                                                                                       =>
                                                                                                       [$crate::fmt::ArgumentV1::new(arg0,
                                                                                                                                     $crate::fmt::Debug::fmt),
                                                                                                        $crate::fmt::ArgumentV1::new(arg1,
                                                                                                                                     $crate::fmt::Debug::fmt),
                                                                                                        $crate::fmt::ArgumentV1::new(arg2,
                                                                                                                                     $crate::fmt::Display::fmt)],
                                                                                                   },
                                                                                                  &[$crate::fmt::rt::v1::Argument{position:
                                                                                                                                      $crate::fmt::rt::v1::Position::At(0usize),
                                                                                                                                  format:
                                                                                                                                      $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                          ' ',
                                                                                                                                                                      align:
                                                                                                                                                                          $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                      flags:
                                                                                                                                                                          0u32,
                                                                                                                                                                      precision:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                      width:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,},},
                                                                                                    $crate::fmt::rt::v1::Argument{position:
                                                                                                                                      $crate::fmt::rt::v1::Position::At(1usize),
                                                                                                                                  format:
                                                                                                                                      $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                          ' ',
                                                                                                                                                                      align:
                                                                                                                                                                          $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                      flags:
                                                                                                                                                                          0u32,
                                                                                                                                                                      precision:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                      width:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,},},
                                                                                                    $crate::fmt::rt::v1::Argument{position:
                                                                                                                                      $crate::fmt::rt::v1::Position::At(2usize),
                                                                                                                                  format:
                                                                                                                                      $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                          ' ',
                                                                                                                                                                      align:
                                                                                                                                                                          $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                      flags:
                                                                                                                                                                          0u32,
                                                                                                                                                                      precision:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                      width:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,},}]),
                                                        &("dpdk-unix/src/Daemonize.rs",
                                                          145u32, 3u32))
                        }
                    }
                }
            }
        };
        Self::make_environment_variables_match_user(user_name,
                                                    home_folder_path);
        pid_file_path
    }
    #[inline(always)]
    fn guard_we_are_root() {
        assert_effective_user_id_is_root("Changing user in daemonize()");
    }
    #[inline(always)]
    fn restrict_umask_to_current_user() { unsafe { umask(63) }; }
    #[inline(always)]
    fn create_pid_file_before_switching_user(&self, uid: uid_t, gid: gid_t)
     -> PathBuf {
        let pid_file_path = self.pid_file_path();
        let pid_file_path_string = pid_file_path.to_c_string();
        let file_descriptor =
            unsafe {
                open(pid_file_path_string.as_ptr(), O_CREAT | O_WRONLY,
                     (S_IRUSR | S_IWUSR | S_IRGRP | S_IROTH) as u32)
            };
        if !(file_descriptor >= 0) {
            {
                $crate::rt::begin_panic_fmt(&$crate::fmt::Arguments::new_v1_formatted(&["Could not create PID file \'",
                                                                                        "\' because \'",
                                                                                        "\'"],
                                                                                      &match (&&pid_file_path_string,
                                                                                              &Self::os_error())
                                                                                           {
                                                                                           (arg0,
                                                                                            arg1)
                                                                                           =>
                                                                                           [$crate::fmt::ArgumentV1::new(arg0,
                                                                                                                         $crate::fmt::Debug::fmt),
                                                                                            $crate::fmt::ArgumentV1::new(arg1,
                                                                                                                         $crate::fmt::Display::fmt)],
                                                                                       },
                                                                                      &[$crate::fmt::rt::v1::Argument{position:
                                                                                                                          $crate::fmt::rt::v1::Position::At(0usize),
                                                                                                                      format:
                                                                                                                          $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                              ' ',
                                                                                                                                                          align:
                                                                                                                                                              $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                          flags:
                                                                                                                                                              0u32,
                                                                                                                                                          precision:
                                                                                                                                                              $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                          width:
                                                                                                                                                              $crate::fmt::rt::v1::Count::Implied,},},
                                                                                        $crate::fmt::rt::v1::Argument{position:
                                                                                                                          $crate::fmt::rt::v1::Position::At(1usize),
                                                                                                                      format:
                                                                                                                          $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                              ' ',
                                                                                                                                                          align:
                                                                                                                                                              $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                          flags:
                                                                                                                                                              0u32,
                                                                                                                                                          precision:
                                                                                                                                                              $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                          width:
                                                                                                                                                              $crate::fmt::rt::v1::Count::Implied,},}]),
                                            &("dpdk-unix/src/Daemonize.rs",
                                              171u32, 3u32))
            }
        };
        {
            match (&(unsafe { fchown(file_descriptor, uid, gid) }), &(0)) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        {
                            $crate::rt::begin_panic_fmt(&$crate::fmt::Arguments::new_v1_formatted(&["assertion failed: `(left == right)`\n  left: `",
                                                                                                    "`,\n right: `",
                                                                                                    "`: "],
                                                                                                  &match (&left_val,
                                                                                                          &right_val,
                                                                                                          &$crate::fmt::Arguments::new_v1_formatted(&["Could not change ownership of PID file \'",
                                                                                                                                                      "\' because \'",
                                                                                                                                                      "\'"],
                                                                                                                                                    &match (&&pid_file_path_string,
                                                                                                                                                            &Self::os_error())
                                                                                                                                                         {
                                                                                                                                                         (arg0,
                                                                                                                                                          arg1)
                                                                                                                                                         =>
                                                                                                                                                         [$crate::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                       $crate::fmt::Debug::fmt),
                                                                                                                                                          $crate::fmt::ArgumentV1::new(arg1,
                                                                                                                                                                                       $crate::fmt::Display::fmt)],
                                                                                                                                                     },
                                                                                                                                                    &[$crate::fmt::rt::v1::Argument{position:
                                                                                                                                                                                        $crate::fmt::rt::v1::Position::At(0usize),
                                                                                                                                                                                    format:
                                                                                                                                                                                        $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                                                                            ' ',
                                                                                                                                                                                                                        align:
                                                                                                                                                                                                                            $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                                                                        flags:
                                                                                                                                                                                                                            0u32,
                                                                                                                                                                                                                        precision:
                                                                                                                                                                                                                            $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                                                                        width:
                                                                                                                                                                                                                            $crate::fmt::rt::v1::Count::Implied,},},
                                                                                                                                                      $crate::fmt::rt::v1::Argument{position:
                                                                                                                                                                                        $crate::fmt::rt::v1::Position::At(1usize),
                                                                                                                                                                                    format:
                                                                                                                                                                                        $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                                                                            ' ',
                                                                                                                                                                                                                        align:
                                                                                                                                                                                                                            $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                                                                        flags:
                                                                                                                                                                                                                            0u32,
                                                                                                                                                                                                                        precision:
                                                                                                                                                                                                                            $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                                                                        width:
                                                                                                                                                                                                                            $crate::fmt::rt::v1::Count::Implied,},}]))
                                                                                                       {
                                                                                                       (arg0,
                                                                                                        arg1,
                                                                                                        arg2)
                                                                                                       =>
                                                                                                       [$crate::fmt::ArgumentV1::new(arg0,
                                                                                                                                     $crate::fmt::Debug::fmt),
                                                                                                        $crate::fmt::ArgumentV1::new(arg1,
                                                                                                                                     $crate::fmt::Debug::fmt),
                                                                                                        $crate::fmt::ArgumentV1::new(arg2,
                                                                                                                                     $crate::fmt::Display::fmt)],
                                                                                                   },
                                                                                                  &[$crate::fmt::rt::v1::Argument{position:
                                                                                                                                      $crate::fmt::rt::v1::Position::At(0usize),
                                                                                                                                  format:
                                                                                                                                      $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                          ' ',
                                                                                                                                                                      align:
                                                                                                                                                                          $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                      flags:
                                                                                                                                                                          0u32,
                                                                                                                                                                      precision:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                      width:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,},},
                                                                                                    $crate::fmt::rt::v1::Argument{position:
                                                                                                                                      $crate::fmt::rt::v1::Position::At(1usize),
                                                                                                                                  format:
                                                                                                                                      $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                          ' ',
                                                                                                                                                                      align:
                                                                                                                                                                          $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                      flags:
                                                                                                                                                                          0u32,
                                                                                                                                                                      precision:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                      width:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,},},
                                                                                                    $crate::fmt::rt::v1::Argument{position:
                                                                                                                                      $crate::fmt::rt::v1::Position::At(2usize),
                                                                                                                                  format:
                                                                                                                                      $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                          ' ',
                                                                                                                                                                      align:
                                                                                                                                                                          $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                      flags:
                                                                                                                                                                          0u32,
                                                                                                                                                                      precision:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                      width:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,},}]),
                                                        &("dpdk-unix/src/Daemonize.rs",
                                                          172u32, 3u32))
                        }
                    }
                }
            }
        };
        unsafe { close(file_descriptor) };
        pid_file_path
    }
    #[inline(always)]
    fn populate_pid_file_when_running(&self, pid_file_path: &Path) {
        pid_file_path.write_value(process::id()).unwrap();
    }
    #[inline(always)]
    fn make_environment_variables_match_user(user_name: NonNull<c_char>,
                                             home_folder_path:
                                                 NonNull<c_char>) {
        const USER: $crate::ConstCStr =
            $crate::ConstCStr{rustValue: "USER", cValue: "USER\u{0}",};
        const LOGNAME: $crate::ConstCStr =
            $crate::ConstCStr{rustValue: "LOGNAME", cValue: "LOGNAME\u{0}",};
        const HOME: $crate::ConstCStr =
            $crate::ConstCStr{rustValue: "HOME", cValue: "HOME\u{0}",};
        Self::set_environment_variable(USER, user_name);
        Self::set_environment_variable(LOGNAME, user_name);
        Self::set_environment_variable(HOME, home_folder_path);
    }
    #[inline(always)]
    fn change_current_working_directory(&self) {
        let c_string = self.working_folder_path.to_c_string();
        {
            match (&(unsafe { chdir(c_string.as_ptr()) }), &(0)) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        {
                            $crate::rt::begin_panic_fmt(&$crate::fmt::Arguments::new_v1_formatted(&["assertion failed: `(left == right)`\n  left: `",
                                                                                                    "`,\n right: `",
                                                                                                    "`: "],
                                                                                                  &match (&left_val,
                                                                                                          &right_val,
                                                                                                          &$crate::fmt::Arguments::new_v1_formatted(&["Could not change current working directory to \'",
                                                                                                                                                      "\' because \'",
                                                                                                                                                      "\'"],
                                                                                                                                                    &match (&c_string,
                                                                                                                                                            &Self::os_error())
                                                                                                                                                         {
                                                                                                                                                         (arg0,
                                                                                                                                                          arg1)
                                                                                                                                                         =>
                                                                                                                                                         [$crate::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                       $crate::fmt::Debug::fmt),
                                                                                                                                                          $crate::fmt::ArgumentV1::new(arg1,
                                                                                                                                                                                       $crate::fmt::Display::fmt)],
                                                                                                                                                     },
                                                                                                                                                    &[$crate::fmt::rt::v1::Argument{position:
                                                                                                                                                                                        $crate::fmt::rt::v1::Position::At(0usize),
                                                                                                                                                                                    format:
                                                                                                                                                                                        $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                                                                            ' ',
                                                                                                                                                                                                                        align:
                                                                                                                                                                                                                            $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                                                                        flags:
                                                                                                                                                                                                                            0u32,
                                                                                                                                                                                                                        precision:
                                                                                                                                                                                                                            $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                                                                        width:
                                                                                                                                                                                                                            $crate::fmt::rt::v1::Count::Implied,},},
                                                                                                                                                      $crate::fmt::rt::v1::Argument{position:
                                                                                                                                                                                        $crate::fmt::rt::v1::Position::At(1usize),
                                                                                                                                                                                    format:
                                                                                                                                                                                        $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                                                                            ' ',
                                                                                                                                                                                                                        align:
                                                                                                                                                                                                                            $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                                                                        flags:
                                                                                                                                                                                                                            0u32,
                                                                                                                                                                                                                        precision:
                                                                                                                                                                                                                            $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                                                                        width:
                                                                                                                                                                                                                            $crate::fmt::rt::v1::Count::Implied,},}]))
                                                                                                       {
                                                                                                       (arg0,
                                                                                                        arg1,
                                                                                                        arg2)
                                                                                                       =>
                                                                                                       [$crate::fmt::ArgumentV1::new(arg0,
                                                                                                                                     $crate::fmt::Debug::fmt),
                                                                                                        $crate::fmt::ArgumentV1::new(arg1,
                                                                                                                                     $crate::fmt::Debug::fmt),
                                                                                                        $crate::fmt::ArgumentV1::new(arg2,
                                                                                                                                     $crate::fmt::Display::fmt)],
                                                                                                   },
                                                                                                  &[$crate::fmt::rt::v1::Argument{position:
                                                                                                                                      $crate::fmt::rt::v1::Position::At(0usize),
                                                                                                                                  format:
                                                                                                                                      $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                          ' ',
                                                                                                                                                                      align:
                                                                                                                                                                          $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                      flags:
                                                                                                                                                                          0u32,
                                                                                                                                                                      precision:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                      width:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,},},
                                                                                                    $crate::fmt::rt::v1::Argument{position:
                                                                                                                                      $crate::fmt::rt::v1::Position::At(1usize),
                                                                                                                                  format:
                                                                                                                                      $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                          ' ',
                                                                                                                                                                      align:
                                                                                                                                                                          $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                      flags:
                                                                                                                                                                          0u32,
                                                                                                                                                                      precision:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                      width:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,},},
                                                                                                    $crate::fmt::rt::v1::Argument{position:
                                                                                                                                      $crate::fmt::rt::v1::Position::At(2usize),
                                                                                                                                  format:
                                                                                                                                      $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                          ' ',
                                                                                                                                                                      align:
                                                                                                                                                                          $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                      flags:
                                                                                                                                                                          0u32,
                                                                                                                                                                      precision:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                      width:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,},}]),
                                                        &("dpdk-unix/src/Daemonize.rs",
                                                          202u32, 3u32))
                        }
                    }
                }
            }
        };
    }
    #[inline(always)]
    fn create_a_new_progress_group_and_session_detach_controlling_terminal() {
        if !(unsafe { setsid() } >= 0) {
            {
                $crate::rt::begin_panic_fmt(&$crate::fmt::Arguments::new_v1_formatted(&["setsid failed because \'",
                                                                                        "\'"],
                                                                                      &match (&Self::os_error(),)
                                                                                           {
                                                                                           (arg0,)
                                                                                           =>
                                                                                           [$crate::fmt::ArgumentV1::new(arg0,
                                                                                                                         $crate::fmt::Display::fmt)],
                                                                                       },
                                                                                      &[$crate::fmt::rt::v1::Argument{position:
                                                                                                                          $crate::fmt::rt::v1::Position::At(0usize),
                                                                                                                      format:
                                                                                                                          $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                              ' ',
                                                                                                                                                          align:
                                                                                                                                                              $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                          flags:
                                                                                                                                                              0u32,
                                                                                                                                                          precision:
                                                                                                                                                              $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                          width:
                                                                                                                                                              $crate::fmt::rt::v1::Count::Implied,},}]),
                                            &("dpdk-unix/src/Daemonize.rs",
                                              208u32, 3u32))
            }
        };
    }
    #[inline(always)]
    fn fork() {
        const ForkedToChild: i32 = 0;
        match unsafe { fork() } {
            ForkedToChild => (),
            -1 => {
                $crate::rt::begin_panic_fmt(&$crate::fmt::Arguments::new_v1_formatted(&["Fork failed with "],
                                                                                      &match (&Self::os_error(),)
                                                                                           {
                                                                                           (arg0,)
                                                                                           =>
                                                                                           [$crate::fmt::ArgumentV1::new(arg0,
                                                                                                                         $crate::fmt::Display::fmt)],
                                                                                       },
                                                                                      &[$crate::fmt::rt::v1::Argument{position:
                                                                                                                          $crate::fmt::rt::v1::Position::At(0usize),
                                                                                                                      format:
                                                                                                                          $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                              ' ',
                                                                                                                                                          align:
                                                                                                                                                              $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                          flags:
                                                                                                                                                              0u32,
                                                                                                                                                          precision:
                                                                                                                                                              $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                          width:
                                                                                                                                                              $crate::fmt::rt::v1::Count::Implied,},}]),
                                            &("dpdk-unix/src/Daemonize.rs",
                                              219u32, 10u32))
            }
            _child_process_id_returned_to_parent@_ => process::exit(0),
        }
    }
    #[inline(always)]
    fn redirect_standard_in_out_and_error(&self) {
        Self::redirect_to_dev_null(&io::stdin());
        Self::redirect_to_dev_null(&io::stdout());
        Self::redirect_to_dev_null(&io::stderr());
    }
    #[inline(always)]
    fn redirect_to_dev_null<A: AsRawFd>(a: &A) {
        const DevNull: $crate::ConstCStr =
            $crate::ConstCStr{rustValue: "/dev/null",
                              cValue: "/dev/null\u{0}",};
        let file_descriptor = a.as_raw_fd();
        let null_file_descriptor =
            unsafe { open(DevNull.as_ptr(), O_WRONLY) };
        if !(null_file_descriptor >= 0) {
            {
                $crate::rt::begin_panic_fmt(&$crate::fmt::Arguments::new_v1_formatted(&["Could not open /dev/null because \'",
                                                                                        "\'"],
                                                                                      &match (&Self::os_error(),)
                                                                                           {
                                                                                           (arg0,)
                                                                                           =>
                                                                                           [$crate::fmt::ArgumentV1::new(arg0,
                                                                                                                         $crate::fmt::Display::fmt)],
                                                                                       },
                                                                                      &[$crate::fmt::rt::v1::Argument{position:
                                                                                                                          $crate::fmt::rt::v1::Position::At(0usize),
                                                                                                                      format:
                                                                                                                          $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                              ' ',
                                                                                                                                                          align:
                                                                                                                                                              $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                          flags:
                                                                                                                                                              0u32,
                                                                                                                                                          precision:
                                                                                                                                                              $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                          width:
                                                                                                                                                              $crate::fmt::rt::v1::Count::Implied,},}]),
                                            &("dpdk-unix/src/Daemonize.rs",
                                              287u32, 3u32))
            }
        };
        {
            match (&(unsafe { dup2(null_file_descriptor, file_descriptor) }),
                   &(0)) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        {
                            $crate::rt::begin_panic_fmt(&$crate::fmt::Arguments::new_v1_formatted(&["assertion failed: `(left == right)`\n  left: `",
                                                                                                    "`,\n right: `",
                                                                                                    "`: "],
                                                                                                  &match (&left_val,
                                                                                                          &right_val,
                                                                                                          &$crate::fmt::Arguments::new_v1_formatted(&["Could not dup2 because \'",
                                                                                                                                                      "\'"],
                                                                                                                                                    &match (&Self::os_error(),)
                                                                                                                                                         {
                                                                                                                                                         (arg0,)
                                                                                                                                                         =>
                                                                                                                                                         [$crate::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                       $crate::fmt::Display::fmt)],
                                                                                                                                                     },
                                                                                                                                                    &[$crate::fmt::rt::v1::Argument{position:
                                                                                                                                                                                        $crate::fmt::rt::v1::Position::At(0usize),
                                                                                                                                                                                    format:
                                                                                                                                                                                        $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                                                                            ' ',
                                                                                                                                                                                                                        align:
                                                                                                                                                                                                                            $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                                                                        flags:
                                                                                                                                                                                                                            0u32,
                                                                                                                                                                                                                        precision:
                                                                                                                                                                                                                            $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                                                                        width:
                                                                                                                                                                                                                            $crate::fmt::rt::v1::Count::Implied,},}]))
                                                                                                       {
                                                                                                       (arg0,
                                                                                                        arg1,
                                                                                                        arg2)
                                                                                                       =>
                                                                                                       [$crate::fmt::ArgumentV1::new(arg0,
                                                                                                                                     $crate::fmt::Debug::fmt),
                                                                                                        $crate::fmt::ArgumentV1::new(arg1,
                                                                                                                                     $crate::fmt::Debug::fmt),
                                                                                                        $crate::fmt::ArgumentV1::new(arg2,
                                                                                                                                     $crate::fmt::Display::fmt)],
                                                                                                   },
                                                                                                  &[$crate::fmt::rt::v1::Argument{position:
                                                                                                                                      $crate::fmt::rt::v1::Position::At(0usize),
                                                                                                                                  format:
                                                                                                                                      $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                          ' ',
                                                                                                                                                                      align:
                                                                                                                                                                          $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                      flags:
                                                                                                                                                                          0u32,
                                                                                                                                                                      precision:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                      width:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,},},
                                                                                                    $crate::fmt::rt::v1::Argument{position:
                                                                                                                                      $crate::fmt::rt::v1::Position::At(1usize),
                                                                                                                                  format:
                                                                                                                                      $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                          ' ',
                                                                                                                                                                      align:
                                                                                                                                                                          $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                      flags:
                                                                                                                                                                          0u32,
                                                                                                                                                                      precision:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                      width:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,},},
                                                                                                    $crate::fmt::rt::v1::Argument{position:
                                                                                                                                      $crate::fmt::rt::v1::Position::At(2usize),
                                                                                                                                  format:
                                                                                                                                      $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                          ' ',
                                                                                                                                                                      align:
                                                                                                                                                                          $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                      flags:
                                                                                                                                                                          0u32,
                                                                                                                                                                      precision:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                      width:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,},}]),
                                                        &("dpdk-unix/src/Daemonize.rs",
                                                          288u32, 3u32))
                        }
                    }
                }
            }
        };
        {
            match (&(unsafe { close(null_file_descriptor) }), &(0)) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        {
                            $crate::rt::begin_panic_fmt(&$crate::fmt::Arguments::new_v1_formatted(&["assertion failed: `(left == right)`\n  left: `",
                                                                                                    "`,\n right: `",
                                                                                                    "`: "],
                                                                                                  &match (&left_val,
                                                                                                          &right_val,
                                                                                                          &$crate::fmt::Arguments::new_v1_formatted(&["Could not close null file descriptor because \'",
                                                                                                                                                      "\'"],
                                                                                                                                                    &match (&Self::os_error(),)
                                                                                                                                                         {
                                                                                                                                                         (arg0,)
                                                                                                                                                         =>
                                                                                                                                                         [$crate::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                       $crate::fmt::Display::fmt)],
                                                                                                                                                     },
                                                                                                                                                    &[$crate::fmt::rt::v1::Argument{position:
                                                                                                                                                                                        $crate::fmt::rt::v1::Position::At(0usize),
                                                                                                                                                                                    format:
                                                                                                                                                                                        $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                                                                            ' ',
                                                                                                                                                                                                                        align:
                                                                                                                                                                                                                            $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                                                                        flags:
                                                                                                                                                                                                                            0u32,
                                                                                                                                                                                                                        precision:
                                                                                                                                                                                                                            $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                                                                        width:
                                                                                                                                                                                                                            $crate::fmt::rt::v1::Count::Implied,},}]))
                                                                                                       {
                                                                                                       (arg0,
                                                                                                        arg1,
                                                                                                        arg2)
                                                                                                       =>
                                                                                                       [$crate::fmt::ArgumentV1::new(arg0,
                                                                                                                                     $crate::fmt::Debug::fmt),
                                                                                                        $crate::fmt::ArgumentV1::new(arg1,
                                                                                                                                     $crate::fmt::Debug::fmt),
                                                                                                        $crate::fmt::ArgumentV1::new(arg2,
                                                                                                                                     $crate::fmt::Display::fmt)],
                                                                                                   },
                                                                                                  &[$crate::fmt::rt::v1::Argument{position:
                                                                                                                                      $crate::fmt::rt::v1::Position::At(0usize),
                                                                                                                                  format:
                                                                                                                                      $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                          ' ',
                                                                                                                                                                      align:
                                                                                                                                                                          $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                      flags:
                                                                                                                                                                          0u32,
                                                                                                                                                                      precision:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                      width:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,},},
                                                                                                    $crate::fmt::rt::v1::Argument{position:
                                                                                                                                      $crate::fmt::rt::v1::Position::At(1usize),
                                                                                                                                  format:
                                                                                                                                      $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                          ' ',
                                                                                                                                                                      align:
                                                                                                                                                                          $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                      flags:
                                                                                                                                                                          0u32,
                                                                                                                                                                      precision:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                      width:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,},},
                                                                                                    $crate::fmt::rt::v1::Argument{position:
                                                                                                                                      $crate::fmt::rt::v1::Position::At(2usize),
                                                                                                                                  format:
                                                                                                                                      $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                          ' ',
                                                                                                                                                                      align:
                                                                                                                                                                          $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                      flags:
                                                                                                                                                                          0u32,
                                                                                                                                                                      precision:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                      width:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,},}]),
                                                        &("dpdk-unix/src/Daemonize.rs",
                                                          289u32, 3u32))
                        }
                    }
                }
            }
        };
    }
    #[inline(always)]
    fn pid_file_path(&self) -> PathBuf {
        self.pid_folder_path.join(PathBuf::from($crate::fmt::format($crate::fmt::Arguments::new_v1_formatted(&["",
                                                                                                               ".pid"],
                                                                                                             &match (&get_program_name(),)
                                                                                                                  {
                                                                                                                  (arg0,)
                                                                                                                  =>
                                                                                                                  [$crate::fmt::ArgumentV1::new(arg0,
                                                                                                                                                $crate::fmt::Display::fmt)],
                                                                                                              },
                                                                                                             &[$crate::fmt::rt::v1::Argument{position:
                                                                                                                                                 $crate::fmt::rt::v1::Position::At(0usize),
                                                                                                                                             format:
                                                                                                                                                 $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                                     ' ',
                                                                                                                                                                                 align:
                                                                                                                                                                                     $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                                 flags:
                                                                                                                                                                                     0u32,
                                                                                                                                                                                 precision:
                                                                                                                                                                                     $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                                 width:
                                                                                                                                                                                     $crate::fmt::rt::v1::Count::Implied,},}]))))
    }
    #[inline(always)]
    fn ensure_environment_variable_is_set(name: &str, value: &str) {
        if var_os(name).is_none() { set_var(name, value) }
    }
    #[inline(always)]
    fn set_environment_variable(name: ConstCStr, value: NonNull<c_char>) {
        const Overwrite: i32 = 1;
        {
            match (&(unsafe {
                         setenv(name.as_ptr(), value.as_ptr(), Overwrite)
                     }), &(0)) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        {
                            $crate::rt::begin_panic_fmt(&$crate::fmt::Arguments::new_v1_formatted(&["assertion failed: `(left == right)`\n  left: `",
                                                                                                    "`,\n right: `",
                                                                                                    "`: "],
                                                                                                  &match (&left_val,
                                                                                                          &right_val,
                                                                                                          &$crate::fmt::Arguments::new_v1_formatted(&["Could not set environment variable \'",
                                                                                                                                                      "\' because \'"],
                                                                                                                                                    &match (&name,
                                                                                                                                                            &Self::os_error())
                                                                                                                                                         {
                                                                                                                                                         (arg0,
                                                                                                                                                          arg1)
                                                                                                                                                         =>
                                                                                                                                                         [$crate::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                       $crate::fmt::Debug::fmt),
                                                                                                                                                          $crate::fmt::ArgumentV1::new(arg1,
                                                                                                                                                                                       $crate::fmt::Display::fmt)],
                                                                                                                                                     },
                                                                                                                                                    &[$crate::fmt::rt::v1::Argument{position:
                                                                                                                                                                                        $crate::fmt::rt::v1::Position::At(0usize),
                                                                                                                                                                                    format:
                                                                                                                                                                                        $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                                                                            ' ',
                                                                                                                                                                                                                        align:
                                                                                                                                                                                                                            $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                                                                        flags:
                                                                                                                                                                                                                            0u32,
                                                                                                                                                                                                                        precision:
                                                                                                                                                                                                                            $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                                                                        width:
                                                                                                                                                                                                                            $crate::fmt::rt::v1::Count::Implied,},},
                                                                                                                                                      $crate::fmt::rt::v1::Argument{position:
                                                                                                                                                                                        $crate::fmt::rt::v1::Position::At(1usize),
                                                                                                                                                                                    format:
                                                                                                                                                                                        $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                                                                            ' ',
                                                                                                                                                                                                                        align:
                                                                                                                                                                                                                            $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                                                                        flags:
                                                                                                                                                                                                                            0u32,
                                                                                                                                                                                                                        precision:
                                                                                                                                                                                                                            $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                                                                        width:
                                                                                                                                                                                                                            $crate::fmt::rt::v1::Count::Implied,},}]))
                                                                                                       {
                                                                                                       (arg0,
                                                                                                        arg1,
                                                                                                        arg2)
                                                                                                       =>
                                                                                                       [$crate::fmt::ArgumentV1::new(arg0,
                                                                                                                                     $crate::fmt::Debug::fmt),
                                                                                                        $crate::fmt::ArgumentV1::new(arg1,
                                                                                                                                     $crate::fmt::Debug::fmt),
                                                                                                        $crate::fmt::ArgumentV1::new(arg2,
                                                                                                                                     $crate::fmt::Display::fmt)],
                                                                                                   },
                                                                                                  &[$crate::fmt::rt::v1::Argument{position:
                                                                                                                                      $crate::fmt::rt::v1::Position::At(0usize),
                                                                                                                                  format:
                                                                                                                                      $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                          ' ',
                                                                                                                                                                      align:
                                                                                                                                                                          $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                      flags:
                                                                                                                                                                          0u32,
                                                                                                                                                                      precision:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                      width:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,},},
                                                                                                    $crate::fmt::rt::v1::Argument{position:
                                                                                                                                      $crate::fmt::rt::v1::Position::At(1usize),
                                                                                                                                  format:
                                                                                                                                      $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                          ' ',
                                                                                                                                                                      align:
                                                                                                                                                                          $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                      flags:
                                                                                                                                                                          0u32,
                                                                                                                                                                      precision:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                      width:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,},},
                                                                                                    $crate::fmt::rt::v1::Argument{position:
                                                                                                                                      $crate::fmt::rt::v1::Position::At(2usize),
                                                                                                                                  format:
                                                                                                                                      $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                          ' ',
                                                                                                                                                                      align:
                                                                                                                                                                          $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                      flags:
                                                                                                                                                                          0u32,
                                                                                                                                                                      precision:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                      width:
                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,},}]),
                                                        &("dpdk-unix/src/Daemonize.rs",
                                                          313u32, 3u32))
                        }
                    }
                }
            }
        };
    }
    #[inline(always)]
    fn os_error() -> io::Error { io::Error::last_os_error() }
    #[inline(always)]
    fn pid_folder_path_default() -> PathBuf { PathBuf::from("/var/run") }
    #[inline(always)]
    fn working_folder_path_default() -> PathBuf { PathBuf::from("/") }
    #[inline(always)]
    fn user_name_default() -> CString { CString::new("root").unwrap() }
}
/// This object encapsulates a piece of behaviour to run on exit to ensure clean-up.
///
/// Currently it justs ensures that PID files are deleted.
pub struct DaemonizeCleanUpOnExit {
    pid_file_path: PathBuf,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::fmt::Debug for DaemonizeCleanUpOnExit {
    fn fmt(&self, f: &mut $crate::fmt::Formatter) -> $crate::fmt::Result {
        match *self {
            DaemonizeCleanUpOnExit { pid_file_path: ref __self_0_0 } => {
                let mut debug_trait_builder =
                    f.debug_struct("DaemonizeCleanUpOnExit");
                let _ =
                    debug_trait_builder.field("pid_file_path",
                                              &&(*__self_0_0));
                debug_trait_builder.finish()
            }
        }
    }
}
impl DaemonizeCleanUpOnExit {
    /// Cleans up.
    #[inline(always)]
    pub fn clean_up(self) {
        if let Err(_) = remove_file(&self.pid_file_path) {
            {
                $crate::io::_eprint($crate::fmt::Arguments::new_v1_formatted(&["Could not remove PID file \'",
                                                                               "\'\n"],
                                                                             &match (&self.pid_file_path,)
                                                                                  {
                                                                                  (arg0,)
                                                                                  =>
                                                                                  [$crate::fmt::ArgumentV1::new(arg0,
                                                                                                                $crate::fmt::Debug::fmt)],
                                                                              },
                                                                             &[$crate::fmt::rt::v1::Argument{position:
                                                                                                                 $crate::fmt::rt::v1::Position::At(0usize),
                                                                                                             format:
                                                                                                                 $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                     ' ',
                                                                                                                                                 align:
                                                                                                                                                     $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                 flags:
                                                                                                                                                     0u32,
                                                                                                                                                 precision:
                                                                                                                                                     $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                 width:
                                                                                                                                                     $crate::fmt::rt::v1::Count::Implied,},}]));
            }
        }
    }
}
/// Gets the program name using the best available technique for the Operating System.
#[cfg(any(target_os = "solaris",
          target_os = "macos",
          target_os = "ios",
          target_os = "freebsd",
          target_os = "dragonfly",
          target_os = "openbsd",
          target_os = "netbsd",
          target_os = "bitrig"))]
#[inline(always)]
pub fn get_program_name() -> String {
    unsafe {
        CStr::from_ptr(::libc::getprogname()).to_string_lossy().into_owned()
    }
}
/// Commonly supported huge page sizes for modern popular CPU architectures (x86, ARM, PowerPC).
///
/// See also <https://en.wikipedia.org/wiki/Page_(computer_memory)#Huge_pages>.
///
/// `repr(u64)` values are in KiloBytes.
#[repr(u64)]
#[structural_match]
#[rustc_copy_clone_marker]
pub enum HugePageSize {

    /// 1MB.
    _1MB = 1024,

    /// 2MB.
    _2MB = 2048,

    /// 4MB.
    _4MB = 4096,

    /// 16MB.
    _16MB = 16384,

    /// 256MB.
    _256MB = 262144,

    /// 512MB.
    ///
    /// aarch64 alternative.
    _512MB = 524288,

    /// 1GB.
    _1GB = 1048576,

    /// 2GB.
    _2GB = 2097152,

    /// 16GB.
    _16GB = 16777216,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::fmt::Debug for HugePageSize {
    fn fmt(&self, f: &mut $crate::fmt::Formatter) -> $crate::fmt::Result {
        match (&*self,) {
            (&HugePageSize::_1MB,) => {
                let mut debug_trait_builder = f.debug_tuple("_1MB");
                debug_trait_builder.finish()
            }
            (&HugePageSize::_2MB,) => {
                let mut debug_trait_builder = f.debug_tuple("_2MB");
                debug_trait_builder.finish()
            }
            (&HugePageSize::_4MB,) => {
                let mut debug_trait_builder = f.debug_tuple("_4MB");
                debug_trait_builder.finish()
            }
            (&HugePageSize::_16MB,) => {
                let mut debug_trait_builder = f.debug_tuple("_16MB");
                debug_trait_builder.finish()
            }
            (&HugePageSize::_256MB,) => {
                let mut debug_trait_builder = f.debug_tuple("_256MB");
                debug_trait_builder.finish()
            }
            (&HugePageSize::_512MB,) => {
                let mut debug_trait_builder = f.debug_tuple("_512MB");
                debug_trait_builder.finish()
            }
            (&HugePageSize::_1GB,) => {
                let mut debug_trait_builder = f.debug_tuple("_1GB");
                debug_trait_builder.finish()
            }
            (&HugePageSize::_2GB,) => {
                let mut debug_trait_builder = f.debug_tuple("_2GB");
                debug_trait_builder.finish()
            }
            (&HugePageSize::_16GB,) => {
                let mut debug_trait_builder = f.debug_tuple("_16GB");
                debug_trait_builder.finish()
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::marker::Copy for HugePageSize { }
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::clone::Clone for HugePageSize {
    #[inline]
    fn clone(&self) -> HugePageSize { { *self } }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::cmp::PartialEq for HugePageSize {
    #[inline]
    fn eq(&self, other: &HugePageSize) -> bool {
        {
            let __self_vi =
                unsafe { $crate::intrinsics::discriminant_value(&*self) } as
                    u64;
            let __arg_1_vi =
                unsafe { $crate::intrinsics::discriminant_value(&*other) } as
                    u64;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) { _ => true, }
            } else { false }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::cmp::Eq for HugePageSize {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () { { } }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::cmp::PartialOrd for HugePageSize {
    #[inline]
    fn partial_cmp(&self, other: &HugePageSize)
     -> $crate::option::Option<$crate::cmp::Ordering> {
        {
            let __self_vi =
                unsafe { $crate::intrinsics::discriminant_value(&*self) } as
                    u64;
            let __arg_1_vi =
                unsafe { $crate::intrinsics::discriminant_value(&*other) } as
                    u64;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    _ =>
                    $crate::option::Option::Some($crate::cmp::Ordering::Equal),
                }
            } else { __self_vi.partial_cmp(&__arg_1_vi) }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::cmp::Ord for HugePageSize {
    #[inline]
    fn cmp(&self, other: &HugePageSize) -> $crate::cmp::Ordering {
        {
            let __self_vi =
                unsafe { $crate::intrinsics::discriminant_value(&*self) } as
                    u64;
            let __arg_1_vi =
                unsafe { $crate::intrinsics::discriminant_value(&*other) } as
                    u64;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) { _ => $crate::cmp::Ordering::Equal, }
            } else { __self_vi.cmp(&__arg_1_vi) }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::hash::Hash for HugePageSize {
    fn hash<__H: $crate::hash::Hasher>(&self, state: &mut __H) -> () {
        match (&*self,) {
            _ => {
                $crate::hash::Hash::hash(&unsafe {
                                              $crate::intrinsics::discriminant_value(self)
                                          }, state)
            }
        }
    }
}
impl HugePageSize {
    /// Potentially supported huge page sizes.
    pub const
    PotentiallySupportedHugePageSizesLargestFirst:
    [HugePageSize; 9]
    =
    [HugePageSize::_16GB, HugePageSize::_2GB, HugePageSize::_1GB,
     HugePageSize::_512MB, HugePageSize::_256MB, HugePageSize::_16MB,
     HugePageSize::_4MB, HugePageSize::_2MB, HugePageSize::_1MB];
    /// Size in mega bytes.
    #[inline(always)]
    pub fn size_in_mega_bytes(self) -> u64 {
        self.size_in_kilo_bytes() / 1024
    }
    /// Size in kilo bytes.
    #[inline(always)]
    pub fn size_in_kilo_bytes(self) -> u64 { self as u64 }
    /// Size in bytes.
    #[inline(always)]
    pub fn size_in_bytes(self) -> u64 { self.size_in_kilo_bytes() * 1024 }
    /// Calculate number of huge pages.
    #[inline(always)]
    pub fn calculate_number_of_huge_pages(&self,
                                          desired_number_of_kilo_bytes: u64)
     -> u64 {
        let size_in_kilo_bytes = self.size_in_kilo_bytes();
        if size_in_kilo_bytes < desired_number_of_kilo_bytes {
            1
        } else { size_in_kilo_bytes / desired_number_of_kilo_bytes }
    }
    /// Converts a value from Linux's `/proc/mem` pseudo-file into a `HugePageSize`.
    #[inline(always)]
    pub fn from_proc_mem_info_value(value: u64) -> Option<Self> {
        use self::HugePageSize::*;
        match value {
            1024 => Some(_1MB),
            2048 => Some(_2MB),
            4096 => Some(_4MB),
            16384 => Some(_16MB),
            262144 => Some(_256MB),
            524288 => Some(_512MB),
            1048576 => Some(_1GB),
            2097152 => Some(_2GB),
            16777216 => Some(_16GB),
            _ => None,
        }
    }
    /// String description including unit.
    #[inline(always)]
    pub fn to_str(&self) -> &'static str {
        use self::HugePageSize::*;
        match *self {
            _1MB => "1MB",
            _2MB => "2MB",
            _4MB => "4MB",
            _16MB => "16MB",
            _256MB => "256MB",
            _512MB => "512MB",
            _1GB => "1GB",
            _2GB => "2GB",
            _16GB => "16GB",
        }
    }
    /// String description including unit.
    #[inline(always)]
    pub fn to_bytes(&self) -> &'static [u8] {
        use self::HugePageSize::*;
        match *self {
            _1MB => b"1MB",
            _2MB => b"2MB",
            _4MB => b"4MB",
            _16MB => b"16MB",
            _256MB => b"256MB",
            _512MB => b"512MB",
            _1GB => b"1GB",
            _2GB => b"2GB",
            _16GB => b"16GB",
        }
    }
    /// Supported huge page sizes, sorted smallest to largest.
    #[inline(always)]
    pub fn largest_supported_huge_page_size(sys_path: &SysPath) -> Self {
        *Self::supported_huge_page_sizes(sys_path).iter().rev().next().expect("Huge pages are not supported")
    }
    /// Supported huge page sizes, sorted smallest to largest.
    #[inline(always)]
    pub fn supported_huge_page_sizes(sys_path: &SysPath) -> BTreeSet<Self> {
        let mut supported = BTreeSet::new();
        for huge_page_size in
            Self::PotentiallySupportedHugePageSizesLargestFirst.iter() {
            if let Ok(_) =
                   huge_page_size.number_of_global_huge_pages(sys_path) {
                supported.insert(*huge_page_size);
            }
        }
        supported
    }
    /// Try to unreserve (clear reservations of) global huge pages.
    ///
    /// Will only work as root.
    #[inline(always)]
    pub fn unreserve_global_huge_pages(self, sys_path: &SysPath)
     -> io::Result<()> {
        assert_effective_user_id_is_root(&$crate::fmt::format($crate::fmt::Arguments::new_v1_formatted(&["Clear all global huge pages of size \'",
                                                                                                         "\'"],
                                                                                                       &match (&self,)
                                                                                                            {
                                                                                                            (arg0,)
                                                                                                            =>
                                                                                                            [$crate::fmt::ArgumentV1::new(arg0,
                                                                                                                                          $crate::fmt::Debug::fmt)],
                                                                                                        },
                                                                                                       &[$crate::fmt::rt::v1::Argument{position:
                                                                                                                                           $crate::fmt::rt::v1::Position::At(0usize),
                                                                                                                                       format:
                                                                                                                                           $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                               ' ',
                                                                                                                                                                           align:
                                                                                                                                                                               $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                           flags:
                                                                                                                                                                               0u32,
                                                                                                                                                                           precision:
                                                                                                                                                                               $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                           width:
                                                                                                                                                                               $crate::fmt::rt::v1::Count::Implied,},}])));
        self.reserve_global_huge_pages(sys_path, 0)
    }
    /// Try to reserve global huge pages.
    ///
    /// Will only work as root.
    #[inline(always)]
    pub fn reserve_global_huge_pages(self, sys_path: &SysPath,
                                     number_to_try_to_reserve: u64)
     -> io::Result<()> {
        assert_effective_user_id_is_root(&$crate::fmt::format($crate::fmt::Arguments::new_v1_formatted(&["Reserve \'",
                                                                                                         "\' global huge pages of size \'",
                                                                                                         "\'"],
                                                                                                       &match (&number_to_try_to_reserve,
                                                                                                               &self)
                                                                                                            {
                                                                                                            (arg0,
                                                                                                             arg1)
                                                                                                            =>
                                                                                                            [$crate::fmt::ArgumentV1::new(arg0,
                                                                                                                                          $crate::fmt::Display::fmt),
                                                                                                             $crate::fmt::ArgumentV1::new(arg1,
                                                                                                                                          $crate::fmt::Debug::fmt)],
                                                                                                        },
                                                                                                       &[$crate::fmt::rt::v1::Argument{position:
                                                                                                                                           $crate::fmt::rt::v1::Position::At(0usize),
                                                                                                                                       format:
                                                                                                                                           $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                               ' ',
                                                                                                                                                                           align:
                                                                                                                                                                               $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                           flags:
                                                                                                                                                                               0u32,
                                                                                                                                                                           precision:
                                                                                                                                                                               $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                           width:
                                                                                                                                                                               $crate::fmt::rt::v1::Count::Implied,},},
                                                                                                         $crate::fmt::rt::v1::Argument{position:
                                                                                                                                           $crate::fmt::rt::v1::Position::At(1usize),
                                                                                                                                       format:
                                                                                                                                           $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                               ' ',
                                                                                                                                                                           align:
                                                                                                                                                                               $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                           flags:
                                                                                                                                                                               0u32,
                                                                                                                                                                           precision:
                                                                                                                                                                               $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                           width:
                                                                                                                                                                               $crate::fmt::rt::v1::Count::Implied,},}])));
        sys_path.global_hugepages_file_path(self,
                                            "nr_hugepages").write_value(number_to_try_to_reserve)?;
        Ok(())
    }
    /// Read number of global huge pages of `self` size.
    #[inline(always)]
    pub fn number_of_global_huge_pages(self, sys_path: &SysPath)
     -> io::Result<u64> {
        sys_path.read_global_hugepages_value(self, "nr_hugepages")
    }
    /// Read number of free global huge pages of `self` size.
    #[inline(always)]
    pub fn number_of_free_global_huge_pages(self, sys_path: &SysPath)
     -> io::Result<u64> {
        sys_path.read_global_hugepages_value(self, "free_hugepages")
    }
    /// Read number of surplus global huge pages of `self` size.
    #[inline(always)]
    pub fn number_of_surplus_global_huge_pages(self, sys_path: &SysPath)
     -> io::Result<u64> {
        sys_path.read_global_hugepages_value(self, "surplus_hugepages")
    }
    /// Read number of reserved global huge pages of `self` size.
    #[inline(always)]
    pub fn number_of_reserved_global_huge_pages(self, sys_path: &SysPath)
     -> io::Result<u64> {
        sys_path.read_global_hugepages_value(self, "resv_hugepages")
    }
    /// Read number of memory policy global huge pages of `self` size.
    #[inline(always)]
    pub fn number_of_memory_policy_global_huge_pages(self, sys_path: &SysPath)
     -> io::Result<u64> {
        sys_path.read_global_hugepages_value(self, "nr_hugepages_mempolicy")
    }
    /// Read number of overcommit global huge pages of `self` size.
    #[inline(always)]
    pub fn number_of_overcommit_global_huge_pages(self, sys_path: &SysPath)
     -> io::Result<u64> {
        sys_path.read_global_hugepages_value(self, "nr_overcommit_hugepages")
    }
    /// Try to unreserve (clear reservations of) NUMA huge pages.
    ///
    /// Will only work as root.
    #[inline(always)]
    pub fn unreserve_numa_huge_pages(self, sys_path: &SysPath, numa_node: u8)
     -> io::Result<()> {
        assert_effective_user_id_is_root(&$crate::fmt::format($crate::fmt::Arguments::new_v1_formatted(&["Clear all NUMA huge pages of size \'",
                                                                                                         "\'"],
                                                                                                       &match (&self,)
                                                                                                            {
                                                                                                            (arg0,)
                                                                                                            =>
                                                                                                            [$crate::fmt::ArgumentV1::new(arg0,
                                                                                                                                          $crate::fmt::Debug::fmt)],
                                                                                                        },
                                                                                                       &[$crate::fmt::rt::v1::Argument{position:
                                                                                                                                           $crate::fmt::rt::v1::Position::At(0usize),
                                                                                                                                       format:
                                                                                                                                           $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                               ' ',
                                                                                                                                                                           align:
                                                                                                                                                                               $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                           flags:
                                                                                                                                                                               0u32,
                                                                                                                                                                           precision:
                                                                                                                                                                               $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                           width:
                                                                                                                                                                               $crate::fmt::rt::v1::Count::Implied,},}])));
        self.reserve_numa_huge_pages(sys_path, numa_node, 0)
    }
    /// Try to reserve NUMA huge pages.
    ///
    /// Will only work as root.
    #[inline(always)]
    pub fn reserve_numa_huge_pages(self, sys_path: &SysPath, numa_node: u8,
                                   number_to_try_to_reserve: u64)
     -> io::Result<()> {
        assert_effective_user_id_is_root(&$crate::fmt::format($crate::fmt::Arguments::new_v1_formatted(&["Reserve \'",
                                                                                                         "\' NUMA huge pages of size \'",
                                                                                                         "\'"],
                                                                                                       &match (&number_to_try_to_reserve,
                                                                                                               &self)
                                                                                                            {
                                                                                                            (arg0,
                                                                                                             arg1)
                                                                                                            =>
                                                                                                            [$crate::fmt::ArgumentV1::new(arg0,
                                                                                                                                          $crate::fmt::Display::fmt),
                                                                                                             $crate::fmt::ArgumentV1::new(arg1,
                                                                                                                                          $crate::fmt::Debug::fmt)],
                                                                                                        },
                                                                                                       &[$crate::fmt::rt::v1::Argument{position:
                                                                                                                                           $crate::fmt::rt::v1::Position::At(0usize),
                                                                                                                                       format:
                                                                                                                                           $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                               ' ',
                                                                                                                                                                           align:
                                                                                                                                                                               $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                           flags:
                                                                                                                                                                               0u32,
                                                                                                                                                                           precision:
                                                                                                                                                                               $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                           width:
                                                                                                                                                                               $crate::fmt::rt::v1::Count::Implied,},},
                                                                                                         $crate::fmt::rt::v1::Argument{position:
                                                                                                                                           $crate::fmt::rt::v1::Position::At(1usize),
                                                                                                                                       format:
                                                                                                                                           $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                               ' ',
                                                                                                                                                                           align:
                                                                                                                                                                               $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                           flags:
                                                                                                                                                                               0u32,
                                                                                                                                                                           precision:
                                                                                                                                                                               $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                           width:
                                                                                                                                                                               $crate::fmt::rt::v1::Count::Implied,},}])));
        sys_path.numa_hugepages_file_path(self, numa_node,
                                          "nr_hugepages").write_value(number_to_try_to_reserve)
    }
    /// Read number of NUMA huge pages of `self` size.
    ///
    /// This will fail if this is not a NUMA-based machine or the node is not present.
    #[inline(always)]
    pub fn number_of_numa_huge_pages(self, sys_path: &SysPath, numa_node: u8)
     -> io::Result<u64> {
        sys_path.read_numa_hugepages_value(self, numa_node, "nr_hugepages")
    }
    /// Read number of free NUMA node huge pages of `self` size.
    ///
    /// This will fail if this is not a NUMA-based machine or the node is not present.
    #[inline(always)]
    pub fn number_of_free_numa_huge_pages(self, sys_path: &SysPath,
                                          numa_node: u8) -> io::Result<u64> {
        sys_path.read_numa_hugepages_value(self, numa_node, "free_hugepages")
    }
    /// Read number of surplus NUMA huge pages of `self` size.
    ///
    /// This will fail if this is not a NUMA-based machine or the node is not present.
    #[inline(always)]
    pub fn number_of_surplus_numa_huge_pages(self, sys_path: &SysPath,
                                             numa_node: u8)
     -> io::Result<u64> {
        sys_path.read_numa_hugepages_value(self, numa_node,
                                           "surplus_hugepages")
    }
}
/// Represents a logical hyper thread, which in Operating System terms is usually a logical CPU (core).
///
/// These usually map 1:1 with `LogicalCore`s.
#[structural_match]
#[rustc_copy_clone_marker]
pub struct HyperThread(u16);
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::default::Default for HyperThread {
    #[inline]
    fn default() -> HyperThread {
        HyperThread($crate::default::Default::default())
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::fmt::Debug for HyperThread {
    fn fmt(&self, f: &mut $crate::fmt::Formatter) -> $crate::fmt::Result {
        match *self {
            HyperThread(ref __self_0_0) => {
                let mut debug_trait_builder = f.debug_tuple("HyperThread");
                let _ = debug_trait_builder.field(&&(*__self_0_0));
                debug_trait_builder.finish()
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::marker::Copy for HyperThread { }
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::clone::Clone for HyperThread {
    #[inline]
    fn clone(&self) -> HyperThread {
        { let _: $crate::clone::AssertParamIsClone<u16>; *self }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::cmp::Ord for HyperThread {
    #[inline]
    fn cmp(&self, other: &HyperThread) -> $crate::cmp::Ordering {
        match *other {
            HyperThread(ref __self_1_0) =>
            match *self {
                HyperThread(ref __self_0_0) =>
                match $crate::cmp::Ord::cmp(&(*__self_0_0), &(*__self_1_0)) {
                    $crate::cmp::Ordering::Equal =>
                    $crate::cmp::Ordering::Equal,
                    cmp => cmp,
                },
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::cmp::PartialOrd for HyperThread {
    #[inline]
    fn partial_cmp(&self, other: &HyperThread)
     -> $crate::option::Option<$crate::cmp::Ordering> {
        match *other {
            HyperThread(ref __self_1_0) =>
            match *self {
                HyperThread(ref __self_0_0) =>
                match $crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                           &(*__self_1_0)) {
                    $crate::option::Option::Some($crate::cmp::Ordering::Equal)
                    =>
                    $crate::option::Option::Some($crate::cmp::Ordering::Equal),
                    cmp => cmp,
                },
            },
        }
    }
    #[inline]
    fn lt(&self, other: &HyperThread) -> bool {
        match *other {
            HyperThread(ref __self_1_0) =>
            match *self {
                HyperThread(ref __self_0_0) =>
                $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                       &(*__self_1_0)),
                                                  $crate::cmp::Ordering::Greater)
                    == $crate::cmp::Ordering::Less,
            },
        }
    }
    #[inline]
    fn le(&self, other: &HyperThread) -> bool {
        match *other {
            HyperThread(ref __self_1_0) =>
            match *self {
                HyperThread(ref __self_0_0) =>
                $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                       &(*__self_1_0)),
                                                  $crate::cmp::Ordering::Greater)
                    != $crate::cmp::Ordering::Greater,
            },
        }
    }
    #[inline]
    fn gt(&self, other: &HyperThread) -> bool {
        match *other {
            HyperThread(ref __self_1_0) =>
            match *self {
                HyperThread(ref __self_0_0) =>
                $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                       &(*__self_1_0)),
                                                  $crate::cmp::Ordering::Less)
                    == $crate::cmp::Ordering::Greater,
            },
        }
    }
    #[inline]
    fn ge(&self, other: &HyperThread) -> bool {
        match *other {
            HyperThread(ref __self_1_0) =>
            match *self {
                HyperThread(ref __self_0_0) =>
                $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                       &(*__self_1_0)),
                                                  $crate::cmp::Ordering::Less)
                    != $crate::cmp::Ordering::Less,
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::cmp::Eq for HyperThread {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        { let _: $crate::cmp::AssertParamIsEq<u16>; }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::cmp::PartialEq for HyperThread {
    #[inline]
    fn eq(&self, other: &HyperThread) -> bool {
        match *other {
            HyperThread(ref __self_1_0) =>
            match *self {
                HyperThread(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &HyperThread) -> bool {
        match *other {
            HyperThread(ref __self_1_0) =>
            match *self {
                HyperThread(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::hash::Hash for HyperThread {
    fn hash<__H: $crate::hash::Hasher>(&self, state: &mut __H) -> () {
        match *self {
            HyperThread(ref __self_0_0) => {
                $crate::hash::Hash::hash(&(*__self_0_0), state)
            }
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_HyperThread: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[allow(unused_macros)]
        macro_rules! try(( $ __expr : expr ) => {
                         match $ __expr {
                         _serde :: export :: Ok ( __val ) => __val , _serde ::
                         export :: Err ( __err ) => {
                         return _serde :: export :: Err ( __err ) ; } } });
        #[automatically_derived]
        impl <'de> _serde::Deserialize<'de> for HyperThread {
            fn deserialize<__D>(__deserializer: __D)
             -> _serde::export::Result<Self, __D::Error> where
             __D: _serde::Deserializer<'de> {
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<HyperThread>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl <'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type
                    Value
                    =
                    HyperThread;
                    fn expecting(&self,
                                 __formatter: &mut _serde::export::Formatter)
                     -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter,
                                                             "tuple struct HyperThread")
                    }
                    #[inline]
                    fn visit_newtype_struct<__E>(self, __e: __E)
                     -> _serde::export::Result<Self::Value, __E::Error> where
                     __E: _serde::Deserializer<'de> {
                        let __field0: u16 =
                            match <u16 as
                                      _serde::Deserialize>::deserialize(__e) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                        _serde::export::Ok(HyperThread(__field0))
                    }
                    #[inline]
                    fn visit_seq<__A>(self, mut __seq: __A)
                     -> _serde::export::Result<Self::Value, __A::Error> where
                     __A: _serde::de::SeqAccess<'de> {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<u16>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(0usize,
                                                                                                 &"tuple struct HyperThread with 1 element"));
                                }
                            };
                        _serde::export::Ok(HyperThread(__field0))
                    }
                }
                _serde::Deserializer::deserialize_newtype_struct(__deserializer,
                                                                 "HyperThread",
                                                                 __Visitor{marker:
                                                                               _serde::export::PhantomData::<HyperThread>,
                                                                           lifetime:
                                                                               _serde::export::PhantomData,})
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_HyperThread: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[allow(unused_macros)]
        macro_rules! try(( $ __expr : expr ) => {
                         match $ __expr {
                         _serde :: export :: Ok ( __val ) => __val , _serde ::
                         export :: Err ( __err ) => {
                         return _serde :: export :: Err ( __err ) ; } } });
        #[automatically_derived]
        impl _serde::Serialize for HyperThread {
            fn serialize<__S>(&self, __serializer: __S)
             -> _serde::export::Result<__S::Ok, __S::Error> where
             __S: _serde::Serializer {
                _serde::Serializer::serialize_newtype_struct(__serializer,
                                                             "HyperThread",
                                                             &self.0)
            }
        }
    };
impl From<u16> for HyperThread {
    #[inline(always)]
    fn from(value: u16) -> Self { HyperThread(value) }
}
impl Into<u16> for HyperThread {
    #[inline(always)]
    fn into(self) -> u16 { self.0 }
}
impl HyperThread {
    #[inline(always)]
    pub(crate) fn hyper_threads_to_mask(hyper_threads: &BTreeSet<Self>)
     -> String {
        let mut mask: u32 = 0;
        for hyper_thread in hyper_threads.iter() {
            let bit = (1 << hyper_thread.0) as u32;
            mask |= bit;
        }
        $crate::fmt::format($crate::fmt::Arguments::new_v1_formatted(&[""],
                                                                     &match (&mask,)
                                                                          {
                                                                          (arg0,)
                                                                          =>
                                                                          [$crate::fmt::ArgumentV1::new(arg0,
                                                                                                        $crate::fmt::LowerHex::fmt)],
                                                                      },
                                                                     &[$crate::fmt::rt::v1::Argument{position:
                                                                                                         $crate::fmt::rt::v1::Position::At(0usize),
                                                                                                     format:
                                                                                                         $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                             ' ',
                                                                                                                                         align:
                                                                                                                                             $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                         flags:
                                                                                                                                             8u32,
                                                                                                                                         precision:
                                                                                                                                             $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                         width:
                                                                                                                                             $crate::fmt::rt::v1::Count::Is(8usize),},}]))
    }
    /// Sets workqueue hyper thread affinity.
    #[inline(always)]
    pub fn set_work_queue_hyper_thread_affinity(hyper_threads:
                                                    &BTreeSet<HyperThread>,
                                                sys_path: &SysPath)
     -> io::Result<()> {
        let mask = Self::hyper_threads_to_mask(hyper_threads);
        sys_path.workqueue_file_path("cpumask").write_value(&mask)?;
        sys_path.workqueue_file_path("writeback/cpumask").write_value(&mask)
    }
    /// Last hyper thread.
    #[inline(always)]
    pub fn last(hyper_threads: &BTreeSet<HyperThread>) -> Option<&Self> {
        hyper_threads.iter().last()
    }
    /// The complement of `hyper_threads`.
    #[inline(always)]
    pub fn complement(hyper_threads: &BTreeSet<Self>, sys_path: &SysPath)
     -> BTreeSet<Self> {
        let present = Self::present(sys_path);
        present.difference(hyper_threads).cloned().collect()
    }
    /// Remove as offline `hyper_threads`.
    #[inline(always)]
    pub fn remove_those_offline(hyper_threads: &BTreeSet<Self>,
                                sys_path: &SysPath) -> BTreeSet<Self> {
        let online = Self::online(sys_path);
        online.intersection(hyper_threads).cloned().collect()
    }
    /// CPUs (hyper threaded logical cores) that are present and that could become online.
    ///
    /// Consider using libnuma instead of this call.
    ///
    /// See <https://www.kernel.org/doc/Documentation/cputopology.txt>.
    #[inline(always)]
    pub fn present(sys_path: &SysPath) -> BTreeSet<Self> {
        Self::parse_list_mask(sys_path, "present")
    }
    /// Hyper threaded logical cores that are online at some point.
    ///
    /// Consider using libnuma instead of this call.
    ///
    /// See <https://www.kernel.org/doc/Documentation/cputopology.txt>.
    #[inline(always)]
    pub fn online(sys_path: &SysPath) -> BTreeSet<Self> {
        Self::parse_list_mask(sys_path, "online")
    }
    /// Hyper threaded logical cores that are offline.
    ///
    /// The maximum CPU index in this list ***can exceed the kernel's maximum in `self.kernel_maximum_index`***.
    ///
    /// Close to useless.
    ///
    /// See <https://www.kernel.org/doc/Documentation/cputopology.txt>.
    #[inline(always)]
    pub fn offline(sys_path: &SysPath) -> BTreeSet<Self> {
        Self::parse_list_mask(sys_path, "offline")
    }
    /// Hyper threaded logical cores that could possibly be online at some point.
    ///
    /// Close to very useless.
    ///
    /// See <https://www.kernel.org/doc/Documentation/cputopology.txt>.
    #[inline(always)]
    pub fn possible(sys_path: &SysPath) -> BTreeSet<Self> {
        Self::parse_list_mask(sys_path, "possible")
    }
    /// Is this hyper thread online?
    ///
    /// See <https://www.kernel.org/doc/Documentation/core-api/cpu_hotplug.rst>.
    #[inline(always)]
    pub fn is_online(self, sys_path: &SysPath) -> bool {
        match &self.online_file_path(sys_path).read_raw_without_line_feed().unwrap()[..]
            {
            b"0" => false,
            b"1" => true,
            invalid@_ => {
                $crate::rt::begin_panic_fmt(&$crate::fmt::Arguments::new_v1_formatted(&["Invalid value for CPU online \'",
                                                                                        "\'"],
                                                                                      &match (&invalid,)
                                                                                           {
                                                                                           (arg0,)
                                                                                           =>
                                                                                           [$crate::fmt::ArgumentV1::new(arg0,
                                                                                                                         $crate::fmt::Debug::fmt)],
                                                                                       },
                                                                                      &[$crate::fmt::rt::v1::Argument{position:
                                                                                                                          $crate::fmt::rt::v1::Position::At(0usize),
                                                                                                                      format:
                                                                                                                          $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                              ' ',
                                                                                                                                                          align:
                                                                                                                                                              $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                          flags:
                                                                                                                                                              0u32,
                                                                                                                                                          precision:
                                                                                                                                                              $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                          width:
                                                                                                                                                              $crate::fmt::rt::v1::Count::Implied,},}]),
                                            &("dpdk-unix/src/HyperThread.rs",
                                              160u32, 19u32))
            }
        }
    }
    /// Is this hyper thread offline?
    ///
    /// See <https://www.kernel.org/doc/Documentation/core-api/cpu_hotplug.rst>.
    #[inline(always)]
    pub fn is_offline(self, sys_path: &SysPath) -> bool {
        !self.is_online(sys_path)
    }
    /// Disable (offline) this hyper thread.
    ///
    /// Requires root.
    ///
    /// Hyper thread (CPU) zero (0) is special on x86 / x86-64 and can not ordinarily be offlined.
    ///
    /// See <https://www.kernel.org/doc/Documentation/core-api/cpu_hotplug.rst>.
    #[inline(always)]
    pub fn set_offline(self, sys_path: &SysPath) -> io::Result<()> {
        assert_effective_user_id_is_root(&$crate::fmt::format($crate::fmt::Arguments::new_v1_formatted(&["Offline CPU \'",
                                                                                                         "\'"],
                                                                                                       &match (&self.0,)
                                                                                                            {
                                                                                                            (arg0,)
                                                                                                            =>
                                                                                                            [$crate::fmt::ArgumentV1::new(arg0,
                                                                                                                                          $crate::fmt::Display::fmt)],
                                                                                                        },
                                                                                                       &[$crate::fmt::rt::v1::Argument{position:
                                                                                                                                           $crate::fmt::rt::v1::Position::At(0usize),
                                                                                                                                       format:
                                                                                                                                           $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                               ' ',
                                                                                                                                                                           align:
                                                                                                                                                                               $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                           flags:
                                                                                                                                                                               0u32,
                                                                                                                                                                           precision:
                                                                                                                                                                               $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                           width:
                                                                                                                                                                               $crate::fmt::rt::v1::Count::Implied,},}])));
        self.online_file_path(sys_path).write_value(0)
    }
    /// Enable (online) this hyper thread.
    ///
    /// Requires root.
    ///
    /// See <https://www.kernel.org/doc/Documentation/core-api/cpu_hotplug.rst>.
    #[inline(always)]
    pub fn set_online(self, sys_path: &SysPath) -> io::Result<()> {
        assert_effective_user_id_is_root(&$crate::fmt::format($crate::fmt::Arguments::new_v1_formatted(&["Online CPU \'",
                                                                                                         "\'"],
                                                                                                       &match (&self.0,)
                                                                                                            {
                                                                                                            (arg0,)
                                                                                                            =>
                                                                                                            [$crate::fmt::ArgumentV1::new(arg0,
                                                                                                                                          $crate::fmt::Display::fmt)],
                                                                                                        },
                                                                                                       &[$crate::fmt::rt::v1::Argument{position:
                                                                                                                                           $crate::fmt::rt::v1::Position::At(0usize),
                                                                                                                                       format:
                                                                                                                                           $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                               ' ',
                                                                                                                                                                           align:
                                                                                                                                                                               $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                           flags:
                                                                                                                                                                               0u32,
                                                                                                                                                                           precision:
                                                                                                                                                                               $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                           width:
                                                                                                                                                                               $crate::fmt::rt::v1::Count::Implied,},}])));
        self.online_file_path(sys_path).write_value(1)
    }
    #[inline(always)]
    fn online_file_path(self, sys_path: &SysPath) -> PathBuf {
        sys_path.hyper_thread_path(self, "online")
    }
    /// Hyper threaded logical cores that are siblings of this one.
    ///
    /// Will include `self`.
    ///
    /// See <https://www.kernel.org/doc/Documentation/cputopology.txt>.
    #[inline(always)]
    pub fn siblings(self, sys_path: &SysPath) -> BTreeSet<Self> {
        sys_path.hyper_thread_path(self,
                                   "topology/core_siblings_list").read_linux_core_or_numa_list(HyperThread::from).unwrap()
    }
    /// Hyper threaded logical cores that are hyper-thread-siblings of this one.
    ///
    /// Will include `self`.
    ///
    /// Usually wrong on virtual machines (eg Parallels Desktop).
    ///
    /// See <https://www.kernel.org/doc/Documentation/cputopology.txt>.
    #[inline(always)]
    pub fn thread_siblings(self, sys_path: &SysPath) -> BTreeSet<Self> {
        sys_path.hyper_thread_path(self,
                                   "topology/thread_siblings_list").read_linux_core_or_numa_list(HyperThread::from).unwrap()
    }
    /// Hyper threaded logical cores grouped as hyper thread groups (eg HT 0 and 1, 2 and 3, etc).
    #[inline(always)]
    pub fn hyper_thread_groups(hyper_threads: &BTreeSet<Self>,
                               sys_path: &SysPath)
     -> BTreeSet<BTreeSet<Self>> {
        let mut hyper_thread_groups = BTreeSet::new();
        for hyper_thread in hyper_threads.iter() {
            let hyper_thread_group =
                (*hyper_thread).level1_cache_hyper_thread_siblings_including_self(sys_path);
            hyper_thread_groups.insert(hyper_thread_group);
        }
        hyper_thread_groups
    }
    /// Tries to find this hyper thread's NUMA node, if this is a NUMA machine.
    #[inline(always)]
    pub fn numa_node(self, sys_path: &SysPath) -> Option<u8> {
        match sys_path.hyper_thread_path(self, "node").canonicalize() {
            Err(_) => None,
            Ok(canonical) =>
            match canonical.file_name() {
                None => None,
                Some(file_name) =>
                match file_name.to_str() {
                    None => None,
                    Some(file_name) =>
                    if file_name.starts_with("node") {
                        u8::from_str(&file_name[("node".len())..]).ok()
                    } else { None },
                },
            },
        }
    }
    /// Hyper threaded logical cores that are thread-siblings of this one according to the level 1 cache.
    ///
    /// Will include `self`.
    ///
    /// Usually reliable.
    #[inline(always)]
    pub fn level1_cache_hyper_thread_siblings_including_self(self,
                                                             sys_path:
                                                                 &SysPath)
     -> BTreeSet<Self> {
        sys_path.hyper_thread_path(self,
                                   "cache/index0/shared_cpu_list").read_linux_core_or_numa_list(HyperThread::from).unwrap()
    }
    /// Hyper threaded logical cores that are thread-siblings of this one according to the level 1 cache.
    ///
    /// Will exclude `self`.
    ///
    /// Usually reliable.
    #[inline(always)]
    pub fn level1_cache_hyper_thread_siblings_excluding_self(self,
                                                             sys_path:
                                                                 &SysPath)
     -> BTreeSet<Self> {
        let mut hyper_threads =
            self.level1_cache_hyper_thread_siblings_including_self(sys_path);
        hyper_threads.remove(&self);
        hyper_threads
    }
    /// Underlying hardware, not Linux, core identifier.
    ///
    /// See <https://www.kernel.org/doc/Documentation/cputopology.txt>.
    #[inline(always)]
    pub fn underlying_hardware_physical_core_identifier(self,
                                                        sys_path: &SysPath)
     -> io::Result<u16> {
        sys_path.hyper_thread_path(self, "topology/core_id").read_value()
    }
    /// Underlying hardware, not Linux, socket identifier.
    ///
    /// See <https://www.kernel.org/doc/Documentation/cputopology.txt>.
    #[inline(always)]
    pub fn underlying_hardware_physical_socket_identifier(self,
                                                          sys_path: &SysPath)
     -> io::Result<u16> {
        sys_path.hyper_thread_path(self,
                                   "topology/physical_package_id").read_value()
    }
    /// Simply reports the maximum *identifier* that could be used by the Linux kernel upto the `CONFIG_` number of CPUs.
    ///
    /// Add one to this to get the exclusive maximum.
    ///
    /// Consider using libnuma instead of this call.
    #[inline(always)]
    pub fn kernel_maximum_index(sys_path: &SysPath) -> io::Result<Self> {
        sys_path.hyper_threads_path("kernel_max").read_value().map(|value|
                                                                       HyperThread(value))
    }
    #[inline(always)]
    fn parse_list_mask(sys_path: &SysPath, file_name: &str)
     -> BTreeSet<Self> {
        sys_path.hyper_threads_path(file_name).read_linux_core_or_numa_list(HyperThread::from).unwrap()
    }
}
/// A hyper thread bitmask.
pub type HyperThreadBitmask = u32;
/// Represents a logical hyper thread, which in Operating System terms is usually a logical CPU (core).
///
/// These usually map 1:1 with `LogicalCore`s
#[structural_match]
#[rustc_copy_clone_marker]
pub struct InterruptRequest(u16);
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::default::Default for InterruptRequest {
    #[inline]
    fn default() -> InterruptRequest {
        InterruptRequest($crate::default::Default::default())
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::fmt::Debug for InterruptRequest {
    fn fmt(&self, f: &mut $crate::fmt::Formatter) -> $crate::fmt::Result {
        match *self {
            InterruptRequest(ref __self_0_0) => {
                let mut debug_trait_builder =
                    f.debug_tuple("InterruptRequest");
                let _ = debug_trait_builder.field(&&(*__self_0_0));
                debug_trait_builder.finish()
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::marker::Copy for InterruptRequest { }
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::clone::Clone for InterruptRequest {
    #[inline]
    fn clone(&self) -> InterruptRequest {
        { let _: $crate::clone::AssertParamIsClone<u16>; *self }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::cmp::Ord for InterruptRequest {
    #[inline]
    fn cmp(&self, other: &InterruptRequest) -> $crate::cmp::Ordering {
        match *other {
            InterruptRequest(ref __self_1_0) =>
            match *self {
                InterruptRequest(ref __self_0_0) =>
                match $crate::cmp::Ord::cmp(&(*__self_0_0), &(*__self_1_0)) {
                    $crate::cmp::Ordering::Equal =>
                    $crate::cmp::Ordering::Equal,
                    cmp => cmp,
                },
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::cmp::PartialOrd for InterruptRequest {
    #[inline]
    fn partial_cmp(&self, other: &InterruptRequest)
     -> $crate::option::Option<$crate::cmp::Ordering> {
        match *other {
            InterruptRequest(ref __self_1_0) =>
            match *self {
                InterruptRequest(ref __self_0_0) =>
                match $crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                           &(*__self_1_0)) {
                    $crate::option::Option::Some($crate::cmp::Ordering::Equal)
                    =>
                    $crate::option::Option::Some($crate::cmp::Ordering::Equal),
                    cmp => cmp,
                },
            },
        }
    }
    #[inline]
    fn lt(&self, other: &InterruptRequest) -> bool {
        match *other {
            InterruptRequest(ref __self_1_0) =>
            match *self {
                InterruptRequest(ref __self_0_0) =>
                $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                       &(*__self_1_0)),
                                                  $crate::cmp::Ordering::Greater)
                    == $crate::cmp::Ordering::Less,
            },
        }
    }
    #[inline]
    fn le(&self, other: &InterruptRequest) -> bool {
        match *other {
            InterruptRequest(ref __self_1_0) =>
            match *self {
                InterruptRequest(ref __self_0_0) =>
                $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                       &(*__self_1_0)),
                                                  $crate::cmp::Ordering::Greater)
                    != $crate::cmp::Ordering::Greater,
            },
        }
    }
    #[inline]
    fn gt(&self, other: &InterruptRequest) -> bool {
        match *other {
            InterruptRequest(ref __self_1_0) =>
            match *self {
                InterruptRequest(ref __self_0_0) =>
                $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                       &(*__self_1_0)),
                                                  $crate::cmp::Ordering::Less)
                    == $crate::cmp::Ordering::Greater,
            },
        }
    }
    #[inline]
    fn ge(&self, other: &InterruptRequest) -> bool {
        match *other {
            InterruptRequest(ref __self_1_0) =>
            match *self {
                InterruptRequest(ref __self_0_0) =>
                $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                       &(*__self_1_0)),
                                                  $crate::cmp::Ordering::Less)
                    != $crate::cmp::Ordering::Less,
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::cmp::Eq for InterruptRequest {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        { let _: $crate::cmp::AssertParamIsEq<u16>; }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::cmp::PartialEq for InterruptRequest {
    #[inline]
    fn eq(&self, other: &InterruptRequest) -> bool {
        match *other {
            InterruptRequest(ref __self_1_0) =>
            match *self {
                InterruptRequest(ref __self_0_0) =>
                (*__self_0_0) == (*__self_1_0),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &InterruptRequest) -> bool {
        match *other {
            InterruptRequest(ref __self_1_0) =>
            match *self {
                InterruptRequest(ref __self_0_0) =>
                (*__self_0_0) != (*__self_1_0),
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::hash::Hash for InterruptRequest {
    fn hash<__H: $crate::hash::Hasher>(&self, state: &mut __H) -> () {
        match *self {
            InterruptRequest(ref __self_0_0) => {
                $crate::hash::Hash::hash(&(*__self_0_0), state)
            }
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_InterruptRequest: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[allow(unused_macros)]
        macro_rules! try(( $ __expr : expr ) => {
                         match $ __expr {
                         _serde :: export :: Ok ( __val ) => __val , _serde ::
                         export :: Err ( __err ) => {
                         return _serde :: export :: Err ( __err ) ; } } });
        #[automatically_derived]
        impl <'de> _serde::Deserialize<'de> for InterruptRequest {
            fn deserialize<__D>(__deserializer: __D)
             -> _serde::export::Result<Self, __D::Error> where
             __D: _serde::Deserializer<'de> {
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<InterruptRequest>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl <'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type
                    Value
                    =
                    InterruptRequest;
                    fn expecting(&self,
                                 __formatter: &mut _serde::export::Formatter)
                     -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter,
                                                             "tuple struct InterruptRequest")
                    }
                    #[inline]
                    fn visit_newtype_struct<__E>(self, __e: __E)
                     -> _serde::export::Result<Self::Value, __E::Error> where
                     __E: _serde::Deserializer<'de> {
                        let __field0: u16 =
                            match <u16 as
                                      _serde::Deserialize>::deserialize(__e) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                        _serde::export::Ok(InterruptRequest(__field0))
                    }
                    #[inline]
                    fn visit_seq<__A>(self, mut __seq: __A)
                     -> _serde::export::Result<Self::Value, __A::Error> where
                     __A: _serde::de::SeqAccess<'de> {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<u16>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(0usize,
                                                                                                 &"tuple struct InterruptRequest with 1 element"));
                                }
                            };
                        _serde::export::Ok(InterruptRequest(__field0))
                    }
                }
                _serde::Deserializer::deserialize_newtype_struct(__deserializer,
                                                                 "InterruptRequest",
                                                                 __Visitor{marker:
                                                                               _serde::export::PhantomData::<InterruptRequest>,
                                                                           lifetime:
                                                                               _serde::export::PhantomData,})
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_InterruptRequest: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[allow(unused_macros)]
        macro_rules! try(( $ __expr : expr ) => {
                         match $ __expr {
                         _serde :: export :: Ok ( __val ) => __val , _serde ::
                         export :: Err ( __err ) => {
                         return _serde :: export :: Err ( __err ) ; } } });
        #[automatically_derived]
        impl _serde::Serialize for InterruptRequest {
            fn serialize<__S>(&self, __serializer: __S)
             -> _serde::export::Result<__S::Ok, __S::Error> where
             __S: _serde::Serializer {
                _serde::Serializer::serialize_newtype_struct(__serializer,
                                                             "InterruptRequest",
                                                             &self.0)
            }
        }
    };
impl From<u16> for InterruptRequest {
    #[inline(always)]
    fn from(value: u16) -> Self { InterruptRequest(value) }
}
impl Into<u16> for InterruptRequest {
    #[inline(always)]
    fn into(self) -> u16 { self.0 }
}
impl InterruptRequest { }
/// List parse error.
pub enum ListParseError {

    /// An IO error.
    IoError(io::Error),

    /// Contains an empty index or range.
    ContainsAnEmptyIndexOrRange,

    /// Could not parse index (not a string).
    CouldNotParseIndexAsNotAString {
        /// Description.
        description: &'static str,
        /// Unparsable index.
        unparsable_index: Box<[u8]>,
        /// Cause.
        cause: Utf8Error,
    },

    /// Could not parse index.
    CouldNotParseIndex {
        /// Description.
        description: &'static str,
        /// Unparsable index.
        unparsable_index: String,
        /// Cause.
        cause: ParseIntError,
    },

    /// Contains mis-sorted indices.
    ContainsMisSortedIndices {
        /// First part of index.
        first: u16,
        /// Minimum expected for next index.
        next_minimum_index_expected: u16,
    },

    /// Range is not an ascending range with more than one element.
    RangeIsNotAnAscendingRangeWithMoreThanOneElement {
        /// First part of index.
        first: u16,
        /// Second part of index.
        second: u16,
    },
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::fmt::Debug for ListParseError {
    fn fmt(&self, f: &mut $crate::fmt::Formatter) -> $crate::fmt::Result {
        match (&*self,) {
            (&ListParseError::IoError(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("IoError");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&ListParseError::ContainsAnEmptyIndexOrRange,) => {
                let mut debug_trait_builder =
                    f.debug_tuple("ContainsAnEmptyIndexOrRange");
                debug_trait_builder.finish()
            }
            (&ListParseError::CouldNotParseIndexAsNotAString {
             description: ref __self_0,
             unparsable_index: ref __self_1,
             cause: ref __self_2 },) => {
                let mut debug_trait_builder =
                    f.debug_struct("CouldNotParseIndexAsNotAString");
                let _ =
                    debug_trait_builder.field("description", &&(*__self_0));
                let _ =
                    debug_trait_builder.field("unparsable_index",
                                              &&(*__self_1));
                let _ = debug_trait_builder.field("cause", &&(*__self_2));
                debug_trait_builder.finish()
            }
            (&ListParseError::CouldNotParseIndex {
             description: ref __self_0,
             unparsable_index: ref __self_1,
             cause: ref __self_2 },) => {
                let mut debug_trait_builder =
                    f.debug_struct("CouldNotParseIndex");
                let _ =
                    debug_trait_builder.field("description", &&(*__self_0));
                let _ =
                    debug_trait_builder.field("unparsable_index",
                                              &&(*__self_1));
                let _ = debug_trait_builder.field("cause", &&(*__self_2));
                debug_trait_builder.finish()
            }
            (&ListParseError::ContainsMisSortedIndices {
             first: ref __self_0, next_minimum_index_expected: ref __self_1
             },) => {
                let mut debug_trait_builder =
                    f.debug_struct("ContainsMisSortedIndices");
                let _ = debug_trait_builder.field("first", &&(*__self_0));
                let _ =
                    debug_trait_builder.field("next_minimum_index_expected",
                                              &&(*__self_1));
                debug_trait_builder.finish()
            }
            (&ListParseError::RangeIsNotAnAscendingRangeWithMoreThanOneElement {
             first: ref __self_0, second: ref __self_1 },) => {
                let mut debug_trait_builder =
                    f.debug_struct("RangeIsNotAnAscendingRangeWithMoreThanOneElement");
                let _ = debug_trait_builder.field("first", &&(*__self_0));
                let _ = debug_trait_builder.field("second", &&(*__self_1));
                debug_trait_builder.finish()
            }
        }
    }
}
impl Display for ListParseError {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        <ListParseError as Debug>::fmt(self, f)
    }
}
impl error::Error for ListParseError {
    #[inline(always)]
    fn source(&self) -> Option<&(error::Error + 'static)> {
        use self::ListParseError::*;
        match self {
            &IoError(ref error) => Some(error),
            &ContainsAnEmptyIndexOrRange => None,
            &CouldNotParseIndexAsNotAString { ref cause, .. } => Some(cause),
            &CouldNotParseIndex { ref cause, .. } => Some(cause),
            &ContainsMisSortedIndices { .. } => None,
            &RangeIsNotAnAscendingRangeWithMoreThanOneElement { .. } => None,
        }
    }
}
impl From<io::Error> for ListParseError {
    #[inline(always)]
    fn from(error: io::Error) -> Self { ListParseError::IoError(error) }
}
impl ListParseError {
    /// Parses a Linux list string used for cpu sets, core masks and NUMA nodes such as "2,4-31,32-63" and "1,2,10-20,100-2000:2/25" (see <https://www.kernel.org/doc/html/latest/admin-guide/kernel-parameters.html> for an awful description of this mad syntax).
    ///
    /// Returns a BTreeSet with the zero-based indices found in the string. For example, "2,4-31,32-63" would return a set with all values between 0 to 63 except 0, 1 and 3.
    pub fn parse_linux_list_string<Mapper: Fn(u16) -> R,
                                   R: Ord>(linux_list_string: &[u8],
                                           mapper: Mapper)
     -> Result<BTreeSet<R>, ListParseError> {
        #[inline(always)]
        fn parse_index(index_string: &[u8], description: &'static str)
         -> Result<u16, ListParseError> {
            use self::ListParseError::*;
            let index_string =
                match from_utf8(index_string) {
                    Ok(index_string) => index_string,
                    Err(cause) =>
                    return Err(CouldNotParseIndexAsNotAString{description,
                                                              unparsable_index:
                                                                  index_string.to_vec().into_boxed_slice(),
                                                              cause,}),
                };
            match index_string.parse() {
                Ok(index) => Ok(index),
                Err(cause) =>
                Err(CouldNotParseIndex{description,
                                       unparsable_index:
                                           index_string.to_owned(),
                                       cause,}),
            }
        }
        let mut result = BTreeSet::new();
        use self::ListParseError::*;
        let mut next_minimum_index_expected = 0;
        for index_or_range in split(linux_list_string, b',') {
            if index_or_range.is_empty() {
                return Err(ContainsAnEmptyIndexOrRange);
            }
            let mut range_iterator = splitn(index_or_range, 2, b'-');
            let first =
                {
                    let index =
                        parse_index(range_iterator.next().unwrap(), "first")?;
                    if index < next_minimum_index_expected {
                        return Err(ContainsMisSortedIndices{first: index,
                                                            next_minimum_index_expected,});
                    }
                    index
                };
            if let Some(second) = range_iterator.last() {
                let mut range_or_range_with_groups = splitn(second, 2, b':');
                let second =
                    {
                        let index =
                            parse_index(range_or_range_with_groups.next().unwrap(),
                                        "second")?;
                        if first >= index {
                            return Err(RangeIsNotAnAscendingRangeWithMoreThanOneElement{first,
                                                                                        second:
                                                                                            index,});
                        }
                        index
                    };
                match range_or_range_with_groups.last() {
                    None => {
                        for index in first..(second + 1) {
                            result.insert(mapper(index));
                        }
                        next_minimum_index_expected = second;
                    }
                    Some(weird_but_rare_group_syntax) => {
                        let mut weird_but_rare_group_syntax =
                            splitn(weird_but_rare_group_syntax, 2, b'/');
                        let used_size =
                            parse_index(weird_but_rare_group_syntax.next().unwrap(),
                                        "used_size")?;
                        let group_size =
                            parse_index(weird_but_rare_group_syntax.last().expect("a group does not have group_size"),
                                        "group_size")?;
                        {
                            match (&(used_size), &(0)) {
                                (left_val, right_val) => {
                                    if *left_val == *right_val {
                                        {
                                            $crate::rt::begin_panic_fmt(&$crate::fmt::Arguments::new_v1_formatted(&["assertion failed: `(left != right)`\n  left: `",
                                                                                                                    "`,\n right: `",
                                                                                                                    "`: "],
                                                                                                                  &match (&left_val,
                                                                                                                          &right_val,
                                                                                                                          &$crate::fmt::Arguments::new_v1(&["used_size is zero"],
                                                                                                                                                          &match ()
                                                                                                                                                               {
                                                                                                                                                               ()
                                                                                                                                                               =>
                                                                                                                                                               [],
                                                                                                                                                           }))
                                                                                                                       {
                                                                                                                       (arg0,
                                                                                                                        arg1,
                                                                                                                        arg2)
                                                                                                                       =>
                                                                                                                       [$crate::fmt::ArgumentV1::new(arg0,
                                                                                                                                                     $crate::fmt::Debug::fmt),
                                                                                                                        $crate::fmt::ArgumentV1::new(arg1,
                                                                                                                                                     $crate::fmt::Debug::fmt),
                                                                                                                        $crate::fmt::ArgumentV1::new(arg2,
                                                                                                                                                     $crate::fmt::Display::fmt)],
                                                                                                                   },
                                                                                                                  &[$crate::fmt::rt::v1::Argument{position:
                                                                                                                                                      $crate::fmt::rt::v1::Position::At(0usize),
                                                                                                                                                  format:
                                                                                                                                                      $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                                          ' ',
                                                                                                                                                                                      align:
                                                                                                                                                                                          $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                                      flags:
                                                                                                                                                                                          0u32,
                                                                                                                                                                                      precision:
                                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                                      width:
                                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,},},
                                                                                                                    $crate::fmt::rt::v1::Argument{position:
                                                                                                                                                      $crate::fmt::rt::v1::Position::At(1usize),
                                                                                                                                                  format:
                                                                                                                                                      $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                                          ' ',
                                                                                                                                                                                      align:
                                                                                                                                                                                          $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                                      flags:
                                                                                                                                                                                          0u32,
                                                                                                                                                                                      precision:
                                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                                      width:
                                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,},},
                                                                                                                    $crate::fmt::rt::v1::Argument{position:
                                                                                                                                                      $crate::fmt::rt::v1::Position::At(2usize),
                                                                                                                                                  format:
                                                                                                                                                      $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                                          ' ',
                                                                                                                                                                                      align:
                                                                                                                                                                                          $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                                      flags:
                                                                                                                                                                                          0u32,
                                                                                                                                                                                      precision:
                                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                                      width:
                                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,},}]),
                                                                        &("dpdk-unix/src/ListParseError.rs",
                                                                          187u32,
                                                                          7u32))
                                        }
                                    }
                                }
                            }
                        };
                        {
                            match (&(group_size), &(0)) {
                                (left_val, right_val) => {
                                    if *left_val == *right_val {
                                        {
                                            $crate::rt::begin_panic_fmt(&$crate::fmt::Arguments::new_v1_formatted(&["assertion failed: `(left != right)`\n  left: `",
                                                                                                                    "`,\n right: `",
                                                                                                                    "`: "],
                                                                                                                  &match (&left_val,
                                                                                                                          &right_val,
                                                                                                                          &$crate::fmt::Arguments::new_v1(&["group_size is zero"],
                                                                                                                                                          &match ()
                                                                                                                                                               {
                                                                                                                                                               ()
                                                                                                                                                               =>
                                                                                                                                                               [],
                                                                                                                                                           }))
                                                                                                                       {
                                                                                                                       (arg0,
                                                                                                                        arg1,
                                                                                                                        arg2)
                                                                                                                       =>
                                                                                                                       [$crate::fmt::ArgumentV1::new(arg0,
                                                                                                                                                     $crate::fmt::Debug::fmt),
                                                                                                                        $crate::fmt::ArgumentV1::new(arg1,
                                                                                                                                                     $crate::fmt::Debug::fmt),
                                                                                                                        $crate::fmt::ArgumentV1::new(arg2,
                                                                                                                                                     $crate::fmt::Display::fmt)],
                                                                                                                   },
                                                                                                                  &[$crate::fmt::rt::v1::Argument{position:
                                                                                                                                                      $crate::fmt::rt::v1::Position::At(0usize),
                                                                                                                                                  format:
                                                                                                                                                      $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                                          ' ',
                                                                                                                                                                                      align:
                                                                                                                                                                                          $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                                      flags:
                                                                                                                                                                                          0u32,
                                                                                                                                                                                      precision:
                                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                                      width:
                                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,},},
                                                                                                                    $crate::fmt::rt::v1::Argument{position:
                                                                                                                                                      $crate::fmt::rt::v1::Position::At(1usize),
                                                                                                                                                  format:
                                                                                                                                                      $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                                          ' ',
                                                                                                                                                                                      align:
                                                                                                                                                                                          $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                                      flags:
                                                                                                                                                                                          0u32,
                                                                                                                                                                                      precision:
                                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                                      width:
                                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,},},
                                                                                                                    $crate::fmt::rt::v1::Argument{position:
                                                                                                                                                      $crate::fmt::rt::v1::Position::At(2usize),
                                                                                                                                                  format:
                                                                                                                                                      $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                                          ' ',
                                                                                                                                                                                      align:
                                                                                                                                                                                          $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                                      flags:
                                                                                                                                                                                          0u32,
                                                                                                                                                                                      precision:
                                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                                      width:
                                                                                                                                                                                          $crate::fmt::rt::v1::Count::Implied,},}]),
                                                                        &("dpdk-unix/src/ListParseError.rs",
                                                                          188u32,
                                                                          7u32))
                                        }
                                    }
                                }
                            }
                        };
                        let mut base_cpu_index = first;
                        while base_cpu_index < second {
                            for cpu_index_increment in 0..used_size {
                                let cpu_index =
                                    base_cpu_index + cpu_index_increment;
                                result.insert(mapper(cpu_index));
                            }
                            base_cpu_index += group_size;
                        }
                    }
                }
            } else {
                let sole = first;
                result.insert(mapper(sole));
                next_minimum_index_expected = sole;
            }
        }
        Ok(result)
    }
}
/// Represents a NUMA node.
#[structural_match]
#[rustc_copy_clone_marker]
pub struct NumaNode(u16);
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::default::Default for NumaNode {
    #[inline]
    fn default() -> NumaNode { NumaNode($crate::default::Default::default()) }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::fmt::Debug for NumaNode {
    fn fmt(&self, f: &mut $crate::fmt::Formatter) -> $crate::fmt::Result {
        match *self {
            NumaNode(ref __self_0_0) => {
                let mut debug_trait_builder = f.debug_tuple("NumaNode");
                let _ = debug_trait_builder.field(&&(*__self_0_0));
                debug_trait_builder.finish()
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::marker::Copy for NumaNode { }
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::clone::Clone for NumaNode {
    #[inline]
    fn clone(&self) -> NumaNode {
        { let _: $crate::clone::AssertParamIsClone<u16>; *self }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::cmp::Ord for NumaNode {
    #[inline]
    fn cmp(&self, other: &NumaNode) -> $crate::cmp::Ordering {
        match *other {
            NumaNode(ref __self_1_0) =>
            match *self {
                NumaNode(ref __self_0_0) =>
                match $crate::cmp::Ord::cmp(&(*__self_0_0), &(*__self_1_0)) {
                    $crate::cmp::Ordering::Equal =>
                    $crate::cmp::Ordering::Equal,
                    cmp => cmp,
                },
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::cmp::PartialOrd for NumaNode {
    #[inline]
    fn partial_cmp(&self, other: &NumaNode)
     -> $crate::option::Option<$crate::cmp::Ordering> {
        match *other {
            NumaNode(ref __self_1_0) =>
            match *self {
                NumaNode(ref __self_0_0) =>
                match $crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                           &(*__self_1_0)) {
                    $crate::option::Option::Some($crate::cmp::Ordering::Equal)
                    =>
                    $crate::option::Option::Some($crate::cmp::Ordering::Equal),
                    cmp => cmp,
                },
            },
        }
    }
    #[inline]
    fn lt(&self, other: &NumaNode) -> bool {
        match *other {
            NumaNode(ref __self_1_0) =>
            match *self {
                NumaNode(ref __self_0_0) =>
                $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                       &(*__self_1_0)),
                                                  $crate::cmp::Ordering::Greater)
                    == $crate::cmp::Ordering::Less,
            },
        }
    }
    #[inline]
    fn le(&self, other: &NumaNode) -> bool {
        match *other {
            NumaNode(ref __self_1_0) =>
            match *self {
                NumaNode(ref __self_0_0) =>
                $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                       &(*__self_1_0)),
                                                  $crate::cmp::Ordering::Greater)
                    != $crate::cmp::Ordering::Greater,
            },
        }
    }
    #[inline]
    fn gt(&self, other: &NumaNode) -> bool {
        match *other {
            NumaNode(ref __self_1_0) =>
            match *self {
                NumaNode(ref __self_0_0) =>
                $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                       &(*__self_1_0)),
                                                  $crate::cmp::Ordering::Less)
                    == $crate::cmp::Ordering::Greater,
            },
        }
    }
    #[inline]
    fn ge(&self, other: &NumaNode) -> bool {
        match *other {
            NumaNode(ref __self_1_0) =>
            match *self {
                NumaNode(ref __self_0_0) =>
                $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                       &(*__self_1_0)),
                                                  $crate::cmp::Ordering::Less)
                    != $crate::cmp::Ordering::Less,
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::cmp::Eq for NumaNode {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        { let _: $crate::cmp::AssertParamIsEq<u16>; }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::cmp::PartialEq for NumaNode {
    #[inline]
    fn eq(&self, other: &NumaNode) -> bool {
        match *other {
            NumaNode(ref __self_1_0) =>
            match *self {
                NumaNode(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &NumaNode) -> bool {
        match *other {
            NumaNode(ref __self_1_0) =>
            match *self {
                NumaNode(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::hash::Hash for NumaNode {
    fn hash<__H: $crate::hash::Hasher>(&self, state: &mut __H) -> () {
        match *self {
            NumaNode(ref __self_0_0) => {
                $crate::hash::Hash::hash(&(*__self_0_0), state)
            }
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_NumaNode: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[allow(unused_macros)]
        macro_rules! try(( $ __expr : expr ) => {
                         match $ __expr {
                         _serde :: export :: Ok ( __val ) => __val , _serde ::
                         export :: Err ( __err ) => {
                         return _serde :: export :: Err ( __err ) ; } } });
        #[automatically_derived]
        impl <'de> _serde::Deserialize<'de> for NumaNode {
            fn deserialize<__D>(__deserializer: __D)
             -> _serde::export::Result<Self, __D::Error> where
             __D: _serde::Deserializer<'de> {
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<NumaNode>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl <'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type
                    Value
                    =
                    NumaNode;
                    fn expecting(&self,
                                 __formatter: &mut _serde::export::Formatter)
                     -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter,
                                                             "tuple struct NumaNode")
                    }
                    #[inline]
                    fn visit_newtype_struct<__E>(self, __e: __E)
                     -> _serde::export::Result<Self::Value, __E::Error> where
                     __E: _serde::Deserializer<'de> {
                        let __field0: u16 =
                            match <u16 as
                                      _serde::Deserialize>::deserialize(__e) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                        _serde::export::Ok(NumaNode(__field0))
                    }
                    #[inline]
                    fn visit_seq<__A>(self, mut __seq: __A)
                     -> _serde::export::Result<Self::Value, __A::Error> where
                     __A: _serde::de::SeqAccess<'de> {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<u16>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(0usize,
                                                                                                 &"tuple struct NumaNode with 1 element"));
                                }
                            };
                        _serde::export::Ok(NumaNode(__field0))
                    }
                }
                _serde::Deserializer::deserialize_newtype_struct(__deserializer,
                                                                 "NumaNode",
                                                                 __Visitor{marker:
                                                                               _serde::export::PhantomData::<NumaNode>,
                                                                           lifetime:
                                                                               _serde::export::PhantomData,})
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_NumaNode: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[allow(unused_macros)]
        macro_rules! try(( $ __expr : expr ) => {
                         match $ __expr {
                         _serde :: export :: Ok ( __val ) => __val , _serde ::
                         export :: Err ( __err ) => {
                         return _serde :: export :: Err ( __err ) ; } } });
        #[automatically_derived]
        impl _serde::Serialize for NumaNode {
            fn serialize<__S>(&self, __serializer: __S)
             -> _serde::export::Result<__S::Ok, __S::Error> where
             __S: _serde::Serializer {
                _serde::Serializer::serialize_newtype_struct(__serializer,
                                                             "NumaNode",
                                                             &self.0)
            }
        }
    };
impl From<u16> for NumaNode {
    #[inline(always)]
    fn from(value: u16) -> Self { NumaNode(value) }
}
impl Into<u16> for NumaNode {
    #[inline(always)]
    fn into(self) -> u16 { self.0 }
}
/// A NUMA node bitmask.
pub type NumaNodeBitmask = u32;
/// An extension trait for `OsStr`.
pub trait OsStrExtMore {
    /// Converts as `OsStr` to a `CString`.
    #[inline(always)]
    fn os_str_to_c_string(&self)
    -> CString;
}
impl OsStrExtMore for OsStr {
    #[inline(always)]
    fn os_str_to_c_string(&self) -> CString {
        CString::new(self.as_bytes()).expect("os_str should not contain interior ASCII NULs")
    }
}
/// A very slightly faster function to get page size than `sysconf(_SC_PAGESIZE)` on musl libc systems.
///
/// Result is normally constant, but is derived from data passed when an executable is first loaded.
#[inline(always)]
pub fn page_size() -> usize {
    unsafe { ::libc_extra::unix::unistd::getpagesize() as usize }
}
/// An extension trait to make it easier to work with sys and proc files and folders.
pub trait PathExt {
    /// Converts a `Path` to a `CString`.
    #[cfg(unix)]
    #[inline(always)]
    fn to_c_string(&self)
    -> CString;
    /// Makes a file read-write to all.
    #[cfg(unix)]
    #[inline(always)]
    fn make_file_read_write_all(&self)
    -> io::Result<()>;
    /// Makes a folder searchable to all (ie gives it read and execute permissions).
    #[cfg(unix)]
    #[inline(always)]
    fn make_folder_searchable_to_all(&self)
    -> io::Result<()>;
    /// Reads a value from a file which is line-feed terminated and is hexadecimal using a parser.
    #[inline(always)]
    fn read_hexadecimal_value_with_prefix<P: Fn(&str) ->
                                          Result<T, ParseIntError>,
                                          T>(&self, size: usize, parser: P)
    -> io::Result<T>;
    /// Reads a value from a file which is line-feed terminated and is hexadecimal into an u16.
    #[inline(always)]
    fn read_hexadecimal_value_with_prefix_u16(&self) -> io::Result<u16> {
        self.read_hexadecimal_value_with_prefix(4,
                                                |raw_string|
                                                    u16::from_str_radix(raw_string,
                                                                        16))
    }
    /// Reads a file as bytes.
    ///
    /// Fails if empty.
    #[inline(always)]
    fn read_raw(&self)
    -> io::Result<Box<[u8]>>;
    /// Reads a file as bytes, expecting a final line feed.
    ///
    /// The returned bytes lack a final line feed.
    #[inline(always)]
    fn read_raw_without_line_feed(&self)
    -> io::Result<Box<[u8]>>;
    /// Reads a file as a string.
    ///
    /// Fails if empty.
    #[inline(always)]
    fn read_raw_string(&self)
    -> io::Result<String>;
    /// Reads a file as a string, expecting a final line feed.
    ///
    /// The returned string lacks a final line feed.
    #[inline(always)]
    fn read_string_without_line_feed(&self)
    -> io::Result<String>;
    /// Reads a value from a file which is line-feed terminated.
    #[inline(always)]
    fn read_value<F>(&self)
    -> io::Result<F>
    where
    F: FromStr,
    <F as FromStr>::Err: 'static +
    Send +
    Sync +
    error::Error;
    /// Writes a value to a file which is line-feed terminated.
    #[inline(always)]
    fn write_value<D: Display>(&self, value: D)
    -> io::Result<()>;
    /// Reads and parses a linux core or numa list string from a file.
    ///
    /// Returns a BTreeSet with the zero-based indices found in the string. For example, "2,4-31,32-63" would return a set with all values between 0 to 63 except 0, 1 and 3.
    #[inline(always)]
    fn read_linux_core_or_numa_list<Mapper: Fn(u16) -> R,
                                    R: Ord>(&self, mapper: Mapper)
    -> Result<BTreeSet<R>, ListParseError>;
    /// Reads and parses a linux core or numa mask string from a file.
    #[inline(always)]
    fn parse_linux_core_or_numa_bitmask(&self)
    -> Result<u32, io::Error>;
    /// Parses a process status, such as `/proc/status/self`.
    #[inline(always)]
    fn parse_process_status_file(&self)
    -> Result<ProcessStatusStatistics, ProcessStatusFileParseError>;
    /// Parses a virtual memory statistics file (`vmstat`).
    #[inline(always)]
    fn parse_virtual_memory_statistics_file(&self)
    -> io::Result<HashMap<VirtualMemoryStatisticName, u64>>;
    /// Parses a memory information file (`meminfo`).
    #[inline(always)]
    fn parse_memory_information_file(&self,
                                     memory_information_name_prefix: &[u8])
    -> Result<MemoryInformation, MemoryInformationParseError>;
}
impl PathExt for Path {
    #[cfg(unix)]
    #[inline(always)]
    fn to_c_string(&self) -> CString {
        CString::new(self.as_os_str().as_bytes()).expect("Paths should not contain interior ASCII NULs")
    }
    #[cfg(unix)]
    #[inline(always)]
    fn make_file_read_write_all(&self) -> io::Result<()> {
        #[inline(always)]
        fn add_read_write_permissions(permissions: Permissions)
         -> Permissions {
            Permissions::from_mode(permissions.mode() | 438)
        }
        let metadata = metadata(self)?;
        set_permissions(self,
                        add_read_write_permissions(metadata.permissions()))
    }
    #[cfg(unix)]
    #[inline(always)]
    fn make_folder_searchable_to_all(&self) -> io::Result<()> {
        #[inline(always)]
        fn add_read_and_execute_permissions(permissions: Permissions)
         -> Permissions {
            Permissions::from_mode(permissions.mode() | 365)
        }
        let metadata = metadata(self)?;
        set_permissions(self,
                        add_read_and_execute_permissions(metadata.permissions()))
    }
    #[inline(always)]
    fn read_hexadecimal_value_with_prefix<P: Fn(&str) ->
                                          Result<T, ParseIntError>,
                                          T>(&self, size: usize, parser: P)
     -> io::Result<T> {
        use self::ErrorKind::InvalidData;
        let raw_string = self.read_string_without_line_feed()?;
        let size_wih_0x_prefix = 2 + size;
        if raw_string.len() != size_wih_0x_prefix {
            return Err(io::Error::new(InvalidData,
                                      $crate::fmt::format($crate::fmt::Arguments::new_v1_formatted(&["",
                                                                                                     " bytes not read"],
                                                                                                   &match (&size_wih_0x_prefix,)
                                                                                                        {
                                                                                                        (arg0,)
                                                                                                        =>
                                                                                                        [$crate::fmt::ArgumentV1::new(arg0,
                                                                                                                                      $crate::fmt::Display::fmt)],
                                                                                                    },
                                                                                                   &[$crate::fmt::rt::v1::Argument{position:
                                                                                                                                       $crate::fmt::rt::v1::Position::At(0usize),
                                                                                                                                   format:
                                                                                                                                       $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                           ' ',
                                                                                                                                                                       align:
                                                                                                                                                                           $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                       flags:
                                                                                                                                                                           0u32,
                                                                                                                                                                       precision:
                                                                                                                                                                           $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                       width:
                                                                                                                                                                           $crate::fmt::rt::v1::Count::Implied,},}]))));
        }
        match &raw_string[..2] {
            "0x" => (),
            _ =>
            return Err(io::Error::new(InvalidData,
                                      "value does not start \'0x\'")),
        }
        match parser(&raw_string[2..]) {
            Err(error) => Err(io::Error::new(InvalidData, error)),
            Ok(value) => Ok(value),
        }
    }
    #[inline(always)]
    fn read_raw(&self) -> io::Result<Box<[u8]>> {
        let raw = ::std::fs::read(self)?.into_boxed_slice();
        if raw.is_empty() {
            Err(io::Error::new(ErrorKind::InvalidData, "Empty file"))
        } else { Ok(raw) }
    }
    #[inline(always)]
    fn read_raw_without_line_feed(&self) -> io::Result<Box<[u8]>> {
        let mut raw = self.read_raw()?.to_vec();
        let length = raw.len();
        let should_be_line_feed = raw.remove(length - 1);
        if should_be_line_feed != b'\n' {
            return Err(io::Error::new(ErrorKind::InvalidData,
                                      "File lacks terminating line feed"));
        }
        Ok(raw.into_boxed_slice())
    }
    #[inline(always)]
    fn read_raw_string(&self) -> io::Result<String> {
        let raw_string = read_to_string(self)?;
        if raw_string.is_empty() {
            Err(io::Error::new(ErrorKind::InvalidData, "Empty file"))
        } else { Ok(raw_string) }
    }
    #[inline(always)]
    fn read_string_without_line_feed(&self) -> io::Result<String> {
        let mut raw_string = self.read_raw_string()?;
        let length = raw_string.len();
        let should_be_line_feed = raw_string.remove(length - 1);
        if should_be_line_feed != '\n' {
            return Err(io::Error::new(ErrorKind::InvalidData,
                                      "File lacks terminating line feed"));
        }
        Ok(raw_string)
    }
    #[inline(always)]
    fn read_value<F>(&self) -> io::Result<F> where F: FromStr,
     <F as FromStr>::Err: 'static + Send + Sync + error::Error {
        let string = self.read_string_without_line_feed()?;
        match string.parse::<F>() {
            Err(error) => Err(io::Error::new(ErrorKind::InvalidData, error)),
            Ok(value) => Ok(value),
        }
    }
    #[inline(always)]
    fn write_value<D: Display>(&self, value: D) -> io::Result<()> {
        let value =
            $crate::fmt::format($crate::fmt::Arguments::new_v1_formatted(&["",
                                                                           "\n"],
                                                                         &match (&value,)
                                                                              {
                                                                              (arg0,)
                                                                              =>
                                                                              [$crate::fmt::ArgumentV1::new(arg0,
                                                                                                            $crate::fmt::Display::fmt)],
                                                                          },
                                                                         &[$crate::fmt::rt::v1::Argument{position:
                                                                                                             $crate::fmt::rt::v1::Position::At(0usize),
                                                                                                         format:
                                                                                                             $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                 ' ',
                                                                                                                                             align:
                                                                                                                                                 $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                             flags:
                                                                                                                                                 0u32,
                                                                                                                                             precision:
                                                                                                                                                 $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                             width:
                                                                                                                                                 $crate::fmt::rt::v1::Count::Implied,},}])).into_bytes();
        let mut file = OpenOptions::new().write(true).open(self)?;
        file.write_all(value.as_slice())
    }
    #[inline(always)]
    fn read_linux_core_or_numa_list<Mapper: Fn(u16) -> R,
                                    R: Ord>(&self, mapper: Mapper)
     -> Result<BTreeSet<R>, ListParseError> {
        let without_line_feed = self.read_raw_without_line_feed()?;
        ListParseError::parse_linux_list_string::<Mapper,
                                                  R>(&without_line_feed,
                                                     mapper)
    }
    #[inline(always)]
    fn parse_linux_core_or_numa_bitmask(&self) -> Result<u32, io::Error> {
        let without_line_feed = self.read_string_without_line_feed()?;
        if without_line_feed.len() != 8 {
            return Err(io::Error::new(ErrorKind::InvalidData,
                                      "Linux core or numa mask string should be 8 characters long"))
        }
        u32::from_str_radix(&without_line_feed,
                            16).map_err(|error|
                                            io::Error::new(ErrorKind::InvalidData,
                                                           error))
    }
    #[inline(always)]
    fn parse_process_status_file(&self)
     -> Result<ProcessStatusStatistics, ProcessStatusFileParseError> {
        let file = File::open(self)?;
        let reader = BufReader::with_capacity(4096, file);
        ProcessStatusStatistics::parse(reader)
    }
    #[inline(always)]
    fn parse_virtual_memory_statistics_file(&self)
     -> io::Result<HashMap<VirtualMemoryStatisticName, u64>> {
        let file = File::open(self)?;
        let reader = BufReader::with_capacity(4096, file);
        let mut statistics = HashMap::with_capacity(6);
        let mut zero_based_line_number = 0;
        for line in reader.split(b'\n') {
            let mut line = line?;
            {
                use self::ErrorKind::InvalidData;
                let mut split = splitn(&line, 2, b' ');
                let statistic_name =
                    VirtualMemoryStatisticName::parse(split.next().unwrap());
                let statistic_value =
                    match split.next() {
                        None =>
                        return Err(io::Error::new(InvalidData,
                                                  $crate::fmt::format($crate::fmt::Arguments::new_v1_formatted(&["Zero based line \'",
                                                                                                                 "\' does not have a value second column"],
                                                                                                               &match (&zero_based_line_number,)
                                                                                                                    {
                                                                                                                    (arg0,)
                                                                                                                    =>
                                                                                                                    [$crate::fmt::ArgumentV1::new(arg0,
                                                                                                                                                  $crate::fmt::Display::fmt)],
                                                                                                                },
                                                                                                               &[$crate::fmt::rt::v1::Argument{position:
                                                                                                                                                   $crate::fmt::rt::v1::Position::At(0usize),
                                                                                                                                               format:
                                                                                                                                                   $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                                       ' ',
                                                                                                                                                                                   align:
                                                                                                                                                                                       $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                                   flags:
                                                                                                                                                                                       0u32,
                                                                                                                                                                                   precision:
                                                                                                                                                                                       $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                                   width:
                                                                                                                                                                                       $crate::fmt::rt::v1::Count::Implied,},}])))),
                        Some(value) => {
                            let str_value =
                                match from_utf8(value) {
                                    Err(utf8_error) =>
                                    return Err(io::Error::new(InvalidData,
                                                              utf8_error)),
                                    Ok(str_value) => str_value,
                                };
                            match str_value.parse::<u64>() {
                                Err(parse_error) =>
                                return Err(io::Error::new(InvalidData,
                                                          parse_error)),
                                Ok(value) => value,
                            }
                        }
                    };
                if let Some(previous) =
                       statistics.insert(statistic_name, statistic_value) {
                    return Err(io::Error::new(InvalidData,
                                              $crate::fmt::format($crate::fmt::Arguments::new_v1_formatted(&["Zero based line \'",
                                                                                                             "\' has a duplicate statistic (was \'",
                                                                                                             "\')"],
                                                                                                           &match (&zero_based_line_number,
                                                                                                                   &previous)
                                                                                                                {
                                                                                                                (arg0,
                                                                                                                 arg1)
                                                                                                                =>
                                                                                                                [$crate::fmt::ArgumentV1::new(arg0,
                                                                                                                                              $crate::fmt::Display::fmt),
                                                                                                                 $crate::fmt::ArgumentV1::new(arg1,
                                                                                                                                              $crate::fmt::Display::fmt)],
                                                                                                            },
                                                                                                           &[$crate::fmt::rt::v1::Argument{position:
                                                                                                                                               $crate::fmt::rt::v1::Position::At(0usize),
                                                                                                                                           format:
                                                                                                                                               $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                                   ' ',
                                                                                                                                                                               align:
                                                                                                                                                                                   $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                               flags:
                                                                                                                                                                                   0u32,
                                                                                                                                                                               precision:
                                                                                                                                                                                   $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                               width:
                                                                                                                                                                                   $crate::fmt::rt::v1::Count::Implied,},},
                                                                                                             $crate::fmt::rt::v1::Argument{position:
                                                                                                                                               $crate::fmt::rt::v1::Position::At(1usize),
                                                                                                                                           format:
                                                                                                                                               $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                                   ' ',
                                                                                                                                                                               align:
                                                                                                                                                                                   $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                               flags:
                                                                                                                                                                                   0u32,
                                                                                                                                                                               precision:
                                                                                                                                                                                   $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                               width:
                                                                                                                                                                                   $crate::fmt::rt::v1::Count::Implied,},}]))))
                }
            }
            line.clear();
            zero_based_line_number += 1;
        }
        Ok(statistics)
    }
    /// Parses the `meminfo` file.
    fn parse_memory_information_file(&self,
                                     memory_information_name_prefix: &[u8])
     -> Result<MemoryInformation, MemoryInformationParseError> {
        let reader = BufReader::with_capacity(4096, File::open(self)?);
        let mut map = HashMap::new();
        let mut zero_based_line_number = 0;
        use self::MemoryInformationParseError::*;
        for line in reader.split(b'\n') {
            let mut line = line?;
            {
                let mut split = splitn(&line, 2, b':');
                let memory_information_name =
                    MemoryInformationName::parse(split.next().unwrap(),
                                                 memory_information_name_prefix);
                let memory_information_value =
                    match split.next() {
                        None =>
                        return Err(MemoryInformationParseError::CouldNotParseMemoryInformationValue{zero_based_line_number,
                                                                                                    memory_information_name,}),
                        Some(raw_value) => {
                            let str_value =
                                match from_utf8(raw_value) {
                                    Err(utf8_error) =>
                                    return Err(CouldNotParseAsUtf8{zero_based_line_number,
                                                                   memory_information_name,
                                                                   bad_value:
                                                                       raw_value.to_vec().into_boxed_slice(),
                                                                   cause:
                                                                       utf8_error,}),
                                    Ok(str_value) => str_value,
                                };
                            let trimmed_str_value = str_value.trim();
                            let ends_with =
                                memory_information_name.unit().ends_with();
                            if !trimmed_str_value.ends_with(ends_with) {
                                return Err(CouldNotParseMemoryInformationValueTrimmed{zero_based_line_number,
                                                                                      memory_information_name,
                                                                                      bad_value:
                                                                                          trimmed_str_value.to_owned(),});
                            }
                            let trimmed =
                                &trimmed_str_value[0..trimmed_str_value.len()
                                                          - ends_with.len()];
                            match trimmed.parse::<u64>() {
                                Ok(value) => value,
                                Err(int_parse_error) =>
                                return Err(CouldNotParseMemoryInformationValueAsU64{zero_based_line_number,
                                                                                    memory_information_name,
                                                                                    bad_value:
                                                                                        trimmed.to_owned(),
                                                                                    cause:
                                                                                        int_parse_error,}),
                            }
                        }
                    };
                if map.contains_key(&memory_information_name) {
                    return Err(DuplicateMemoryInformation{zero_based_line_number,
                                                          memory_information_name,
                                                          new_value:
                                                              memory_information_value,});
                }
                map.insert(memory_information_name, memory_information_value);
            }
            line.clear();
            zero_based_line_number += 1;
        }
        Ok(MemoryInformation(map))
    }
}
/// Represents `/proc`.
///
/// Frankly, there are files in `/proc` that really belong in `/sys`.
#[structural_match]
pub struct ProcPath(PathBuf);
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::fmt::Debug for ProcPath {
    fn fmt(&self, f: &mut $crate::fmt::Formatter) -> $crate::fmt::Result {
        match *self {
            ProcPath(ref __self_0_0) => {
                let mut debug_trait_builder = f.debug_tuple("ProcPath");
                let _ = debug_trait_builder.field(&&(*__self_0_0));
                debug_trait_builder.finish()
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::clone::Clone for ProcPath {
    #[inline]
    fn clone(&self) -> ProcPath {
        match *self {
            ProcPath(ref __self_0_0) =>
            ProcPath($crate::clone::Clone::clone(&(*__self_0_0))),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::cmp::Ord for ProcPath {
    #[inline]
    fn cmp(&self, other: &ProcPath) -> $crate::cmp::Ordering {
        match *other {
            ProcPath(ref __self_1_0) =>
            match *self {
                ProcPath(ref __self_0_0) =>
                match $crate::cmp::Ord::cmp(&(*__self_0_0), &(*__self_1_0)) {
                    $crate::cmp::Ordering::Equal =>
                    $crate::cmp::Ordering::Equal,
                    cmp => cmp,
                },
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::cmp::PartialOrd for ProcPath {
    #[inline]
    fn partial_cmp(&self, other: &ProcPath)
     -> $crate::option::Option<$crate::cmp::Ordering> {
        match *other {
            ProcPath(ref __self_1_0) =>
            match *self {
                ProcPath(ref __self_0_0) =>
                match $crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                           &(*__self_1_0)) {
                    $crate::option::Option::Some($crate::cmp::Ordering::Equal)
                    =>
                    $crate::option::Option::Some($crate::cmp::Ordering::Equal),
                    cmp => cmp,
                },
            },
        }
    }
    #[inline]
    fn lt(&self, other: &ProcPath) -> bool {
        match *other {
            ProcPath(ref __self_1_0) =>
            match *self {
                ProcPath(ref __self_0_0) =>
                $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                       &(*__self_1_0)),
                                                  $crate::cmp::Ordering::Greater)
                    == $crate::cmp::Ordering::Less,
            },
        }
    }
    #[inline]
    fn le(&self, other: &ProcPath) -> bool {
        match *other {
            ProcPath(ref __self_1_0) =>
            match *self {
                ProcPath(ref __self_0_0) =>
                $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                       &(*__self_1_0)),
                                                  $crate::cmp::Ordering::Greater)
                    != $crate::cmp::Ordering::Greater,
            },
        }
    }
    #[inline]
    fn gt(&self, other: &ProcPath) -> bool {
        match *other {
            ProcPath(ref __self_1_0) =>
            match *self {
                ProcPath(ref __self_0_0) =>
                $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                       &(*__self_1_0)),
                                                  $crate::cmp::Ordering::Less)
                    == $crate::cmp::Ordering::Greater,
            },
        }
    }
    #[inline]
    fn ge(&self, other: &ProcPath) -> bool {
        match *other {
            ProcPath(ref __self_1_0) =>
            match *self {
                ProcPath(ref __self_0_0) =>
                $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                       &(*__self_1_0)),
                                                  $crate::cmp::Ordering::Less)
                    != $crate::cmp::Ordering::Less,
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::cmp::Eq for ProcPath {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        { let _: $crate::cmp::AssertParamIsEq<PathBuf>; }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::cmp::PartialEq for ProcPath {
    #[inline]
    fn eq(&self, other: &ProcPath) -> bool {
        match *other {
            ProcPath(ref __self_1_0) =>
            match *self {
                ProcPath(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &ProcPath) -> bool {
        match *other {
            ProcPath(ref __self_1_0) =>
            match *self {
                ProcPath(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::hash::Hash for ProcPath {
    fn hash<__H: $crate::hash::Hasher>(&self, state: &mut __H) -> () {
        match *self {
            ProcPath(ref __self_0_0) => {
                $crate::hash::Hash::hash(&(*__self_0_0), state)
            }
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_ProcPath: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[allow(unused_macros)]
        macro_rules! try(( $ __expr : expr ) => {
                         match $ __expr {
                         _serde :: export :: Ok ( __val ) => __val , _serde ::
                         export :: Err ( __err ) => {
                         return _serde :: export :: Err ( __err ) ; } } });
        #[automatically_derived]
        impl <'de> _serde::Deserialize<'de> for ProcPath {
            fn deserialize<__D>(__deserializer: __D)
             -> _serde::export::Result<Self, __D::Error> where
             __D: _serde::Deserializer<'de> {
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<ProcPath>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl <'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type
                    Value
                    =
                    ProcPath;
                    fn expecting(&self,
                                 __formatter: &mut _serde::export::Formatter)
                     -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter,
                                                             "tuple struct ProcPath")
                    }
                    #[inline]
                    fn visit_newtype_struct<__E>(self, __e: __E)
                     -> _serde::export::Result<Self::Value, __E::Error> where
                     __E: _serde::Deserializer<'de> {
                        let __field0: PathBuf =
                            match <PathBuf as
                                      _serde::Deserialize>::deserialize(__e) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                        _serde::export::Ok(ProcPath(__field0))
                    }
                    #[inline]
                    fn visit_seq<__A>(self, mut __seq: __A)
                     -> _serde::export::Result<Self::Value, __A::Error> where
                     __A: _serde::de::SeqAccess<'de> {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<PathBuf>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(0usize,
                                                                                                 &"tuple struct ProcPath with 1 element"));
                                }
                            };
                        _serde::export::Ok(ProcPath(__field0))
                    }
                }
                _serde::Deserializer::deserialize_newtype_struct(__deserializer,
                                                                 "ProcPath",
                                                                 __Visitor{marker:
                                                                               _serde::export::PhantomData::<ProcPath>,
                                                                           lifetime:
                                                                               _serde::export::PhantomData,})
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_ProcPath: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[allow(unused_macros)]
        macro_rules! try(( $ __expr : expr ) => {
                         match $ __expr {
                         _serde :: export :: Ok ( __val ) => __val , _serde ::
                         export :: Err ( __err ) => {
                         return _serde :: export :: Err ( __err ) ; } } });
        #[automatically_derived]
        impl _serde::Serialize for ProcPath {
            fn serialize<__S>(&self, __serializer: __S)
             -> _serde::export::Result<__S::Ok, __S::Error> where
             __S: _serde::Serializer {
                _serde::Serializer::serialize_newtype_struct(__serializer,
                                                             "ProcPath",
                                                             &self.0)
            }
        }
    };
impl Default for ProcPath {
    #[inline(always)]
    fn default() -> Self { ProcPath(PathBuf::from("/proc")) }
}
impl ProcPath { }
/// Sets the current thread name.
pub fn set_current_thread_name(name: &str)
 -> Result<(), SetCurrentThreadNameError> {
    match name.len() {
        0 => Err(SetCurrentThreadNameError::NameIsEmpty),
        length if length > 15 =>
        Err(SetCurrentThreadNameError::NameIsTooLong),
        _ => {
            let c_string = CString::new(name.to_owned())?;
            let pointer = c_string.as_ptr();

            #[cfg(any(target_os = "ios", target_os = "macos"))]
            unsafe { ::libc::pthread_setname_np(pointer) };
            Ok(())
        }
    }
}
/// An error occurred when setting the current thread name.
pub enum SetCurrentThreadNameError {

    /// A thread name is empty.
    NameIsEmpty,

    /// A thread name is too long (it must be 15 characters or less).
    NameIsTooLong,

    /// A thread name contains an ASCII NUL.
    NameContainsNul(NulError),
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::fmt::Debug for SetCurrentThreadNameError {
    fn fmt(&self, f: &mut $crate::fmt::Formatter) -> $crate::fmt::Result {
        match (&*self,) {
            (&SetCurrentThreadNameError::NameIsEmpty,) => {
                let mut debug_trait_builder = f.debug_tuple("NameIsEmpty");
                debug_trait_builder.finish()
            }
            (&SetCurrentThreadNameError::NameIsTooLong,) => {
                let mut debug_trait_builder = f.debug_tuple("NameIsTooLong");
                debug_trait_builder.finish()
            }
            (&SetCurrentThreadNameError::NameContainsNul(ref __self_0),) => {
                let mut debug_trait_builder =
                    f.debug_tuple("NameContainsNul");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
        }
    }
}
impl Display for SetCurrentThreadNameError {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        <SetCurrentThreadNameError as Debug>::fmt(self, f)
    }
}
impl error::Error for SetCurrentThreadNameError {
    #[inline(always)]
    fn source(&self) -> Option<&(error::Error + 'static)> {
        use self::SetCurrentThreadNameError::*;
        match self {
            &NameIsEmpty => None,
            &NameIsTooLong => None,
            &NameContainsNul(ref error) => Some(error),
        }
    }
}
impl From<NulError> for SetCurrentThreadNameError {
    #[inline(always)]
    fn from(error: NulError) -> Self {
        SetCurrentThreadNameError::NameContainsNul(error)
    }
}
/// Represents `/sys`.
#[structural_match]
pub struct SysPath(PathBuf);
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::fmt::Debug for SysPath {
    fn fmt(&self, f: &mut $crate::fmt::Formatter) -> $crate::fmt::Result {
        match *self {
            SysPath(ref __self_0_0) => {
                let mut debug_trait_builder = f.debug_tuple("SysPath");
                let _ = debug_trait_builder.field(&&(*__self_0_0));
                debug_trait_builder.finish()
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::clone::Clone for SysPath {
    #[inline]
    fn clone(&self) -> SysPath {
        match *self {
            SysPath(ref __self_0_0) =>
            SysPath($crate::clone::Clone::clone(&(*__self_0_0))),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::cmp::Ord for SysPath {
    #[inline]
    fn cmp(&self, other: &SysPath) -> $crate::cmp::Ordering {
        match *other {
            SysPath(ref __self_1_0) =>
            match *self {
                SysPath(ref __self_0_0) =>
                match $crate::cmp::Ord::cmp(&(*__self_0_0), &(*__self_1_0)) {
                    $crate::cmp::Ordering::Equal =>
                    $crate::cmp::Ordering::Equal,
                    cmp => cmp,
                },
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::cmp::PartialOrd for SysPath {
    #[inline]
    fn partial_cmp(&self, other: &SysPath)
     -> $crate::option::Option<$crate::cmp::Ordering> {
        match *other {
            SysPath(ref __self_1_0) =>
            match *self {
                SysPath(ref __self_0_0) =>
                match $crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                           &(*__self_1_0)) {
                    $crate::option::Option::Some($crate::cmp::Ordering::Equal)
                    =>
                    $crate::option::Option::Some($crate::cmp::Ordering::Equal),
                    cmp => cmp,
                },
            },
        }
    }
    #[inline]
    fn lt(&self, other: &SysPath) -> bool {
        match *other {
            SysPath(ref __self_1_0) =>
            match *self {
                SysPath(ref __self_0_0) =>
                $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                       &(*__self_1_0)),
                                                  $crate::cmp::Ordering::Greater)
                    == $crate::cmp::Ordering::Less,
            },
        }
    }
    #[inline]
    fn le(&self, other: &SysPath) -> bool {
        match *other {
            SysPath(ref __self_1_0) =>
            match *self {
                SysPath(ref __self_0_0) =>
                $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                       &(*__self_1_0)),
                                                  $crate::cmp::Ordering::Greater)
                    != $crate::cmp::Ordering::Greater,
            },
        }
    }
    #[inline]
    fn gt(&self, other: &SysPath) -> bool {
        match *other {
            SysPath(ref __self_1_0) =>
            match *self {
                SysPath(ref __self_0_0) =>
                $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                       &(*__self_1_0)),
                                                  $crate::cmp::Ordering::Less)
                    == $crate::cmp::Ordering::Greater,
            },
        }
    }
    #[inline]
    fn ge(&self, other: &SysPath) -> bool {
        match *other {
            SysPath(ref __self_1_0) =>
            match *self {
                SysPath(ref __self_0_0) =>
                $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                       &(*__self_1_0)),
                                                  $crate::cmp::Ordering::Less)
                    != $crate::cmp::Ordering::Less,
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::cmp::Eq for SysPath {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        { let _: $crate::cmp::AssertParamIsEq<PathBuf>; }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::cmp::PartialEq for SysPath {
    #[inline]
    fn eq(&self, other: &SysPath) -> bool {
        match *other {
            SysPath(ref __self_1_0) =>
            match *self {
                SysPath(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &SysPath) -> bool {
        match *other {
            SysPath(ref __self_1_0) =>
            match *self {
                SysPath(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl $crate::hash::Hash for SysPath {
    fn hash<__H: $crate::hash::Hasher>(&self, state: &mut __H) -> () {
        match *self {
            SysPath(ref __self_0_0) => {
                $crate::hash::Hash::hash(&(*__self_0_0), state)
            }
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_SysPath: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[allow(unused_macros)]
        macro_rules! try(( $ __expr : expr ) => {
                         match $ __expr {
                         _serde :: export :: Ok ( __val ) => __val , _serde ::
                         export :: Err ( __err ) => {
                         return _serde :: export :: Err ( __err ) ; } } });
        #[automatically_derived]
        impl <'de> _serde::Deserialize<'de> for SysPath {
            fn deserialize<__D>(__deserializer: __D)
             -> _serde::export::Result<Self, __D::Error> where
             __D: _serde::Deserializer<'de> {
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<SysPath>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl <'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type
                    Value
                    =
                    SysPath;
                    fn expecting(&self,
                                 __formatter: &mut _serde::export::Formatter)
                     -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter,
                                                             "tuple struct SysPath")
                    }
                    #[inline]
                    fn visit_newtype_struct<__E>(self, __e: __E)
                     -> _serde::export::Result<Self::Value, __E::Error> where
                     __E: _serde::Deserializer<'de> {
                        let __field0: PathBuf =
                            match <PathBuf as
                                      _serde::Deserialize>::deserialize(__e) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                        _serde::export::Ok(SysPath(__field0))
                    }
                    #[inline]
                    fn visit_seq<__A>(self, mut __seq: __A)
                     -> _serde::export::Result<Self::Value, __A::Error> where
                     __A: _serde::de::SeqAccess<'de> {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<PathBuf>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(0usize,
                                                                                                 &"tuple struct SysPath with 1 element"));
                                }
                            };
                        _serde::export::Ok(SysPath(__field0))
                    }
                }
                _serde::Deserializer::deserialize_newtype_struct(__deserializer,
                                                                 "SysPath",
                                                                 __Visitor{marker:
                                                                               _serde::export::PhantomData::<SysPath>,
                                                                           lifetime:
                                                                               _serde::export::PhantomData,})
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_SysPath: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[allow(unused_macros)]
        macro_rules! try(( $ __expr : expr ) => {
                         match $ __expr {
                         _serde :: export :: Ok ( __val ) => __val , _serde ::
                         export :: Err ( __err ) => {
                         return _serde :: export :: Err ( __err ) ; } } });
        #[automatically_derived]
        impl _serde::Serialize for SysPath {
            fn serialize<__S>(&self, __serializer: __S)
             -> _serde::export::Result<__S::Ok, __S::Error> where
             __S: _serde::Serializer {
                _serde::Serializer::serialize_newtype_struct(__serializer,
                                                             "SysPath",
                                                             &self.0)
            }
        }
    };
impl Default for SysPath {
    #[inline(always)]
    fn default() -> Self { SysPath(PathBuf::from("/sys")) }
}
impl SysPath {
    /// Is this a NUMA-based machine?
    #[inline(always)]
    pub fn is_a_numa_machine(&self) -> bool {
        self.numa_nodes_parent_path().exists()
    }
    /// Is this a NUMA node (assuming we're on a NUMA-based machine)?
    ///
    /// Note that this might be a fake NUMA node, ie one lacking any hyper threads.
    #[inline(always)]
    pub fn is_a_numa_node(&self, numa_node: u8) -> bool {
        self.numa_node_folder_path(numa_node).exists()
    }
    /// A hyper thread file.
    #[inline(always)]
    pub fn hyper_thread_path(&self, hyper_thread: HyperThread,
                             file_name: &str) -> PathBuf {
        let mut path = self.hyper_thread_folder_path(hyper_thread);
        path.push(file_name);
        path
    }
    /// A NUMA node file.
    #[inline(always)]
    pub fn numa_node_path(&self, numa_node: u8, file_name: &str) -> PathBuf {
        let mut path = self.numa_node_folder_path(numa_node);
        path.push(file_name);
        path
    }
    /// A PCI device file.
    #[inline(always)]
    pub fn pci_device_path(&self, pci_device: (u32, u8, u8, u8),
                           file_name: &str) -> PathBuf {
        let mut path = self.pci_device_folder_path(pci_device);
        path.push(file_name);
        path
    }
    /// A path about all hyper threads.
    #[inline(always)]
    pub fn hyper_threads_path(&self, file_name: &str) -> PathBuf {
        let mut path = self.hyper_threads_parent_path();
        path.push(file_name);
        path
    }
    /// A path about all NUMA nodes.
    #[inline(always)]
    pub fn numa_nodes_path(&self, file_name: &str) -> PathBuf {
        let mut path = self.numa_nodes_parent_path();
        path.push(file_name);
        path
    }
    /// A path about all PCI devices.
    #[inline(always)]
    pub fn pci_devices_path(&self, file_name: &str) -> PathBuf {
        let mut path = self.pci_devices_parent_path();
        path.push(file_name);
        path
    }
    /// Rescans all PCI buses and devices.
    ///
    /// Errors are swallowed.
    #[inline(always)]
    pub fn rescan_all_pci_buses_and_devices(&self) -> io::Result<()> {
        let mut path = self.path();
        path.push("bus/pci/rescan");
        path.write_value(1)
    }
    #[inline(always)]
    pub(crate) fn workqueue_file_path(&self, file_name: &str) -> PathBuf {
        let mut path = self.path();
        path.push("devices/virtual/workqueue");
        path.push(file_name);
        path
    }
    #[inline(always)]
    pub(crate) fn read_global_hugepages_value(&self,
                                              huge_page_size: HugePageSize,
                                              file_name: &str)
     -> io::Result<u64> {
        self.global_hugepages_file_path(huge_page_size,
                                        file_name).read_value()
    }
    #[inline(always)]
    pub(crate) fn read_numa_hugepages_value(&self,
                                            huge_page_size: HugePageSize,
                                            numa_node: u8, file_name: &str)
     -> io::Result<u64> {
        self.numa_hugepages_file_path(huge_page_size, numa_node,
                                      file_name).read_value()
    }
    #[inline(always)]
    pub(crate) fn global_hugepages_file_path(&self,
                                             huge_page_size: HugePageSize,
                                             file_name: &str) -> PathBuf {
        let mut file_path = self.global_memory_folder_path();
        file_path.push($crate::fmt::format($crate::fmt::Arguments::new_v1_formatted(&["hugepages/hugepages-",
                                                                                      "kB"],
                                                                                    &match (&huge_page_size.size_in_kilo_bytes(),)
                                                                                         {
                                                                                         (arg0,)
                                                                                         =>
                                                                                         [$crate::fmt::ArgumentV1::new(arg0,
                                                                                                                       $crate::fmt::Display::fmt)],
                                                                                     },
                                                                                    &[$crate::fmt::rt::v1::Argument{position:
                                                                                                                        $crate::fmt::rt::v1::Position::At(0usize),
                                                                                                                    format:
                                                                                                                        $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                            ' ',
                                                                                                                                                        align:
                                                                                                                                                            $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                        flags:
                                                                                                                                                            0u32,
                                                                                                                                                        precision:
                                                                                                                                                            $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                        width:
                                                                                                                                                            $crate::fmt::rt::v1::Count::Implied,},}])));
        file_path.push(file_name);
        file_path
    }
    #[inline(always)]
    pub(crate) fn numa_hugepages_file_path(&self,
                                           huge_page_size: HugePageSize,
                                           numa_node: u8, file_name: &str)
     -> PathBuf {
        let mut file_path = self.numa_node_folder_path(numa_node);
        file_path.push($crate::fmt::format($crate::fmt::Arguments::new_v1_formatted(&["hugepages/hugepages-",
                                                                                      "kB"],
                                                                                    &match (&huge_page_size.size_in_kilo_bytes(),)
                                                                                         {
                                                                                         (arg0,)
                                                                                         =>
                                                                                         [$crate::fmt::ArgumentV1::new(arg0,
                                                                                                                       $crate::fmt::Display::fmt)],
                                                                                     },
                                                                                    &[$crate::fmt::rt::v1::Argument{position:
                                                                                                                        $crate::fmt::rt::v1::Position::At(0usize),
                                                                                                                    format:
                                                                                                                        $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                            ' ',
                                                                                                                                                        align:
                                                                                                                                                            $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                        flags:
                                                                                                                                                            0u32,
                                                                                                                                                        precision:
                                                                                                                                                            $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                        width:
                                                                                                                                                            $crate::fmt::rt::v1::Count::Implied,},}])));
        file_path.push(file_name);
        file_path
    }
    #[inline(always)]
    pub(crate) fn hyper_thread_folder_path(&self, hyper_thread: HyperThread)
     -> PathBuf {
        let into: u16 = hyper_thread.into();
        self.hyper_threads_path(&$crate::fmt::format($crate::fmt::Arguments::new_v1_formatted(&["cpu"],
                                                                                              &match (&into,)
                                                                                                   {
                                                                                                   (arg0,)
                                                                                                   =>
                                                                                                   [$crate::fmt::ArgumentV1::new(arg0,
                                                                                                                                 $crate::fmt::Display::fmt)],
                                                                                               },
                                                                                              &[$crate::fmt::rt::v1::Argument{position:
                                                                                                                                  $crate::fmt::rt::v1::Position::At(0usize),
                                                                                                                              format:
                                                                                                                                  $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                      ' ',
                                                                                                                                                                  align:
                                                                                                                                                                      $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                  flags:
                                                                                                                                                                      0u32,
                                                                                                                                                                  precision:
                                                                                                                                                                      $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                  width:
                                                                                                                                                                      $crate::fmt::rt::v1::Count::Implied,},}])))
    }
    #[inline(always)]
    pub(crate) fn numa_node_folder_path(&self, numa_node: u8) -> PathBuf {
        self.numa_nodes_path(&$crate::fmt::format($crate::fmt::Arguments::new_v1_formatted(&["node"],
                                                                                           &match (&numa_node,)
                                                                                                {
                                                                                                (arg0,)
                                                                                                =>
                                                                                                [$crate::fmt::ArgumentV1::new(arg0,
                                                                                                                              $crate::fmt::Display::fmt)],
                                                                                            },
                                                                                           &[$crate::fmt::rt::v1::Argument{position:
                                                                                                                               $crate::fmt::rt::v1::Position::At(0usize),
                                                                                                                           format:
                                                                                                                               $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                   ' ',
                                                                                                                                                               align:
                                                                                                                                                                   $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                               flags:
                                                                                                                                                                   0u32,
                                                                                                                                                               precision:
                                                                                                                                                                   $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                               width:
                                                                                                                                                                   $crate::fmt::rt::v1::Count::Implied,},}])))
    }
    #[inline(always)]
    pub(crate) fn pci_device_folder_path(&self, pci_device: (u32, u8, u8, u8))
     -> PathBuf {
        self.pci_devices_path(&$crate::fmt::format($crate::fmt::Arguments::new_v1_formatted(&["",
                                                                                              ":",
                                                                                              ":",
                                                                                              "."],
                                                                                            &match (&pci_device.0,
                                                                                                    &pci_device.1,
                                                                                                    &pci_device.2,
                                                                                                    &pci_device.3)
                                                                                                 {
                                                                                                 (arg0,
                                                                                                  arg1,
                                                                                                  arg2,
                                                                                                  arg3)
                                                                                                 =>
                                                                                                 [$crate::fmt::ArgumentV1::new(arg0,
                                                                                                                               $crate::fmt::LowerHex::fmt),
                                                                                                  $crate::fmt::ArgumentV1::new(arg1,
                                                                                                                               $crate::fmt::LowerHex::fmt),
                                                                                                  $crate::fmt::ArgumentV1::new(arg2,
                                                                                                                               $crate::fmt::LowerHex::fmt),
                                                                                                  $crate::fmt::ArgumentV1::new(arg3,
                                                                                                                               $crate::fmt::LowerHex::fmt)],
                                                                                             },
                                                                                            &[$crate::fmt::rt::v1::Argument{position:
                                                                                                                                $crate::fmt::rt::v1::Position::At(0usize),
                                                                                                                            format:
                                                                                                                                $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                    ' ',
                                                                                                                                                                align:
                                                                                                                                                                    $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                flags:
                                                                                                                                                                    8u32,
                                                                                                                                                                precision:
                                                                                                                                                                    $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                width:
                                                                                                                                                                    $crate::fmt::rt::v1::Count::Is(4usize),},},
                                                                                              $crate::fmt::rt::v1::Argument{position:
                                                                                                                                $crate::fmt::rt::v1::Position::At(1usize),
                                                                                                                            format:
                                                                                                                                $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                    ' ',
                                                                                                                                                                align:
                                                                                                                                                                    $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                flags:
                                                                                                                                                                    8u32,
                                                                                                                                                                precision:
                                                                                                                                                                    $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                width:
                                                                                                                                                                    $crate::fmt::rt::v1::Count::Is(2usize),},},
                                                                                              $crate::fmt::rt::v1::Argument{position:
                                                                                                                                $crate::fmt::rt::v1::Position::At(2usize),
                                                                                                                            format:
                                                                                                                                $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                    ' ',
                                                                                                                                                                align:
                                                                                                                                                                    $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                flags:
                                                                                                                                                                    8u32,
                                                                                                                                                                precision:
                                                                                                                                                                    $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                width:
                                                                                                                                                                    $crate::fmt::rt::v1::Count::Is(2usize),},},
                                                                                              $crate::fmt::rt::v1::Argument{position:
                                                                                                                                $crate::fmt::rt::v1::Position::At(3usize),
                                                                                                                            format:
                                                                                                                                $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                    ' ',
                                                                                                                                                                align:
                                                                                                                                                                    $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                                flags:
                                                                                                                                                                    8u32,
                                                                                                                                                                precision:
                                                                                                                                                                    $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                                width:
                                                                                                                                                                    $crate::fmt::rt::v1::Count::Is(1usize),},}])))
    }
    #[inline(always)]
    fn hyper_threads_parent_path(&self) -> PathBuf {
        let mut path = self.path();
        path.push("devices/system/cpu");
        path
    }
    #[inline(always)]
    fn numa_nodes_parent_path(&self) -> PathBuf {
        let mut path = self.path();
        path.push("devices/system/node");
        path
    }
    #[inline(always)]
    fn pci_devices_parent_path(&self) -> PathBuf {
        let mut path = self.path();
        path.push("bus/pci/devices");
        path
    }
    #[inline(always)]
    fn global_memory_folder_path(&self) -> PathBuf {
        let mut path = self.path();
        path.push("kernel/mm");
        path
    }
    #[inline(always)]
    fn path(&self) -> PathBuf { self.0.to_owned() }
}
/// A list of known virtual memory statistics related to NUMA nodes.
///
/// There are far more statistics than those listed here.
#[allow(missing_docs)]
#[structural_match]
pub enum VirtualMemoryStatisticName {

    /// Found in `/sys/devices/system/node/node<X>/vmstat` where `<X>` is a zero-based NUMA node number.
    NumberOFreePages,
    NumberOfBatchAllocatedPages,
    NumberOfInactiveAnonymousPages,
    NumberOfActiveAnonymousPages,
    NumberOfInactiveFilePages,
    NumberOfActiveFilePages,
    NumberOfUnevictablePages,
    NumberOfLockedPages,
    NumberOfAnonymousPages,
    NumberOfMappedPages,
    NumberOfFilePages,
    NumberOfDirtyPages,
    NumberOfWritebackPages,
    NumberOfReclaimableSlabPages,
    NumberOfUnreclaimableSlabPages,
    NumberOfPageTablePages,
    NumberOfKernelStackPages,
    NumberOfUnstablePages,
    NumberOfBouncePages,
    NumberOfVirtualMemoryWritePages,
    NumberOfVirtualMemoryImmediateReclaimPages,
    NumberOfWritebackTemporaryPages,
    NumberOfIsolatedAnonymousPages,
    NumberOfIsolatedFilePages,
    NumberOfShmemPages,
    NumberOfDirtiedPages,
    NumberOfWrittenPages,
    NumberOfAnonymousTransparentHugePages,
    NumberOfFreeCmaPages,

    /// Found in `/sys/devices/system/node/node<X>/vmstat` and `/sys/devices/system/node/node<X>/numastat` where `<X>` is a zero-based NUMA node number.
    NumaHit,
    NumaMiss,
    NumaForeign,
    NumaInterleaveHit,
    NumaLocalNode,
    NumaOtherNode,

    /// This is for all other possibilities.
    Unknown(Box<[u8]>),
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(missing_docs)]
impl $crate::fmt::Debug for VirtualMemoryStatisticName {
    fn fmt(&self, f: &mut $crate::fmt::Formatter) -> $crate::fmt::Result {
        match (&*self,) {
            (&VirtualMemoryStatisticName::NumberOFreePages,) => {
                let mut debug_trait_builder =
                    f.debug_tuple("NumberOFreePages");
                debug_trait_builder.finish()
            }
            (&VirtualMemoryStatisticName::NumberOfBatchAllocatedPages,) => {
                let mut debug_trait_builder =
                    f.debug_tuple("NumberOfBatchAllocatedPages");
                debug_trait_builder.finish()
            }
            (&VirtualMemoryStatisticName::NumberOfInactiveAnonymousPages,) =>
            {
                let mut debug_trait_builder =
                    f.debug_tuple("NumberOfInactiveAnonymousPages");
                debug_trait_builder.finish()
            }
            (&VirtualMemoryStatisticName::NumberOfActiveAnonymousPages,) => {
                let mut debug_trait_builder =
                    f.debug_tuple("NumberOfActiveAnonymousPages");
                debug_trait_builder.finish()
            }
            (&VirtualMemoryStatisticName::NumberOfInactiveFilePages,) => {
                let mut debug_trait_builder =
                    f.debug_tuple("NumberOfInactiveFilePages");
                debug_trait_builder.finish()
            }
            (&VirtualMemoryStatisticName::NumberOfActiveFilePages,) => {
                let mut debug_trait_builder =
                    f.debug_tuple("NumberOfActiveFilePages");
                debug_trait_builder.finish()
            }
            (&VirtualMemoryStatisticName::NumberOfUnevictablePages,) => {
                let mut debug_trait_builder =
                    f.debug_tuple("NumberOfUnevictablePages");
                debug_trait_builder.finish()
            }
            (&VirtualMemoryStatisticName::NumberOfLockedPages,) => {
                let mut debug_trait_builder =
                    f.debug_tuple("NumberOfLockedPages");
                debug_trait_builder.finish()
            }
            (&VirtualMemoryStatisticName::NumberOfAnonymousPages,) => {
                let mut debug_trait_builder =
                    f.debug_tuple("NumberOfAnonymousPages");
                debug_trait_builder.finish()
            }
            (&VirtualMemoryStatisticName::NumberOfMappedPages,) => {
                let mut debug_trait_builder =
                    f.debug_tuple("NumberOfMappedPages");
                debug_trait_builder.finish()
            }
            (&VirtualMemoryStatisticName::NumberOfFilePages,) => {
                let mut debug_trait_builder =
                    f.debug_tuple("NumberOfFilePages");
                debug_trait_builder.finish()
            }
            (&VirtualMemoryStatisticName::NumberOfDirtyPages,) => {
                let mut debug_trait_builder =
                    f.debug_tuple("NumberOfDirtyPages");
                debug_trait_builder.finish()
            }
            (&VirtualMemoryStatisticName::NumberOfWritebackPages,) => {
                let mut debug_trait_builder =
                    f.debug_tuple("NumberOfWritebackPages");
                debug_trait_builder.finish()
            }
            (&VirtualMemoryStatisticName::NumberOfReclaimableSlabPages,) => {
                let mut debug_trait_builder =
                    f.debug_tuple("NumberOfReclaimableSlabPages");
                debug_trait_builder.finish()
            }
            (&VirtualMemoryStatisticName::NumberOfUnreclaimableSlabPages,) =>
            {
                let mut debug_trait_builder =
                    f.debug_tuple("NumberOfUnreclaimableSlabPages");
                debug_trait_builder.finish()
            }
            (&VirtualMemoryStatisticName::NumberOfPageTablePages,) => {
                let mut debug_trait_builder =
                    f.debug_tuple("NumberOfPageTablePages");
                debug_trait_builder.finish()
            }
            (&VirtualMemoryStatisticName::NumberOfKernelStackPages,) => {
                let mut debug_trait_builder =
                    f.debug_tuple("NumberOfKernelStackPages");
                debug_trait_builder.finish()
            }
            (&VirtualMemoryStatisticName::NumberOfUnstablePages,) => {
                let mut debug_trait_builder =
                    f.debug_tuple("NumberOfUnstablePages");
                debug_trait_builder.finish()
            }
            (&VirtualMemoryStatisticName::NumberOfBouncePages,) => {
                let mut debug_trait_builder =
                    f.debug_tuple("NumberOfBouncePages");
                debug_trait_builder.finish()
            }
            (&VirtualMemoryStatisticName::NumberOfVirtualMemoryWritePages,) =>
            {
                let mut debug_trait_builder =
                    f.debug_tuple("NumberOfVirtualMemoryWritePages");
                debug_trait_builder.finish()
            }
            (&VirtualMemoryStatisticName::NumberOfVirtualMemoryImmediateReclaimPages,)
            => {
                let mut debug_trait_builder =
                    f.debug_tuple("NumberOfVirtualMemoryImmediateReclaimPages");
                debug_trait_builder.finish()
            }
            (&VirtualMemoryStatisticName::NumberOfWritebackTemporaryPages,) =>
            {
                let mut debug_trait_builder =
                    f.debug_tuple("NumberOfWritebackTemporaryPages");
                debug_trait_builder.finish()
            }
            (&VirtualMemoryStatisticName::NumberOfIsolatedAnonymousPages,) =>
            {
                let mut debug_trait_builder =
                    f.debug_tuple("NumberOfIsolatedAnonymousPages");
                debug_trait_builder.finish()
            }
            (&VirtualMemoryStatisticName::NumberOfIsolatedFilePages,) => {
                let mut debug_trait_builder =
                    f.debug_tuple("NumberOfIsolatedFilePages");
                debug_trait_builder.finish()
            }
            (&VirtualMemoryStatisticName::NumberOfShmemPages,) => {
                let mut debug_trait_builder =
                    f.debug_tuple("NumberOfShmemPages");
                debug_trait_builder.finish()
            }
            (&VirtualMemoryStatisticName::NumberOfDirtiedPages,) => {
                let mut debug_trait_builder =
                    f.debug_tuple("NumberOfDirtiedPages");
                debug_trait_builder.finish()
            }
            (&VirtualMemoryStatisticName::NumberOfWrittenPages,) => {
                let mut debug_trait_builder =
                    f.debug_tuple("NumberOfWrittenPages");
                debug_trait_builder.finish()
            }
            (&VirtualMemoryStatisticName::NumberOfAnonymousTransparentHugePages,)
            => {
                let mut debug_trait_builder =
                    f.debug_tuple("NumberOfAnonymousTransparentHugePages");
                debug_trait_builder.finish()
            }
            (&VirtualMemoryStatisticName::NumberOfFreeCmaPages,) => {
                let mut debug_trait_builder =
                    f.debug_tuple("NumberOfFreeCmaPages");
                debug_trait_builder.finish()
            }
            (&VirtualMemoryStatisticName::NumaHit,) => {
                let mut debug_trait_builder = f.debug_tuple("NumaHit");
                debug_trait_builder.finish()
            }
            (&VirtualMemoryStatisticName::NumaMiss,) => {
                let mut debug_trait_builder = f.debug_tuple("NumaMiss");
                debug_trait_builder.finish()
            }
            (&VirtualMemoryStatisticName::NumaForeign,) => {
                let mut debug_trait_builder = f.debug_tuple("NumaForeign");
                debug_trait_builder.finish()
            }
            (&VirtualMemoryStatisticName::NumaInterleaveHit,) => {
                let mut debug_trait_builder =
                    f.debug_tuple("NumaInterleaveHit");
                debug_trait_builder.finish()
            }
            (&VirtualMemoryStatisticName::NumaLocalNode,) => {
                let mut debug_trait_builder = f.debug_tuple("NumaLocalNode");
                debug_trait_builder.finish()
            }
            (&VirtualMemoryStatisticName::NumaOtherNode,) => {
                let mut debug_trait_builder = f.debug_tuple("NumaOtherNode");
                debug_trait_builder.finish()
            }
            (&VirtualMemoryStatisticName::Unknown(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("Unknown");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(missing_docs)]
impl $crate::clone::Clone for VirtualMemoryStatisticName {
    #[inline]
    fn clone(&self) -> VirtualMemoryStatisticName {
        match (&*self,) {
            (&VirtualMemoryStatisticName::NumberOFreePages,) =>
            VirtualMemoryStatisticName::NumberOFreePages,
            (&VirtualMemoryStatisticName::NumberOfBatchAllocatedPages,) =>
            VirtualMemoryStatisticName::NumberOfBatchAllocatedPages,
            (&VirtualMemoryStatisticName::NumberOfInactiveAnonymousPages,) =>
            VirtualMemoryStatisticName::NumberOfInactiveAnonymousPages,
            (&VirtualMemoryStatisticName::NumberOfActiveAnonymousPages,) =>
            VirtualMemoryStatisticName::NumberOfActiveAnonymousPages,
            (&VirtualMemoryStatisticName::NumberOfInactiveFilePages,) =>
            VirtualMemoryStatisticName::NumberOfInactiveFilePages,
            (&VirtualMemoryStatisticName::NumberOfActiveFilePages,) =>
            VirtualMemoryStatisticName::NumberOfActiveFilePages,
            (&VirtualMemoryStatisticName::NumberOfUnevictablePages,) =>
            VirtualMemoryStatisticName::NumberOfUnevictablePages,
            (&VirtualMemoryStatisticName::NumberOfLockedPages,) =>
            VirtualMemoryStatisticName::NumberOfLockedPages,
            (&VirtualMemoryStatisticName::NumberOfAnonymousPages,) =>
            VirtualMemoryStatisticName::NumberOfAnonymousPages,
            (&VirtualMemoryStatisticName::NumberOfMappedPages,) =>
            VirtualMemoryStatisticName::NumberOfMappedPages,
            (&VirtualMemoryStatisticName::NumberOfFilePages,) =>
            VirtualMemoryStatisticName::NumberOfFilePages,
            (&VirtualMemoryStatisticName::NumberOfDirtyPages,) =>
            VirtualMemoryStatisticName::NumberOfDirtyPages,
            (&VirtualMemoryStatisticName::NumberOfWritebackPages,) =>
            VirtualMemoryStatisticName::NumberOfWritebackPages,
            (&VirtualMemoryStatisticName::NumberOfReclaimableSlabPages,) =>
            VirtualMemoryStatisticName::NumberOfReclaimableSlabPages,
            (&VirtualMemoryStatisticName::NumberOfUnreclaimableSlabPages,) =>
            VirtualMemoryStatisticName::NumberOfUnreclaimableSlabPages,
            (&VirtualMemoryStatisticName::NumberOfPageTablePages,) =>
            VirtualMemoryStatisticName::NumberOfPageTablePages,
            (&VirtualMemoryStatisticName::NumberOfKernelStackPages,) =>
            VirtualMemoryStatisticName::NumberOfKernelStackPages,
            (&VirtualMemoryStatisticName::NumberOfUnstablePages,) =>
            VirtualMemoryStatisticName::NumberOfUnstablePages,
            (&VirtualMemoryStatisticName::NumberOfBouncePages,) =>
            VirtualMemoryStatisticName::NumberOfBouncePages,
            (&VirtualMemoryStatisticName::NumberOfVirtualMemoryWritePages,) =>
            VirtualMemoryStatisticName::NumberOfVirtualMemoryWritePages,
            (&VirtualMemoryStatisticName::NumberOfVirtualMemoryImmediateReclaimPages,)
            =>
            VirtualMemoryStatisticName::NumberOfVirtualMemoryImmediateReclaimPages,
            (&VirtualMemoryStatisticName::NumberOfWritebackTemporaryPages,) =>
            VirtualMemoryStatisticName::NumberOfWritebackTemporaryPages,
            (&VirtualMemoryStatisticName::NumberOfIsolatedAnonymousPages,) =>
            VirtualMemoryStatisticName::NumberOfIsolatedAnonymousPages,
            (&VirtualMemoryStatisticName::NumberOfIsolatedFilePages,) =>
            VirtualMemoryStatisticName::NumberOfIsolatedFilePages,
            (&VirtualMemoryStatisticName::NumberOfShmemPages,) =>
            VirtualMemoryStatisticName::NumberOfShmemPages,
            (&VirtualMemoryStatisticName::NumberOfDirtiedPages,) =>
            VirtualMemoryStatisticName::NumberOfDirtiedPages,
            (&VirtualMemoryStatisticName::NumberOfWrittenPages,) =>
            VirtualMemoryStatisticName::NumberOfWrittenPages,
            (&VirtualMemoryStatisticName::NumberOfAnonymousTransparentHugePages,)
            =>
            VirtualMemoryStatisticName::NumberOfAnonymousTransparentHugePages,
            (&VirtualMemoryStatisticName::NumberOfFreeCmaPages,) =>
            VirtualMemoryStatisticName::NumberOfFreeCmaPages,
            (&VirtualMemoryStatisticName::NumaHit,) =>
            VirtualMemoryStatisticName::NumaHit,
            (&VirtualMemoryStatisticName::NumaMiss,) =>
            VirtualMemoryStatisticName::NumaMiss,
            (&VirtualMemoryStatisticName::NumaForeign,) =>
            VirtualMemoryStatisticName::NumaForeign,
            (&VirtualMemoryStatisticName::NumaInterleaveHit,) =>
            VirtualMemoryStatisticName::NumaInterleaveHit,
            (&VirtualMemoryStatisticName::NumaLocalNode,) =>
            VirtualMemoryStatisticName::NumaLocalNode,
            (&VirtualMemoryStatisticName::NumaOtherNode,) =>
            VirtualMemoryStatisticName::NumaOtherNode,
            (&VirtualMemoryStatisticName::Unknown(ref __self_0),) =>
            VirtualMemoryStatisticName::Unknown($crate::clone::Clone::clone(&(*__self_0))),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(missing_docs)]
impl $crate::cmp::PartialEq for VirtualMemoryStatisticName {
    #[inline]
    fn eq(&self, other: &VirtualMemoryStatisticName) -> bool {
        {
            let __self_vi =
                unsafe { $crate::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { $crate::intrinsics::discriminant_value(&*other) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&VirtualMemoryStatisticName::Unknown(ref __self_0),
                     &VirtualMemoryStatisticName::Unknown(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    _ => true,
                }
            } else { false }
        }
    }
    #[inline]
    fn ne(&self, other: &VirtualMemoryStatisticName) -> bool {
        {
            let __self_vi =
                unsafe { $crate::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { $crate::intrinsics::discriminant_value(&*other) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&VirtualMemoryStatisticName::Unknown(ref __self_0),
                     &VirtualMemoryStatisticName::Unknown(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    _ => false,
                }
            } else { true }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(missing_docs)]
impl $crate::cmp::Eq for VirtualMemoryStatisticName {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        { let _: $crate::cmp::AssertParamIsEq<Box<[u8]>>; }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(missing_docs)]
impl $crate::cmp::PartialOrd for VirtualMemoryStatisticName {
    #[inline]
    fn partial_cmp(&self, other: &VirtualMemoryStatisticName)
     -> $crate::option::Option<$crate::cmp::Ordering> {
        {
            let __self_vi =
                unsafe { $crate::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { $crate::intrinsics::discriminant_value(&*other) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&VirtualMemoryStatisticName::Unknown(ref __self_0),
                     &VirtualMemoryStatisticName::Unknown(ref __arg_1_0)) =>
                    match $crate::cmp::PartialOrd::partial_cmp(&(*__self_0),
                                                               &(*__arg_1_0))
                        {
                        $crate::option::Option::Some($crate::cmp::Ordering::Equal)
                        =>
                        $crate::option::Option::Some($crate::cmp::Ordering::Equal),
                        cmp => cmp,
                    },
                    _ =>
                    $crate::option::Option::Some($crate::cmp::Ordering::Equal),
                }
            } else { __self_vi.partial_cmp(&__arg_1_vi) }
        }
    }
    #[inline]
    fn lt(&self, other: &VirtualMemoryStatisticName) -> bool {
        {
            let __self_vi =
                unsafe { $crate::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { $crate::intrinsics::discriminant_value(&*other) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&VirtualMemoryStatisticName::Unknown(ref __self_0),
                     &VirtualMemoryStatisticName::Unknown(ref __arg_1_0)) =>
                    $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0),
                                                                                           &(*__arg_1_0)),
                                                      $crate::cmp::Ordering::Greater)
                        == $crate::cmp::Ordering::Less,
                    _ => false,
                }
            } else { __self_vi.lt(&__arg_1_vi) }
        }
    }
    #[inline]
    fn le(&self, other: &VirtualMemoryStatisticName) -> bool {
        {
            let __self_vi =
                unsafe { $crate::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { $crate::intrinsics::discriminant_value(&*other) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&VirtualMemoryStatisticName::Unknown(ref __self_0),
                     &VirtualMemoryStatisticName::Unknown(ref __arg_1_0)) =>
                    $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0),
                                                                                           &(*__arg_1_0)),
                                                      $crate::cmp::Ordering::Greater)
                        != $crate::cmp::Ordering::Greater,
                    _ => true,
                }
            } else { __self_vi.le(&__arg_1_vi) }
        }
    }
    #[inline]
    fn gt(&self, other: &VirtualMemoryStatisticName) -> bool {
        {
            let __self_vi =
                unsafe { $crate::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { $crate::intrinsics::discriminant_value(&*other) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&VirtualMemoryStatisticName::Unknown(ref __self_0),
                     &VirtualMemoryStatisticName::Unknown(ref __arg_1_0)) =>
                    $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0),
                                                                                           &(*__arg_1_0)),
                                                      $crate::cmp::Ordering::Less)
                        == $crate::cmp::Ordering::Greater,
                    _ => false,
                }
            } else { __self_vi.gt(&__arg_1_vi) }
        }
    }
    #[inline]
    fn ge(&self, other: &VirtualMemoryStatisticName) -> bool {
        {
            let __self_vi =
                unsafe { $crate::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { $crate::intrinsics::discriminant_value(&*other) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&VirtualMemoryStatisticName::Unknown(ref __self_0),
                     &VirtualMemoryStatisticName::Unknown(ref __arg_1_0)) =>
                    $crate::option::Option::unwrap_or($crate::cmp::PartialOrd::partial_cmp(&(*__self_0),
                                                                                           &(*__arg_1_0)),
                                                      $crate::cmp::Ordering::Less)
                        != $crate::cmp::Ordering::Less,
                    _ => true,
                }
            } else { __self_vi.ge(&__arg_1_vi) }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(missing_docs)]
impl $crate::cmp::Ord for VirtualMemoryStatisticName {
    #[inline]
    fn cmp(&self, other: &VirtualMemoryStatisticName)
     -> $crate::cmp::Ordering {
        {
            let __self_vi =
                unsafe { $crate::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { $crate::intrinsics::discriminant_value(&*other) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&VirtualMemoryStatisticName::Unknown(ref __self_0),
                     &VirtualMemoryStatisticName::Unknown(ref __arg_1_0)) =>
                    match $crate::cmp::Ord::cmp(&(*__self_0), &(*__arg_1_0)) {
                        $crate::cmp::Ordering::Equal =>
                        $crate::cmp::Ordering::Equal,
                        cmp => cmp,
                    },
                    _ => $crate::cmp::Ordering::Equal,
                }
            } else { __self_vi.cmp(&__arg_1_vi) }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(missing_docs)]
impl $crate::hash::Hash for VirtualMemoryStatisticName {
    fn hash<__H: $crate::hash::Hasher>(&self, state: &mut __H) -> () {
        match (&*self,) {
            (&VirtualMemoryStatisticName::Unknown(ref __self_0),) => {
                $crate::hash::Hash::hash(&unsafe {
                                              $crate::intrinsics::discriminant_value(self)
                                          }, state);
                $crate::hash::Hash::hash(&(*__self_0), state)
            }
            _ => {
                $crate::hash::Hash::hash(&unsafe {
                                              $crate::intrinsics::discriminant_value(self)
                                          }, state)
            }
        }
    }
}
impl VirtualMemoryStatisticName {
    #[inline]
    pub(crate) fn parse(name: &[u8]) -> Self {
        use self::VirtualMemoryStatisticName::*;
        match name {
            b"nr_free_pages" => NumberOFreePages,
            b"nr_alloc_batch" => NumberOfBatchAllocatedPages,
            b"nr_inactive_anon" => NumberOfInactiveAnonymousPages,
            b"nr_active_anon" => NumberOfActiveAnonymousPages,
            b"nr_inactive_file" => NumberOfInactiveFilePages,
            b"nr_active_file" => NumberOfActiveFilePages,
            b"nr_unevictable" => NumberOfUnevictablePages,
            b"nr_mlock" => NumberOfLockedPages,
            b"nr_anon_pages" => NumberOfAnonymousPages,
            b"nr_mapped" => NumberOfMappedPages,
            b"nr_file_pages" => NumberOfFilePages,
            b"nr_dirty" => NumberOfDirtyPages,
            b"nr_writeback" => NumberOfWritebackPages,
            b"nr_slab_reclaimable" => NumberOfReclaimableSlabPages,
            b"nr_slab_unreclaimable" => NumberOfUnreclaimableSlabPages,
            b"nr_page_table_pages" => NumberOfPageTablePages,
            b"nr_kernel_stack" => NumberOfKernelStackPages,
            b"nr_unstable" => NumberOfUnstablePages,
            b"nr_bounce" => NumberOfBouncePages,
            b"nr_vmscan_write" => NumberOfVirtualMemoryWritePages,
            b"nr_vmscan_immediate_reclaim" =>
            NumberOfVirtualMemoryImmediateReclaimPages,
            b"nr_writeback_temp" => NumberOfWritebackTemporaryPages,
            b"nr_isolated_anon" => NumberOfIsolatedAnonymousPages,
            b"nr_isolated_file" => NumberOfIsolatedFilePages,
            b"nr_shmem" => NumberOfShmemPages,
            b"nr_dirtied" => NumberOfDirtiedPages,
            b"nr_written" => NumberOfWrittenPages,
            b"nr_anon_transparent_hugepages" =>
            NumberOfAnonymousTransparentHugePages,
            b"nr_free_cma" => NumberOfFreeCmaPages,
            b"numa_hit" => NumaHit,
            b"numa_miss" => NumaMiss,
            b"numa_foreign" => NumaForeign,
            b"interleave_hit" => NumaInterleaveHit,
            b"local_node" => NumaLocalNode,
            b"other_node" => NumaOtherNode,
            other@_ => Unknown(other.to_vec().into_boxed_slice()),
        }
    }
}
