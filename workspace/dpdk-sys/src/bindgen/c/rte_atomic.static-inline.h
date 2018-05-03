#include <rte_atomic.h>

void rust_rte_smp_mb( );

void rust_rte_atomic32_inc(rte_atomic32_t * v);

void rust_rte_atomic32_dec(rte_atomic32_t * v);

void rust_rte_atomic16_inc(rte_atomic16_t * v);

void rust_rte_atomic16_dec(rte_atomic16_t * v);

uint32_t rust_rte_atomic32_exchange(volatile uint32_t * dst, uint32_t val);

uint16_t rust_rte_atomic16_exchange(volatile uint16_t * dst, uint16_t val);

int rust_rte_atomic32_test_and_set(rte_atomic32_t * v);

int rust_rte_atomic32_inc_and_test(rte_atomic32_t * v);

int rust_rte_atomic32_dec_and_test(rte_atomic32_t * v);

int rust_rte_atomic32_cmpset(volatile uint32_t * dst, uint32_t exp, uint32_t src);

int rust_rte_atomic16_test_and_set(rte_atomic16_t * v);

int rust_rte_atomic16_inc_and_test(rte_atomic16_t * v);

int rust_rte_atomic16_dec_and_test(rte_atomic16_t * v);

int rust_rte_atomic16_cmpset(volatile uint16_t * dst, uint16_t exp, uint16_t src);
