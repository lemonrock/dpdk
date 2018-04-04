// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use ::rust_extra::u31;
use ::rust_extra::u5;
use ::logicalCores::NumaSocketId;
use ::devices::virtualDevices::VirtualDevice;
use ::devices::DeviceDriverName;
use ::devices::virtualDevices::VirtualDeviceName;


include!("AesNiMultiBufferCryptoVirtualDevice.rs");
include!("AesNiGcmCryptoVirtualDevice.rs");
include!("CryptoVirtualDevice.rs");
include!("CryptoVirtualDeviceDriverName.rs");
include!("CryptoVirtualDeviceName.rs");
include!("KasumiCryptoVirtualDevice.rs");
include!("NullCryptoVirtualDevice.rs");
include!("Snow3gCryptoVirtualDevice.rs");
