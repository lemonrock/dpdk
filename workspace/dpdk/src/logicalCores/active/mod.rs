// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use ::serde::de::Deserialize;
use ::serde::de::Deserializer;
use ::serde::de::Visitor;
use ::serde::ser::Serialize;
use ::serde::ser::Serializer;
use ::std::collections::HashSet;
use ::std::ffi::CString;
use ::std::fmt;
use ::std::fs::File;
use ::std::hash::Hash;
use ::std::path::Path;
use ::std::io::Error;
use ::std::io::Read;
use ::std::num::ParseIntError;
use ::logicalCores::LogicalCore;
use ::logicalCores::MaximumLogicalCores;
use ::logicalCores::NumaSocketId;
use ::logicalCores::MaximumNumaSockets;


// Relies on MaximumLogicalCores == 256
pub use ::rust_extra::arrays::Array256 as LogicalCoresActiveArray;


include!("Active.rs");
include!("LogicalCoresActive.rs");
include!("NumaSocketsActive.rs");
include!("ListParseError.rs");
