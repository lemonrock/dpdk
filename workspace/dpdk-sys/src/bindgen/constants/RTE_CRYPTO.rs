// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub const RTE_CRYPTODEV_ATTACHED: u32 = 1;
pub const RTE_CRYPTODEV_DETACHED: u32 = 0;
pub const RTE_CRYPTODEV_FF_ASYMMETRIC_CRYPTO: u32 = 2;
pub const RTE_CRYPTODEV_FF_CPU_AESNI: u32 = 64;
pub const RTE_CRYPTODEV_FF_CPU_ARM_CE: u32 = 2048;
pub const RTE_CRYPTODEV_FF_CPU_AVX2: u32 = 32;
pub const RTE_CRYPTODEV_FF_CPU_AVX512: u32 = 256;
pub const RTE_CRYPTODEV_FF_CPU_AVX: u32 = 16;
pub const RTE_CRYPTODEV_FF_CPU_NEON: u32 = 1024;
pub const RTE_CRYPTODEV_FF_CPU_SSE: u32 = 8;
pub const RTE_CRYPTODEV_FF_HW_ACCELERATED: u32 = 128;
pub const RTE_CRYPTODEV_FF_MBUF_SCATTER_GATHER: u32 = 512;
pub const RTE_CRYPTODEV_FF_SECURITY: u32 = 4096;
pub const RTE_CRYPTODEV_FF_SYMMETRIC_CRYPTO: u32 = 1;
pub const RTE_CRYPTODEV_FF_SYM_OPERATION_CHAINING: u32 = 4;
pub const RTE_CRYPTODEV_NAME_MAX_LEN: u32 = 64;
pub const RTE_CRYPTODEV_PMD_DEFAULT_MAX_NB_QUEUE_PAIRS: u32 = 8;
pub const RTE_CRYPTODEV_PMD_DEFAULT_MAX_NB_SESSIONS: u32 = 2048;
pub const RTE_CRYPTODEV_PMD_MAX_NB_QP_ARG: &'static [u8; 19usize] = b"max_nb_queue_pairs\0";
pub const RTE_CRYPTODEV_PMD_MAX_NB_SESS_ARG: &'static [u8; 16usize] = b"max_nb_sessions\0";
pub const RTE_CRYPTODEV_PMD_NAME_ARG: &'static [u8; 5usize] = b"name\0";
pub const RTE_CRYPTODEV_PMD_SOCKET_ID_ARG: &'static [u8; 10usize] = b"socket_id\0";
pub const RTE_CRYPTODEV_SCHEDULER_DESC_MAX_LEN: u32 = 256;
pub const RTE_CRYPTODEV_SCHEDULER_MAX_NB_SLAVES: u32 = 8;
pub const RTE_CRYPTODEV_SCHEDULER_MAX_NB_WORKER_CORES: u32 = 64;
pub const RTE_CRYPTODEV_SCHEDULER_NAME_MAX_LEN: u32 = 64;
pub const RTE_CRYPTO_MAX_DEVS: u32 = 64;
