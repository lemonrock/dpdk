// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


impl EthernetPort
{
	#[inline(always)]
	pub fn numberOfAttachedPorts() -> usize
	{
		(unsafe { rte_eth_dev_count() }) as usize
	}
	
	#[inline(always)]
	pub fn new_vec_with_capacity_for_all_attached_ports<E>() -> Vec<E>
	{
		Vec::with_capacity(Self::numberOfAttachedPorts())
	}
	
	#[inline(always)]
	fn isAttachedPort(portIdentifier: u8) -> bool
	{
		// rte_eth_dev_is_valid_port() also checks that portIdentifier < RTE_MAX_PORTS
		isTrue(unsafe { rte_eth_dev_is_valid_port(portIdentifier) })
	}
	
	pub fn allAttachedEthernetPorts() -> Vec<EthernetPort>
	{
		let mut list: Vec<EthernetPort> = Self::new_vec_with_capacity_for_all_attached_ports();
		
		let mut portIdentifier = 0;
		while portIdentifier < Self::MaximumEthernetPortsU8
		{
			if unlikely(Self::isAttachedPort(portIdentifier))
			{
				list.push(EthernetPort
				{
					portIdentifier,
				});
			}
			portIdentifier += 1;
		}
		
		list
	}
	
	pub fn allEthernetPortsExcludingBondedSlavesAndBondedSlaves() -> (HashSet<EthernetPort>, HashSet<EthernetPort>)
	{
		let allEthernetPorts = HashSet::from_iter(Self::allAttachedEthernetPorts());
		
		let mut allBondedSlaves = HashSet::with_capacity(allEthernetPorts.len());
		
		for ethernetPort in allEthernetPorts.iter()
		{
			if let Some(bondedEthernetPort) = ethernetPort.asBondedEthernetPort()
			{
				for slave in bondedEthernetPort.getAllSlaves()
				{
					assert!(ethernetPort.ne(&slave), "ethernetPort '{}' is a slave of itself!", ethernetPort);
					let original = allBondedSlaves.insert(slave);
					assert!(original, "Duplicate slave ethernet port '{}'", slave);
					
				}
			}
		}
		
		(allEthernetPorts.intersection(&allBondedSlaves).cloned().collect(), allBondedSlaves)
	}
	
	/// devicePciAddressOrName examples: 0000:01:00.0  and  eth_pcap0
	#[inline(always)]
	pub fn attach(devicePciAddressOrName: &str) -> Result<EthernetPort, c_int>
	{
		let devicePciAddressOrNameCStr = CString::new(devicePciAddressOrName).expect("The provided devicePciAddressOrName contained an interior ASCII NUL");

		let mut portIdentifier = unsafe { uninitialized() };
		let result = unsafe { rte_eth_dev_attach(devicePciAddressOrNameCStr.as_ptr(), &mut portIdentifier) };
		if likely(result == 0)
		{
			Ok(EthernetPort::new(portIdentifier).unwrap())
		}
		else
		{
			forget(portIdentifier);
			
			Err(result)
		}
	}
	
	/// Can only be called after close(); consumes EthernetPort unless a detach error occurs
	/// Ok returns device name string
	#[inline(always)]
	pub fn detach(self) -> Result<String, (CouldNotDetachError, Self)>
	{
		let (device_name, pointerToDeviceName) = Self::initialiseDeviceNameBuffer();
		let result = unsafe { rte_eth_dev_detach(self.portIdentifier(), pointerToDeviceName) };
		
		if likely(result == 0)
		{
			if let Ok(ourDeviceName) = Self::parseDeviceName(device_name, pointerToDeviceName)
			{
				Ok(ourDeviceName)
			}
			else
			{
				Err((CouldNotDetachError::CouldNotParseDeviceNameAsUtf8, self))
			}
		}
		else
		{
			Err((CouldNotDetachError::Unknown {result }, self))
		}
	}
}
