// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	#[link_name = "\u{1}rte_power_freq_disable_turbo"] pub static mut rte_power_freq_disable_turbo: rte_power_freq_change_t;
	#[link_name = "\u{1}rte_power_freq_down"] pub static mut rte_power_freq_down: rte_power_freq_change_t;
	#[link_name = "\u{1}rte_power_freq_enable_turbo"] pub static mut rte_power_freq_enable_turbo: rte_power_freq_change_t;
	#[link_name = "\u{1}rte_power_freq_max"] pub static mut rte_power_freq_max: rte_power_freq_change_t;
	#[link_name = "\u{1}rte_power_freq_min"] pub static mut rte_power_freq_min: rte_power_freq_change_t;
	#[link_name = "\u{1}rte_power_freq_up"] pub static mut rte_power_freq_up: rte_power_freq_change_t;
	#[link_name = "\u{1}rte_power_freqs"] pub static mut rte_power_freqs: rte_power_freqs_t;
	#[link_name = "\u{1}rte_power_get_freq"] pub static mut rte_power_get_freq: rte_power_get_freq_t;
	#[link_name = "\u{1}rte_power_set_freq"] pub static mut rte_power_set_freq: rte_power_set_freq_t;
	#[link_name = "\u{1}rte_power_turbo_status"] pub static mut rte_power_turbo_status: rte_power_freq_change_t;
}
