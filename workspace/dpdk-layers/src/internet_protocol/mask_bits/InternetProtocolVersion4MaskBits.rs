// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/*
Generated using the logic:-

fn main()
{
    for prefix_length in 0 .. 33
    {
        let mut prefix: u32 = 0;
    
        for bit in ( (32 - prefix_length) .. 32)
        {
            prefix |= 1 << bit;
        }
        
        println!("\t/// `/{}`", prefix_length);
        println!("\t#[cfg(target_endian = \"big\")] _{} = 0x{:08X},", prefix_length, prefix);
        println!("\t#[cfg(target_endian = \"little\")] _{} = 0x{:08X},", prefix_length, prefix.to_be());
        println!("\t");
    }
}
*/

/// Mask bits for `/0` to `/32`.
///
/// Stored internally in network byte (big endian) byte order.
#[repr(u32)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
pub enum InternetProtocolVersion4MaskBits
{
	/// `/0`
	#[cfg(target_endian = "big")] _0 = 0x00000000,
	#[cfg(target_endian = "little")] _0 = 0x00000000,
	
	/// `/1`
	#[cfg(target_endian = "big")] _1 = 0x80000000,
	#[cfg(target_endian = "little")] _1 = 0x00000080,
	
	/// `/2`
	#[cfg(target_endian = "big")] _2 = 0xC0000000,
	#[cfg(target_endian = "little")] _2 = 0x000000C0,
	
	/// `/3`
	#[cfg(target_endian = "big")] _3 = 0xE0000000,
	#[cfg(target_endian = "little")] _3 = 0x000000E0,
	
	/// `/4`
	#[cfg(target_endian = "big")] _4 = 0xF0000000,
	#[cfg(target_endian = "little")] _4 = 0x000000F0,
	
	/// `/5`
	#[cfg(target_endian = "big")] _5 = 0xF8000000,
	#[cfg(target_endian = "little")] _5 = 0x000000F8,
	
	/// `/6`
	#[cfg(target_endian = "big")] _6 = 0xFC000000,
	#[cfg(target_endian = "little")] _6 = 0x000000FC,
	
	/// `/7`
	#[cfg(target_endian = "big")] _7 = 0xFE000000,
	#[cfg(target_endian = "little")] _7 = 0x000000FE,
	
	/// `/8`
	#[cfg(target_endian = "big")] _8 = 0xFF000000,
	#[cfg(target_endian = "little")] _8 = 0x000000FF,
	
	/// `/9`
	#[cfg(target_endian = "big")] _9 = 0xFF800000,
	#[cfg(target_endian = "little")] _9 = 0x000080FF,
	
	/// `/10`
	#[cfg(target_endian = "big")] _10 = 0xFFC00000,
	#[cfg(target_endian = "little")] _10 = 0x0000C0FF,
	
	/// `/11`
	#[cfg(target_endian = "big")] _11 = 0xFFE00000,
	#[cfg(target_endian = "little")] _11 = 0x0000E0FF,
	
	/// `/12`
	#[cfg(target_endian = "big")] _12 = 0xFFF00000,
	#[cfg(target_endian = "little")] _12 = 0x0000F0FF,
	
	/// `/13`
	#[cfg(target_endian = "big")] _13 = 0xFFF80000,
	#[cfg(target_endian = "little")] _13 = 0x0000F8FF,
	
	/// `/14`
	#[cfg(target_endian = "big")] _14 = 0xFFFC0000,
	#[cfg(target_endian = "little")] _14 = 0x0000FCFF,
	
	/// `/15`
	#[cfg(target_endian = "big")] _15 = 0xFFFE0000,
	#[cfg(target_endian = "little")] _15 = 0x0000FEFF,
	
	/// `/16`
	#[cfg(target_endian = "big")] _16 = 0xFFFF0000,
	#[cfg(target_endian = "little")] _16 = 0x0000FFFF,
	
	/// `/17`
	#[cfg(target_endian = "big")] _17 = 0xFFFF8000,
	#[cfg(target_endian = "little")] _17 = 0x0080FFFF,
	
	/// `/18`
	#[cfg(target_endian = "big")] _18 = 0xFFFFC000,
	#[cfg(target_endian = "little")] _18 = 0x00C0FFFF,
	
	/// `/19`
	#[cfg(target_endian = "big")] _19 = 0xFFFFE000,
	#[cfg(target_endian = "little")] _19 = 0x00E0FFFF,
	
	/// `/20`
	#[cfg(target_endian = "big")] _20 = 0xFFFFF000,
	#[cfg(target_endian = "little")] _20 = 0x00F0FFFF,
	
	/// `/21`
	#[cfg(target_endian = "big")] _21 = 0xFFFFF800,
	#[cfg(target_endian = "little")] _21 = 0x00F8FFFF,
	
	/// `/22`
	#[cfg(target_endian = "big")] _22 = 0xFFFFFC00,
	#[cfg(target_endian = "little")] _22 = 0x00FCFFFF,
	
	/// `/23`
	#[cfg(target_endian = "big")] _23 = 0xFFFFFE00,
	#[cfg(target_endian = "little")] _23 = 0x00FEFFFF,
	
	/// `/24`
	#[cfg(target_endian = "big")] _24 = 0xFFFFFF00,
	#[cfg(target_endian = "little")] _24 = 0x00FFFFFF,
	
	/// `/25`
	#[cfg(target_endian = "big")] _25 = 0xFFFFFF80,
	#[cfg(target_endian = "little")] _25 = 0x80FFFFFF,
	
	/// `/26`
	#[cfg(target_endian = "big")] _26 = 0xFFFFFFC0,
	#[cfg(target_endian = "little")] _26 = 0xC0FFFFFF,
	
	/// `/27`
	#[cfg(target_endian = "big")] _27 = 0xFFFFFFE0,
	#[cfg(target_endian = "little")] _27 = 0xE0FFFFFF,
	
	/// `/28`
	#[cfg(target_endian = "big")] _28 = 0xFFFFFFF0,
	#[cfg(target_endian = "little")] _28 = 0xF0FFFFFF,
	
	/// `/29`
	#[cfg(target_endian = "big")] _29 = 0xFFFFFFF8,
	#[cfg(target_endian = "little")] _29 = 0xF8FFFFFF,
	
	/// `/30`
	#[cfg(target_endian = "big")] _30 = 0xFFFFFFFC,
	#[cfg(target_endian = "little")] _30 = 0xFCFFFFFF,
	
	/// `/31`
	#[cfg(target_endian = "big")] _31 = 0xFFFFFFFE,
	#[cfg(target_endian = "little")] _31 = 0xFEFFFFFF,
	
	/// `/32`
	#[cfg(target_endian = "big")] _32 = 0xFFFFFFFF,
	#[cfg(target_endian = "little")] _32 = 0xFFFFFFFF,
}

impl InternetProtocolMaskBits for InternetProtocolVersion4MaskBits
{
	type InternetProtocolHostAddress = InternetProtocolVersion4HostAddress;
}
