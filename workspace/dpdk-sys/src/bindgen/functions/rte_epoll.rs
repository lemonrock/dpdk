// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn rte_epoll_ctl(epfd: c_int, op: c_int, fd: c_int, event: *mut rte_epoll_event) -> c_int;
	pub fn rte_epoll_wait(epfd: c_int, events: *mut rte_epoll_event, maxevents: c_int, timeout: c_int) -> c_int;
}
