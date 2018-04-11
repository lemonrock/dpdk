// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub type rte_rx_callback_fn = Option<unsafe extern "C" fn(port_id: u16, queue: u16, pkts: *mut *mut rte_mbuf, nb_pkts: u16, max_pkts: u16, user_param: *mut c_void) -> u16>;
