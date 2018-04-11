// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn tle_udp_rx_bulk(dev: *mut tle_dev, pkt: *mut *mut rte_mbuf, rp: *mut *mut rte_mbuf, rc: *mut i32, num: u16) -> u16;
	pub fn tle_udp_stream_close(s: *mut tle_stream) -> c_int;
	pub fn tle_udp_stream_get_param(s: *const tle_stream, prm: *mut tle_udp_stream_param) -> c_int;
	pub fn tle_udp_stream_open(ctx: *mut tle_ctx, prm: *const tle_udp_stream_param) -> *mut tle_stream;
	pub fn tle_udp_stream_recv(s: *mut tle_stream, pkt: *mut *mut rte_mbuf, num: u16) -> u16;
	pub fn tle_udp_stream_send(s: *mut tle_stream, pkt: *mut *mut rte_mbuf, num: u16, dst_addr: *const sockaddr) -> u16;
	pub fn tle_udp_tx_bulk(dev: *mut tle_dev, pkt: *mut *mut rte_mbuf, num: u16) -> u16;
}
