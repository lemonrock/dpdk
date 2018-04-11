// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum rte_crypto_auth_algorithm
{
	RTE_CRYPTO_AUTH_NULL = 1,
	RTE_CRYPTO_AUTH_AES_CBC_MAC = 2,
	RTE_CRYPTO_AUTH_AES_CMAC = 3,
	RTE_CRYPTO_AUTH_AES_GMAC = 4,
	RTE_CRYPTO_AUTH_AES_XCBC_MAC = 5,
	RTE_CRYPTO_AUTH_KASUMI_F9 = 6,
	RTE_CRYPTO_AUTH_MD5 = 7,
	RTE_CRYPTO_AUTH_MD5_HMAC = 8,
	RTE_CRYPTO_AUTH_SHA1 = 9,
	RTE_CRYPTO_AUTH_SHA1_HMAC = 10,
	RTE_CRYPTO_AUTH_SHA224 = 11,
	RTE_CRYPTO_AUTH_SHA224_HMAC = 12,
	RTE_CRYPTO_AUTH_SHA256 = 13,
	RTE_CRYPTO_AUTH_SHA256_HMAC = 14,
	RTE_CRYPTO_AUTH_SHA384 = 15,
	RTE_CRYPTO_AUTH_SHA384_HMAC = 16,
	RTE_CRYPTO_AUTH_SHA512 = 17,
	RTE_CRYPTO_AUTH_SHA512_HMAC = 18,
	RTE_CRYPTO_AUTH_SNOW3G_UIA2 = 19,
	RTE_CRYPTO_AUTH_ZUC_EIA3 = 20,
	RTE_CRYPTO_AUTH_LIST_END = 21,
}
