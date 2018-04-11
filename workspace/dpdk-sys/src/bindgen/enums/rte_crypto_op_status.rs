// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum rte_crypto_op_status
{
	RTE_CRYPTO_OP_STATUS_SUCCESS = 0,
	RTE_CRYPTO_OP_STATUS_NOT_PROCESSED = 1,
	RTE_CRYPTO_OP_STATUS_AUTH_FAILED = 2,
	RTE_CRYPTO_OP_STATUS_INVALID_SESSION = 3,
	RTE_CRYPTO_OP_STATUS_INVALID_ARGS = 4,
	RTE_CRYPTO_OP_STATUS_ERROR = 5,
}
