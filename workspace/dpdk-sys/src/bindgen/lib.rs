// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.



extern crate libc;


use ::std::clone::Clone;
use ::std::default::Default;
use ::std::fmt::Debug;
use ::std::fmt::Formatter;
use ::std::fmt::Result;
use ::std::marker::Copy;
use ::std::marker::PhantomData;
use ::std::mem::transmute;
use ::std::mem::zeroed;
use ::std::option::Option;
use ::std::ops::BitOr;
use ::std::ops::BitOrAssign;
use ::std::ops::BitAnd;
use ::std::ops::BitAndAssign;
use ::std::hash::Hash;
use ::std::hash::Hasher;
use ::std::cmp::PartialEq;
use ::std::cmp::Eq;
use ::std::slice::from_raw_parts;
use ::std::slice::from_raw_parts_mut;
use ::libc::c_char;
use ::libc::c_int;
use ::libc::c_longlong;
use ::libc::c_uint;
use ::libc::c_ulong;
use ::libc::c_ushort;
use ::libc::c_void;

#[link(name = "rte_acl", kind = "static-nobundle")]
#[link(name = "rte_bbdev", kind = "static-nobundle")]
#[link(name = "rte_bitratestats", kind = "static-nobundle")]
#[link(name = "rte_bus_pci", kind = "static-nobundle")]
#[link(name = "rte_bus_vdev", kind = "static-nobundle")]
#[link(name = "rte_cfgfile", kind = "static-nobundle")]
#[link(name = "rte_cmdline", kind = "static-nobundle")]
#[link(name = "rte_common_octeontx", kind = "static-nobundle")]
#[link(name = "rte_cryptodev", kind = "static-nobundle")]
#[link(name = "rte_distributor", kind = "static-nobundle")]
#[link(name = "rte_eal", kind = "static-nobundle")]
#[link(name = "rte_efd", kind = "static-nobundle")]
#[link(name = "rte_ethdev", kind = "static-nobundle")]
#[link(name = "rte_eventdev", kind = "static-nobundle")]
#[link(name = "rte_flow_classify", kind = "static-nobundle")]
#[link(name = "rte_gro", kind = "static-nobundle")]
#[link(name = "rte_gso", kind = "static-nobundle")]
#[link(name = "rte_hash", kind = "static-nobundle")]
#[link(name = "rte_ifcvf_vdpa", kind = "static-nobundle")]
#[link(name = "rte_ip_frag", kind = "static-nobundle")]
#[link(name = "rte_jobstats", kind = "static-nobundle")]
#[link(name = "rte_kni", kind = "static-nobundle")]
#[link(name = "rte_kvargs", kind = "static-nobundle")]
#[link(name = "rte_latencystats", kind = "static-nobundle")]
#[link(name = "rte_lpm", kind = "static-nobundle")]
#[link(name = "rte_mbuf", kind = "static-nobundle")]
#[link(name = "rte_member", kind = "static-nobundle")]
#[link(name = "rte_mempool", kind = "static-nobundle")]
#[link(name = "rte_mempool_bucket", kind = "static-nobundle")]
#[link(name = "rte_mempool_octeontx", kind = "static-nobundle")]
#[link(name = "rte_mempool_ring", kind = "static-nobundle")]
#[link(name = "rte_mempool_stack", kind = "static-nobundle")]
#[link(name = "rte_meter", kind = "static-nobundle")]
#[link(name = "rte_metrics", kind = "static-nobundle")]
#[link(name = "rte_net", kind = "static-nobundle")]
#[link(name = "rte_pci", kind = "static-nobundle")]
#[link(name = "rte_pdump", kind = "static-nobundle")]
#[link(name = "rte_pipeline", kind = "static-nobundle")]
#[link(name = "rte_pmd_af_packet", kind = "static-nobundle")]
#[link(name = "rte_pmd_ark", kind = "static-nobundle")]
#[link(name = "rte_pmd_avf", kind = "static-nobundle")]
#[link(name = "rte_pmd_avp", kind = "static-nobundle")]
#[link(name = "rte_pmd_axgbe", kind = "static-nobundle")]
#[link(name = "rte_pmd_bbdev_null", kind = "static-nobundle")]
#[link(name = "rte_pmd_bnxt", kind = "static-nobundle")]
#[link(name = "rte_pmd_bond", kind = "static-nobundle")]
#[link(name = "rte_pmd_crypto_scheduler", kind = "static-nobundle")]
#[link(name = "rte_pmd_cxgbe", kind = "static-nobundle")]
#[link(name = "rte_pmd_e1000", kind = "static-nobundle")]
#[link(name = "rte_pmd_ena", kind = "static-nobundle")]
#[link(name = "rte_pmd_enic", kind = "static-nobundle")]
#[link(name = "rte_pmd_failsafe", kind = "static-nobundle")]
#[link(name = "rte_pmd_fm10k", kind = "static-nobundle")]
#[link(name = "rte_pmd_i40e", kind = "static-nobundle")]
#[link(name = "rte_pmd_ixgbe", kind = "static-nobundle")]
#[link(name = "rte_pmd_kni", kind = "static-nobundle")]
#[link(name = "rte_pmd_lio", kind = "static-nobundle")]
#[link(name = "rte_pmd_mlx4", kind = "static-nobundle")]
#[link(name = "rte_pmd_mlx5", kind = "static-nobundle")]
#[link(name = "rte_pmd_null", kind = "static-nobundle")]
#[link(name = "rte_pmd_null_crypto", kind = "static-nobundle")]
#[link(name = "rte_pmd_octeontx", kind = "static-nobundle")]
#[link(name = "rte_pmd_octeontx_ssovf", kind = "static-nobundle")]
#[link(name = "rte_pmd_opdl_event", kind = "static-nobundle")]
#[link(name = "rte_pmd_qede", kind = "static-nobundle")]
#[link(name = "rte_pmd_ring", kind = "static-nobundle")]
#[link(name = "rte_pmd_sfc_efx", kind = "static-nobundle")]
#[link(name = "rte_pmd_skeleton_event", kind = "static-nobundle")]
#[link(name = "rte_pmd_skeleton_rawdev", kind = "static-nobundle")]
#[link(name = "rte_pmd_softnic", kind = "static-nobundle")]
#[link(name = "rte_pmd_sw_event", kind = "static-nobundle")]
#[link(name = "rte_pmd_tap", kind = "static-nobundle")]
#[link(name = "rte_pmd_thunderx_nicvf", kind = "static-nobundle")]
#[link(name = "rte_pmd_vdev_netvsc", kind = "static-nobundle")]
#[link(name = "rte_pmd_vhost", kind = "static-nobundle")]
#[link(name = "rte_pmd_virtio", kind = "static-nobundle")]
#[link(name = "rte_pmd_virtio_crypto", kind = "static-nobundle")]
#[link(name = "rte_pmd_vmxnet3_uio", kind = "static-nobundle")]
#[link(name = "rte_port", kind = "static-nobundle")]
#[link(name = "rte_power", kind = "static-nobundle")]
#[link(name = "rte_rawdev", kind = "static-nobundle")]
#[link(name = "rte_reorder", kind = "static-nobundle")]
#[link(name = "rte_ring", kind = "static-nobundle")]
#[link(name = "rte_sched", kind = "static-nobundle")]
#[link(name = "rte_security", kind = "static-nobundle")]
#[link(name = "rte_table", kind = "static-nobundle")]
#[link(name = "rte_timer", kind = "static-nobundle")]
#[link(name = "rte_vhost", kind = "static-nobundle")]
#[link(name = "tle_misc", kind = "static-nobundle")]
#[link(name = "tle_dring", kind = "static-nobundle")]
#[link(name = "tle_timer", kind = "static-nobundle")]
#[link(name = "tle_l4p", kind = "static-nobundle")]
extern "C"
{
}

include!("constants.rs");
include!("enums.rs");
include!("functions.rs");
include!("statics.rs");
include!("structs.rs");
include!("types.rs");
include!("unions.rs");
include!("opaques.rs");
