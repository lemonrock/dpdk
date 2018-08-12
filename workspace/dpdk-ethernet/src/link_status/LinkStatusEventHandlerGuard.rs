// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Link status event handler guard.
///
/// When this is dropped, the underlying event handler is dropped.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LinkStatusEventHandlerGuard<Handler: LinkStatusEventHandler>
{
	handler: NonNull<Handler>,
	ethernet_port_identifier: EthernetPortIdentifier,
}

impl<Handler: LinkStatusEventHandler> Drop for LinkStatusEventHandlerGuard<Handler>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		let handler = unsafe { Box::from_raw(self.handler.as_ptr()) };
		unsafe { rte_eth_dev_callback_unregister(self.ethernet_port_identifier.into(), Self::OurEventType, Self::link_up_or_down_events_callback, self.handler.as_ptr() as *mut _) };
		drop(handler)
	}
}

impl<Handler: LinkStatusEventHandler> LinkStatusEventHandlerGuard<Handler>
{
	const OurEventType: rte_eth_event_type = rte_eth_event_type::RTE_ETH_EVENT_INTR_LSC;
	
	/// Register a handler for link up or link down events.
	///
	/// The returned `EthernetPortLinkStatusEventHandlerGuard` guard, when dropped, will unregister the event handler.
	#[inline(always)]
	pub fn register(ethernet_port_identifier: EthernetPortIdentifier, handler: Handler) -> Self
	{
		let boxed_handled = Box::new(handler);
		let argument = Box::into_raw(boxed_handled);
		
		let result = unsafe { rte_eth_dev_callback_register(ethernet_port_identifier.into(), Self::OurEventType, Self::link_up_or_down_events_callback, argument as *mut _) };
		if likely!(result == 0)
		{
			return Self
			{
				handler: unsafe { NonNull::new_unchecked(argument) },
				ethernet_port_identifier,
			}
		}
		
		panic!("rte_eth_dev_callback_register failed '{}'", result)
	}
	
	unsafe extern "C" fn link_up_or_down_events_callback(ethernet_port_identifier: u16, event: rte_eth_event_type, cb_arg: *mut c_void, ret_param: *mut c_void) -> i32
	{
		debug_assert_eq!(event, Self::OurEventType, "event '{:?}' was not OurEventType", event);
		debug_assert!(!cb_arg.is_null(), "cb_arg is null");
		debug_assert!(ret_param.is_null(), "ret_param is not null");
		
		let mut link_status = uninitialized();
		rte_eth_link_get_nowait(ethernet_port_identifier, &mut link_status);
		
		let handler = &mut * (cb_arg as *mut Handler);
		let ethernet_port_identifier = EthernetPortIdentifier(ethernet_port_identifier);
		
		if Self::link_status_is_down(&link_status)
		{
			handler.link_has_gone_down(ethernet_port_identifier);
		}
		else
		{
			let (is_full_duplex, was_auto_negotiated, speed_in_megabits_per_second) = Self::link_status_is_up(&link_status);
			handler.link_has_come_up(ethernet_port_identifier, is_full_duplex, was_auto_negotiated, speed_in_megabits_per_second)
		}
		
		0
	}
	
	#[inline(always)]
	fn link_status_is_down(link_status: &rte_eth_link) -> bool
	{
		link_status.link_status() == 0
	}
	
	#[inline(always)]
	fn link_status_is_up(link_status: &rte_eth_link) -> (bool, bool, u32)
	{
		let is_full_duplex = link_status.link_duplex() == 1;
		let was_auto_negotiated = link_status.link_autoneg() == 1;
		let speed_in_megabits_per_second = link_status.link_speed;
		
		(is_full_duplex, was_auto_negotiated, speed_in_megabits_per_second)
	}
}
