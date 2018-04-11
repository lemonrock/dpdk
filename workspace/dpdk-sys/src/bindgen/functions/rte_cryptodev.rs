// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn rte_cryptodev_allocate_driver(crypto_drv: *mut cryptodev_driver, drv: *const rte_driver) -> u8;
	pub fn rte_cryptodev_callback_register(dev_id: u8, event: rte_cryptodev_event_type, cb_fn: rte_cryptodev_cb_fn, cb_arg: *mut c_void) -> c_int;
	pub fn rte_cryptodev_callback_unregister(dev_id: u8, event: rte_cryptodev_event_type, cb_fn: rte_cryptodev_cb_fn, cb_arg: *mut c_void) -> c_int;
	pub fn rte_cryptodev_close(dev_id: u8) -> c_int;
	pub fn rte_cryptodev_configure(dev_id: u8, config: *mut rte_cryptodev_config) -> c_int;
	pub fn rte_cryptodev_count() -> u8;
	pub fn rte_cryptodev_device_count_by_driver(driver_id: u8) -> u8;
	pub fn rte_cryptodev_devices_get(driver_name: *const c_char, devices: *mut u8, nb_devices: u8) -> u8;
	pub fn rte_cryptodev_driver_id_get(name: *const c_char) -> c_int;
	pub fn rte_cryptodev_driver_name_get(driver_id: u8) -> *const c_char;
	pub fn rte_cryptodev_get_aead_algo_enum(algo_enum: *mut rte_crypto_aead_algorithm, algo_string: *const c_char) -> c_int;
	pub fn rte_cryptodev_get_auth_algo_enum(algo_enum: *mut rte_crypto_auth_algorithm, algo_string: *const c_char) -> c_int;
	pub fn rte_cryptodev_get_cipher_algo_enum(algo_enum: *mut rte_crypto_cipher_algorithm, algo_string: *const c_char) -> c_int;
	pub fn rte_cryptodev_get_dev_id(name: *const c_char) -> c_int;
	pub fn rte_cryptodev_get_feature_name(flag: u64) -> *const c_char;
	pub fn rte_cryptodev_get_header_session_size() -> c_uint;
	pub fn rte_cryptodev_get_private_session_size(dev_id: u8) -> c_uint;
	pub fn rte_cryptodev_get_sec_ctx(dev_id: u8) -> *mut c_void;
	pub fn rte_cryptodev_info_get(dev_id: u8, dev_info: *mut rte_cryptodev_info);
	pub fn rte_cryptodev_name_get(dev_id: u8) -> *const c_char;
	pub fn rte_cryptodev_pmd_allocate(name: *const c_char, socket_id: c_int) -> *mut rte_cryptodev;
	pub fn rte_cryptodev_pmd_callback_process(dev: *mut rte_cryptodev, event: rte_cryptodev_event_type);
	pub fn rte_cryptodev_pmd_create(name: *const c_char, device: *mut rte_device, params: *mut rte_cryptodev_pmd_init_params) -> *mut rte_cryptodev;
	pub fn rte_cryptodev_pmd_create_dev_name(name: *mut c_char, dev_name_prefix: *const c_char) -> c_int;
	pub fn rte_cryptodev_pmd_destroy(cryptodev: *mut rte_cryptodev) -> c_int;
	pub fn rte_cryptodev_pmd_get_dev(dev_id: u8) -> *mut rte_cryptodev;
	pub fn rte_cryptodev_pmd_get_named_dev(name: *const c_char) -> *mut rte_cryptodev;
	pub fn rte_cryptodev_pmd_is_valid_dev(dev_id: u8) -> c_uint;
	pub fn rte_cryptodev_pmd_parse_input_args(params: *mut rte_cryptodev_pmd_init_params, args: *const c_char) -> c_int;
	pub fn rte_cryptodev_pmd_release_device(cryptodev: *mut rte_cryptodev) -> c_int;
	pub fn rte_cryptodev_queue_pair_attach_sym_session(dev_id: u8, qp_id: u16, session: *mut rte_cryptodev_sym_session) -> c_int;
	pub fn rte_cryptodev_queue_pair_count(dev_id: u8) -> u16;
	pub fn rte_cryptodev_queue_pair_detach_sym_session(dev_id: u8, qp_id: u16, session: *mut rte_cryptodev_sym_session) -> c_int;
	pub fn rte_cryptodev_queue_pair_setup(dev_id: u8, queue_pair_id: u16, qp_conf: *const rte_cryptodev_qp_conf, socket_id: c_int, session_pool: *mut rte_mempool) -> c_int;
	pub fn rte_cryptodev_queue_pair_start(dev_id: u8, queue_pair_id: u16) -> c_int;
	pub fn rte_cryptodev_queue_pair_stop(dev_id: u8, queue_pair_id: u16) -> c_int;
	pub fn rte_cryptodev_scheduler_load_user_scheduler(scheduler_id: u8, scheduler: *mut rte_cryptodev_scheduler) -> c_int;
	pub fn rte_cryptodev_scheduler_mode_get(scheduler_id: u8) -> rte_cryptodev_scheduler_mode;
	pub fn rte_cryptodev_scheduler_mode_set(scheduler_id: u8, mode: rte_cryptodev_scheduler_mode) -> c_int;
	pub fn rte_cryptodev_scheduler_option_get(scheduler_id: u8, option_type: rte_cryptodev_schedule_option_type, option: *mut c_void) -> c_int;
	pub fn rte_cryptodev_scheduler_option_set(scheduler_id: u8, option_type: rte_cryptodev_schedule_option_type, option: *mut c_void) -> c_int;
	pub fn rte_cryptodev_scheduler_ordering_get(scheduler_id: u8) -> c_int;
	pub fn rte_cryptodev_scheduler_ordering_set(scheduler_id: u8, enable_reorder: u32) -> c_int;
	pub fn rte_cryptodev_scheduler_slave_attach(scheduler_id: u8, slave_id: u8) -> c_int;
	pub fn rte_cryptodev_scheduler_slave_detach(scheduler_id: u8, slave_id: u8) -> c_int;
	pub fn rte_cryptodev_scheduler_slaves_get(scheduler_id: u8, slaves: *mut u8) -> c_int;
	pub fn rte_cryptodev_socket_id(dev_id: u8) -> c_int;
	pub fn rte_cryptodev_start(dev_id: u8) -> c_int;
	pub fn rte_cryptodev_stats_get(dev_id: u8, stats: *mut rte_cryptodev_stats) -> c_int;
	pub fn rte_cryptodev_stats_reset(dev_id: u8);
	pub fn rte_cryptodev_stop(dev_id: u8);
	pub fn rte_cryptodev_sym_capability_check_aead(capability: *const rte_cryptodev_symmetric_capability, key_size: u16, digest_size: u16, aad_size: u16, iv_size: u16) -> c_int;
	pub fn rte_cryptodev_sym_capability_check_auth(capability: *const rte_cryptodev_symmetric_capability, key_size: u16, digest_size: u16, iv_size: u16) -> c_int;
	pub fn rte_cryptodev_sym_capability_check_cipher(capability: *const rte_cryptodev_symmetric_capability, key_size: u16, iv_size: u16) -> c_int;
	pub fn rte_cryptodev_sym_capability_get(dev_id: u8, idx: *const rte_cryptodev_sym_capability_idx) -> *const rte_cryptodev_symmetric_capability;
	pub fn rte_cryptodev_sym_session_clear(dev_id: u8, sess: *mut rte_cryptodev_sym_session) -> c_int;
	pub fn rte_cryptodev_sym_session_create(mempool: *mut rte_mempool) -> *mut rte_cryptodev_sym_session;
	pub fn rte_cryptodev_sym_session_free(sess: *mut rte_cryptodev_sym_session) -> c_int;
	pub fn rte_cryptodev_sym_session_init(dev_id: u8, sess: *mut rte_cryptodev_sym_session, xforms: *mut rte_crypto_sym_xform, mempool: *mut rte_mempool) -> c_int;
}
