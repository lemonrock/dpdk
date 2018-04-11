#include <tle_event.h>

void rust_tle_evq_idle(struct tle_evq * evq, struct tle_event * ev[], uint32_t num);

void rust_tle_event_raise(struct tle_event * ev);

void rust_tle_event_idle(struct tle_event * ev);

void rust_tle_event_down(struct tle_event * ev);

void rust_tle_event_active(struct tle_event * ev, enum tle_ev_state st);

int32_t rust_tle_evq_get(struct tle_evq * evq, const void * evd[], uint32_t num);

enum tle_ev_state rust_tle_event_state(const struct tle_event * ev);
