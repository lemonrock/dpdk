// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use super::*;
use ::std::mem::size_of;
use ::std::mem::uninitialized;
use ::std::ptr::NonNull;
use ::std::fs::File;
use ::std::io;
use ::std::io::prelude::*;
use ::std::io::SeekFrom;
use ::std::ops::Add;
use ::std::ops::AddAssign;
use ::std::ops::Sub;
use ::std::ops::SubAssign;


include!("PageMapEntry.rs");
include!("PhysicalAddress.rs");
include!("PhysicalPageFrameNumber.rs");
include!("VirtualAddress.rs");
include!("VirtualPageFrameNumber.rs");
