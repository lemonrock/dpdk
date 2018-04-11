#include <rte_atomic.h>
#include "bindgen/c/rte_atomic.static-inline.h"

void rust_rte_smp_mb( )
{
	rte_smp_mb();
}

void rust_rte_atomic32_inc(rte_atomic32_t * v)
{
	rte_atomic32_inc(v);
}

void rust_rte_atomic32_dec(rte_atomic32_t * v)
{
	rte_atomic32_dec(v);
}

void rust_rte_atomic16_inc(rte_atomic16_t * v)
{
	rte_atomic16_inc(v);
}

void rust_rte_atomic16_dec(rte_atomic16_t * v)
{
	rte_atomic16_dec(v);
}

int rust_rte_atomic32_test_and_set(rte_atomic32_t * v)
{
	return rte_atomic32_test_and_set(v);
}

int rust_rte_atomic32_inc_and_test(rte_atomic32_t * v)
{
	return rte_atomic32_inc_and_test(v);
}

int rust_rte_atomic32_dec_and_test(rte_atomic32_t * v)
{
	return rte_atomic32_dec_and_test(v);
}

int rust_rte_atomic32_cmpset(volatile uint32_t * dst, uint32_t exp, uint32_t src)
{
	return rte_atomic32_cmpset(dst, exp, src);
}

int rust_rte_atomic16_test_and_set(rte_atomic16_t * v)
{
	return rte_atomic16_test_and_set(v);
}

int rust_rte_atomic16_inc_and_test(rte_atomic16_t * v)
{
	return rte_atomic16_inc_and_test(v);
}

int rust_rte_atomic16_dec_and_test(rte_atomic16_t * v)
{
	return rte_atomic16_dec_and_test(v);
}

int rust_rte_atomic16_cmpset(volatile uint16_t * dst, uint16_t exp, uint16_t src)
{
	return rte_atomic16_cmpset(dst, exp, src);
}
