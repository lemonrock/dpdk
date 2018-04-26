#include <rte_bus_vdev.h>
#include "bindgen/c/rte_bus_vdev.static-inline.h"

const char * rust_rte_vdev_device_name(const struct rte_vdev_device * dev)
{
	return rte_vdev_device_name(dev);
}

const char * rust_rte_vdev_device_args(const struct rte_vdev_device * dev)
{
	return rte_vdev_device_args(dev);
}
