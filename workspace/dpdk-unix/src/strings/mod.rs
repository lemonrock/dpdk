// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use ::libc::c_char;
use ::rust_extra::unlikely;
use ::std::ffi::CStr;
use ::std::ffi::OsStr;
use ::std::os::unix::ffi::OsStrExt;
use ::std::path::PathBuf;


include!("c_string_pointer_to_path_buf.rs");
include!("c_string_pointer_to_string_with_replacements_if_any.rs");
