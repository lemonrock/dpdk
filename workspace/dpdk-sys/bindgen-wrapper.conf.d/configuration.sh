# This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
# Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


_extract_from_linker_script()
{
	local linkerScript="$1"

	sed -e 's;^GROUP ( ;;g' -e 's; )$;;g' -e 's;\.a ; ;g' -e 's;\.a$;;g' -e 's;^lib;;g' -e 's; lib; ;g' "$linkerScript"
}

bindingsName='dpdk'
rootIncludeFileName='dpdk.h'
macosXHomebrewPackageNames=''
alpineLinuxPackageNames='rsync make gcc linux-headers libunwind-dev linux-grsec-dev'
clangAdditionalArguments="-I${rootOutputFolderPath}/extra-musl-headers -I${temporaryFolderPath}/includes -I${outputFolderPath}/.."
headersFolderPath='DESTDIR/usr/include/dpdk'
link="$(_extract_from_linker_script "$rootOutputFolderPath"/DESTDIR/usr/lib/libdpdk.a) tle_misc tle_dring tle_timer tle_l4p"
link_kind='static-nobundle'

bindgen_wrapper_generateStaticFunctions()
{
	local file="$1"

	local relativeHeaderFile="$file".h
	local outputFileBaseName="$file".static-inline

	local srcBindgenCFolderPath="$outputFolderPath"/c

	mkdir -m 0700 -p "$srcBindgenCFolderPath"

	local headerFilePath="$srcBindgenCFolderPath"/$outputFileBaseName.h
	local sourceFilePath="$srcBindgenCFolderPath"/$outputFileBaseName.c

	clang-format -style="{BasedOnStyle: Chromium, IndentWidth: 4, ColumnLimit: 4000, BreakBeforeBraces: Allman}" "$headersFolderPath"/"$relativeHeaderFile" \
	| sed \
		-e 's/static __rte_always_inline /static inline /g' \
		-e 's/^[ \t]*static inline /static inline /g' \
	| grep '^static inline ' \
	| sed \
		-e 's/^static inline //g' \
		-e 's/ __attribute__((always_inline))//g' \
		-e 's/\*/ */g' \
		-e 's/(void)/()/g' \
		-e 's/, /,/g' \
		-e 's/)$//g' \
		-e 's/(/,/g' \
		-e 's/ {.*$//g' \
		-e 's/)$//g' \
		-e 's/);$//g' \
	| sort -r -u -t',' -k 1,1 \
		>"$temporaryFolderPath"/$outputFileBaseName.functions

	{
		printf '#include <%s>\n' "$relativeHeaderFile" >"$headerFilePath"

		cat <<-EOF
			#include <${relativeHeaderFile}>
			#include "bindgen/c/${outputFileBaseName}.h"
		EOF

		local line
		while IFS= read -r line
		do
			local returnValueAndFunctionName="$(printf '%s\n' "$line" | cut -f1 -d ',')"
			local returnValue="$(printf '%s\n' "$returnValueAndFunctionName" | awk -F' ' '{$NF=""}1' | sed 's/ $//g')"
			local functionName="$(printf '%s\n' "$returnValueAndFunctionName" | awk -F' ' '{print $NF}')"

			local definitionOfFunction="$returnValue rust_${functionName}("

			local bodyOfFunction
			if [ "$returnValue" = 'void' ]; then
				bodyOfFunction="${functionName}("
			else
				bodyOfFunction="return ${functionName}("
			fi

			local numberOfArguments="$(printf '%s\n' "$line" | awk -F',' '{print NF-1}')"

			local argumentField
			local argumentIndex=0
			while [ $argumentIndex -lt $numberOfArguments ]
			do
				local cutIndex=$((2 + argumentIndex))

				local argumentValueAndArgumentName="$(printf '%s\n' "$line" | cut -f${cutIndex} -d ',')"
				local argumentValue="$(printf '%s\n' "$argumentValueAndArgumentName" | awk -F' ' '{$NF=""}1' | sed 's/ $//g')"
				local argumentName="$(printf '%s\n' "$argumentValueAndArgumentName" | awk -F' ' '{print $NF}')"

				if [ $argumentIndex -ne 0 ]; then
					definitionOfFunction="${definitionOfFunction}, "
					bodyOfFunction="${bodyOfFunction}, "
				fi
				definitionOfFunction="${definitionOfFunction}${argumentValue} ${argumentName}"

				local argumentNameStrippedOfArraySyntax=""${argumentName%[*}""
				bodyOfFunction="${bodyOfFunction}${argumentNameStrippedOfArraySyntax}"

				argumentIndex=$((argumentIndex+1))
			done

			definitionOfFunction="${definitionOfFunction})"
			bodyOfFunction="${bodyOfFunction});"

			printf '\n%s;\n' "$definitionOfFunction" >>"$headerFilePath"

			printf '\n%s\n{\n\t%s\n}\n' "$definitionOfFunction" "$bodyOfFunction"

		done <"$temporaryFolderPath"/$outputFileBaseName.functions
	} >"$sourceFilePath"

	printf '#include "bindgen/c/%s.h"\n' "$outputFileBaseName" >>"$srcBindgenCFolderPath"/lib.h
	printf '#include "bindgen/c/%s.c"\n' "$outputFileBaseName" >>"$srcBindgenCFolderPath"/lib.c
}

preprocess_before_headersFolderPath()
{
	local file
	for file in \
		rte_atomic \
		rte_bus_vdev \
		rte_ethdev \
		rte_ether \
		rte_event_ring \
		rte_eventdev \
		rte_io \
		rte_ip \
		rte_ip_frag \
		rte_lcore \
		rte_lpm \
		rte_mbuf \
		rte_mempool \
		rte_net \
		rte_ring \
		rte_vhost
	do
		bindgen_wrapper_generateStaticFunctions "$file"
	done
}

final_chance_to_tweak()
{
	# Make these compatible with PosixErrorNumber; can't be done as constant type mapping as conversion of unnamed enums occurs after constant type mapping
	sed -i -e 's/: u32 /: c_int /g' "$outputFolderPath"/constants/E_RTE.rs

	# Remove rte_vlog as it uses a va_list.
	sed -i -e '/pub fn rte_vlog/d' "$outputFolderPath"/functions/miscellany.rs

	# Make code work on FreeBSD - fix cpu_set_t
	{
		cat "$bindgenWrapperConfDFolderPath"/preamble.rs
		cat <<-EOF
			#[cfg(any(target_os = "android", target_os = "linux"))] pub type rte_cpuset_t = cpu_set_t;
			#[cfg(target_os = "freebsd")] pub type rte_cpuset_t = cpuset_t;
		EOF
	} >"$outputFolderPath"/types/rte_cpuset_t.rs
	sed -i -e 's/#\[repr(C)\]/#[repr(C, align(16))]/g' "$outputFolderPath"/structs/rte_thash_tuple.rs

	# Fix non-Optional callbacks in types.
	local type
	for type in \
		eth_rx_burst_t \
		eth_tx_burst_t \
		lcore_function_t \
		rte_bus_cmp_t \
		rte_bus_find_device_t \
		rte_bus_get_iommu_class_t \
		rte_bus_probe_t \
		rte_bus_scan_t \
		rte_eal_alarm_callback \
		rte_eth_dev_cb_fn \
		rte_hash_cmp_eq_t \
		rte_hash_function \
		rte_power_freq_change_t \
		rte_power_freqs_t \
		rte_power_get_freq_t \
		rte_power_set_freq_t \
		rte_service_func \
	 	rte_timer_cb_t
	do
		sed -i -e 's/Option<//g' -e 's/>;$/;/g' "$outputFolderPath"/types/"$type".rs
	done

#	# Fix non-Optional callbacks in structs.
#	# eg: Option<unsafe extern "C" fn(arg1: *mut c_void, arg2: *mut tle_stream)>,
#	local struct
#	for struct in \
#		tle_stream_cb
#	do
#		sed -i -e 's/: Option<unsafe extern "C" fn(\(.*\))>,$/: unsafe extern "C" fn(\1),/g' "$outputFolderPath"/structs/"$struct".rs
#	done

	# Use the correct definition
	sed -i -e 's/0usize/RTE_MAX_ETHPORTS/g' "$outputFolderPath"/statics/rte_eth.rs

	# Use the correct definition
	sed -i -e 's/: cpu_set_t/: rte_cpuset_t/g' "$outputFolderPath"/statics/lcore.rs

	#  Fix bindgen cussedness (1): enum with missing Ord and PartialOrd
	local enum
	for enum in \
		rte_bbdev_op_type \
		rte_bus_scan_mode \
		rte_cryptodev_scheduler_mode \
		rte_eth_nb_pools \
		rte_flow_action_type \
		rte_flow_classify_table_type \
		rte_flow_error_type \
		rte_flow_item_type \
		rte_mtr_error_type \
		rte_security_ipsec_sa_protocol \
		rte_security_ipsec_sa_mode \
		rte_security_ipsec_sa_direction \
		rte_tm_error_type
	do
		sed -i -e 's/PartialEq, Eq/PartialEq, Eq, PartialOrd, Ord/g' "$outputFolderPath"/enums/"$enum".rs
	done

	# Fix bindgen cussedness (2): Remove implementations of Debug where a struct is packed
	local struct
	for struct in \
		rte_avp_request \
		rte_config \
		rte_kni_request \
		rte_mem_config \
		rte_memseg \
		rte_memzone
	do
		mv "$outputFolderPath"/structs/"$struct".rs "$outputFolderPath"/structs/"$struct".rs.orig
		{
			local line
			while IFS='' read -r line
			do
				case "$line" in

					'impl Debug for '*)
						break
					;;

					*)
						printf '%s\n' "$line"
					;;

				esac

			done <"$outputFolderPath"/structs/"$struct".rs.orig
		} >"$outputFolderPath"/structs/"$struct".rs

		rm "$outputFolderPath"/structs/"$struct".rs.orig
	done

	# Fix bindgen cussedness (3): Fix bindgen packed structs and auto-derive'd traits which use a packed representation (or rely on a struct with one).
	local struct
	for struct in \
		esp_hdr \
		rte_flow_item_esp
	do
		sed -i -e '/#\[derive(/d' "$outputFolderPath"/structs/"$struct".rs
	done
}
