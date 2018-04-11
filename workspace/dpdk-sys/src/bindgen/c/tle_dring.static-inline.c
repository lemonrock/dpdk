#include <tle_dring.h>
#include "bindgen/c/tle_dring.static-inline.h"

void rust_tle_dring_reset(struct tle_dring * dr, uint32_t flags)
{
	tle_dring_reset(dr, flags);
}

void rust___tle_dring_copy_objs(const void * dst[], const void * const src[], uint32_t num)
{
	__tle_dring_copy_objs(dst, src, num);
}

uint32_t rust_tle_dring_sp_enqueue(struct tle_dring * dr, const void * const objs[], uint32_t nb_obj, struct tle_drb * drbs[], uint32_t * nb_drb)
{
	return tle_dring_sp_enqueue(dr, objs, nb_obj, drbs, nb_drb);
}

uint32_t rust_tle_dring_sc_dequeue(struct tle_dring * dr, const void * objs[], uint32_t nb_obj, struct tle_drb * drbs[], uint32_t * nb_drb)
{
	return tle_dring_sc_dequeue(dr, objs, nb_obj, drbs, nb_drb);
}

uint32_t rust_tle_dring_mp_enqueue(struct tle_dring * dr, const void * const objs[], uint32_t nb_obj, struct tle_drb * drbs[], uint32_t * nb_drb)
{
	return tle_dring_mp_enqueue(dr, objs, nb_obj, drbs, nb_drb);
}

uint32_t rust_tle_dring_mc_dequeue(struct tle_dring * dr, const void * objs[], uint32_t nb_obj, struct tle_drb * drbs[], uint32_t * nb_drb)
{
	return tle_dring_mc_dequeue(dr, objs, nb_obj, drbs, nb_drb);
}

uint32_t rust_tle_dring_enqueue(struct tle_dring * dr, const void * const objs[], uint32_t nb_obj, struct tle_drb * drbs[], uint32_t * nb_drb)
{
	return tle_dring_enqueue(dr, objs, nb_obj, drbs, nb_drb);
}

uint32_t rust_tle_dring_dequeue(struct tle_dring * dr, const void * objs[], uint32_t nb_obj, struct tle_drb * drbs[], uint32_t * nb_drb)
{
	return tle_dring_dequeue(dr, objs, nb_obj, drbs, nb_drb);
}

uint32_t rust_tle_dring_count(const struct tle_dring * dr)
{
	return tle_dring_count(dr);
}

uint32_t rust___tle_dring_enqueue(struct tle_dring * dr, uint32_t head, const void * const objs[], uint32_t nb_obj, struct tle_drb * drbs[], uint32_t nb_drb)
{
	return __tle_dring_enqueue(dr, head, objs, nb_obj, drbs, nb_drb);
}

uint32_t rust___tle_dring_dequeue(struct tle_dring * dr, uint32_t head, const void * objs[], uint32_t nb_obj, struct tle_drb * drbs[], uint32_t nb_drb)
{
	return __tle_dring_dequeue(dr, head, objs, nb_obj, drbs, nb_drb);
}

size_t rust_tle_drb_calc_size(uint32_t num)
{
	return tle_drb_calc_size(num);
}
