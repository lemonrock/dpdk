// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


serde_pub_enum_u8!
(
	IpV4MaskBits
	{
		_1 = 1,
		_2 = 2,
		_3 = 3,
		_4 = 4,
		_5 = 5,
		_6 = 6,
		_7 = 7,
		_8 = 8,
		_9 = 9,
		_10 = 10,
		_11 = 11,
		_12 = 12,
		_13 = 13,
		_14 = 14,
		_15 = 15,
		_16 = 16,
		_17 = 17,
		_18 = 18,
		_19 = 19,
		_20 = 20,
		_21 = 21,
		_22 = 22,
		_23 = 23,
		_24 = 24,
		_25 = 25,
		_26 = 26,
		_27 = 27,
		_28 = 28,
		_29 = 29,
		_30 = 30,
		_31 = 31,
		_32 = 32,
	}
);

impl MaskBits for IpV4MaskBits
{
	type IpHostAddress = IpV4HostAddress;
}

impl IpV4MaskBits
{
	#[inline(always)]
	pub fn asMask(&self) -> u32
	{
		!(0xFFFFFFFF >> (*self as u32))
	}
}
