// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub const PKT_RX_EIP_CKSUM_BAD: uint64_t = 32;
pub const PKT_RX_FDIR: uint64_t = 4;
pub const PKT_RX_FDIR_FLX: uint64_t = 16384;
pub const PKT_RX_FDIR_ID: uint64_t = 8192;
pub const PKT_RX_IEEE1588_PTP: uint64_t = 512;
pub const PKT_RX_IEEE1588_TMST: uint64_t = 1024;
pub const PKT_RX_IP_CKSUM_BAD: uint64_t = 16;
pub const PKT_RX_IP_CKSUM_GOOD: uint64_t = 128;
pub const PKT_RX_IP_CKSUM_MASK: uint64_t = 144;
pub const PKT_RX_IP_CKSUM_NONE: uint64_t = 144;
pub const PKT_RX_IP_CKSUM_UNKNOWN: uint64_t = 0;
pub const PKT_RX_L4_CKSUM_BAD: uint64_t = 8;
pub const PKT_RX_L4_CKSUM_GOOD: uint64_t = 256;
pub const PKT_RX_L4_CKSUM_MASK: uint64_t = 264;
pub const PKT_RX_L4_CKSUM_NONE: uint64_t = 264;
pub const PKT_RX_L4_CKSUM_UNKNOWN: uint64_t = 0;
pub const PKT_RX_LRO: uint64_t = 65536;
pub const PKT_RX_QINQ: u32 = 1048576;
pub const PKT_RX_QINQ_STRIPPED: uint64_t = 32768;
pub const PKT_RX_RSS_HASH: uint64_t = 2;
pub const PKT_RX_SEC_OFFLOAD: uint64_t = 262144;
pub const PKT_RX_SEC_OFFLOAD_FAILED: uint64_t = 524288;
pub const PKT_RX_TIMESTAMP: uint64_t = 131072;
pub const PKT_RX_VLAN: u32 = 1;
pub const PKT_RX_VLAN_STRIPPED: uint64_t = 64;
