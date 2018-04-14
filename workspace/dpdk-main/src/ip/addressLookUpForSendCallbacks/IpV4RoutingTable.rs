// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


const MaximumRoutesExcludingDefault: usize = 255;
const MaximumRoutes: usize = 1 + MaximumRoutesExcludingDefault;

#[allow(missing_debug_implementations)]
pub struct IpV4RoutingTable
{
	longestPrefixMatchTable: IpV4LongestPrefixMatchTable,
	nextHopsToDeviceAndMtuAndIpAddress: ArrayVec<[IpV4Route; MaximumRoutes]>,
	arpCache: ArpCache,
	defaultMaximumTransmissionUnit: MaximumTransmissionUnitSizeInBytes,
}

impl IpV4RoutingTable
{
	#[inline(always)]
	pub fn new(name: &LongestPrefixMatchName, logicalCoreMemorySocket: Option<NumaSocketId>, arpCache: ArpCache, defaultMaximumTransmissionUnit: MaximumTransmissionUnitSizeInBytes) -> Self
	{
		const MaximumRules: u32 = MaximumRoutes as u32;
		const NumberOfTable8sToAllocate: u32 = 16;
		
		let name = name.toName("IpV4-RT");
		let table = LongestPrefixMatchTable::new(&name, MaximumRules, NumberOfTable8sToAllocate, logicalCoreMemorySocket).expect("Could not allocate");
		
		Self
		{
			longestPrefixMatchTable: table,
			nextHopsToDeviceAndMtuAndIpAddress: ArrayVec::new(),
			arpCache,
			defaultMaximumTransmissionUnit,
		}
	}
	
	pub fn reconfigureRulesAndRoutes(&mut self, rules: &HashMap<IpV4NetworkAddress, NextHop>, routes: &[IpV4Route])
	{
		assert!(!routes.is_empty(), "routes is empty");
		
		self.longestPrefixMatchTable.deleteAllRules();
		self.nextHopsToDeviceAndMtuAndIpAddress.clear();
		
		for (&ipNetworkAddress, &nextHop) in rules.iter()
		{
			if nextHop as usize > MaximumRoutesExcludingDefault
			{
				continue;
			}
			if nextHop as usize >= routes.len()
			{
				continue;
			}
			self.longestPrefixMatchTable.addRule(&ipNetworkAddress, nextHop);
		}
		
		for route in routes
		{
			self.nextHopsToDeviceAndMtuAndIpAddress.push(*route);
		}
	}
	
	#[inline(always)]
	pub fn route(&self, destinationAddress: *const in_addr, outParameterForResult: *mut tle_dest) -> i32
	{
		let (ref ethernetAddress, tldkMtu) =
		{
			const AlwaysPresentNextHop: NextHop = 0;
			
			let internet_protocol_address: &InternetProtocolVersion4HostAddress = unsafe { transmute(destinationAddress) };
			
			match self.arpCache.find(internet_protocol_address)
			{
				Some(ethernetAddress) => (ethernetAddress, self.defaultMaximumTransmissionUnit),
				None =>
				{
					let ipV4Route =
					{
						let nextHop =
						{
							match self.longestPrefixMatchTable.lookUp(internet_protocol_address)
							{
								None => AlwaysPresentNextHop,
								Some(nextHop) => nextHop,
							}
						};
						self.nextHopsToDeviceAndMtuAndIpAddress.get(nextHop as usize).unwrap()
					};
					
					match self.arpCache.find(internet_protocol_address)
					{
						None => return NegativeE::EDESTADDRREQ,
						Some(ethernetAddress) => (ethernetAddress, ipV4Route.tldkMtu)
					}
				}
			}
		};
		
		// Overwrite destination MAC address
		#[allow(trivial_casts)]
		unsafe
		{
			let from = ethernetAddress as *const _ as *const u8;
			let hdrOffsetWhichStartsWithDestinationEthernetAddress = offsetOf!(tle_dest, hdr);
			let to = (outParameterForResult as *mut u8).offset(hdrOffsetWhichStartsWithDestinationEthernetAddress);
			const length: usize = SizeOfEthernetAddress as usize;
			copy_nonoverlapping(from, to, length);
		}
		
		// Overwrite destination MTU
		(unsafe { *outParameterForResult }).mtu = tldkMtu.as_u16();
		
		0
	}
}
