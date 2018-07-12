// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Bits 57-60 (zero-based) should be set to zero and currently do not have any meaning.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct PageMapEntry(u64);

impl PageMapEntry
{
	/// Read the current process' pagemap file.
	#[inline(always)]
	pub fn read_our_pagemap_file<HVA: HasVirtualAddress>(have_virtual_addresses: impl Iterator<Item=HVA>, page_map_entry_user: impl FnMut(HVA, VirtualAddress, PageMapEntry)) -> io::Result<()>
	{
		Self::read_pagemap_file(have_virtual_addresses, page_map_entry_user, "/proc/self/pagemap")
	}
	
	/// Read the a process' pagemap file, typically at `/proc/PID/pagemap`, where `UPID` is the process' identifier ('pid').
	#[inline(always)]
	pub fn read_pagemap_file<HVA: HasVirtualAddress>(have_virtual_addresses: impl Iterator<Item=HVA>, mut page_map_entry_user: impl FnMut(HVA, VirtualAddress, PageMapEntry), page_map_file_path: impl AsRef<Path>) -> io::Result<()>
	{
		let mut file = File::open(page_map_file_path)?;
		
		for has_virtual_address in have_virtual_addresses
		{
			let virtual_address = has_virtual_address.virtual_address();
			let page_map_entry = PageMapEntry::read_from_pagemap_file(&mut file, virtual_address)?;
			page_map_entry_user(has_virtual_address, virtual_address, page_map_entry)
		}
		
		Ok(())
	}
	
	/// Physical Page Frame Number (PFN).
	///
	/// May be zero if an user lacks the capability `CAP_SYS_ADMIN`.
	///
	/// This data is not valid if the page can be swapped out; to prevent that, lock the page using `mlock()`.
	/// (If swapped, the bottom 5 bits are the swap type and the bits 5 (zero-based, ie this is the 6th bit) to 54 inclusive are the swap offset; we panic if this is the case in debug).
	///
	/// The top 9 bits are unset (ie always zero).
	#[inline(always)]
	pub fn physical_page_frame_number(self) -> PhysicalPageFrameNumber
	{
		debug_assert!(!self.is_swapped(), "page is swapped");
		
		PhysicalPageFrameNumber(self.0 & 0x7FFFFFFFFFFFFF)
	}
	
	/// Page is swapped.
	///
	/// Bit 62 (zero-based).
	#[inline(always)]
	pub fn is_swapped(self) -> bool
	{
		self.0 & 0x4000000000000000 != 0
	}
	
	/// Page is present.
	///
	/// Bit 63 (zero-based).
	#[inline(always)]
	pub fn is_present(self) -> bool
	{
		self.0 & 0x8000000000000000 != 0
	}
	
	/// Page is exclusively mapped.
	///
	/// Introduced in Linux 4.2.
	///
	/// Bit 56 (zero-based).
	#[inline(always)]
	pub fn is_exclusively_mapped(self) -> bool
	{
		self.0 & 0x100000000000000 != 0
	}
	
	/// Page is exclusively mapped.
	///
	/// Introduced in Linux 3.5.
	///
	/// Bit 61 (zero-based).
	#[inline(always)]
	pub fn is_file_page_or_shared_anonymous(self) -> bool
	{
		self.0 & 0x2000000000000000 != 0
	}
	
	/// Page Table Entry (PTE) is 'soft dirty'.
	///
	/// For more information, see the Linux documentation at `Documentation/vm/soft-dirty.txt`.
	///
	/// Bit 55 (zero-based).
	#[inline(always)]
	pub fn page_table_entry_is_soft_dirty(self) -> bool
	{
		self.0 & 0x80000000000000 != 0
	}
	
	#[inline(always)]
	fn read_from_pagemap_file(file: &mut File, virtual_address: VirtualAddress) -> io::Result<Self>
	{
		let virtual_page_frame_number: VirtualPageFrameNumber = virtual_address.into();
		
		file.seek(virtual_page_frame_number.into())?;
		
		let mut buffer: [u8; 8] = unsafe { uninitialized() };
		file.read_exact(&mut buffer)?;
		
		Ok(PageMapEntry(u64::from_bytes(buffer)))
	}
}
