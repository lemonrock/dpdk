// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CryptoVirtualDeviceDriverName(&'static str);

impl DeviceDriverName for CryptoVirtualDeviceDriverName
{
	#[inline(always)]
	fn value(&self) -> &'static str
	{
		self.0
	}
}

impl CryptoVirtualDeviceDriverName
{
	pub const AesNiGcm: CryptoVirtualDeviceDriverName = CryptoVirtualDeviceDriverName("crypto_aesni_gcm");
	pub const AesNiMultiBuffer: CryptoVirtualDeviceDriverName = CryptoVirtualDeviceDriverName("crypto_aesni_mb");
	pub const Kasumi: CryptoVirtualDeviceDriverName = CryptoVirtualDeviceDriverName("crypto_kasumi");
	pub const Null: CryptoVirtualDeviceDriverName = CryptoVirtualDeviceDriverName("crypto_null");
	pub const Snow3g: CryptoVirtualDeviceDriverName = CryptoVirtualDeviceDriverName("crypto_now3g");
}
