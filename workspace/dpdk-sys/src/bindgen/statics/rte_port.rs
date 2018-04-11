// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	#[link_name = "\u{1}rte_port_ethdev_reader_ops"] pub static mut rte_port_ethdev_reader_ops: rte_port_in_ops;
	#[link_name = "\u{1}rte_port_ethdev_writer_nodrop_ops"] pub static mut rte_port_ethdev_writer_nodrop_ops: rte_port_out_ops;
	#[link_name = "\u{1}rte_port_ethdev_writer_ops"] pub static mut rte_port_ethdev_writer_ops: rte_port_out_ops;
	#[link_name = "\u{1}rte_port_fd_reader_ops"] pub static mut rte_port_fd_reader_ops: rte_port_in_ops;
	#[link_name = "\u{1}rte_port_fd_writer_nodrop_ops"] pub static mut rte_port_fd_writer_nodrop_ops: rte_port_out_ops;
	#[link_name = "\u{1}rte_port_fd_writer_ops"] pub static mut rte_port_fd_writer_ops: rte_port_out_ops;
	#[link_name = "\u{1}rte_port_kni_reader_ops"] pub static mut rte_port_kni_reader_ops: rte_port_in_ops;
	#[link_name = "\u{1}rte_port_kni_writer_nodrop_ops"] pub static mut rte_port_kni_writer_nodrop_ops: rte_port_out_ops;
	#[link_name = "\u{1}rte_port_kni_writer_ops"] pub static mut rte_port_kni_writer_ops: rte_port_out_ops;
	#[link_name = "\u{1}rte_port_ring_multi_reader_ops"] pub static mut rte_port_ring_multi_reader_ops: rte_port_in_ops;
	#[link_name = "\u{1}rte_port_ring_multi_writer_nodrop_ops"] pub static mut rte_port_ring_multi_writer_nodrop_ops: rte_port_out_ops;
	#[link_name = "\u{1}rte_port_ring_multi_writer_ops"] pub static mut rte_port_ring_multi_writer_ops: rte_port_out_ops;
	#[link_name = "\u{1}rte_port_ring_reader_ipv4_frag_ops"] pub static mut rte_port_ring_reader_ipv4_frag_ops: rte_port_in_ops;
	#[link_name = "\u{1}rte_port_ring_reader_ipv6_frag_ops"] pub static mut rte_port_ring_reader_ipv6_frag_ops: rte_port_in_ops;
	#[link_name = "\u{1}rte_port_ring_reader_ops"] pub static mut rte_port_ring_reader_ops: rte_port_in_ops;
	#[link_name = "\u{1}rte_port_ring_writer_ipv4_ras_ops"] pub static mut rte_port_ring_writer_ipv4_ras_ops: rte_port_out_ops;
	#[link_name = "\u{1}rte_port_ring_writer_ipv6_ras_ops"] pub static mut rte_port_ring_writer_ipv6_ras_ops: rte_port_out_ops;
	#[link_name = "\u{1}rte_port_ring_writer_nodrop_ops"] pub static mut rte_port_ring_writer_nodrop_ops: rte_port_out_ops;
	#[link_name = "\u{1}rte_port_ring_writer_ops"] pub static mut rte_port_ring_writer_ops: rte_port_out_ops;
	#[link_name = "\u{1}rte_port_sched_reader_ops"] pub static mut rte_port_sched_reader_ops: rte_port_in_ops;
	#[link_name = "\u{1}rte_port_sched_writer_ops"] pub static mut rte_port_sched_writer_ops: rte_port_out_ops;
	#[link_name = "\u{1}rte_port_sink_ops"] pub static mut rte_port_sink_ops: rte_port_out_ops;
	#[link_name = "\u{1}rte_port_source_ops"] pub static mut rte_port_source_ops: rte_port_in_ops;
}
