// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use ::libc::c_void;
use ::libc::ioctl;
use ::libc_extra::android_linux::linux::ethtool::ETHTOOL_BUSINFO_LEN;
use ::libc_extra::android_linux::linux::ethtool::ETHTOOL_GDRVINFO;
use ::libc_extra::android_linux::linux::ethtool::ethtool_drvinfo;
use ::libc_extra::android_linux::linux::sockios::SIOCETHTOOL;
use ::libc_extra::android_linux::net::if_::ifreq;
use ::std::ffi::CStr;
use ::std::mem::transmute;
use ::std::ptr::write;
use ::NumberOfBytesInPciAddressString;
use ::openSocketForIoCtl;
use ::closeSocketFileDescriptor;


include!("rawPciBusAddressForNetworkInterfaceIndex.rs");
