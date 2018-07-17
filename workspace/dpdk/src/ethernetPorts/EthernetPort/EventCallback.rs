// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


// NOTE: This API should be considered highly unstable
impl EthernetPort
{
	#[inline(always)]
	fn privateRegisterEventCallback<E: EthernetPortEventCallback>(&self, eventType: rte_eth_event_type, ethernetPortEventCallback: &mut E) -> bool
	{
		let result = unsafe { rte_eth_dev_callback_register(self.portIdentifier(), eventType, E::asFunctionPointer(), ethernetPortEventCallback.asFunctionArgument()) };
		if likely!(result == 0)
		{
			true
		}
		else
		{
			match result
			{
				negative if negative < 0 => false,

				_ => panic!("rte_eth_dev_callback_register() returned illegal result '{}'", result),
			}
		}
	}

	/// TODO: Not really a complete solution. Strong liklihood of change - we ought to hold a reference to the callback - some sort of global state?
	/// TODO: Support multiple callbacks, deregistration (rte_eth_dev_callback_unregister) too
	/// TODO: Need to pass our actual ethernet port, not just portIdentifier
	/// Make sure you hold a reference to ethernetPortEventCallback that lives longer than self...
	#[inline(always)]
	pub fn registerLscInterruptEventCallback<E: EthernetPortEventCallback>(&self, ethernetPortEventCallback: &mut E) -> bool
	{
		self.privateRegisterEventCallback(rte_eth_event_type::RTE_ETH_EVENT_INTR_LSC, ethernetPortEventCallback)
	}

	/// TODO: Not really a complete solution. Strong liklihood of change - we ought to hold a reference to the callback
	/// TODO: Need to pass our actual ethernet port, not just portIdentifier
	/// Make sure you hold a reference to ethernetPortEventCallback that lives longer than self...
	#[inline(always)]
	pub fn registerQueueStateEventCallback<E: EthernetPortEventCallback>(&self, ethernetPortEventCallback: &mut E) -> bool
	{
		self.privateRegisterEventCallback(rte_eth_event_type::RTE_ETH_EVENT_QUEUE_STATE, ethernetPortEventCallback)
	}

	/// TODO: Not really a complete solution. Strong liklihood of change - we ought to hold a reference to the callback
	/// TODO: Need to pass our actual ethernet port, not just portIdentifier
	/// Make sure you hold a reference to ethernetPortEventCallback that lives longer than self...
	#[inline(always)]
	pub fn registerInterruptResetSentToPciVirtualFunctionOnPciPhysicalFunctionResetEventCallback<E: EthernetPortEventCallback>(&self, ethernetPortEventCallback: &mut E) -> bool
	{
		self.privateRegisterEventCallback(rte_eth_event_type::RTE_ETH_EVENT_INTR_RESET, ethernetPortEventCallback)
	}
}
