// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[allow(missing_debug_implementations)]
pub struct IpV4AddressLookUpForSendCallback
{
	ipV4RoutingTable: Rc<RefCell<IpV4RoutingTable>>,
	tleDestinationTemplate: tle_dest,
	numberOfBytesOfTemplateToMemCopy: usize,
}

impl AddressLookUpForSendCallback<in_addr> for IpV4AddressLookUpForSendCallback
{
	#[inline(always)]
	fn call(&mut self, destinationAddress: *const in_addr, outParameterForResult: *mut tle_dest) -> i32
	{
		// Copy our template
		#[allow(trivial_casts)]
		unsafe
		{
			let from = &mut self.tleDestinationTemplate as *mut _ as *mut u8;
			copy_nonoverlapping(from, outParameterForResult as *mut u8, self.numberOfBytesOfTemplateToMemCopy)
		}

		// Routing can fail, which means we just wasted time doing the copy above. Routing failure is uncommon.
		self.ipV4RoutingTable.borrow().route(destinationAddress, outParameterForResult)
	}
}

impl IpV4AddressLookUpForSendCallback
{
	#[inline(always)]
	pub fn tleDestinationTemplate(udpFragmentsAndTcpControlPacketBufferPool: *mut rte_mempool, virtualLanTagging: &VirtualLanTagging, differentiatedServiceCodePoint: DifferentiatedServiceCodePoint, hopLimits: u8, internet_protocol_version: InternetProtocolVersion, layer4Protocol: Layer4Protocol) -> (tle_dest, usize)
	{
		const TldkMaximumLengthFor_hdr: usize = 96;

		const VlanTaggingOffset: isize = (SizeOfEthernetHeaderLessEtherType as isize);

		let mut tleDestinationTemplate = tle_dest
		{
			head_mp: udpFragmentsAndTcpControlPacketBufferPool,
			dev: null_mut(),
			mtu: 0, // get from lookup; used by TLDK to work out if IP fragmentation is needed, so this is the IP Path MTU
			l2_len: 0, // See below
			l3_len: 0, // See below
			hdr: unsafe { uninitialized() }
		};

		let buffer = &mut tleDestinationTemplate.hdr as *mut _ as *mut u8;

		let layer2Length = virtualLanTagging.write_layer_2_header_data(unsafe { buffer.offset(VlanTaggingOffset) }, internet_protocol_version.to_ether_type());
		debug_assert!(layer2Length <= MaximumSizeOfLayer2 as usize, "Layer 2 header is too big");
		tleDestinationTemplate.l2_len = layer2Length as u8;

		let offsetToLayer3 = VlanTaggingOffset + layer2Length as isize;
		debug_assert!(offsetToLayer3 as usize <= TldkMaximumLengthFor_hdr, "offsetToLayer3 '{}' exceeds TLDK maximum, '{}'", offsetToLayer3, TldkMaximumLengthFor_hdr);

		let (layer3Length, trailingAddressBytesLength) = internet_protocol_version.write_layer_3_header(unsafe { NonNull::new_unchecked(buffer.offset(VlanTaggingOffset + layer2Length as isize)) }, differentiatedServiceCodePoint, hopLimits, layer4Protocol);
		debug_assert!(layer2Length + layer3Length <= TldkMaximumLengthFor_hdr, "Layer 2 header '{}' + Layer 3 header '{}' is bigger than TLDK maximum '{}'", layer2Length, layer3Length, TldkMaximumLengthFor_hdr);

		tleDestinationTemplate.l3_len = layer3Length as u8;
		let numberOfBytesOfTemplateToMemCopy = size_of::<tle_dest>() - layer2Length + layer3Length - trailingAddressBytesLength;

		(tleDestinationTemplate, numberOfBytesOfTemplateToMemCopy)
	}

	pub fn new
	(
		ipV4RoutingTable: Rc<RefCell<IpV4RoutingTable>>,
		udpFragmentsAndTcpControlPacketBufferPool: *mut rte_mempool,
		virtualLanTagging: &VirtualLanTagging,
		differentiatedServiceCodePoint: DifferentiatedServiceCodePoint,
		hopLimits: u8,
		layer4Protocol: Layer4Protocol,
	) -> Self
	{
		let (tleDestinationTemplate, numberOfBytesOfTemplateToMemCopy) = Self::tleDestinationTemplate(udpFragmentsAndTcpControlPacketBufferPool, virtualLanTagging, differentiatedServiceCodePoint, hopLimits, InternetProtocolVersion::V4, layer4Protocol);
		Self
		{
			ipV4RoutingTable,
			tleDestinationTemplate,
			numberOfBytesOfTemplateToMemCopy,
		}
	}

	#[inline(always)]
	pub fn assignTleDeviceAfterContextCreated(&mut self, device: *mut tle_dev)
	{
		self.tleDestinationTemplate.dev = device;
	}
}
