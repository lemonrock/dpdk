#include <tle_dring.h>

void rust_tle_dring_reset(struct tle_dring * dr, uint32_t flags);

void rust___tle_dring_copy_objs(const void * dst[], const void * const src[], uint32_t num);

uint32_t rust_tle_dring_sp_enqueue(struct tle_dring * dr, const void * const objs[], uint32_t nb_obj, struct tle_drb * drbs[], uint32_t * nb_drb);

uint32_t rust_tle_dring_sc_dequeue(struct tle_dring * dr, const void * objs[], uint32_t nb_obj, struct tle_drb * drbs[], uint32_t * nb_drb);

uint32_t rust_tle_dring_mp_enqueue(struct tle_dring * dr, const void * const objs[], uint32_t nb_obj, struct tle_drb * drbs[], uint32_t * nb_drb);

uint32_t rust_tle_dring_mc_dequeue(struct tle_dring * dr, const void * objs[], uint32_t nb_obj, struct tle_drb * drbs[], uint32_t * nb_drb);

uint32_t rust_tle_dring_enqueue(struct tle_dring * dr, const void * const objs[], uint32_t nb_obj, struct tle_drb * drbs[], uint32_t * nb_drb);

uint32_t rust_tle_dring_dequeue(struct tle_dring * dr, const void * objs[], uint32_t nb_obj, struct tle_drb * drbs[], uint32_t * nb_drb);

uint32_t rust_tle_dring_count(const struct tle_dring * dr);

uint32_t rust___tle_dring_enqueue(struct tle_dring * dr, uint32_t head, const void * const objs[], uint32_t nb_obj, struct tle_drb * drbs[], uint32_t nb_drb);

uint32_t rust___tle_dring_dequeue(struct tle_dring * dr, uint32_t head, const void * objs[], uint32_t nb_obj, struct tle_drb * drbs[], uint32_t nb_drb);

size_t rust_tle_drb_calc_size(uint32_t num);
