# This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
# Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


_extract_from_linker_script()
{
	local linkerScript="$1"

	sed -e 's;^GROUP ( ;;g' -e 's; )$;;g' -e 's;\.a ; ;g' -e 's;\.a$;;g' -e 's;^lib;;g' -e 's; lib; ;g' "$linkerScript"
}

bindingsName='dpdk'
rootIncludeFileName='dpdk-and-tldk.h'
macosXHomebrewPackageNames=''
alpineLinuxPackageNames='rsync make gcc linux-headers libunwind-dev linux-grsec-dev'
clangAdditionalArguments=-I"$rootOutputFolderPath"/extra-musl-headers
headersFolderPath='DESTDIR/usr/include/dpdk'
link="$(_extract_from_linker_script "$rootOutputFolderPath"/DESTDIR/usr/lib/libdpdk.a) tle_misc tle_dring tle_timer tle_l4p"
link_kind='static-nobundle'

final_chance_to_tweak()
{
	:
	
	# # Make these compatible with PosixErrorNumber; can't be done as constant type mapping as conversion of unnamed enums occurs after constant type mapping
	# sed -i -e 's/: u32 /: c_int /g' "$outputFolderPath"/constants/E_RTE.rs
	#
	# # Fix up lcore_config - the presence of rte_cpuset_t (in the Linux version) creates problems
	# sed -i -e 's/#\[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)\]/#\[derive(Copy, Clone)\]/g' "$outputFolderPath"/structs/lcore_config.rs
	#
	# # Make code work on FreeBSD - fix cpu_set_t
	# {
	# 	cat "$configurationFolderPath"/preamble.rs
	# 	cat <<-EOF
	# 		#[cfg(any(target_os = "android", target_os = "linux"))] pub type rte_cpuset_t = cpu_set_t;
	# 		#[cfg(target_os = "freebsd")] pub type rte_cpuset_t = cpuset_t;
	# 		#[cfg(not(any(target_os = "android", target_os = "linux", target_os = "freebsd")))] pub type rte_cpuset_t = [u64; 16];
	# 	EOF
	# } >"$outputFolderPath"/types/rte_cpuset_t.rs
	#
	# # If a struct has just one bitfield, rename it without a trailing number
	# local structWithBitfield
	# for structWithBitfield in rte_eth_rxmode rte_eth_txmode rte_cryptodev_data rte_eth_dev_data rte_eth_link rte_lpm_tbl_entry rte_lpm_tbl_entry_v20
	# do
	# 	sed -i \
	# 		-e 's/pub _bindgen_bitfield_1_: /pub __bindgen_bitfield: /g' \
	# 		-e '/pub _bindgen_bitfield_[2-9][0-9]*_/d' \
	# 		"$outputFolderPath"/structs/"$structWithBitfield".rs
	# done
	#
	# # Fix up an union whose size changes depending on SSE options
	# sed -i -e 's/u8; 48usize/u32; 9usize/g' "$outputFolderPath"/structs/rte_thash_tuple.rs
	#
	# # Replace generated rte_mbuf. This isn't ideal
	# printf '\n\n\nWARNING: %s\n\n\n\n' "Overriding definition of rte_mbuf. Not ideal, but necessary probably"
	# {
	# 	cat "$configurationFolderPath"/preamble.rs
	# 	cat "$configurationFolderPath"/rte_mbuf.fragment.rs
	# } >"$outputFolderPath"/structs/rte_mbuf.rs
	#
	# # rte_timer_status isn't used as a union; the _u32 'field' exists to allow code to atomically set states and owner
	# {
	# 	cat "$configurationFolderPath"/preamble.rs
	# 	cat <<-EOF
	# 		#[repr(C, packed)]
	# 		#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
	# 		pub struct rte_timer_status
	# 		{
	# 			pub state: uint16_t,
	# 			pub owner: int16_t,
	# 		}
	#
	# 		impl Default for rte_timer_status
	# 		{
	# 			#[inline(always)]
	# 			fn default() -> Self
	# 			{
	# 				rte_timer_status
	# 				{
	# 					state: RTE_TIMER_STOP,
	# 					owner: RTE_TIMER_NO_OWNER,
	# 				}
	# 			}
	# 		}
	# 	EOF
	# } >"$outputFolderPath"/structs/rte_timer_status.rs
	#
	# # Remove thread-local statics, as there seems to be a problem with them when linking
	# sed -e '/pub static mut per_lcore__/d' "$outputFolderPath"/statics/lcore.rs
	#
	# # Use the correct definition
	# sed -i -e 's/0usize/RTE_MAX_ETHPORTS/g' "$outputFolderPath"/statics/rte_eth.rs
	#
	# # Fix duplicate padding name
	# sed -i -e 's/_bindgen_padding_0_: \[u64; 16usize\]/_bindgen_padding_1_: [u64; 16usize]/g' "$outputFolderPath"/structs/tle_dring.rs
}
