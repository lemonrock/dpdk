// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// As of DPDK 18.02, there is a DPDK internal value, `RTE_SERVICE_NUM_MAX`, which defines the maximum number of services. It is set to 64.
///
/// The master logical core, even if it has the service role is not considered for running services on.
///
/// `rte_service_set_runstate_mapped_check` and `rte_service_run_iter_on_app_lcore` are not implemented, but may be useful to take over running a service on a non-service logical (or other) core.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Service(u32);

impl Into<u32> for Service
{
	#[inline(always)]
	fn into(self) -> u32
	{
		self.0 as u32
	}
}

impl PrintInformation for Service
{
	#[inline(always)]
	fn print_information_to_stream(&self, stream: *mut FILE)
	{
		if unlikely!(rte_service_dump(stream, self.0) != 0)
		{
			panic!("rte_service_dump() failed")
		}
	}
}

impl Service
{
	#[inline(always)]
	fn hack() -> &'static mut HashMap<Service, Box<Any>>
	{
		static mut HorribleHackToManageLifetimeOfRustRegisteredServices: *mut () = null_mut();
		
		if unlikely!(HorribleHackToManageLifetimeOfRustRegisteredServices == null_mut())
		{
			let map: Box<HashMap<Service, Box<Any>>> = Box::new(HashMap::with_capacity(64));
			unsafe { HorribleHackToManageLifetimeOfRustRegisteredServices = Box::into_raw(map) as *mut () }
		}
		
		unsafe { & mut * (HorribleHackToManageLifetimeOfRustRegisteredServices as *mut HashMap<Service, Box<Any>>) }
	}
	
	/// Registers a service with `name` and `function` to be called repeatedly.
	///
	/// Also tells DPDK that the service is ready-to-be-used.
	#[inline(always)]
	pub fn register<F: 'static + ServiceFunction>(name: &[u8], function: Box<F>, numa_node_choice: NumaNodeChoice) -> Self
	{
		assert!(name.len() <= 32, "name exceeds 32 characters");
		
		let mut service_identifier = unsafe { uninitialized() };
		
		let capabilities = if function.is_multi_thread_safe()
		{
			RTE_SERVICE_CAP_MT_SAFE
		}
		else
		{
			0
		};
		
		let callback_userdata = Box::into_raw(function) as *mut c_void;
		let mut service_specification = rte_service_spec
		{
			name: unsafe { zeroed() },
			callback: F::callback,
			callback_userdata,
			capabilities,
			socket_id: numa_node_choice.into(),
		};
		(&mut service_specification.name[0 .. name.len()]).copy_from_slice(unsafe { transmute(name) });
		let function = unsafe { Box::from_raw(callback_userdata) };
		
		let result = unsafe { rte_service_component_register(&service_specification, &mut service_identifier) };
		if likely!(result == 0)
		{
			if unlikely!(rte_service_component_runstate_set(service_identifier, 1) != 0)
			{
				panic!("rte_service_component_runstate_set failed");
			}
			let service = Service(service_identifier);
			let hack = Self::hack();
			hack.insert(service, function);
			service
		}
		else
		{
			panic!("rte_service_component_register failed")
		}
	}
	
	/// Unregisters a service.
	///
	/// Fails if the service is running or is not one registered by our code (as opposed to DPDK's).
	#[inline(always)]
	pub fn unregister(self) -> Result<(), ()>
	{
		let hack = Self::hack();
		
		match hack.remove(&self)
		{
			None => return Err(()),
			
			Some(data_to_drop_when_unregistered) =>
			{
				let result = unsafe { rte_service_component_unregister(self.into()) };
				if likely!(result == 0)
				{
					// we need to do something here...
					
					drop(data_to_drop_when_unregistered);
					Ok(())
				}
				else
				{
					match result
					{
						NegativeE::EBUSY =>
						{
							hack.insert(self, data_to_drop_when_unregistered);
							Err(())
						}
						_ => panic!("rte_service_component_unregister() failed"),
					}
				}
			}
		}
	}
	
	/// The number of services registered.
	#[inline(always)]
	pub fn number_of_services_registered() -> usize
	{
		(unsafe { rte_service_get_count() }) as usize
	}
	
	/// * Unmaps all services for all service cores
	/// * Stops all service cores
	/// * Does not change each services run state.
	#[inline(always)]
	pub fn reset_all_service_cores()
	{
		if unlikely!(rte_service_lcore_reset_all() != 0)
		{
			panic!("rte_service_lcore_reset_all failed")
		}
	}
	
	/// Find a service by its name.
	///
	/// A name may have a maximum length of 32 characters, excluding trailing ASCII NUL.
	#[inline(always)]
	pub fn find_by_name(name: &CStr) -> Option<Self>
	{
		let mut service_identifer = unsafe { uninitialized() };
		let result = unsafe { rte_service_get_by_name(name.as_ptr(), &mut service_identifer) };
		
		if likely!(result == 0)
		{
			Some(Service(service_identifer))
		}
		else if likely!(result == NegativeE::ENODEV)
		{
			None
		}
		else
		{
			panic!("Unexpected error '{}' from rte_service_get_by_name", result)
		}
	}
	
	/// Name of this service.
	#[inline(always)]
	pub fn name<'a>(self) -> &'a CStr
	{
		let name = unsafe { rte_service_get_name(self.into()) };
		debug_assert!(name.is_not_null(), "name was null implying an invalid service identifier");
		
		unsafe { CStr::from_ptr(name) }
	}
	
	/// Is this service multi-thread safe, ie can it run on more than one service logical core at the same time?
	#[inline(always)]
	pub fn has_capability_multi_thread_safe(self) -> bool
	{
		const RTE_SERVICE_CAP_MT_SAFE: u32 = 1 << 0;
		
		(unsafe { rte_service_probe_capability(self.into(), RTE_SERVICE_CAP_MT_SAFE) }) == 1
	}
	
	/// Enable running this service on the provided `logical_core`.
	///
	/// A service can run on more than one logical core; if `self.has_capability_multi_thread_safe()` is true, then it can run in parallel on more than one logical core.
	#[inline(always)]
	pub fn enable_running_on_service_logical_core(self, logical_core: LogicalCore)
	{
		debug_assert!(logical_core.is_role_service(), "logical_core '{}' is not a service core", logical_core);
		
		if unlikely!(rte_service_map_lcore_set(self.into(), logical_core.into(), 1) != 0)
		{
			panic!("rte_service_map_lcore_set failed")
		}
	}
	
	/// Start this service running.
	#[inline(always)]
	pub fn start_running(self)
	{
		if unlikely!(rte_service_runstate_set(self.into(), 1) != 0)
		{
			panic!("rte_service_runstate_set failed")
		}
	}
	
	/// Stop this service running.
	#[inline(always)]
	pub fn stop_running(self)
	{
		if unlikely!(rte_service_runstate_set(self.into(), 0) != 0)
		{
			panic!("rte_service_runstate_set failed")
		}
	}
	
	/// Query whether this service running.
	#[inline(always)]
	pub fn is_running(self) -> bool
	{
		match unsafe { rte_service_runstate_get(self.into()) }
		{
			1 => true,
			0 => false,
			_ => panic!("rte_service_runstate_get failed")
		}
	}
	
	/// Disable running this service on the provided `logical_core`.
	#[inline(always)]
	pub fn disable_running_on_service_logical_core(self, logical_core: LogicalCore)
	{
		debug_assert!(logical_core.is_role_service(), "logical_core '{}' is not a service core", logical_core);
		
		if unlikely!(rte_service_map_lcore_set(self.into(), logical_core.into(), 0) != 0)
		{
			panic!("rte_service_map_lcore_set failed")
		}
	}
	
	/// Enable the gathering of statistics.
	#[inline(always)]
	pub fn enable_gathering_statistics(self)
	{
		if unlikely!(rte_service_set_stats_enable(self.into(), 1) != 0)
		{
			panic!("rte_service_set_stats_enable failed")
		}
	}
	
	/// Disable the gathering of statistics.
	#[inline(always)]
	pub fn disable_gathering_statistics(self)
	{
		if unlikely!(rte_service_set_stats_enable(self.into(), 0) != 0)
		{
			panic!("rte_service_set_stats_enable failed")
		}
	}
	
	/// Get the number of cycles consumed by this service.
	///
	/// Returns zero unless `self.enable_gathering_statistics()` has been called.
	///
	/// Not sure why DPDK uses a u32 for this.
	#[inline(always)]
	pub fn get_number_of_cycles_consumed_statistic(self) -> u32
	{
		self.get_attribute(RTE_SERVICE_ATTR_CYCLES)
	}
	
	/// Get the number of call invocations of this service's handler function.
	///
	/// Returns zero unless `self.enable_gathering_statistics()` has been called.
	///
	/// Not sure why DPDK uses a u32 for this.
	#[inline(always)]
	pub fn get_number_of_call_invocations_statistic(self) -> u32
	{
		self.get_attribute(RTE_SERVICE_ATTR_CALL_COUNT)
	}
	
	/// Resets statistics.
	#[inline(always)]
	pub fn reset_statistics(self)
	{
		if unlikely!(rte_service_attr_reset_all(self.into()) == 0)
		{
			panic!("rte_service_attr_reset_all() failed")
		}
	}
	
	#[inline(always)]
	fn get_attribute(self, attribute_identifier: u32) -> u32
	{
		let mut value = unsafe { uninitialized() };
		if likely!(rte_service_attr_get(self.into(), attribute_identifier, &mut value) == 0)
		{
			value
		}
		else
		{
			panic!("rte_service_attr_get failed")
		}
	}
}
