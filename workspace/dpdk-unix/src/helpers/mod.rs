// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use ::libc::geteuid;
use ::nix::sys::signal::Signal;
use ::nix::sys::signal::SigSet;
use ::std::fmt::Display;
use ::std::env::var_os;
use ::std::error::Error;
use ::std::ffi::CStr;
use ::std::fs::File;
use ::std::fs::metadata;
use ::std::fs::OpenOptions;
use ::std::fs::Permissions;
use ::std::fs::set_permissions;
use ::std::io;
use ::std::io::ErrorKind;
use ::std::io::Read;
use ::std::io::Write;
use ::std::num::ParseIntError;
use ::std::os::unix::fs::PermissionsExt;
use ::std::path::Path;
use ::std::process::Command;
use ::std::process::Stdio;
use ::std::str::FromStr;


include!("assertEffectiveUserIsRoot.rs");
include!("blockNearlyAllSignals.rs");
include!("blockAllSignalsBarChild.rs");
include!("blockAllSignals.rs");
include!("getProgramName.rs");
include!("makeFileReadWriteAll.rs");
include!("makeFolderReadableAndExecutable.rs");
include!("newSafeCommand.rs");
include!("readHexadecimalValueWithPrefixFromFile.rs");
include!("readHexadecimalValueWithPrefixFromFile_u16.rs");
include!("readValueFromFile.rs");
include!("writeValueToFile.rs");
