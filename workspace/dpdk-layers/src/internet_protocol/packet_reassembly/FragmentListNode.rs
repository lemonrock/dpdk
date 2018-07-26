// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone)]
struct FragmentListNode(PacketBuffer);

impl FragmentListNode
{
	#[inline(always)]
	fn new(packet_buffer: PacketBuffer, offset: u16, length: u16, fragmented_payload_offset: u16) -> Self
	{
		let this = PacketBufferNode(packet_buffer);
		
		debug_assert!(this.left().is_none(), "left is not None");
		debug_assert!(this.right().is_none(), "right is not None");
		
		debug_assert_eq!(this.reference().ol_flags | PKT_RX_TIMESTAMP, 0, "Receive offloading of IEEE 1588 timestamps is not supported for fragmented packets");
		debug_assert_eq!(this.timestamp(), 0, "Timestamp field is not zero; it needs to be as it needs to be re-used to store fragment fields");
		this.set_fields(offset, length, fragmented_payload_offset);
		
		this
	}
	
	#[inline(always)]
	fn is_first_fragment(self) -> bool
	{
		fragment_list_node_to_insert.fields().offset == 0
	}
	
	#[inline(always)]
	fn fields(self) -> Fields
	{
		let fields = TimestampReusedAsFields
		{
			timestamp: self.timestamp(),
		};
		unsafe { fields.fields }
	}
	
	#[inline(always)]
	fn set_fields(self, offset: u16, length: u16, fragmented_payload_offset: u16)
	{
		let fields = TimestampReusedAsFields
		{
			fields: Fields
			{
				offset,
				length,
				fragmented_payload_offset,
			}
		};
		this.mutable_reference().timestamp = unsafe { fields.timestamp };
	}
	
	#[inline(always)]
	fn is_to_left_of(self, other: Self) -> Result<bool, ()>
	{
		if self.overlaps(other)
		{
			return Err(())
		}
		
		let self_fields = self.fields();
		let other_fields = other.fields();
		Ok(self_fields.offset < other_fields.offset)
	}
	
	#[inline(always)]
	fn overlaps(self, other: Self)
	{
		let self_fields = self.fields();
		let other_fields = other.fields();
		
		self_fields.overlaps(other_fields)
	}
	
	#[inline(always)]
	fn timestamp(self) -> u64
	{
		this.reference().timestamp
	}
	
	#[inline(always)]
	fn left(self) -> Option<Self>
	{
		let left = self.reference()._4.userdata as *mut rte_mbuf;
		if left.is_null()
		{
			None
		}
		else
		{
			Some(FragmentListNode(PacketBuffer(unsafe { NonNull::new_unchecked(left) })))
		}
	}
	
	#[inline(always)]
	fn set_left(self, next: Self)
	{
		self.mutable_reference()._4.userdata = next.as_ptr() as *mut c_void
	}
	
	#[inline(always)]
	fn right(self) -> Option<Self>
	{
		let right = self.reference().next;
		if right.is_null()
		{
			None
		}
		else
		{
			Some(FragmentListNode(PacketBuffer(unsafe { NonNull::new_unchecked(right) })))
		}
	}
	
	#[inline(always)]
	fn set_right(self, next: Self)
	{
		self.mutable_reference().next = next.as_ptr()
	}
	
	#[inline(always)]
	fn reference<'a>(self) -> &'a rte_mbuf
	{
		self.0.reference()
	}
	
	#[inline(always)]
	fn mutable_reference<'a>(self) -> &'a mut rte_mbuf
	{
		self.0.mutable_reference()
	}
}
