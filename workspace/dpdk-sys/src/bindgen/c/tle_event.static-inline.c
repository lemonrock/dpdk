#include <tle_event.h>
#include "bindgen/c/tle_event.static-inline.h"

void rust_tle_evq_idle(struct tle_evq * evq, struct tle_event * ev[], uint32_t num)
{
	tle_evq_idle(evq, ev, num);
}

void rust_tle_event_raise(struct tle_event * ev)
{
	tle_event_raise(ev);
}

void rust_tle_event_idle(struct tle_event * ev)
{
	tle_event_idle(ev);
}

void rust_tle_event_down(struct tle_event * ev)
{
	tle_event_down(ev);
}

void rust_tle_event_active(struct tle_event * ev, enum tle_ev_state st)
{
	tle_event_active(ev, st);
}

int32_t rust_tle_evq_get(struct tle_evq * evq, const void * evd[], uint32_t num)
{
	return tle_evq_get(evq, evd, num);
}

enum tle_ev_state rust_tle_event_state(const struct tle_event * ev)
{
	return tle_event_state(ev);
}
