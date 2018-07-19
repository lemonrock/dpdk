// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// RFC 5227: Before beginning to use an internet protocol (IP) version 4 host address (even one from DHCP) a host must test to see if the address is already in use, by broadcasting ARP probe (request) packets.
///
/// Beginning to use means:-
///
/// * Network hardware becoming active;
/// * Link status changes from down to up;
/// * Awaking from sleep;
/// * Connecting to a Wireless LAN;
#[derive(Debug, Copy, Clone)]
pub enum AddressResolutionProtocolAddressConflictState
{
	/// The first ARP probe is sent randomly between `0` to `PROBE_WAIT` seconds.
	YetToSendProbe
	{
		send_first_probe_at: Seconds,
	},

	/// `PROBE_NUM` ARP probes are sent, spaced uniformly `probe_interval` seconds apart, `probe_interval` is chosen once from `PROBE_MIN` to `PROBE_MAX` inclusive.
	///
	/// If within `ANNOUNCE_WAIT` seconds after the first probe is sent a host receives either a ARP request or reply where the packet's `sender_protocol_address` is the address being probed for then there is an internet protocol (IP) version 4 host address conflict.
	Probing
	{
		subsequent_probes_left_to_send: u8,
		probe_interval: Seconds,
		first_probe_sent_at: Seconds,
	},

	/// The host can begin using the internet protocol (IP) version 4 host address after sending the first ARP announcement.
	///
	/// ARP announcements are sent `ANNOUNCE_INTERVAL` seconds apart.
	Announcing
	{
		first_probe_sent_at: Seconds,
		subsequent_announcements_left_to_send: u8,
	},

	/// There are 3 defensive strategies in RFC 5227; we use the third.
	///
	/// If a host receives an ARP request or reply where the `sender_protocol_address` is one of its own but `sender_hardware_address` is not then there is an address conflict.
	///
	/// If a host has been configured such that it should not give up its address under any circumstances then it MAY elect to defend its address indefinitely.
	///
	/// If the host has not seen any other conflicting ARP packets recently, within the last `DEFEND_INTERVAL` seconds, then it MUST record the time that the conflicting ARP packet was received, and then broadcast one single ARP Announcement,giving its own IP and hardware addresses.
	DefendingAddressConflict
	{
		last_conflicting_packet_acted_on_at: Seconds,
	},

	/// If a conflict occurs during `Probe` or `Announcing`, then there's nothing we can do.
	Conflicted,
}

impl AddressResolutionProtocolAddressConflictState
{
	const PROBE_WAIT: Seconds = x;
	const PROBE_MIN: Seconds = x;
	const PROBE_MAX: Seconds = x;
	const ANNOUNCE_WAIT: Seconds = x;
	const DEFEND_INTERVAL: Seconds = x;

	// Use rte_timer or rte_alarmclock.

	#[inline(always)]
	pub(crate) fn progress(&mut self)
	{
		use self::AddressResolutionProtocolAddressConflictState::*;

		match *self
		{
			YetToSendProbe { send_first_probe_at } =>
			{
				if send_first_probe_at >= Self::current_time()
				{
					AddressResolutionProtocolPacket::send_arp_probe(packet, XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX);

					let probe_interval = thrd_rng().range(XXXXXXXXXXXXXXXX);

					*self = Probing
					{
						subsequent_probes_left_to_send: x,
						probe_interval,
						first_probe_sent_at: send_first_probe_at,
					}
				}
			}
		}
	}

	#[inline(always)]
	pub(crate) fn internet_protocol_version_4_host_address_conflict(&mut self, packet: PacketBuffer, packet_processing: &PacketProcessing)
	{
		use self::AddressResolutionProtocolAddressConflictState::*;

		match *self
		{
			YetToSendProbe { .. } => drop!(XXX, packet_processing, packet),

			Probing { first_probe_sent_at, .. } =>
			{
				let current_time = Self::current_time();

				if first_probe_sent_at + Self::ANNOUNCE_WAIT < current_time
				{
					*self = Conflicted;
				}
				// ? has our timer not expired ?
			}

			Announcing { first_probe_sent_at } =>
			{
				let current_time = Self::current_time();

				if first_probe_sent_at + Self::ANNOUNCE_WAIT < current_time
				{
					*self = Conflicted;
				}
				// ????
			}

			DefendingAddressConflict { ref mut last_conflicting_packet_acted_on_at } =>
			{
				let current_time = Self::current_time();

				if *last_conflicting_packet_acted_on_at + Self::DEFEND_INTERVAL < current_time
				{
					*last_conflicting_packet_acted_on_at = current_time;
					AddressResolutionProtocolPacket::send_arp_announcement(packet, XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX);
				}
				else
				{
					drop!(XXX, packet_processing, packet)
				}
			}

			Conflicted => drop!(XXX, packet_processing, packet)
		}
	}

	#[inline(always)]
	fn current_time() -> Seconds
	{
		XXXX
	}
}
