// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#include "lib.h"



// Static wrappers generated in bindgen-wrapper.conf.d/configuration.sh
#include "bindgen/c/lib.c"




int rust_rte_errno()
{
	return rte_errno;
}

void rust_rte_reset_errno()
{
	rte_errno = 0;
}



uint64_t rust_rte_timespec_to_ns(const struct timespec * ts)
{
	return rte_timespec_to_ns(ts);
}

struct timespec rust_rte_ns_to_timespec(uint64_t nsec)
{
	return rte_ns_to_timespec(nsec);
}



uint64_t rust_rte_rdtsc()
{
	return rte_rdtsc();
}

uint64_t rust_rte_rdtsc_precise()
{
	return rte_rdtsc_precise();
}

uint64_t rust_rte_get_tsc_cycles()
{
	return rte_get_tsc_cycles();
}

uint64_t rust_rte_get_timer_cycles()
{
	return rte_get_timer_cycles();
}

uint64_t rust_rte_get_timer_hz()
{
	return rte_get_timer_hz();
}

void rust_rte_delay_ms(unsigned ms)
{
	rte_delay_ms(ms);
}




struct rte_pci_device * rust_RTE_DEV_TO_PCI(struct rte_device * device)
{
	return RTE_DEV_TO_PCI(device);
}



struct rte_vdev_device * rust_RTE_DEV_TO_VDEV(struct rte_device * device)
{
	return RTE_DEV_TO_VDEV(device);
}
