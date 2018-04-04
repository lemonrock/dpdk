// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


///
///
///
///                                IMPORTANT NOTICE
///
///     This code uses the words 'deathRow' and 'free' and seems quite tasteless and trivialising.
///     Be assured that the use of this language is solely to ensure correspondence with the
///     underlying logic in the DPDK library. By doing so, we allow a maintainer of this code to analyze
///     it effectively.
///
///     I, the original developer, find DPDK's choice of analogy deeply inappropriate and juvenile.
///
///       Raphael James Cohn
///         Original Developer
///         Skipton, 12th April 2017
///
///
///
///
#[allow(missing_debug_implementations)]
pub struct IpPacketReassemblyTable
{
	ipV4FragmentationTable: *mut rte_ip_frag_tbl,
	deathRow: rte_ip_frag_death_row,
	deathRowPreFetchFactor: u32,
}

impl Drop for IpPacketReassemblyTable
{
	/// Please see the IMPORTANT NOTICE above which explains why this language has been used
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { rust_rte_ip_frag_table_destroy(self.ipV4FragmentationTable) };
		self.freeAllPacketsOnDeathRow();
	}
}

impl IpPacketReassemblyTable
{
	/// numberOfBuckets is really the maximum number of active flows, ie, of sets of fragmented packets
	#[inline(always)]
	pub fn create(numberOfBuckets: u16, entriesPerBucket: PowerOfTwoSixteenBit, maximumFlowTimeToLiveInMilliseconds: u64, numaSocketId: Option<NumaSocketId>) -> Option<Self>
	{
		assert!(numberOfBuckets != 0, "numberOfBuckets can not be zero");
		
		// numberOfBuckets
		//   librte_port/rte_port_ras.c uses RTE_PORT_RAS_N_BUCKETS, 4094
		//   main.c in the DPDK ip_reassembly example uses maximumFlowNumber, 1 - 65535, default value is 4096
		//   Documentation uses maximumFlowNumber + maximumFlowNumber / 4
		let numberOfBuckets = numberOfBuckets as u32;
		
		// entriesPerBucket
		//   librte_port/rte_port_ras.c uses RTE_PORT_RAS_N_ENTRIES_PER_BUCKET, 8
		//   main.c in the DPDK ip_reassembly example uses IP_FRAG_TBL_BUCKET_ENTRIES, 16
		let entriesPerBucket = (entriesPerBucket as u16) as u32;
		
		// maximumEntries
		//   librte_port/rte_port_ras.c uses RTE_PORT_RAS_N_ENTRIES, (RTE_PORT_RAS_N_BUCKETS * RTE_PORT_RAS_N_ENTRIES_PER_BUCKET)
		//   main.c in the DPDK ip_reassembly example uses maximumFlowNumber, 1 - 65535, default value is 4096
		let maximumEntries = numberOfBuckets * entriesPerBucket;
		
		// maximumFlowTimeToLiveInMilliseconds is calculated from maximumTimeToLiveInCyclesForEachFragmentedPacket in all examples using the same calculation
		//   librte_port/rte_port_ras.c uses maximumFlowTimeToLiveInMilliseconds of MS_PER_S * 100, ie 100 seconds
		//   main.c in the DPDK ip_reassembly example uses 1 second
		let maximumTimeToLiveInCyclesForEachFragmentedPacket = (unsafe { rte_get_tsc_hz() } + MS_PER_S - 1) / MS_PER_S * maximumFlowTimeToLiveInMilliseconds;
		
		// deathRowPreFetchFactor
		//   librte_port/rte_port_ras.c uses 3
		//   main.c in the DPDK ip_reassembly example uses PREFETCH_OFFSET, 3
		//   We restrict to an u8 as the maximum size of the array used internally is less than 256
		const deathRowPreFetchFactor: u8 = 3;
		
		Self::new(numberOfBuckets as u32, unsafe { PowerOfTwoThirtyTwoBit::from_u32_unchecked(entriesPerBucket) }, maximumEntries, maximumTimeToLiveInCyclesForEachFragmentedPacket, numaSocketId, deathRowPreFetchFactor)
	}
	
	/// Please see the IMPORTANT NOTICE above which explains why this language has been used
	#[inline(always)]
	fn new(numberOfBuckets: u32, entriesPerBucket: PowerOfTwoThirtyTwoBit, maximumEntries: u32, maximumTimeToLiveInCyclesForEachFragmentedPacket: u64, numaSocketId: Option<NumaSocketId>, deathRowPreFetchFactor: u8) -> Option<Self>
	{
		assert!(numberOfBuckets != 0, "numberOfBuckets can not be zero");
		assert!(maximumTimeToLiveInCyclesForEachFragmentedPacket != 0, "maximumCycles can not be zero");
		let bucketEntriesAsU32 = entriesPerBucket.as_u32();
		assert!(maximumEntries <= (numberOfBuckets * bucketEntriesAsU32), "maximumEntries should be less than or equal to numberOfBuckets * entriesPerBucket");
		
		// This is a horrible solution - we can not just hardcode the array size because it is a compile time constant that can be changed by DPDK
		let deathRow = rte_ip_frag_death_row::default();
		assert!(deathRowPreFetchFactor as usize <= deathRow.row.len(), "The deathRowPreFetchFactor '{}' exceeds the maximum size of deathRow '{}'. Please see the IMPORTANT NOTICE above which explains why this language has been used");
		
		let ipV4FragmentationTable = unsafe { rte_ip_frag_table_create(numberOfBuckets, bucketEntriesAsU32, maximumEntries, maximumTimeToLiveInCyclesForEachFragmentedPacket, numaSocketId.as_c_int()) };
		if unlikely(ipV4FragmentationTable.is_null())
		{
			None
		}
		else
		{
			Some
			(
				Self
				{
					ipV4FragmentationTable: ipV4FragmentationTable,
					deathRow: deathRow,
					deathRowPreFetchFactor: deathRowPreFetchFactor as u32,
				}
			)
		}
	}
	
	// At point of entry, l2_len / l3_len need to be set correctly
	// pointerToIpV4HeaderInsideIncomingFragmentedPacket must be inside rte_mbuf, not a buffer copy
	// Result may be null. It may be the same packet or a different packet.
	#[inline(always)]
	pub fn reassembleFragmentedIpV4Packet(&mut self, incomingFragmentedPacket: *mut rte_mbuf, fragmentArrivalTimeStampFromRdTsc: u64, pointerToIpV4HeaderInsideIncomingFragmentedPacket: *mut ipv4_hdr) -> *mut rte_mbuf
	{
		// This is an expensive function to call
		// fragmentArrivalTimeStamp = rte_rdtsc();
		unsafe { rte_ipv4_frag_reassemble_packet(self.ipV4FragmentationTable, &mut self.deathRow, incomingFragmentedPacket, fragmentArrivalTimeStampFromRdTsc, pointerToIpV4HeaderInsideIncomingFragmentedPacket) }
	}
	
	#[inline(always)]
	pub fn reassembleFragmentedIpV6Packet(&mut self, incomingFragmentedPacket: *mut rte_mbuf, fragmentArrivalTimeStampFromRdTsc: u64, pointerToIpV6HeaderInsideIncomingFragmentedPacket: *mut ipv6_hdr, pointerToIpFragmentExtensionHeader: *mut ipv6_extension_fragment) -> *mut rte_mbuf
	{
		// This is an expensive function to call
		// fragmentArrivalTimeStamp = rte_rdtsc();
		unsafe { rte_ipv6_frag_reassemble_packet(self.ipV4FragmentationTable, &mut self.deathRow, incomingFragmentedPacket, fragmentArrivalTimeStampFromRdTsc, pointerToIpV6HeaderInsideIncomingFragmentedPacket, pointerToIpFragmentExtensionHeader) }
	}
	
	/// Please see the IMPORTANT NOTICE above which explains why this language has been used
	#[inline(always)]
	pub fn freeAllPacketsOnDeathRowIfFull(&mut self)
	{
		if unlikely(self.isDeathRowFull())
		{
			self.freeAllPacketsOnDeathRow()
		}
	}
	
	/// Please see the IMPORTANT NOTICE above which explains why this language has been used
	#[inline(always)]
	fn isDeathRowFull(&self) -> bool
	{
		self.deathRow.cnt == self.deathRow.row.len() as u32
	}
	
	/// Please see the IMPORTANT NOTICE above which explains why this language has been used
	#[inline(always)]
	fn freeAllPacketsOnDeathRow(&mut self)
	{
		unsafe { rte_ip_frag_free_death_row(&mut self.deathRow, self.deathRowPreFetchFactor) };
	}
}

