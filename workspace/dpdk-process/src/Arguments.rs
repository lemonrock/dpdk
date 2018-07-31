// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug)]
struct Arguments
{
	arguments: Vec<*const c_char>,
	keep_alive: Vec<CString>,
}

impl Arguments
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
	const __pci_whitelist: ConstCStr = ConstCStr(b"--pci-whitelist\0");
	
	/// PCI blacklisted device.
	///
	/// May be specified never, once or more than once.
	///
	/// Followed by PCI device address string in lower-case hexadecimal (domain, bus, device identifier, function).
	///
	/// Specify either this, `__pci_whitelist` or `__no_pci`.
	///
	/// Also known as `-b`.
	const __pci_blacklist: ConstCStr = ConstCStr(b"--pci-blacklist\0");
	
	/// Virtual devices.
	///
	/// May be specified never, once or more than once.
	///
	/// Followed by virtual device string, which consists of a type and number, followed by comma-separated virtual device specific arguments.
	const __vdev: ConstCStr = ConstCStr(b"--vdev\0");
	
	/// For multi-process set ups.
	///
	/// May be specified never or only once. If never, then defaults to `auto`.
	///
	/// May be one of `auto`, `primary` or `secondary`.
	const __proc_type: ConstCStr = ConstCStr(b"--proc-type\0");
	
	/// Logical core mask.
	///
	/// May be specified never or only once.
	///
	/// Specify either this, `_l` or `__lcores`.
	///
	/// Followed by `COREMASK`.
	///
	/// A `COREMASK` maps hyper-threads (logical CPUs) 1:1 with DPDK logical cores.
	const _c: ConstCStr = ConstCStr(b"-c\0");
	
	/// Logical core list.
	///
	/// May be specified never or only once.
	///
	/// Specify either this, `_c` or `__lcores`.
	///
	/// Followed by `CORELIST`.
	///
	/// A `CORELIST` maps hyper-threads (logical CPUs) 1:1 with DPDK logical cores.
	const _l: ConstCStr = ConstCStr(b"-l\0");
	
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
	const __lcores: ConstCStr = ConstCStr(b"--lcores\0");
	
	/// Master logical core.
	///
	/// May be specified never or only once.
	///
	/// Followed by a positive, zero-based integer (`u32`) for a DPDK logical core to treat as a 'master' logical core.
	///
	/// If omitted defaults to the first DPDK logical core.
	const __master_lcore: ConstCStr = ConstCStr(b"--master-lcore\0");
	
	/// Logical core mask for service cores.
	///
	/// May be specified never or only once.
	///
	/// Specify either this, or `-S`.
	///
	/// Followed by `COREMASK`.
	///
	/// A `COREMASK` maps hyper-threads (logical CPUs) 1:1 with DPDK logical cores.
	const _s: ConstCStr = ConstCStr(b"-s\0");
	
	/// Logical core list for service cores.
	///
	/// May be specified never or only once.
	///
	/// Specify either this, or `-s`.
	///
	/// Followed by `CORELIST`.
	///
	/// A `CORELIST` maps hyper-threads (logical CPUs) 1:1 with DPDK logical cores.
	const _S: ConstCStr = ConstCStr(b"-S\0");
	
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
	const _m: ConstCStr = ConstCStr(b"-m\0");
	
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
	const __socket_mem: ConstCStr = ConstCStr(b"--socket-mem\0");
	
	/// Specify the location of a mounted hugetlbfs file system.
	///
	/// Linux-only.
	///
	/// May be specified never or only once.
	///
	/// Specify this if omitting `__no_huge`.
	///
	/// Followed by an absolute file system path.
	const __huge_dir: ConstCStr = ConstCStr(b"--huge-dir\0");
	
	/// Specify whether to not use huge pages.
	///
	/// May be specified never or only once.
	///
	/// Do not specify this is `__legacy_mem` is specified.
	///
	/// Specify this if omitting `__huge_dir`.
	///
	/// Not recommended.
	const __no_huge: ConstCStr = ConstCStr(b"--no-huge\0");
	
	/// Specify to unlike huge pages on process exit.
	///
	/// May be specified never or only once.
	///
	/// ? Linux-only ?
	///
	/// Do not specify this if omitting `__huge_dir`.
	const __huge_unlink: ConstCStr = ConstCStr(b"--huge-unlink\0");
	
	/// Specify to define prefix for huge page file mappings in `__huge_dir`.
	///
	/// Linux-only.
	///
	/// May be specified never or only once.
	///
	/// Specify this if omitting `__huge_dir` or specifying `__huge_dir`.
	const __file_prefix: ConstCStr = ConstCStr(b"--file-prefix\0");
	
	/// Legacy memory mode (no dynamic allocation, io virtual address contiguous segments).
	///
	/// Linux-only.
	///
	/// Do not specify this if `__no_huge` or `__single_file_segments` is specified.
	const __legacy_mem: ConstCStr = ConstCStr(b"legacy-mem\0");
	
	/// Put all huge page memory in single files.
	///
	/// Linux-only.
	///
	/// Do not specify this if `__legacy_mem` is specified.
	const __single_file_segments: ConstCStr = ConstCStr(b"single-file-segments\0");
	
	/// Override number of memory channels to use.
	///
	/// May be specified never or only once.
	///
	/// Followed by 32-bit positive, one-based integer in practice limited to values between 1 and 4 inclusive.
	const _n: ConstCStr = ConstCStr(b"-n\0");
	
	/// Override number of memory ranks to use.
	///
	/// May be specified never or only once.
	///
	/// Followed by 32-bit positive, one-based integer in practice limited to values between 1 and 16 inclusive.
	const _r: ConstCStr = ConstCStr(b"-r\0");
	
	/// Disable use of the High Precision Event Timer (HPET).
	///
	/// May be specified never or only once.
	///
	/// Recommended only for debugging.
	const __no_hpet: ConstCStr = ConstCStr(b"--no-hpet\0");
	
	/// Disable use of PCI devices.
	///
	/// May be specified never or only once.
	///
	/// Recommended only for debugging.
	const __no_pci: ConstCStr = ConstCStr(b"--no-pci\0");
	
	/// Disable use of shared configuration.
	///
	/// May be specified never or only once.
	///
	/// Do not specify this if intending to use a secondary process.
	///
	/// Recommended only for debugging.
	const __no_shconf: ConstCStr = ConstCStr(b"--no-shconf\0");
	
	/// Enable use of VMWare Time Stamp Counter (TSC) map.
	///
	/// May be specified never or only once.
	///
	/// Recommended only if running in a VMWare environment.
	///
	/// Note that DPDK needs to have been compiled with support for VMWare in this case (currently this is the case for the Rust `dpdk-sys` crate).
	const __vmware_tsc_map: ConstCStr = ConstCStr(b"--vmware-tsc-map\0");
	
	/// `syslog` facility.
	///
	/// May be specified never or only once.
	///
	/// Followed by the lower-case name of a syslog facility, eg `auth`.
	const __syslog: ConstCStr = ConstCStr(b"--syslog\0");
	
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
	const __log_level: ConstCStr = ConstCStr(b"--log-level\0");
	
	/// Base virtual address for allocations.
	///
	/// Linux-only.
	///
	/// May be specified never or only once.
	///
	/// Root-only.
	///
	/// Followed by a 64-bit, zero-based, unsigned integer.
	const __base_virtaddr: ConstCStr = ConstCStr(b"--base-virtaddr\0");
	
	/// Virtual function I/O interrrupt mode.
	///
	/// Linux-only.
	///
	/// May be specified never or only once.
	///
	/// Followed by one of `legacy`, `msi` or `msix`.
	const __vfio_intr: ConstCStr = ConstCStr(b"--vfio-intr\0");
	
	/// Enable creation in `/dev` of the UIO device.
	///
	/// Linux-only.
	///
	/// May be specified never or only once.
	///
	/// Root-only.
	///
	/// If omitted, a device will not be created if it is missing.
	const __create_uio_dev: ConstCStr = ConstCStr(b"--create-uio-dev\0");
	
	/// Load a plugin.
	///
	/// May be specified never, once or more than once.
	///
	/// Followed by a file name, typically of the form `lib<X>.so`, where `<X>` is the name of the plugin.
	///
	/// Searches a DPDK compile-time specific directory.
	///
	/// Will only work if DPDK was compiled as a dynamic library (`.so`).
	const _d: ConstCStr = ConstCStr(b"-d\0");
	
	#[inline(always)]
	pub(crate) fn new() -> Self
	{
		const AnticipatedKeepAliveCount: usize = 16;
		
		let mut this = Self
		{
			arguments: Vec::with_capacity(2 * AnticipatedKeepAliveCount),
			keep_alive: Vec::with_capacity(AnticipatedKeepAliveCount)
		};
		this.push_keep_alive("UnusedProgramName");
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
		self.push_keep_alive(value);
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
		self.push_argument(null_mut());
		let argv = self.arguments.as_mut_ptr() as *mut *mut c_char;
		user(argc, argv)
	}
	
	#[inline(always)]
	fn push_const_c_str(&mut self, const_c_str: ConstCStr)
	{
		self.push_argument(const_c_str.as_ptr());
	}
	
	#[inline(always)]
	fn push_keep_alive(&mut self, string: &str)
	{
		self.keep_alive.push(CString::new(string).unwrap());
		let last_index = self.keep_alive.len() - 1;
		let value = self.keep_alive.get(last_index).unwrap();
		self.push_argument(value.as_ptr())
	}
	
	#[inline(always)]
	fn push_argument(&mut self, pointer: *const c_char)
	{
		self.arguments.push(pointer)
	}
}
