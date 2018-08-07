// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Wraps the DPDK generic bus logic.
///
/// The DPDK API functios `rte_bus_find_by_device`, `rte_eal_hotplug_add` and `rte_eal_hotplug_remove` are not yet supported.
///
/// Currently known buses can be found by looking for use of `RTE_REGISTER_BUS` in DPDK source code.
/// Those in 18.02 are:-
///
/// * `FSL_DPAA_BUS`
/// * `fslmc`
/// * `pci`
/// * `vdev`
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct DpdkBus(NonNull<rte_bus>);

impl PrintAllInformation for DpdkBus
{
	#[inline(always)]
	fn print_information_to_stream(stream: *mut FILE)
	{
		unsafe { rte_bus_dump(stream) };
	}
}

impl DpdkBus
{
	/// Register a bus handler.
	#[inline(always)]
	pub fn register(&self)
	{
		unsafe { rte_bus_register(self.0.as_ptr()) }
	}
	
	/// Unregister a bus handler.
	#[inline(always)]
	pub fn unregister(&self)
	{
		unsafe { rte_bus_register(self.0.as_ptr()) }
	}
	
	/// Next bus.
	#[inline(always)]
	pub fn next(&self) -> Option<Self>
	{
		let next = unsafe { self.0.as_ref().next.tqe_next };
		if next.is_null()
		{
			None
		}
		else
		{
			Some(DpdkBus(unsafe { NonNull::new_unchecked(next) }))
		}
	}
	
	/// Get the name of this bus.
	#[inline(always)]
	pub fn name<'a>(&'a self) -> &'a CStr
	{
		unsafe { CStr::from_ptr(self.0.as_ref().name) }
	}
	
	/// Get the scan mode of this bus.
	#[inline(always)]
	pub fn scan_mode(&self) -> rte_bus_scan_mode
	{
		(unsafe { &self.0.as_ref().conf }).scan_mode
	}
	
	/// Get the IOMMU class of this bus.
	#[inline(always)]
	pub fn get_iommu_class(&self) -> rte_iova_mode
	{
		unsafe { (self.0.as_ref().get_iommu_class)() }
	}
	
	/// Get the common IOMMU class of devices bound on to buses available in the system.
	///
	/// Default is `PA` (physically addressed).
	#[inline(always)]
	pub fn get_common_iommu_class() -> rte_iova_mode
	{
		unsafe { rte_bus_get_iommu_class() }
	}
	
	/// Scan for all buses.
	#[inline(always)]
	pub fn scan_for_all_buses() -> Result<(), ()>
	{
		if unsafe { rte_bus_scan() } == 0
		{
			Ok(())
		}
		else
		{
			Err(())
		}
	}
	
	/// Probe and initialize all buses.
	///
	/// Call this after `Self::scan_for_all_buses()`.
	///
	/// Matches bus devices with drivers and initializes each driver.
	#[inline(always)]
	pub fn probe_and_initialize_all_buses() -> Result<(), ()>
	{
		if unsafe { rte_bus_probe() } == 0
		{
			Ok(())
		}
		else
		{
			Err(())
		}
	}
	
	/// Finds a bus by name.
	#[inline(always)]
	pub fn find_bus_by_name(name: ConstCStr) -> Option<Self>
	{
		let result = unsafe { rte_bus_find_by_name(name.as_ptr() )};
		if unlikely!(result.is_null())
		{
			None
		}
		else
		{
			Some(DpdkBus(unsafe { NonNull::new_unchecked(result) }))
		}
	}
	
	/// Finds a bus by comparison.
	///
	/// `comparator` should return `None` for no match and `Some(Ordering::Equal)` for a match where order does not matter.
	///
	/// Comparison short circuits (stops) when `Some(Ordering::Equal)` is returned.
	///
	/// `start_from` should be passed as `None` to start from the beginning of all known buses. Alternatively, pass a value seen during a previous comparison (note that the comparator is passed by immutable reference due to limitations in the DPDK API).
	#[inline(always)]
	pub fn find_bus_by_comparator<DpdkBusComparator: PartialOrd<NonNull<rte_bus>>>(comparator: &DpdkBusComparator, start_from: Option<Self>) -> Option<Self>
	{
		unsafe extern "C" fn callback<DpdkBusComparator: PartialOrd<NonNull<rte_bus>>>(bus: *const rte_bus, data: *const c_void) -> i32
		{
			use self::Ordering::*;
			
			debug_assert!(!bus.is_null(), "bus is null");
			debug_assert!(!data.is_null(), "data is null");
			
			let comparator = & * (data as *const DpdkBusComparator);
			match comparator.partial_cmp(&(NonNull::new_unchecked(bus as *mut _)))
			{
				None => !0,
				Some(Less) => -1,
				Some(Equal) => 0,
				Some(Greater) => 1,
			}
		}
		
		let start = match start_from
		{
			None => null(),
			Some(DpdkBus(non_null)) => non_null.as_ptr() as *const _,
		};
		
		let x: rte_bus_cmp_t = callback::<DpdkBusComparator>;
		
		let result = unsafe { rte_bus_find(start, x, comparator as *const _ as
			*const c_void) };
		if result.is_null()
		{
			None
		}
		else
		{
			Some(DpdkBus(unsafe { NonNull::new_unchecked(result) }))
		}
	}
}
