// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2019 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct CpuFeatures
{
	/// Has hyper threading.
	pub has_hyper_threading: bool,
	
	/// ie per NUMA socket.
	pub maximum_logical_processor_identifiers_per_package: u8,

	/// Has 1Gb Huge Pages.
	pub has_1gb_huge_pages: bool,
}

impl CpuFeatures
{
	#[inline(always)]
	pub(crate) fn validate_minimal_cpu_features(warnings_to_suppress: &WarningsToSuppress, uses_enhanced_intel_speedstep_technology: bool) -> Result<Self, String>
	{
		macro_rules! check
		{
			($boolean_expression: expr, $string: literal) =>
			{
				if !$boolean_expression
				{
					return Err($string.to_string())
				}
			}
		}

		if cfg!(target_arch = "x86_64")
		{
			let cpu_id = CpuId::new();
			
			let feature_information = cpu_id.get_feature_info().ok_or("CPU architecture does not support feature information".to_string())?;
			let extended_function_information = cpu_id.get_extended_function_info().ok_or("CPU architecture does not support extended function information".to_string())?;
			let extended_feature_information = cpu_id.get_extended_feature_info().ok_or("CPU architecture does not support extended features".to_string())?;

			// As of 2019 it is very reasonable to expect the lowest featured CPU to be an Intel Ivy Bridge CPU (as used on the current range of MacPro trash cans).
			#[inline(always)]
			fn instructions_modes_and_features_it_is_safe_to_assume_for_all_x86_64_cpu_architectures_as_of_q2_2018(feature_information: &FeatureInfo, extended_function_information: &ExtendedFunctionInfo, extended_features: &ExtendedFeatures, uses_enhanced_intel_speedstep_technology: bool) -> Result<(), String>
			{
				check!(extended_function_information.has_64bit_mode(), "CPU architecture does not support 64-bit");
				check!(feature_information.has_cmpxchg8b(), "CPU architecture does not support 64-bit CAS");
				check!(feature_information.has_sysenter_sysexit() && extended_function_information.has_syscall_sysret(), "CPU architecture does not support 64-bit fast syscalls");
				check!(feature_information.has_cmov(), "CPU architecture does not support the legacy CMOV instruction");
				check!(feature_information.has_cmpxchg16b(), "CPU architecture does not support the CMPXCHG16B (128-bit CAS) instruction");
				check!(feature_information.has_pclmulqdq(), "CPU architecture does not support the PCLMULQDQ instruction");
				check!(feature_information.has_msr(), "CPU architecture does not have Read Model Specific Register (RDMSR) and Write Model Specific Register WRMSR");
				check!(extended_features.has_fsgsbase(), "CPU architecture does not have 'fsgsbase' instructions RDFSBASE, RDGSBASE, WRFSBASE and WRGSBASE");
				check!(feature_information.has_dca(), "CPU architecture does not have Direct Cache Access (DCA) for DMA writes");
				
				// Security-related.
				check!(feature_information.has_pcid(), "CPU architecture does not support PCID (Essential for Meltdown vulnerability protection)");
				check!(extended_features.has_smep(), "CPU architecture does not support Supervisor Model Execution Protection (SMEP)");
				check!(extended_function_information.has_execute_disable(), "CPU architecture does not support (or does not have enabled) execute-disable bit (this may be due to Intel VT-d being disabled in the BIOS)");
				check!(extended_function_information.has_lahf_sahf(), "CPU architecture does not support (or does not have enabled) LAHF / SAHF (this may be due to Intel VT-d being disabled in the BIOS)");
				
				// Memory and huge pages.
				check!(feature_information.has_pse(), "CPU architecture does not support Page Size Extensions; ie does not support huge pages");
				check!(feature_information.has_pse36(), "CPU architecture does not support 36-Bit Page Size Extension; ie does not support huge pages");
				check!(feature_information.has_pae(), "CPU architecture does not support Physical Address Extension; ie does not support huge pages");
				check!(feature_information.has_pge(), "CPU architecture does not support Page Global Bit");
				check!(feature_information.has_pat(), "CPU architecture does not support Page Attribute Table");
				
				// Floating point and SIMD instruction sets (and related instructions introduced with them).
				check!(feature_information.has_fpu(), "CPU architecture does not support essential SIMD (x87 Floating Point)");
				check!(feature_information.has_fxsave_fxstor(), "CPU architecture does not support essential SIMD (FXSAVE and FXRSTOR instructions)");
				check!(feature_information.has_mmx(), "CPU architecture does not support essential SIMD (MMX)");
				check!(feature_information.has_sse(), "CPU architecture does not support essential SIMD (SSE)");
				check!(feature_information.has_sse2(), "CPU architecture does not support essential SIMD (SSE2)");
				check!(feature_information.has_clflush(), "CPU architecture does not support essential SIMD (CLFLUSH instruction)");
				check!(feature_information.has_sse3(), "CPU architecture does not support essential SIMD (SSE3)");
				check!(feature_information.has_ssse3(), "CPU architecture does not support essential SIMD (Suplemental SSE3, aka SSSE3)");
				check!(feature_information.has_monitor_mwait(), "CPU architecture does not support essential SIMD (MONITOR and MWAIT instructions)");
				check!(feature_information.has_sse41(), "CPU architecture does not support essential SIMD (SSE4.1)");
				check!(feature_information.has_sse42(), "CPU architecture does not support essential SIMD (SSE4.2)");
				check!(feature_information.has_popcnt(), "CPU architecture does not support essential SIMD (POPCNT instruction)");
				
				// Timing related.
				check!(feature_information.has_apic(), "CPU architecture does not have an Advanced Programmable Interrupt Controller (APIC)");
				check!(feature_information.has_x2apic(), "CPU architecture does not have an x2 Advanced Programmable Interrupt Controller (x2APIC)");
				check!(feature_information.has_acpi(), "CPU architecture does not have Thermal Monitor and Software Controlled Clock Facilities (ACPI)");
				check!(feature_information.has_tsc(), "CPU architecture does not support Time Stamp Counter (TSC)");
				check!(feature_information.has_tsc_deadline(), "CPU architecture does not support Time Stamp Counter (TSC) deadline timer");
				check!(extended_function_information.has_rdtscp(), "CPU architecture does not support (or does not have enabled) Read Time Stamp Counter and Processor ID (RDTSCP)");
				check!(extended_function_information.has_invariant_tsc(), "CPU architecture does not support (or does not have enabled) invariant Time Stamp Counter (TSC)");
				//check!(extended_feature_information.has_tsc_adjust_msr(), "CPU architecture does not support Time Stamp Counter (TSC) adjust Model Specific Registers (MSR)");
				
				// Power management.
				if uses_enhanced_intel_speedstep_technology
				{
					check!(feature_information.has_eist(), "Enhanced Intel SpeedStep® Technology must be enabled in the platform BIOS if the power management feature of DPDK is to be used")
				}

				Ok(())
			}

			#[allow(unused_variables)]
			#[inline(always)]
			fn compiled_target_features_are_available_at_runtime(feature_information: &FeatureInfo, extended_function_information: &ExtendedFunctionInfo, extended_feature_information: &ExtendedFeatures) -> Result<(), String>
			{
				// Atom and similar processors do not usually support AVX, although it is common in most other CPUs.
				#[cfg(target_feature = "avx")]
				{
					check!(feature_information.has_avx(), "CPU architecture does not support compilation options: the AVX instructions not supported by currently executing CPU");
				}
				
				#[cfg(target_feature = "f16c")]
				{
					check!(feature_information.has_f16c(), "CPU architecture does not support compilation options: the F16C instructions not supported by currently executing CPU");
				}
				
				#[cfg(target_feature = "avx2")]
				{
					check!(extended_feature_information.has_avx2(), "CPU architecture does not support compilation options: the AVX2 instructions not supported by currently executing CPU");
				}
				
				#[cfg(target_feature = "has_fma")]
				{
					check!(feature_information.has_fma(), "CPU architecture does not support compilation options: FMA3 instructions not supported by currently executing CPU");
				}
				
				#[cfg(target_feature = "hle")]
				{
					check!(feature_information.has_hle(), "CPU architecture does not support compilation options: HLE transaction memory extension (TSX) instructions not supported by currently executing CPU");
				}
				
				#[cfg(target_feature = "rtm")]
				{
					check!(feature_information.has_rtm(), "CPU architecture does not support compilation options: RTM transaction memory extension (TSX) instructions not supported by currently executing CPU");
				}
				
				#[cfg(target_feature = "bmi1")]
				{
					check!(feature_information.has_bmi1(), "CPU architecture does not support compilation options: BMI1 instructions not supported by currently executing CPU");
				}
				
				#[cfg(target_feature = "bmi2")]
				{
					check!(feature_information.has_bmi2(), "CPU architecture does not support compilation options: BMI2 instructions not supported by currently executing CPU");
				}
				
				#[cfg(target_feature = "adx")]
				{
					check!(extended_feature_information.has_adx(), "CPU architecture does not support compilation options: ADX instructions not supported by currently executing CPU");
				}
				
				// Properly `rdrnd`, but not correctly encoded by Rust.
				#[cfg(target_feature = "rdrand")]
				{
					check!(feature_information.has_rdrand(), "CPU architecture does not support compilation options: RDRAND instruction not supported by currently executing CPU");
				}
				
				#[cfg(target_feature = "rdseed")]
				{
					// `rdseet` (sic) - typo in raw_cpuid library.
					check!(feature_information.has_rdseet(), "CPU architecture does not support compilation options: RDSEED instruction not supported by currently executing CPU");
				}
				
				#[cfg(target_feature = "movbe")]
				{
					// Only Atom and similar processors usually support MOVBE.
					check!(feature_information.has_movbe(), "CPU architecture does not support compilation options: MOVBE instruction not supported by currently executing CPU");
				}
				
				#[cfg(target_feature = "lzcnt")]
				{
					check!(feature_information.has_lzcnt(), "CPU architecture does not support compilation options: LZCNT instruction not supported by currently executing CPU");
				}
				
				#[cfg(target_feature = "clflushopt")]
				{
					check!(extended_feature_information.has_clflushopt(), "CPU architecture does not support compilation options: CLFLUSHOPT instruction not supported by currently executing CPU");
				}

				Ok(())
			}
			
			instructions_modes_and_features_it_is_safe_to_assume_for_all_x86_64_cpu_architectures_as_of_q2_2018(&feature_information, &extended_function_information, &extended_feature_information, uses_enhanced_intel_speedstep_technology)?;
			
			compiled_target_features_are_available_at_runtime(&feature_information, &extended_function_information, &extended_feature_information)?;
			
			warnings_to_suppress.performance_warnings_it_is_safe_to_assume_for_all_x86_64_cpu_architectures_as_of_q2_2018(&feature_information, &extended_function_information, &extended_feature_information);
			
			warnings_to_suppress.performance_warnings_for_new_features(&feature_information, &extended_function_information, &extended_feature_information);
			
			warnings_to_suppress.security_warnings_for_new_features(&feature_information, &extended_function_information, &extended_feature_information);

			Ok
			(
				Self
				{
					has_hyper_threading: feature_information.has_htt(),
					maximum_logical_processor_identifiers_per_package: feature_information.max_logical_processor_ids(),
					has_1gb_huge_pages: extended_function_information.has_1gib_pages()
				}
			)
		}
		else
		{
			Err("Unsupported CPU architecture".to_string())
		}
	}
}
