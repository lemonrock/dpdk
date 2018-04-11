// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub type rte_mtr_meter_dscp_table_update_t = Option<unsafe extern "C" fn(dev: *mut rte_eth_dev, mtr_id: u32, dscp_table: *mut rte_mtr_color, error: *mut rte_mtr_error) -> c_int>;
