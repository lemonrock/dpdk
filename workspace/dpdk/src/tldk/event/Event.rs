// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Event<D>(*mut tle_event, PhantomData<D>);

impl<D> Drop for Event<D>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe
		{
			let dataToDrop = self.rehydrateRcForFfiData();
			::dpdk_sys::tle_event_free(self.0);
			drop(dataToDrop);
		}
	}
}

impl<D> Event<D>
{
	#[inline(always)]
	fn rehydrateRcForFfiData(&mut self) -> Rc<D>
	{
		unsafe
		{
			let ffiData = (*self.0).data as *mut D;
			Rc::from_raw(ffiData)
		}
	}
	
	#[inline(always)]
	pub(crate) fn new(opaqueFfiHandle: *mut tle_event) -> Event<D>
	{
		Event(opaqueFfiHandle, PhantomData)
	}
	
	#[inline(always)]
	pub(crate) fn opaqueFfiHandle(&mut self) -> *mut tle_event
	{
		self.0
	}
	
	#[inline(always)]
	pub(crate) fn forget(mut self) -> *mut tle_event
	{
		let result = self.opaqueFfiHandle();
		forget(self);
		result
	}
	
	#[inline(always)]
	pub(crate) fn dropEvent(opaqueFfiHandle: *mut tle_event)
	{
		if !opaqueFfiHandle.is_null()
		{
			drop(Event::<D>::new(opaqueFfiHandle));
		}
	}
	
	#[inline(always)]
	pub fn create(eventQueue: EventQueue, data: Rc<D>) -> Option<Event<D>>
	{
		let ffiData = Rc::into_raw(data);
		
		let result = unsafe { ::dpdk_sys::tle_event_alloc(eventQueue.0, ffiData as *const c_void) };

		if unlikely(result.is_null())
		{
			let dataToDrop = unsafe { Rc::from_raw(ffiData) };
			drop(dataToDrop);
			
			match unsafe { rust_rte_errno() }
			{
				E::ENOMEM => None,
			
				E::EINVAL => panic!("Supplied an invalid value"),
			
				illegal @ _ => panic!("Unexpected errno '{}' from tle_event_alloc()", illegal),
			}
		}
		else
		{
			Some(Self::new(result))
		}
	}
	
	#[inline(always)]
	pub fn data(&mut self) -> Rc<D>
	{
		let data = self.rehydrateRcForFfiData();
		let referencedCopy = data.clone();
		forget(data);
		referencedCopy
	}
	
	#[inline(always)]
	pub fn raise(&mut self)
	{
		unsafe { ::dpdk_sys::rust_tle_event_raise(self.0) };
	}
	
	#[inline(always)]
	pub fn down(&mut self)
	{
		unsafe { ::dpdk_sys::rust_tle_event_down(self.0) };
	}
	
	#[inline(always)]
	pub fn idle(&mut self)
	{
		unsafe { ::dpdk_sys::rust_tle_event_idle(self.0) };
	}
	
	#[inline(always)]
	pub fn state(&mut self) -> EventState
	{
		let ffiState = unsafe { ::dpdk_sys::rust_tle_event_state(self.0) };
		return unsafe { transmute(ffiState) }
	}
	
	#[inline(always)]
	pub fn active(&mut self, eventState: EventState)
	{
		unsafe { ::dpdk_sys::rust_tle_event_active(self.0, transmute(eventState))}
	}
}
