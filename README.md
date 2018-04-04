[](This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.)
[](Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.)

# dpdk


## Licensing

The license for this project is AGPL3.

## Overview

### Network Capabilites

* Up to 32 NICs
* Up to 256 logical cores
* Up to 8 NUMA nodes (sockets)
* Up to 512 Gb of RAM for network processing
* Over 500 Super Huge Pages of 1Gb

## Known Restrictions

### TCP/IP Stack
* Only unicast addresses are supported
* IEEE 802.3 frames are always dropped
* IP v4 packets with options are always dropped
* VLAN and QinQ VLAN packets with Tag Control Information (TCI) with the Drop Eligible Indicator Bit (formerly CFI) are always dropped
* VLANs and QinQ VLANs are supported for parsing but are not yet permitted to receive data; it is not clear how TLDK handles VLANs when doing lookups (I suspect it doesn't)
* VLAN priority information is ignored, but VLANs with VLAN ID 0 (port assigned) are supported and treated as if no VLAN id was present
* Triple-tagged VLANs are not supported
* Multiple IP addresses per ethernet port are not yet supported
* Multiple ethernet addresses per ethernet port are only partially supported
* Either TCP or UDP is supported per ethernet port
* ICMPv4 packets are always dropped; just about anything ICMPv4 is a security risk, if only because it takes traffic away from other flows; this means that Path MTU discovery for IPv4 is not supported
* ICMPv4 packets are never sent for blocked UDP ports
* TCP RST packets are never sent for blocked TCP ports

### Issues

- Memory Leaks with PacketBufferPool::new() (used for Rx-Tx queue pairs and fragment pools)
	- we can free with rte_mempool_free
	- BUT we can not do so until we can be certain the pool is no longer in use by any logical core
	- SO we could free at application exit
	- At which point

## Notes

### TODO
- Consider setting CONFIG_RTE_MBUF_REFCNT_ATOMIC=n to force refcnt to not be atomic
- Consider setting CONFIG_RTE_MBUF_DEFAULT_MEMPOOL_OPS to use a non-multiproducer setting if we are NOT sharing a mempool amongst more than one core
- CONFIG_RTE_LIBRTE_VHOST_NUMA=y
- CONFIG_RTE_ETHDEV_RXTX_CALLBACKS=n
- ?CONFIG_RTE_FORCE_INTRINSICS=y?
- ?CONFIG_RTE_ETHDEV_TX_PREPARE_NOOP=n?
- ?CONFIG_RTE_NIC_BYPASS=y?
- With TLDK, overwrite all use of rte spinlocks, etc with non-atomic operations

### Testing
- Fake linux nodes: https://linux-hacks.blogspot.co.uk/2009/07/fake-numa-nodes-in-linux.html

### TODO
- RTE_LOG ring buffer per core to store messages for quick debugging
- Try to determine memory channels / memory ranks using dmidecode
	- replace with libsmbios?
- Replace modprobe? Probably not realistic
- Consider a fork'd process to manage unmounting of kernel modules
- Reassign PCI devices to alternative NUMA nodes (only if value not -1)
	- eg echo 3 > /sys/devices/pci0000\:00/0000\:00\:05.0/numa_node

### Compilation of DPDK
- Machine optimisations for rust

Statistics
    NIC           ipackets   opackets  ibytes  obytes  imissed  ierrors  oerrors  rx_nombuf  q_ipackets   q_opacktes     q_ibytes    q_obytes  q_errors
    af_packet      y          y         y       y       n        n        y        n          y            y            y         y         y
    bnx2x         y          y         y       y       y        y        y        y          n            n            n         n         n
    bnxt          y          y         y       y       y        y        y        n          y            y            y         y         y
    bonding       y          y         y       y       y        y        y        y          y            y            y         y         y
    cxgbe         y          y         y       y       y        y        y        n          y            y            y         y         y
    e1000(igb)     y          y         y       y       y        y        y        n          n            n            n         n         n
    e1000(igbvf)   y          y         y       y       n        n        n        n          n            n            n         n         n
    ena           y          y         y       y       y        y        y        y          n            n            n         n         n
    enic          y          y         y       y       y        y        y        y          n            n            n         n         n
    fm10k         y          y         y       y       n        n        n        n          y            y            y         y         n
    i40e          y          y         y       y       y        y        y        n          n            n            n         n         n
    i40evf        y          y         y       y       n        y        y        n          n            n            n         n         n
    ixgbe         y          y         y       y       y        y        y        n          y            y            y         y         y
    ixgbevf       y          y         y       y       n        n        n        n          n            n            n         n         n
    mlx4          y          y         y       y       n        y        y        y          y            y            y         y         y
    mlx5          y          y         y       y       n        y        y        y          y            y            y         y         y
    mpipe         y          y         y       y       n        y        y        y          y            y            y         y         y
    nfp           y          y         y       y       y        y        y        y          y            y            y         y         n
    null          y          y         n       n       n        n        y        n          y            y            n         n         y
    pcap          y          y         y       y       n        n        y        n          y            y            y         y         y
    qede          y          y         y       y       y        y        y        y          n            n            n         n         n
    ring          y          y         n       n       n        n        y        n          y            y            n         n         y
    szedata2      y          y         y       y       n        n        y        n          y            y            y         y         y
    thunderx      y          y         y       y       y        y        y        n          y            y            y         y         n
    vhost         y          y         y       y       n        n        y        n          y            y            y         y         n
    virtio         y          y         y       y       n        y        y        y          y            y            y         y         n
    vmxnet3      y          y         y       y       n        y        y        y          y            y            y         y         y
    xenvirt       y          y         n       n       n        n        n        n          n            n            n         n         n



Checks for Virtio in containers
    Control queue and multi-queue are not supported yet.
    Doesn’t work with --huge-unlink.
    Doesn’t work with --no-huge.
    Doesn’t work when there are more than VHOST_MEMORY_MAX_NREGIONS(8) hugepages.
    Root privilege is required for sorting hugepages by physical address.
    Can only be used with the vhost user backend.

Additional Configuration fo VHOST Client mode (as opposed to server mode)
    Client mode is enabled when the RTE_VHOST_USER_CLIENT flag is set while calling rte_vhost_driver_register.
    QEMU version v2.7 or above is required for this feature.
    Reconnection attempts can be turned off by setting RTE_VHOST_USER_NO_RECONNECT

Memory Ranks and Channels Discovery

    // Memory channels:-
    // dmidecode, Memory Controller Information (DMI type 5) (Current Interleave: One-way Interleave)
    // dmidecode, Memory Device Mapped Address (DMI type 20) (Interleaved Data Depth: 2)
    // neither result is reliable; (Interleaved Data Depth: Unknown) has also been seen
    // See http://dpdk.org/ml/archives/dev/2013-June/000226.html
    // "dmidecode -t 17 | grep -c 'Size:'" => I.e 6 memory slots probably implies 3 channels.

    // Memory ranks:-
    // dmidecode type 16 (Number Of Devices) ???

    // Use libsmbios https://github.com/dell/libsmbios ?


Notes from tldk-dev mailing list
	- The functions tle_<proto>_rx_bulk, tle_<proto>_tx_bulk, tle_tcp_process take a tle_dev
		- They are not multi-thread safe
		- Always call from same core for a device
	- One context per protocol
		- Can be one context per protocol per application
		- Can be one context per protocol per logical, with one device per context
		- Context contains lookup pointers WHICH AREN'T multi-thread safe
		- OR,
			- We have 2 contexts per RX-TX queue pair
				- one for UDP traffic
				- one for TCP traffic
			- We can remove locking from the context
				- Indeed, we can compile TLDK with NO LOCKING WHATSOEVER
			- We have two sets of lookup functions (a set is a pair of IPv4 and IPv6 lookups)
				- one set for the UDP context
				- one set for the TCP context
			- The lookup function set
				- returns the same tle_dev the packets came in on
					- if we have one tle_dev per context, this is trivial
					- if we have multiple tle_dev per context, but one per IP address (or IPv4-v6 address pair)
					- then this is more complex
						- but we could still use a very simple look up table. However, I'm not sure we have the SOURCE (eg our local) IP address then available
		- SO,
			- it looks like we have two contexts PER RX-TX queue pair
			- each context has just one tle_dev used for in and out traffic
			- the lookup function sets are then trivial logic - indeed, could be coded in C so no need to go through a catch_panic.
			- each RX-TX queue pair is on a different logical core (DONE)
			- for now, we perhaps don't have multiple IPv4 or multiple IPv6 address support
	- Device
		- Can be one per logical core / one per rx-tx queue pair
		- Not clear how the rx & tx offloading knowledge is used
		- Has a local (ie our) IP address (actually a pair of IPv4 and IPv6)
		- If one wants multiple local IP addresses, then need multiple devices
			- Interesting interaction with RSS scaling, in that RSS scaling can be done so that a particular IP-port tuple goes to a particular RX-TX queue pair
			- Thus one needs a RSS or flow director configuration that ensures each RX-TX queue pair gets data ONLY for the local IP address assigned on the device
			- OR,
				- We define multiple devices PER RX-TX queue pair
	
Application Layout
	- Do all our initialisation as now, BUT
		- need a way to assign IPv4-IPv6 address pairs to an ethernet port
			- probably best done with a MAC => Set of address pairs look up
	- Initialisation function for a RX-TX queue pair to set up TLDK
		- Create a TLDK UDP context per RX-TX queue pair
			- Create a pair of lookup4 - lookup6 lookups to find destination devices
				- Sadly these lookups only seem to be given DESTINATION address, not source address
			- Create tle_dev devices, one for each local (our) IPv4-IPv6 address pair
				- IPv4-IPv6 address pair can lack an IPv4 or an IPv6 address, but not both
			- Thus can
		- Create a TLDK TCP context per RX-TX queue pair
			- Create tle_dev devices, one for each local (our) IPv4-IPv6 address pair
				- IPv4-IPv6 address pair can lack an IPv4 or an IPv6 address, but not both
		- Need to pass configuration data
			- max_streams / max_stream_rbufs / max_stream_sbufs / send_bulk_size
	- Main loop for a RX-TX queue pair, one Context per RX-TX queue pair
		- Get packets from networking using DPDK
		- For UDP packets, get UDP context
			- Decide which UDP packets to forward to TLDK
				- For each local (our) IP address, find the tle_dev
					- For the tle_dev, tle_udp_rx_bulk for UDP packets
			- For each tle_dev on this context, call
				- Call tle_udp_tx_bulk, send packets to network using DPDK
		- For TCP packets, get TCP context
			- Decide which TCP packets to forward to TLDK
				- For each local (our) IP address, find the tle_dev
		
				- Call tle_tcp_tx_bulk
		- Call tle_tcp_rx_bulk for TCP packets
		- Call tle_tcp_process BETWEEN rx and tx, as it make create some extra packets, or, at the very least, after before sending packets to DPDK
	- Main loop for Front End streams
		- If a TCP listening stream has a connection request, its RX event will be raised (or callback called)
		- If a TCP / UDP stream has received some data, it there is data we can read, RX event will be up
		- If a TCP / UDP stream has free space in its TX queue, TX event will be up (or TX callback called just once)
		
		
	- Main Loop for a RX-TX queue pair example, one tle_dev device
		- do receive
			- Look at netbe_rx() in common.h
			- Logic is same for TCP or UDP
		- do ARP receive (once regardless of TCP or UDP)
		- do tle_tcp_process(ctx, TCP_MAX_PROCESS) if TCP context
		- do send
			- Look at netbe_tx() in common.h
			- Logic is same for TCP or UDP
	- Full main loop example for TCP
		 - lcore_main_tcp in tcp.h; slightly at odds with how we would do it (they loop over be cores)
		 	- Does front end work
				- netfe_lcore_tcp_req (tcp.h) (TCP SYN)
					- effectively, process incoming new connection requests (ie listen stuff)
					- we could do this with a callback rather than an event queue; it's a choice
					- call backs are potentially expensive, but event queue (tle_evq_get) uses barriers and spinlocks...
				- netfe_lcore_tcp_rst (tcp.h) (TCP RST)
					- looks for error events
				- netfe_lcore_tcp
					- looks for rx events, does work
					- looks for tx events, does work
			- Does back end work
				- netbe_lcore_tcp
					- receive bulk, etc (netbe_rx())
					- receive ARP (netbe_rx())
					- tle_tcp_process
					- transmit bulk, etc (netbe_tx())

Receiving Data on a Rx / Tx queue

Sending Data on a Rx / Tx queue


Email
	- context lookup functions don't seem to be supplied with local (source) address
	- makes it impossible for the following model?
		- TCP context per HW Rx/Tx queue pair
		- multiple tle_dev devices for that context
		- each device is for a different local IP address
	- does that mean I need a TCP context per queue pair per local IP address, with one device?
	

Functions / structs accounted for
	port.h
		prepare_hash_key() DONE
		update_rss_conf() DONE
		port_init() DONE
		check_lcore() DONE
		log_netbe_prt() DONE
		log_netbe_cfg() DONE
		find_initilized_lcore() DONE
		queue_init() DONE
		pool_init() DONE
		frag_pool_init() NOT YET IMPLEMENTED
			- creates a fragmentation pool
		netbe_port_init()
	netbe.h
		netbe_port DONE
		netbe_lcore
			- holds ctx
			- holds ipv4 and ipv6 lpm / dest logic (256 max entries)
			
	
To tackle
	netbe.h
		netbe_dest
		netbe_dest_prm
		pkt_buf
		netbe_dev
	
	lcore.h
		create_context()
			- done per BE logical core
			- creates tldk ctx
			- setups up LPM forwarding
			- setups up ip fragmentation
			- TODO: We need a wrapper structure
				=> && We should create one per rx / tx queue pair
		lcore_init()
			- calls create_context() (only works once per lcore)
			- tle_add_dev()
		netbe_lcore_init() - calls lcore_init()
			- QUERY - does blocklist refer to ethernet devices ('ports') rather than IPV4 ones?
				- may be - otherwise how else does it find the associated ethernet device?
	
	tldk / stream.h
		- stream_get_dest()
			- given a tle_stream, a destination address (v4 or v6), and a destination struct (tle_dest), finds the tle_dev used for writing (sending)
			- call the look up function for v4 or v6 on the context
			- look up function modifies (or overwrites - this is what l4fwd does in dpdk_legacy.h: lpm4_dst_lookup) tle_dest destination struct
				- l4fwd uses LPM to find tle_dev
				- l4fwd maps to a cache of 256 entries and just uses a memcpy, but with an ?weird? size
				- the dest struct can contain a memory pool for fragmented data (see frag_pool_init())
	
	
	
	
		
				
			
	
	
	
	? LPM ? (eg lpm4 and LCORE_MAX_DST (256))
		- longest prefix match
		- librte_lpm
		- Related to tle_dest
		- Is this used to look up which front end to send packets to?
		- LPM has separate config for IPv4 and IPv6; it's quite messy because DPDK have changed it several times, eg see dpdk_legacy.h, which is solely to abstract this away somewhat

Context(s)
	- lcore.h => create_context
	- one created per BE logical core (just look there is one frag_pool per BE logical core)
	- LPM is initialised at same time (lcore_lpm_init() in dpdk_legacy.h)
	- An IP Fragment table (rte_ip_frag_table_create()) is also init'd here; not sure when death_row is, though.



lcore_prm
	- one for every logical core
	- contains data for BE (netbe_lcore)
		- holds a context
		- holds tle_dest table for ipv4 and ipv6
			- tables have 256 entries
		- holds rte_lpm and rte_lpm6
		- holds data to help with ip fragments
	- contains data for FE (netfe_lcore_prm)
		- maximum streams
		- number of streams
		- pointer to stream parameters (netfe_stream_prm)
			- logical core identifier (? for this, ie front end)
			- backend logical core identifier
			- op data
				- line / op / txlen => to do with type of operation, not interesting
				- source local / remote addr (for op)
				- forwarding local / remote addr
			- ops seem to BE RXONLY, TXONLY, RXTX, FWD

Naming
	- "port" in the BE is not a UDP port, but an ethernet device I think

globals
	- BE config, becfg
	- a rte_eth_conf (called port_conf_default)
		- apart from defaults, only overrides SOME of rxmode with:-
		.rxmode = {
			.max_rx_pkt_len = ETHER_MAX_VLAN_FRAME_LEN,
			.hw_vlan_strip = 1,
			.jumbo_frame = 1,
		},
		- ETHER_MAX_VLAN_FRAME_LEN is defined in DPDK

parsing of command line arguments
	- arp is only possible with tcp
	- this app only runs as a forwarder for one of UDP or TCP per instance
	- context parameters left as 0 - what are the defaults?
	- allocations for variables used across sockets for the heap are made with rte_zmalloc(NULL, size, RTE_CACHE_LINE_SIZE)
	- number of ports == number of remaining command line arguments after parsing everything else, ie arguments after '--' (if it supported it)
	- ports are part of the BE
	- adds each port to an array (prt) on netbe_cfg
		- sets a field on BE config
		- netbe_port stores config for each BE port
			- id ?
			- an array of logical cores 'active' for this port
				- nb_lcore (array size)
				- lcore_id[]
				- array is of logical core identifiers
			- MTU
				- 'val[2]'
				- ETHER_MAX_VLAN_FRAME_LEN - ETHER_CRC_LEN (DPDK constants)
			- rx and tx offload bit flags
			- ipv4 and ipv6 addresses
			- MAC address (ether_addr)
			- RSS (receive side scaling) Hash Key / Hash Key Size
				- RSS_HASH_KEY_LENGTH, defined as 64 bytes
		- also incrementally populates a cpuset (Linux libc feature)
			- so we know which cores are active
			- doesn't keep the cpuset
			- just uses it to allocate an array on netbe_cfg called 'cpu' of netbe_lcore
			
netbe_cfg
	- contains basic information
		- promiscous
			- Used by rte_eth_promiscuous_enable() in netbe_port_init()
			- 'Global' option, but set per port
		- protocol (UDP or TCP)
		- server - ?what?
		- arp - ?active?
		- array of netbe_port - data for Ethernet devices
			- fields are prt_num (size) and prt
		- array of netbe_lcore
			- fields are cpu_num (size) and cpu
			- size is number of active logical cores used by union of all ethernet devices (ports)
			- Each entry
				- id ?logical core id?
				- is TCP or UDP
				- holds a context
				- holds tle_dest array table for ipv4 (dst4_num, dst4) and rte_lpm (lpm) of 256 entries
				- holds tle_dest array table for ipv6 (dst4_num, dst4) and rte_lpm6 (lpm6) of 256 entries
				- holds rte_lpm and rte_lpm6
				- holds data to help with ip fragments (rte_ip_frag_death_row)
				- holds an array of netbe_dev (prtq_num, prtq); these 'wrap' tle_dev and netbe_port (ethernet device details)
					- receive queue id (rxqid)
					- transmit queue id (txqid)
					- netbe_port (port)
					- tle_dev (dev)
					- receive statistics (rx_stat)
					- transmit statistics (tx_stat)
					- transmit buffer, one pkt_buf (tx_buf)
						- pkt_buf is an app structure
						- It is an array of rte_mbuf, sized as 2 * MAX_PKT_BURST
						- MAX_PKT_BURST is 32
					- ARP buffer, one pkt_buf (arp_buf)
				- TCP statistics

netbe_port
	- Used to configure the DPDK device itself
	- DPDK 'port' (ethernet device) id (ie 0 - 31 usually) (field is 'id')
	- an array of logical cores 'active' for this port
		- nb_lcore (array size)
		- lcore_id[]
		- array is of logical core identifiers
	- MTU
		- Default is ETHER_MAX_VLAN_FRAME_LEN - ETHER_CRC_LEN (DPDK constants); is supposed to be 1514
	- rx and tx offload bit flags
	- ipv4 and ipv6 addresses
	- Present on the structure but NOT set by command line arguments:-
		- MAC address (ether_addr)
			- set by rte_eth_macaddr_get() in netbe_port_init()
		- RSS (receive side scaling) Hash Key / Hash Key Size
			- RSS_HASH_KEY_LENGTH, defined as 64 bytes

main
	- eal_init
	- parses options
		- gets context data
		- gets BE config
		- gets config file names (FE, BE)
		- DOES NOT GET FE config at this stage
	- setups up data structures
		- func_ptrs_init (using BE config)
		- netbe_port_init (using BE config)
		- netbe_lcore_init (using BE config and pointer to context)
	- loops over all BE config port numbers
		- calls rte_eth_dev_start
		- calls rte_eth_dev_info_get to obtain rte_eth_dev_info
		- cals update_rss_reta with rte_eth_dev_info
	xxx


[dpdk]: https://github.com/lemonrock/dpdk "dpdk GitHub page"
