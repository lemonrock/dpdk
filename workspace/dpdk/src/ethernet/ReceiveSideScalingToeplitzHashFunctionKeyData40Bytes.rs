// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Receive side scaling toeplitz hash function key data (40 byte variants).
#[derive(Deserialize, Serialize)]
pub struct ReceiveSideScalingToeplitzHashFunctionKeyData40Bytes([u8; 40]);

impl ReceiveSideScalingToeplitzHashFunctionKeyData40Bytes
{
	/// Microsoft key, found at <http://www.ran-lifshitz.com/2014/08/28/symmetric-rss-receive-side-scaling/>.
	///
	/// Good distribution apparently.
	pub const Microsoft: Self = ReceiveSideScalingToeplitzHashFunctionKeyData40Bytes
	([
		0x6D, 0x5A, 0x56, 0xDA, 0x25, 0x5B, 0x0E, 0xC2,
		0x41, 0x67, 0x25, 0x3D, 0x43, 0xA3, 0x8F, 0xB0,
		0xD0, 0xCA, 0x2B, 0xCB, 0xAE, 0x7B, 0x30, 0xB4,
		0x77, 0xCB, 0x2D, 0xA3, 0x80, 0x30, 0xF2, 0x0C,
		0x6A, 0x42, 0xB7, 0x3B, 0xBE, 0xAC, 0x01, 0xFA,
	]);
	
	/// Symmetric with good queue distribution, found at <http://www.ran-lifshitz.com/2014/08/28/symmetric-rss-receive-side-scaling/> and <https://galsagie.github.io/2015/02/26/dpdk-tips-1/>.
	///
	/// Essential when applying RSS to both sides of a TCP or UDP connection, eg if one if a man-in-the-middle.
	pub const Symmetric: Self = ReceiveSideScalingToeplitzHashFunctionKeyData40Bytes
	([
		0x6D, 0x5A, 0x6D, 0x5A, 0x6D, 0x5A, 0x6D, 0x5A,
		0x6D, 0x5A, 0x6D, 0x5A, 0x6D, 0x5A, 0x6D, 0x5A,
		0x6D, 0x5A, 0x6D, 0x5A, 0x6D, 0x5A, 0x6D, 0x5A,
		0x6D, 0x5A, 0x6D, 0x5A, 0x6D, 0x5A, 0x6D, 0x5A,
		0x6D, 0x5A, 0x6D, 0x5A, 0x6D, 0x5A, 0x6D, 0x5A,
	]);
	
	/// Default Mellanox key.
	pub const Mellanox: Self = ReceiveSideScalingToeplitzHashFunctionKeyData40Bytes
	([
		0xD1, 0x81, 0xC6, 0x2C, 0xF7, 0xF4, 0xDB, 0x5B,
		0x19, 0x83, 0xA2, 0xFC, 0x94, 0x3E, 0x1A, 0xDB,
		0xD9, 0x38, 0x9E, 0x6B, 0xD1, 0x03, 0x9C, 0x2C,
		0xA7, 0x44, 0x99, 0xAD, 0x59, 0x3D, 0x56, 0xD9,
		0xF3, 0x25, 0x3C, 0x06, 0x2A, 0xDC, 0x1F, 0xFC,
	]);
	
	/// The RSS `receive_queue_identifier` will handle the stream according to the TCP/UDP `source_port` of the stream. The `receive_queue_identifier` can be calculated as `receive_queue_identifier = (source_port % power_of_2(number_of_receive_queues)) % number_of_receive_queues`.
	#[inline(always)]
	pub const fn for_layer_4_one_way_for_number_of_queues(number_of_receive_queues: u16) -> Self
	{
		let variable_byte = (number_of_receive_queues.next_power_of_two() & 0xFF) as u8;
		
		ReceiveSideScalingToeplitzHashFunctionKeyData40Bytes
		([
			0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
			0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, variable_byte,
			0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
			0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, variable_byte,
			0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
		])
	}
	
	/// To a vector of bytes.
	#[inline(always)]
	pub fn to_vec(&self) -> Vec<u8>
	{
		(&self.0[..]).to_vec()
	}
}
