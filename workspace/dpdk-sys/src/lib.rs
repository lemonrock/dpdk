// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]
#![feature(static_nobundle)]


#[cfg(any(target_os = "android", target_os = "linux"))] use ::libc::__u8;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::libc::__u16;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::libc::__u32;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::libc::__u64;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::libc::__s32;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::libc::cpu_set_t;
#[cfg(target_os = "freebsd")] use ::libc::cpuset_t;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::libc::FILE;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::libc::in_addr;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::libc::in6_addr;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::libc::int16_t;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::libc::iovec;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::libc::off_t;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::libc::pthread_t;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::libc::sockaddr_storage;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::libc::sockaddr;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::libc::size_t;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::libc::uint8_t;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::libc::uint16_t;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::libc::uint32_t;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::libc::uint64_t;
#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] use ::libc::timespec;


#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))] include!("bindgen/lib.rs");


pub type MARKER8 = uint8_t;

pub type MARKER64 = uint64_t;


#[repr(C, align(16))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct __m128i
{
	a: u64,
	b: u64,
}


///// Logical mapping of anonymous field.
//pub type cryptodev_driver_next = cryptodev_driver__bindgen_ty_1;
//
///// Logical mapping of anonymous field.
//pub type ip_frag_pkt_lru = ip_frag_pkt__bindgen_ty_1;
//
///// Logical mapping of anonymous field.
//pub type malloc_heap_free_head = malloc_heap__bindgen_ty_1;
//
///// Logical mapping of anonymous field.
//pub type rte_bbdev_op_cap_cap = rte_bbdev_op_cap__bindgen_ty_1;
//
///// Logical mapping of anonymous field.
//pub type rte_crypto_auth_xform_key = rte_crypto_auth_xform__bindgen_ty_1;
//
///// Logical mapping of anonymous field.
//pub type rte_crypto_auth_xform_iv = rte_crypto_auth_xform__bindgen_ty_2;
//
///// Logical mapping of anonymous field.
//pub type rte_crypto_aead_xform_key = rte_crypto_aead_xform__bindgen_ty_1;
//
///// Logical mapping of anonymous field.
//pub type rte_crypto_aead_xform_iv = rte_crypto_aead_xform__bindgen_ty_2;
//
///// Logical mapping of anonymous field.
//pub type rte_crypto_cipher_xform_key = rte_crypto_cipher_xform__bindgen_ty_1;
//
///// Logical mapping of anonymous field.
//pub type rte_crypto_cipher_xform_iv = rte_crypto_cipher_xform__bindgen_ty_2;
//
///// Logical mapping of anonymous field.
//pub type rte_crypto_sym_op__bindgen_ty_2_aead = rte_crypto_sym_op__bindgen_ty_2__bindgen_ty_1;
//
///// Logical mapping of anonymous field.
//pub type rte_crypto_sym_op__bindgen_ty_2_aead_data = rte_crypto_sym_op__bindgen_ty_2__bindgen_ty_1__bindgen_ty_1;
//
///// Logical mapping of anonymous field.
//pub type rte_crypto_sym_op__bindgen_ty_2_aead_digest = rte_crypto_sym_op__bindgen_ty_2__bindgen_ty_1__bindgen_ty_2;
//
///// Logical mapping of anonymous field.
//pub type rte_crypto_sym_op__bindgen_ty_2_aead_aad = rte_crypto_sym_op__bindgen_ty_2__bindgen_ty_1__bindgen_ty_3;
//
///// Logical mapping of anonymous field.
//pub type rte_crypto_sym_op__bindgen_ty_2__bindgen_ty_2_cipher = rte_crypto_sym_op__bindgen_ty_2__bindgen_ty_2__bindgen_ty_1;
//
///// Logical mapping of anonymous field.
//pub type rte_crypto_sym_op__bindgen_ty_2__bindgen_ty_2_auth = rte_crypto_sym_op__bindgen_ty_2__bindgen_ty_2__bindgen_ty_2;
//
///// Logical mapping of anonymous field.
//pub type rte_crypto_sym_op__bindgen_ty_2__bindgen_ty_2_cipher_data = rte_crypto_sym_op__bindgen_ty_2__bindgen_ty_2__bindgen_ty_1__bindgen_ty_1;
//
///// Logical mapping of anonymous field.
//pub type rte_cryptodev_info_sym = rte_cryptodev_info__bindgen_ty_1;
//
///// Logical mapping of anonymous field.
//pub type rte_cryptodev_symmetric_capability__bindgen_ty_1_auth = rte_cryptodev_symmetric_capability__bindgen_ty_1__bindgen_ty_1;
//
///// Logical mapping of anonymous field.
//pub type rte_cryptodev_symmetric_capability__bindgen_ty_1_cipher = rte_cryptodev_symmetric_capability__bindgen_ty_1__bindgen_ty_1;
//
///// Logical mapping of anonymous field.
//pub type rte_cryptodev_symmetric_capability__bindgen_ty_1_aead = rte_cryptodev_symmetric_capability__bindgen_ty_1__bindgen_ty_1;
//
///// Logical mapping of anonymous field.
//pub type rte_devargs_next = rte_devargs__bindgen_ty_1;
//
///// Logical mapping of anonymous field.
//pub type rte_device_next = rte_device__bindgen_ty_1;
//
///// Logical mapping of anonymous field.
//pub type rte_driver_next = rte_driver__bindgen_ty_1;
//
///// Logical mapping of anonymous field.
//pub type rte_eth_conf_rx_adv_conf = rte_eth_conf__bindgen_ty_1;
//
///// Logical mapping of anonymous field.
//pub type rte_eth_conf_tx_adv_conf = rte_eth_conf__bindgen_ty_2;
//
///// Logical mapping of anonymous field.
//pub type rte_eth_dcb_tc_queue_mapping_tc_rxq = rte_eth_dcb_tc_queue_mapping__bindgen_ty_1;
//
///// Logical mapping of anonymous field.
//pub type rte_eth_dcb_tc_queue_mapping_tc_txq = rte_eth_dcb_tc_queue_mapping__bindgen_ty_2;
//
///// Logical mapping of anonymous field.
//pub type rte_eth_fdir_filter_info_info = rte_eth_fdir_filter_info__bindgen_ty_1;
//
///// Logical mapping of anonymous field.
//pub type rte_eth_hash_filter_info_info = rte_eth_hash_filter_info__bindgen_ty_1;
//
///// Logical mapping of anonymous field.
//pub type rte_eth_rxtx_callback_fn_ = rte_eth_rxtx_callback__bindgen_ty_1;
//
///// Logical mapping of anonymous field.
//pub type rte_eth_tunnel_filter_conf_ip_addr = rte_eth_tunnel_filter_conf__bindgen_ty_1;
//
///// Logical mapping of anonymous field.
//pub type rte_eth_vmdq_dcb_conf_pool_map = rte_eth_vmdq_dcb_conf__bindgen_ty_1;
//
///// Logical mapping of anonymous field.
//pub type rte_eth_vmdq_rx_conf_pool_map = rte_eth_vmdq_rx_conf__bindgen_ty_1;
//
///// Logical mapping of anonymous field.
//pub type rte_fbk_hash_entry_entry = rte_fbk_hash_entry__bindgen_ty_1;
//
///// Logical mapping of anonymous field.
//pub type rte_mbuf_hash = rte_mbuf__bindgen_ty_4;
//
///// Logical mapping of anonymous field.
//pub type rte_mbuf_hash_fdir = rte_mbuf__bindgen_ty_4__bindgen_ty_1;
//
///// Logical mapping of anonymous field.
//pub type rte_mbuf_hash_sched = rte_mbuf__bindgen_ty_4__bindgen_ty_2;
//
///// Logical mapping of anonymous field.
//pub type rte_mempool_memhdr_next = rte_mempool_memhdr__bindgen_ty_1;
//
///// Logical mapping of anonymous field.
//pub type rte_mempool_objhdr_next = rte_mempool_objhdr__bindgen_ty_1;
//
///// Logical mapping of anonymous field.
//pub type rte_mtr_meter_profile__bindgen_ty_1_srtcm_rfc2697 = rte_mtr_meter_profile__bindgen_ty_1__bindgen_ty_1;
//
///// Logical mapping of anonymous field.
//pub type rte_mtr_meter_profile__bindgen_ty_1_trtcm_rfc2698 = rte_mtr_meter_profile__bindgen_ty_1__bindgen_ty_2;
//
///// Logical mapping of anonymous field.
//pub type rte_mtr_meter_profile__bindgen_ty_1_trtcm_rfc4115 = rte_mtr_meter_profile__bindgen_ty_1__bindgen_ty_3;
//
///// Logical mapping of anonymous field.
//pub type rte_pci_device_next = rte_pci_device__bindgen_ty_1;
//
///// Logical mapping of anonymous field.
//pub type rte_pci_driver_next = rte_pci_driver__bindgen_ty_1;
//
///// Logical mapping of anonymous field.
//pub type rte_security_capability__bindgen_ty_1_ipsec = rte_security_capability__bindgen_ty_1__bindgen_ty_1;
//
///// Logical mapping of anonymous field.
//pub type rte_security_capability__bindgen_ty_1_macsec = rte_security_capability__bindgen_ty_1__bindgen_ty_2;
//
///// Logical mapping of anonymous field.
//pub type rte_security_capability_idx__bindgen_ty_1_ipsec = rte_security_capability_idx__bindgen_ty_1__bindgen_ty_1;
//
///// Logical mapping of anonymous field.
//pub type rte_security_ipsec_tunnel_param__bindgen_ty_1_ipv4 = rte_security_ipsec_tunnel_param__bindgen_ty_1__bindgen_ty_1;
//
///// Logical mapping of anonymous field.
//pub type rte_security_ipsec_tunnel_param__bindgen_ty_1_ipv6 = rte_security_ipsec_tunnel_param__bindgen_ty_1__bindgen_ty_2;
//
///// Logical mapping of anonymous field.
//pub type rte_tailq_elem_next = rte_tailq_elem__bindgen_ty_1;
//
///// Logical mapping of anonymous field.
//pub type rte_tailq_entry_next = rte_tailq_entry__bindgen_ty_1;
//
///// Logical mapping of anonymous field.
//pub type rte_tm_level_capabilities__bindgen_ty_1_nonleaf = rte_tm_level_capabilities__bindgen_ty_1__bindgen_ty_1;
//
///// Logical mapping of anonymous field.
//pub type rte_tm_level_capabilities__bindgen_ty_1_leaf = rte_tm_level_capabilities__bindgen_ty_1__bindgen_ty_2;
//
///// Logical mapping of anonymous field.
//pub type rte_tm_node_capabilities__bindgen_ty_1_nonleaf = rte_tm_node_capabilities__bindgen_ty_1__bindgen_ty_1;
//
///// Logical mapping of anonymous field.
//pub type rte_tm_node_capabilities__bindgen_ty_1_leaf = rte_tm_node_capabilities__bindgen_ty_1__bindgen_ty_2;
//
///// Logical mapping of anonymous field.
//pub type rte_tm_node_params__bindgen_ty_1_nonleaf = rte_tm_node_params__bindgen_ty_1__bindgen_ty_1;
//
///// Logical mapping of anonymous field.
//pub type rte_tm_node_params__bindgen_ty_1_leaf = rte_tm_node_params__bindgen_ty_1__bindgen_ty_2;
//
///// Logical mapping of anonymous field.
//pub type rte_tm_node_params__bindgen_ty_1_leaf_wred = rte_tm_node_params__bindgen_ty_1__bindgen_ty_2__bindgen_ty_1;
//
///// Logical mapping of anonymous field.
//pub type rte_tm_node_stats_leaf = rte_tm_node_stats__bindgen_ty_1;
//
///// Logical mapping of anonymous field.
//pub type rte_vdev_device_next = rte_vdev_device__bindgen_ty_1;
//
///// Logical mapping of anonymous field.
//pub type rte_vdev_driver_next = rte_vdev_driver__bindgen_ty_1;
//
///// Logical mapping of anonymous field.
//pub type tle_dring_prod = tle_dring__bindgen_ty_1;
//
///// Logical mapping of anonymous field.
//pub type tle_dring_cons = tle_dring__bindgen_ty_2;
//
///// Logical mapping of anonymous field.
//pub type tle_event_ql = tle_event__bindgen_ty_1;
//
///// Logical mapping of anonymous field.
//pub type tle_evq_armed = tle_evq__bindgen_ty_1;
//
///// Logical mapping of anonymous field.
//pub type tle_evq_free = tle_evq__bindgen_ty_2;


// Not generated correctly: rte_event et al


// Remove: rte_timer_status__bindgen_ty_1

// Anonymous union fields with no name
// rte_avp_request__bindgen_ty_1
// rte_bbdev_op_turbo_dec__bindgen_ty_1
// rte_bbdev_op_turbo_enc__bindgen_ty_1
// rte_crypto_op__bindgen_ty_1
// rte_crypto_sym_xform__bindgen_ty_1
// rte_crypto_sym_op__bindgen_ty_1
// rte_crypto_sym_op__bindgen_ty_2
// rte_crypto_sym_op__bindgen_ty_2__bindgen_ty_2
// rte_cryptodev_capabilities__bindgen_ty_1
// rte_cryptodev_symmetric_capability__bindgen_ty_1
// rte_ipv4_tuple__bindgen_ty_1
// rte_ipv4_tuple__bindgen_ty_1__bindgen_ty_1
// rte_ipv6_tuple__bindgen_ty_1
// rte_ipv6_tuple__bindgen_ty_1__bindgen_ty_1
// rte_kni_request__bindgen_ty_1
// rte_mbuf__bindgen_ty_1
// rte_mbuf__bindgen_ty_2
// rte_mbuf__bindgen_ty_3
// rte_mbuf__bindgen_ty_5
// rte_mbuf__bindgen_ty_6
// rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1
// rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1
// rte_mbuf__bindgen_ty_4__bindgen_ty_1__bindgen_ty_1
// rte_mbuf__bindgen_ty_4__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1
// rte_mbuf__bindgen_ty_6__bindgen_ty_1
// rte_mempool_memhdr__bindgen_ty_2
// rte_mempool_objhdr__bindgen_ty_2
// rte_memseg__bindgen_ty_1
// rte_memseg__bindgen_ty_2
// rte_memzone__bindgen_ty_1
// rte_memzone__bindgen_ty_2
// rte_mtr_meter_profile__bindgen_ty_1
// rte_security_capability__bindgen_ty_1
// rte_security_capability_idx__bindgen_ty_1
// rte_security_ipsec_tunnel_param__bindgen_ty_1
// rte_security_session_conf__bindgen_ty_1
// rte_security_stats__bindgen_ty_1
// rte_tm_level_capabilities__bindgen_ty_1
// rte_tm_node_capabilities__bindgen_ty_1
// rte_tm_node_params__bindgen_ty_1
// vfio_eeh_pe_op__bindgen_ty_1
