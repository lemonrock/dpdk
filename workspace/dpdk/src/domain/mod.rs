// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use super::*;


/// ARP.
pub mod arp;

/// Ethernet.
pub mod ethernet;

/// Internet Protocol (IP) versions 4 and 6.
pub mod ip;

/// Internet Protocol (IP) host addresses.
pub mod ipHostAddresses;

/// Internet Protocol (IP) network addresses.
pub mod ipNetworkAddresses;

/// Layer 4 protocols (TCP, UDP).
pub mod layer4;

/// Masks.
pub mod maskBits;

/// Virtual LANs (VLANs).
pub mod virtualLans;


include!("NetworkByteOrderEndianU16.rs");
include!("NetworkInterfaceName.rs");
