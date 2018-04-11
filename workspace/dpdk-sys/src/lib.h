// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#ifndef _GNU_SOURCE
#define _GNU_SOURCE
#endif

#ifndef _BSD_SOURCE
#define _BSD_SOURCE
#endif


// Static wrappers generated in bindgen-wrapper.conf.d/configuration.sh
#include "bindgen/c/lib.h"


#include <rte_errno.h>
int rust_rte_errno();
void rust_rte_reset_errno();


#include <rte_time.h>
uint64_t rust_rte_timespec_to_ns(const struct timespec * ts);
struct timespec rust_rte_ns_to_timespec(uint64_t nsec);


#include <rte_cycles.h>
uint64_t rust_rte_rdtsc();
uint64_t rust_rte_rdtsc_precise();
uint64_t rust_rte_get_tsc_cycles();
uint64_t rust_rte_get_timer_cycles();
uint64_t rust_rte_get_timer_hz();
void rust_rte_delay_ms(unsigned ms);


#include <rte_bus_pci.h>
struct rte_pci_device * rust_RTE_DEV_TO_PCI(struct rte_device * device);


#include <rte_mbuf.h>
void * rust_rte_pktmbuf_mtod(struct rte_mbuf * m);
void * rust_rte_pktmbuf_mtod_offset(struct rte_mbuf * m, uint16_t o);


#include <rte_memcpy.h>
void rust_rte_mov16(uint8_t * dst, const uint8_t * src);
void rust_rte_mov32(uint8_t * dst, const uint8_t * src);
void rust_rte_mov64(uint8_t * dst, const uint8_t * src);
void rust_rte_mov128(uint8_t * dst, const uint8_t * src);
void * rust_rte_memcpy_generic(void * dst, const void * src, size_t n);


#include <rte_mbuf.h>
int rust_rte_pktmbuf_write(const struct rte_mbuf * m, uint32_t off, uint32_t len, const void * buf);
