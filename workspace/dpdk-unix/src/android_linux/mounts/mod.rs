// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use ::errno::errno;
use ::libc::c_void;
use ::libc::FILE;
use ::libc::gid_t;
use ::libc::mode_t;
use ::libc::uid_t;
use ::libc_extra::android_linux::mntent::setmntent;
use ::libc_extra::android_linux::mntent::getmntent;
use ::libc_extra::android_linux::mntent::addmntent;
use ::libc_extra::android_linux::mntent::endmntent;
use ::libc_extra::android_linux::mntent::mntent;
use ::rust_extra::unlikely;
use ::std::collections::HashMap;
use ::std::ffi::CStr;
use ::std::ffi::CString;
use ::std::fs::File;
use ::std::io::BufReader;
use ::std::io::BufRead;
use ::std::path::Path;
use ::std::path::PathBuf;
use ::std::io;
use ::std::io::ErrorKind;
use ::syscall_alt::constants::E;
use ::HugePageSize;
use ::strings::cStringPointerToPathBuf;
use ::strings::pathToCString;
use ::strings::cStringPointerToStringWithReplacementsIfAny;


include!("FileSystemType.rs");
include!("HasNoAssociatedDevice.rs");
include!("HugePageMountSettings.rs");
include!("Mount.rs");
include!("MountFlags.rs");
include!("Mounts.rs");
include!("UnmountFlags.rs");
