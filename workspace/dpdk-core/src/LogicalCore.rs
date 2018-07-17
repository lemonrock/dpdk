// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Logical core numbers start at zero.
///
/// Logical core numbers are not necessarily contiguous but usually are.
///
/// To get a contiguous range, use `self.index()`.
///
/// A logical core is equivalent to a hyper thread.
///
/// Not all logical cores may have been assigned to be used by DPDK, and, of those that have, they may have one of these roles:-
///
/// * Master
/// * Slave
///
/// And also:-
///
/// * Normal
/// * Service
///
/// Only one core of all cores in a process can be a Master.
///
/// A logical core belongs to a NUMA node.
///
/// DPDK 18.02 defaults to a maximum of 128 logical cores.
///
/// To iterate over known logical cores, use either `AllLogicalCoreIterator` or `SlaveLogicalCoreIterator`.
///
/// To find the logical core the running code is currently executing on, use `LogicalCoreChoice::current_logical_core()`.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
pub struct LogicalCore(u16);

impl Display for LogicalCore
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "{}", self.0)
	}
}

impl Into<u16> for LogicalCore
{
	#[inline(always)]
	fn into(self) -> u16
	{
		self.0 as u16
	}
}

impl Into<u32> for LogicalCore
{
	#[inline(always)]
	fn into(self) -> u32
	{
		self.0 as u32
	}
}

impl Into<u64> for LogicalCore
{
	#[inline(always)]
	fn into(self) -> u64
	{
		self.0 as u64
	}
}

impl Into<usize> for LogicalCore
{
	#[inline(always)]
	fn into(self) -> usize
	{
		self.0 as usize
	}
}

impl LogicalCore
{
	/// Maximum number of `LogicalCore`s.
	pub const Maximum: usize = RTE_MAX_LCORE;
	
	/// Creates a new instance.
	///
	/// Only valid after `rte_eal_init()` called.
	///
	/// Returns an error if this logical core is not for use by DPDK EAL.
	///
	/// Panics if this logical core equals or exceeds `Maximum`.
	#[inline(always)]
	pub fn from_u16(value: u16) -> Result<Self, ()>
	{
		if unlikely!(Self::is_invalid(value))
		{
			Err(())
		}
		else
		{
			Ok(LogicalCore(value))
		}
	}
	
	#[inline(always)]
	pub(crate) fn is_invalid(value: u16) -> bool
	{
		debug_assert!((value as usize) < Self::Maximum, "value '{}' exceeds Self::Maximum '{}'", value, Self::Maximum);
		
		Self::logical_core_global_configuration()[value as usize].core_index < 0
	}
	
	/// Current logical core cpu set.
	///
	/// From a DPDK thread-local static.
	#[inline(always)]
	pub fn current_logical_core_cpu_set() -> &'static mut rte_cpuset_t
	{
		unsafe { &mut per_lcore__cpuset }
	}
	
	/// Current logical core `errno`.
	///
	/// From a DPDK thread-local static.
	#[inline(always)]
	pub fn current_logical_core_error_number() -> i32
	{
		unsafe { per_lcore__rte_errno }
	}
	
	/// Current logical core thread affinity.
	///
	/// Ordinarily logical cores are bound 1:1 to one thread to a hyper threaded CPU core.
	#[inline(always)]
	pub fn current_logical_core_thread_affinity() -> rte_cpuset_t
	{
		let mut cpu_set = unsafe { uninitialized() };
		unsafe { rte_thread_get_affinity(&mut cpu_set) }
		cpu_set
	}
	
	/// Override current logical core thread affinity.
	///
	/// Ordinarily logical cores are bound 1:1 to one thread to a hyper threaded CPU core. This binding is set up be `rte_eal_init`.
	#[inline(always)]
	pub fn override_current_logical_core_thread_affinity(cpu_set: &mut rte_cpuset_t)
	{
		if unlikely!(rte_thread_set_affinity(cpu_set) != 0)
		{
			panic!("Could not set current logical core thread affinity");
		}
	}
	
	/// Gets the number of logical cores.
	#[inline(always)]
	pub fn number_of_logical_cores() -> usize
	{
		DpdkProcess::global_configuration().lcore_count as usize
	}
	
	/// Gets the number of logical cores.
	///
	/// Should be equal to or (usually) smaller than `Self::number_of_logical_cores()`.
	#[inline(always)]
	pub fn number_of_service_cores() -> usize
	{
		DpdkProcess::global_configuration().service_lcore_count as usize
	}
	
	/// Gets the master logical core.
	#[inline(always)]
	pub fn master() -> Self
	{
		let master = DpdkProcess::global_configuration().master_lcore;
		debug_assert!(master <= (::std::u16::MAX as u32), "master '{}' is larger than ::std::u16::MAX '{}'", master, ::std::u16::MAX);
		LogicalCore::from_u16(master as u16).unwrap()
	}
	
	/// The number of service cores.
	#[inline(always)]
	pub fn number_of_logical_cores_used_as_service_cores() -> usize
	{
		let result = unsafe { rte_service_lcore_count() };
		if likely!(result >= 0)
		{
			result as usize
		}
		else
		{
			panic!("rte_service_lcore_count failed")
		}
	}
	
	/// List of all current logical cores used as service cores.
	///
	/// Result is out-of-date if `add_to_logical_cores_used_as_service_cores()` or `remove_from_logical_cores_used_as_service_cores()` is called.
	///
	/// Size of result is the same as `self.number_of_logical_core_used_as_service_cores()`.
	#[inline(always)]
	pub fn list_service_logical_cores() -> Vec<LogicalCore>
	{
		let mut array: [u32; LogicalCore::Maximum] = unsafe { uninitialized() };
		
		let result = unsafe { rte_service_lcore_list(array.as_mut_ptr(), array.len() as u32) };
		if likely!(result >= 0)
		{
			let count = result as usize;
			let mut list = Vec::with_capacity(count);
			let mut index = 0;
			while index < count
			{
				list.push(LogicalCore::from_u16(array[index] as u16).unwrap());
				index += 1;
			}
			list
		}
		else
		{
			panic!("rte_service_lcore_list failed");
		}
	}
	
	/// Start power management.
	#[inline(always)]
	pub fn start_power_management(self) -> Result<LogicalCorePowerManagement, ()>
	{
		LogicalCorePowerManagement::start(self)
	}
	
	/// Gets if the logical core role is normal.
	#[inline(always)]
	pub fn is_role_normal(self) -> bool
	{
		self.logical_core_role() == rte_lcore_role_t::ROLE_RTE
	}
	
	/// Gets if the logical core role is service.
	///
	/// Some DPDK functionality (rte_timer threads) will only run on logical cores which are in the service role.
	#[inline(always)]
	pub fn is_role_service(self) -> bool
	{
		self.logical_core_role() == rte_lcore_role_t::ROLE_SERVICE
	}
	
	/// Gets the number of services running on this core if it is a service core.
	#[inline(always)]
	pub fn number_of_services_running_on_this_service_core(self) -> Option<usize>
	{
		if self.is_role_service()
		{
			let result = unsafe { rte_service_lcore_count_services(self.into()) };
			if likely!(result >= 0)
			{
				Some(result as usize)
			}
			else
			{
				panic!("rte_service_lcore_count_services() failed")
			}
		}
		else
		{
			None
		}
	}
	
	/// This core must be:-
	///
	/// * In the wait state  (error returned if not); use `self.stop_service_core()`;
	/// * Not already added (error returned if not)
	#[inline(always)]
	pub fn add_to_logical_cores_used_as_service_cores(self) -> Result<(), ()>
	{
		let result = unsafe { rte_service_lcore_add(self.into()) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else
		{
			match result
			{
				NegativeE::EBUSY | NegativeE::EALREADY => Err(()),
				NegativeE::EINVAL => panic!("EINVAL from rte_service_lcore_add()"),
				unexpected @ _ => panic!("Unexpected '{}' from rte_service_lcore_add()", unexpected),
			}
		}
	}
	
	/// This core must be:-
	///
	/// * In the wait state  (error returned if not); use `self.stop_service_core()`;
	/// * Already added (error returned if not)
	/// * A service core (panics if not)
	#[inline(always)]
	pub fn remove_from_logical_cores_used_as_service_cores(self) -> Result<(), ()>
	{
		debug_assert!(self.is_role_service(), "Is not a service core");
		
		let result = unsafe { rte_service_lcore_del(self.into()) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else
		{
			match result
			{
				NegativeE::EBUSY => Err(()),
				NegativeE::EINVAL => panic!("EINVAL from rte_service_lcore_del()"),
				unexpected @ _ => panic!("Unexpected '{}' from rte_service_lcore_del()", unexpected),
			}
		}
	}
	
	/// Panics if not a service core.
	pub fn start_service_core(self) -> Result<(), ()>
	{
		debug_assert!(self.is_role_service(), "Is not a service core");
		
		let result = unsafe { rte_service_lcore_start(self.into()) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else
		{
			match result
			{
				NegativeE::EINVAL => panic!("EINVAL from rte_service_lcore_start()"),
				unexpected @ _ => panic!("Unexpected '{}' from rte_service_lcore_start()", unexpected),
			}
		}
	}
	
	/// Panics if not a service core.
	///
	/// Returns an error if already stopped or is not in wait state (stop all services on this logical core first).
	pub fn stop_service_core(self) -> Result<(), ()>
	{
		debug_assert!(self.is_role_service(), "Is not a service core");
		
		let result = unsafe { rte_service_lcore_stop(self.into()) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else
		{
			match result
			{
				NegativeE::EBUSY | NegativeE::EALREADY => Err(()),
				NegativeE::EINVAL => panic!("EINVAL from rte_service_lcore_stop()"),
				unexpected @ _ => panic!("Unexpected '{}' from rte_service_lcore_stop()", unexpected),
			}
		}
	}
	
	/// Contiguous index.
	#[inline(always)]
	pub fn index(self) -> usize
	{
		let core_index = self.logical_core_configuration().core_index;
		debug_assert!(core_index >= 0, "logical core is not one configured for use with DPDK");
		core_index as usize
	}
	
	/// NUMA node of this core.
	#[inline(always)]
	pub fn numa_node(self) -> NumaNode
	{
		let socket_id = self.logical_core_configuration().socket_id;
		NumaNode::from_u32(socket_id)
	}
	
	/// POSIX thread associated with this logical core.
	///
	/// Ordinarily logical cores are bound 1:1 to one thread to a hyper threaded CPU core.
	#[inline(always)]
	pub fn thread(self) -> pthread_t
	{
		self.logical_core_configuration().thread_id
	}
	
	/// Set POSIX thread name for this logical core.
	///
	/// Ordinarily logical cores are bound 1:1 to one thread to a hyper threaded CPU core.
	///
	/// Many internal DPDK functions will set this, so it may not always be appropriate to override it.
	///
	/// Panics if can not be set.
	#[inline(always)]
	pub fn set_thread_name(self, name: &CStr)
	{
		if unlikely!(rte_thread_setname(self.thread(), name.as_ptr()) != 0)
		{
			panic!("Could not set thread name for logical core");
		}
	}
	
	/// An iterator over all logical cores.
	#[inline(always)]
	pub fn all_logical_cores() -> AllLogicalCoreIterator
	{
		Default::default()
	}
	
	/// An iterator over slave logical cores.
	#[inline(always)]
	pub fn slave_logical_cores() -> SlaveLogicalCoreIterator
	{
		Default::default()
	}
	
	/// An iterator over slave logical cores with service cores omitted.
	#[inline(always)]
	pub fn slave_logical_cores_without_service_cores() -> impl Iterator<Item=Self>
	{
		Self::slave_logical_cores().filter(|slave_logical_core| !slave_logical_core.is_role_service())
	}
	
	/// Gets if the logical core execution state.
	///
	/// **WARNING:** Can only be called by code currently running on the master logical core.
	#[inline(always)]
	pub fn execution_state(self) -> rte_lcore_state_t
	{
		Self::debug_assert_code_is_currently_running_on_the_master_logical_core();
		
		self.logical_core_configuration().state
	}
	
	/// Gets if the logical core execution state.
	///
	/// **WARNING:** Can only be called by code currently running on the master logical core.
	///
	/// In debug mode, will also panic if `self` is the master core.
	///
	/// Execution will fail and an error is returned if ths slave logical core is not in the wait state.
	#[inline(always)]
	pub fn execute_code_on_slave<F: SlaveLogicalCoreFunction>(self, function_to_execute_on_slave: Box<F>) -> Result<(), ()>
	{
		Self::debug_assert_code_is_currently_running_on_the_master_logical_core();
		debug_assert_ne!(&self, &Self::master(), "Can not wait for a slave when self is master logical core");
		
		unsafe extern "C" fn execute<F: SlaveLogicalCoreFunction>(arg1: *mut c_void) -> i32
		{
			debug_assert!(arg1.is_not_null(), "arg1 is null");
			
			let mut this = Box::from_raw(arg1 as *mut F);
			
			this.execute();
			
			0
		}
		
		let arg1 = Box::into_raw(function_to_execute_on_slave) as *mut c_void;
		
		match unsafe { rte_eal_remote_launch(execute::<F>, arg1, self.into()) }
		{
			0 => Ok(()),
			
			NegativeE::EBUSY =>
			{
				drop(unsafe { Box::from_raw(arg1) });
				
				Err(())
			}
			
			invalid @ _ => panic!("Invalid result from rte_eal_remote_launch '{}'", invalid),
		}
	}
	
	/// Blocks until all slave logical cores enter the wait state.
	///
	/// **WARNING:** Can only be called by code currently running on the master logical core.
	///
	/// This will also update slaves that are in the finished state to the wait state.
	///
	/// Uses a busy-loop with a CPU pause.
	///
	/// Note that we ignore the return values from the result of executing on the slave logical cores.
	#[inline(always)]
	pub fn block_until_all_slaves_are_in_the_wait_state()
	{
		Self::debug_assert_code_is_currently_running_on_the_master_logical_core();
		
		for slave_logical_core in Self::slave_logical_cores()
		{
			slave_logical_core.block_until_this_slave_is_in_the_wait_state()
		}
	}
	
	/// Blocks until this slave logical core enters the wait state.
	///
	/// **WARNING:** Can only be called by code currently running on the master logical core.
	///
	/// In debug mode, will also panic if `self` is the master core.
	///
	/// This will also update slaves that are in the finished state to the wait state.
	///
	/// Use a busy-loop with a CPU pause.
	///
	/// Note that we ignore the return value from the result of executing on a slave logical core.
	pub fn block_until_this_slave_is_in_the_wait_state(self)
	{
		Self::debug_assert_code_is_currently_running_on_the_master_logical_core();
		debug_assert_ne!(&self, &Self::master(), "Can not wait for a slave when self is master logical core");
		
		unsafe { rte_eal_wait_lcore(self.into()) };
	}
	
	#[inline(always)]
	fn debug_assert_code_is_currently_running_on_the_master_logical_core()
	{
		debug_assert_eq!(Self::master(), LogicalCoreChoice::current_logical_core().expect("current core is not a logical core"), "Code must be running on the master logical core to use this functionality");
	}
	
	#[inline(always)]
	fn logical_core_role(self) -> rte_lcore_role_t
	{
		let index: usize = self.into();
		DpdkProcess::global_configuration().lcore_role[index]
	}
	
	/// Global logical core configuration.
	///
	/// Only valid after `rte_eal_init()` called.
	///
	/// From a DPDK global static.
	#[inline(always)]
	fn logical_core_configuration(self) -> &'static mut lcore_config
	{
		let index: usize = self.into();
		&mut Self::logical_core_global_configuration()[index]
	}
	
	/// Global logical core configuration.
	///
	/// Only valid after `rte_eal_init()` called.
	///
	/// From a DPDK global static.
	#[inline(always)]
	fn logical_core_global_configuration() -> &'static mut [lcore_config; Self::Maximum]
	{
		unsafe { &mut lcore_config }
	}
}
