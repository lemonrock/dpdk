// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C, packed)]
pub struct tcp_hdr
{
	pub src_port: u16,
	pub dst_port: u16,
	pub sent_seq: u32,
	pub recv_ack: u32,
	pub data_off: u8,
	pub tcp_flags: u8,
	pub rx_win: u16,
	pub cksum: u16,
	pub tcp_urp: u16,
}

impl Default for tcp_hdr
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for tcp_hdr
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "tcp_hdr {{  }}")
	}
}
