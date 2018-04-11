// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	#[link_name = "\u{1}rte_table_acl_ops"] pub static mut rte_table_acl_ops: rte_table_ops;
	#[link_name = "\u{1}rte_table_array_ops"] pub static mut rte_table_array_ops: rte_table_ops;
	#[link_name = "\u{1}rte_table_hash_cuckoo_ops"] pub static mut rte_table_hash_cuckoo_ops: rte_table_ops;
	#[link_name = "\u{1}rte_table_hash_ext_ops"] pub static mut rte_table_hash_ext_ops: rte_table_ops;
	#[link_name = "\u{1}rte_table_hash_key16_ext_ops"] pub static mut rte_table_hash_key16_ext_ops: rte_table_ops;
	#[link_name = "\u{1}rte_table_hash_key16_lru_ops"] pub static mut rte_table_hash_key16_lru_ops: rte_table_ops;
	#[link_name = "\u{1}rte_table_hash_key32_ext_ops"] pub static mut rte_table_hash_key32_ext_ops: rte_table_ops;
	#[link_name = "\u{1}rte_table_hash_key32_lru_ops"] pub static mut rte_table_hash_key32_lru_ops: rte_table_ops;
	#[link_name = "\u{1}rte_table_hash_key8_ext_ops"] pub static mut rte_table_hash_key8_ext_ops: rte_table_ops;
	#[link_name = "\u{1}rte_table_hash_key8_lru_ops"] pub static mut rte_table_hash_key8_lru_ops: rte_table_ops;
	#[link_name = "\u{1}rte_table_hash_lru_ops"] pub static mut rte_table_hash_lru_ops: rte_table_ops;
	#[link_name = "\u{1}rte_table_lpm_ipv6_ops"] pub static mut rte_table_lpm_ipv6_ops: rte_table_ops;
	#[link_name = "\u{1}rte_table_lpm_ops"] pub static mut rte_table_lpm_ops: rte_table_ops;
	#[link_name = "\u{1}rte_table_stub_ops"] pub static mut rte_table_stub_ops: rte_table_ops;
}
