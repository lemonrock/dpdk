// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents a logical hyper thread, which in Operating System terms is usually a logical CPU (core).
///
/// These usually map 1:1 with `LogicalCore`s
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
pub struct InterruptRequest(u16);

impl From<u16> for InterruptRequest
{
	#[inline(always)]
	fn from(value: u16) -> Self
	{
		InterruptRequest(value)
	}
}

impl Into<u16> for InterruptRequest
{
	#[inline(always)]
	fn into(self) -> u16
	{
		self.0
	}
}

impl InterruptRequest
{
	#[inline(always)]
	#[cfg(any(target_os = "android", target_os = "linux"))]
	pub fn read_interrupt_request_to_hyper_threads_affinity(&self, proc_path: &ProcPath) -> Result<BTreeSet<HyperThread>, io::Error>
	{
		proc_path.file_path(&format!("irq/{}/smp_affinity_list", self.0)).read_linux_core_or_numa_list(HyperThread::from)
	}
	
	/// This logic seems to exist in the Linux kernel to provide a place for the `irqbalance` daemon to store some configuration.
	#[inline(always)]
	#[cfg(any(target_os = "android", target_os = "linux"))]
	pub fn read_interrupt_request_to_hyper_threads_affinity_hint(&self, proc_path: &ProcPath) -> Result<BTreeSet<HyperThread>, io::Error>
	{
		proc_path.file_path(&format!("irq/{}/affinity_hint", self.0)).read_linux_core_or_numa_list(HyperThread::from)
	}
	
	/// ?numa node? As always, the Linux documentation sucks.
	#[inline(always)]
	#[cfg(any(target_os = "android", target_os = "linux"))]
	pub fn read_interrupt_request_node(&self, proc_path: &ProcPath) -> Result<u8, io::Error>
	{
		proc_path.file_path(&format!("irq/{}/node", self.0)).read_value()
	}
	
	/// Default interrupt request affinity hyper thread mask.
	#[inline(always)]
	#[cfg(any(target_os = "android", target_os = "linux"))]
	pub fn read_default_interrupt_request_affinity_hyper_thread_mask(proc_path: &ProcPath) -> Result<u32, io::Error>
	{
		proc_path.file_path("irq/default_smp_affinity").parse_linux_core_or_numa_mask()
	}
	
	/// We ignore failures as the `/proc` for interrupt requests is brittle.
	#[inline(always)]
	#[cfg(any(target_os = "android", target_os = "linux"))]
	pub fn force_all_interrupt_requests_to_just_these_hyper_threads(hyper_threads: &BTreeSet<HyperThread>, proc_path: &ProcPath)
	{
		let mask = HyperThread::hyper_threads_to_mask(hyper_threads);
		
		proc_path.file_path("irq/default_smp_affinity").write_value(&mask);
		
		if let Some(interrupt_requests) = InterruptRequest::interrupt_requests()
		{
			for interrupt_request in interrupt_requests.iter()
			{
				proc_path.file_path(&format!("irq/{}/smp_affinity", interrupt_request)).write_value(&mask);
				proc_path.file_path(&format!("irq/{}/affinity_hint", interrupt_request)).write_value(&mask);
			}
		}
	}
	
	/// Returns known interrupt requests (IRQs).
	#[inline(always)]
	#[cfg(any(target_os = "android", target_os = "linux"))]
	pub fn interrupt_requests(proc_path: &ProcPath) -> Result<BTreeSet<Self>, io::Error>
	{
		let mut interrupt_requests = BTreeSet::new();
		
		let irq_folder_path = proc_path.file_path("irq");
		for entry in irq_folder_path.read_dir()?
		{
			let entry = entry?;
			if entry.file_type()?.is_file()
			{
				if let Ok(string) = entry.file_name().as_os_str().to_str()
				{
					if let Ok(interrupt_request) = string.parse::<u16>()
					{
						interrupt_requests.push(interrupt_request)
					}
				}
			}
		}
		
		Ok(interrupt_requests)
	}
}
