// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug)]
struct Arguments
{
	arguments: Vec<*const c_char>,
	keep_alive: Vec<CString>,
}

impl Arguments
{
	const_cstr!
	{
		/// PCI whitelisted device.
		///
		/// May be specified never, once or more than once.
		///
		/// Followed by PCI device address string in lower-case hexadecimal (domain, bus, device identifier, function).
		///
		/// Specify either this, or `__pci_blacklist` or `__no_pci`.
		///
		/// Also known as `-w`.
		__pci_whitelist = "--pci-whitelist";
		
		/// PCI blacklisted device.
		///
		/// May be specified never, once or more than once.
		///
		/// Followed by PCI device address string in lower-case hexadecimal (domain, bus, device identifier, function).
		///
		/// Specify either this, `__pci_whitelist` or `__no_pci`.
		///
		/// Also known as `-b`.
		__pci_blacklist = "--pci-blacklist";
		
		/// Virtual devices.
		///
		/// May be specified never, once or more than once.
		///
		/// Followed by virtual device string, which consists of a type and number, followed by comma-separated virtual device specific arguments.
		__vdev = "--vdev";
		
		/// For multi-process set ups.
		///
		/// May be specified never or only once. If never, then defaults to `auto`.
		///
		/// May be one of `auto`, `primary` or `secondary`.
		__proc_type = "--proc-type";
		
		/// Logical core mask.
		///
		/// May be specified never or only once.
		///
		/// Specify either this, `_l` or `__lcores`.
		///
		/// Followed by `COREMASK`.
		///
		/// A `COREMASK` maps hyper-threads (logical CPUs) 1:1 with DPDK logical cores.
		_c = "-c";
		
		/// Logical core list.
		///
		/// May be specified never or only once.
		///
		/// Specify either this, `_c` or `__lcores`.
		///
		/// Followed by `CORELIST`.
		///
		/// A `CORELIST` maps hyper-threads (logical CPUs) 1:1 with DPDK logical cores.
		_l = "-l";
		
		/// Logical core map.
		///
		/// May be specified never or only once.
		///
		/// Specify either this, `_l` or `_c`.
		///
		/// Followed by `COREMAP`.
		///
		/// A `COREMAP` maps one or more hyper-threads (logical CPUs) to one or more DPDK logical cores.
		///
		/// When a `COREMAP` is used, DPDK does not have valid NUMA node information.
		__lcores = "--lcores";
		
		/// Master logical core.
		///
		/// May be specified never or only once.
		///
		/// Followed by a positive, zero-based integer (`u32`) for a DPDK logical core to treat as a 'master' logical core.
		///
		/// If omitted defaults to the first DPDK logical core.
		__master_lcore = "--master-lcore";
		
		/// Logical core mask for service cores.
		///
		/// May be specified never or only once.
		///
		/// Specify either this, or `-S`.
		///
		/// Followed by `COREMASK`.
		///
		/// A `COREMASK` maps hyper-threads (logical CPUs) 1:1 with DPDK logical cores.
		_s = "-s";
		
		/// Logical core list for service cores.
		///
		/// May be specified never or only once.
		///
		/// Specify either this, or `-s`.
		///
		/// Followed by `CORELIST`.
		///
		/// A `CORELIST` maps hyper-threads (logical CPUs) 1:1 with DPDK logical cores.
		_S = "-S";
		
		/// Memory to consume for entire machine.
		///
		/// May be specified never or only once.
		///
		/// Specify either this or `socket_mem`.
		///
		/// Followed by a positive, zero-based integer (`u32`) representing mega bytes to assign.
		///
		/// Mega bytes are capped at 512Mb.
		///
		/// Do not use this on NUMA machines.
		_m= "-m";
		
		/// Memory to consume per-NUMA node.
		///
		/// May be specified never or only once.
		///
		/// Linux-only.
		///
		/// Specify either this or `_m`.
		///
		/// Do not specify `__use_huge_pages` as false when using this option.
		///
		/// Followed by a comma-separated list of positive, zero-based integers representing mega bytes to assign for each NUMA node.
		///
		/// ? mega bytes are capped at 512Mb ?
		///
		/// Do not use this on non-NUMA machines.
		__socket_mem = "--socket-mem";
		
		/// Specify the location of a mounted hugetlbfs file system.
		///
		/// Linux-only.
		///
		/// May be specified never or only once.
		///
		/// Specify this if omitting `__no_huge`.
		///
		/// Followed by an absolute file system path.
		__huge_dir = "--huge-dir";
		
		/// Specify whether to not use huge pages.
		///
		/// May be specified never or only once.
		///
		/// Do not specify this is `__legacy_mem` is specified.
		///
		/// Specify this if omitting `__huge_dir`.
		///
		/// Not recommended.
		__no_huge = "--no-huge";
		
		/// Specify to unlike huge pages on process exit.
		///
		/// May be specified never or only once.
		///
		/// ? Linux-only ?
		///
		/// Do not specify this if omitting `__huge_dir`.
		__huge_unlink = "--huge-unlink";
		
		/// Specify to define prefix for huge page file mappings in `__huge_dir`.
		///
		/// Linux-only.
		///
		/// May be specified never or only once.
		///
		/// Specify this if omitting `__huge_dir` or specifying `__huge_dir`.
		__file_prefix = "--file-prefix";
		
		/// Legacy memory mode (no dynamic allocation, io virtual address contiguous segments).
		///
		/// Linux-only.
		///
		/// Do not specify this if `__no_huge` or `__single_file_segments` is specified.
		__legacy_mem = "legacy-mem";
		
		/// Put all huge page memory in single files.
		///
		/// Linux-only.
		///
		/// Do not specify this if `__legacy_mem` is specified.
		__single_file_segments = "single-file-segments";
		
		/// Override number of memory channels to use.
		///
		/// May be specified never or only once.
		///
		/// Followed by 32-bit positive, one-based integer in practice limited to values between 1 and 4 inclusive.
		_n = "-n";
		
		/// Override number of memory ranks to use.
		///
		/// May be specified never or only once.
		///
		/// Followed by 32-bit positive, one-based integer in practice limited to values between 1 and 16 inclusive.
		_r = "-r";
		
		/// Disable use of the High Precision Event Timer (HPET).
		///
		/// May be specified never or only once.
		///
		/// Recommended only for debugging.
		__no_hpet = "--no-hpet";
		
		/// Disable use of PCI devices.
		///
		/// May be specified never or only once.
		///
		/// Recommended only for debugging.
		__no_hpet = "--no-pci";
		
		/// Disable use of shared configuration.
		///
		/// May be specified never or only once.
		///
		/// Do not specify this if intending to use a secondary process.
		///
		/// Recommended only for debugging.
		__no_shconf = "--no-shconf";
		
		/// Enable use of VMWare Time Stamp Counter (TSC) map.
		///
		/// May be specified never or only once.
		///
		/// Recommended only if running in a VMWare environment.
		///
		/// Note that DPDK needs to have been compiled with support for VMWare in this case (currently this is the case for the Rust `dpdk-sys` crate).
		__vmware_tsc_map = "--vmware-tsc-map";
		
		/// `syslog` facility.
		///
		/// May be specified never or only once.
		///
		/// Followed by the lower-case name of a syslog facility, eg `auth`.
		__syslog = "--syslog";
		
		/// `syslog` log level.
		///
		/// May be specified never or only once.
		///
		/// Followed by the lower-case name of a syslog log-level:-
		///
		/// * `emergency`.
		/// * `alert`.
		/// * `critical`.
		/// * `error`.
		/// * `warning`.
		/// * `notice`.
		/// * `info`.
		/// * `debug`.
		__log_level = "--log-level";
		
		/// Base virtual address for allocations.
		///
		/// Linux-only.
		///
		/// May be specified never or only once.
		///
		/// Root-only.
		///
		/// Followed by a 64-bit, zero-based, unsigned integer.
		__base_virtaddr = "--base-virtaddr";
		
		/// Virtual function I/O interrrupt mode.
		///
		/// Linux-only.
		///
		/// May be specified never or only once.
		///
		/// Followed by one of `legacy`, `msi` or `msix`.
		__vfio_intr = "--vfio-intr";
		
		/// Enable creation in `/dev` of the UIO device.
		///
		/// Linux-only.
		///
		/// May be specified never or only once.
		///
		/// Root-only.
		///
		/// If omitted, a device will not be created if it is missing.
		__vfio_intr = "--create-uio-dev";
		
		/// Load a plugin.
		///
		/// May be specified never, once or more than once.
		///
		/// Followed by a file name, typically of the form `lib<X>.so`, where `<X>` is the name of the plugin.
		///
		/// Searches a DPDK compile-time specific directory.
		///
		/// Will only work if DPDK was compiled as a dynamic library (`.so`).
		_d = "-d";
	}
	
	#[inline(always)]
	pub(crate) fn new() -> Self
	{
		const AnticipatedKeepAliveCount: usize = 16;
		
		let mut this = Self
		{
			arguments: Vec::with_capacity(2 * AnticipatedKeepAliveCount),
			keep_alive: Vec::with_capacity(AnticipatedKeepAliveCount)
		};
		this.keep_alive("UnusedProgramName");
		this
	}
	
	#[inline(always)]
	pub(crate) fn option(&mut self, name: ConstCStr)
	{
		self.push_const_c_str(name);
	}
	
	#[inline(always)]
	pub(crate) fn constant_argument(&mut self, name: ConstCStr, value: ConstCStr)
	{
		self.push_const_c_str(name);
		self.push_const_c_str(value);
	}
	
	#[inline(always)]
	pub(crate) fn variable_argument(&mut self, name: ConstCStr, value: &str)
	{
		self.push_const_c_str(name);
		self.keep_alive(value);
	}
	
	#[inline(always)]
	pub(crate) fn option_argument(&mut self, name: ConstCStr, value: bool)
	{
		if value
		{
			self.option(name)
		}
	}
	
	#[inline(always)]
	pub(crate) fn use_arguments<F: FnOnce(c_int, *mut *mut c_char) -> R, R>(mut self, user: F) -> R
	{
		let argc = self.arguments.len() as c_int;
		self.arguments.push(null_mut());
		let argv = self.arguments.as_mut_ptr() as *mut *mut c_char;
		user(argc, argv)
	}
	
	#[inline(always)]
	fn push_const_c_str(&mut self, const_c_str: ConstCStr)
	{
		self.arguments.push(const_c_str.as_ptr());
	}
	
	#[inline(always)]
	fn keep_alive(&mut self, string: &str)
	{
		self.keep_alive.push(CString::from(string).unwrap());
		let value = self.keep_alive.get(self.keep_alive.len() - 1);
		self.arguments.push(value.as_ptr())
	}
}
