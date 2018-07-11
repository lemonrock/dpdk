// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use super::*;
#[cfg(any(target_os = "android", target_os = "linux"))] use super::android_linux::page_table::*;
use ::std::collections::HashMap;
use ::std::io::Error;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::std::io::SeekFrom;
use ::std::mem::size_of;
use ::std::num::ParseIntError;
use ::std::ops::Add;
use ::std::ops::AddAssign;
use ::std::ops::Sub;
use ::std::ops::SubAssign;
use ::std::ptr::NonNull;
use ::HugePageSize;


include!("MemoryInformationName.rs");
include!("MemoryInformationUnit.rs");
include!("MemoryInformation.rs");
include!("MemoryInformationParseError.rs");
include!("PhysicalAddress.rs");
include!("PhysicalPageFrameNumber.rs");
include!("VirtualAddress.rs");
include!("VirtualPageFrameNumber.rs");
