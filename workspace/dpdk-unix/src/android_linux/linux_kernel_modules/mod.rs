// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use super::super::ProcPath;
use ::errno::errno;
use ::libc::syscall;
use ::std::collections::HashSet;
use ::std::env::var_os;
use ::std::ffi::CString;
use ::std::fs::File;
use ::std::fs::OpenOptions;
use ::std::io::BufReader;
use ::std::io::BufRead;
use ::std::io;
use ::std::io::Error;
use ::std::io::ErrorKind;
use ::std::os::unix::io::AsRawFd;
use ::std::path::Path;
use ::std::path::PathBuf;
use ::std::process::Command;
use ::std::process::Stdio;
use ::assert_effective_user_id_is_root;
use ::syscall_alt::constants::E;
use ::syscall_alt::constants::SYS::SYS_finit_module;
use ::syscall_alt::constants::SYS::SYS_delete_module;


include!("modprobe.rs");
include!("ModProbeError.rs");
include!("LinuxKernelModulesList.rs");
include!("LinuxKernelModulesListParseError.rs");
