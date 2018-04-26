// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub const RTE_MBUF_DEFAULT_BUF_SIZE: uint16_t = 2176;
pub const RTE_MBUF_DEFAULT_DATAROOM: uint16_t = 2048;
pub const RTE_MBUF_DEFAULT_MEMPOOL_OPS: &'static [u8; 11usize] = b"ring_mp_mc\0";
pub const RTE_MBUF_MAX_NB_SEGS: uint16_t = 65535;
pub const RTE_MBUF_PRIV_ALIGN: u32 = 8;
pub const RTE_MBUF_REFCNT_ATOMIC: u32 = 1;
