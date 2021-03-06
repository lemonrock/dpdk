# This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
# Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


compile_library_name='dpdk'

compile_library()
{
	compile_dpdk_configure()
	{
		local configurationName="$1"

		cat >>"$rootOutputFolderPath"/config/defconfig_"$configurationName" <<-EOF
			# Do not build kernel modules
			CONFIG_RTE_EAL_IGB_UIO=n
			CONFIG_RTE_KNI_KMOD=n

			# Sensible configuration overrides
			CONFIG_RTE_LIBEAL_USE_HPET=y
			CONFIG_RTE_LIBRTE_I40E_16BYTE_RX_DESC=y
			CONFIG_RTE_LIBRTE_I40E_ITR_INTERVAL=16
			CONFIG_RTE_LIBRTE_IP_FRAG=y
			CONFIG_RTE_MAX_LCORE=128

			# Optimization (For Haswell and later)
			CONFIG_RTE_MACHINE=hsw
			CONFIG_RTE_FORCE_INTRINSICS=y
			CONFIG_RTE_RING_USE_C11_MEM_MODEL=y

			# Disable test apps
			CONFIG_RTE_APP_TEST=n
			CONFIG_RTE_TEST_PMD=n
			CONFIG_RTE_APP_CRYPTO_PERF=n

			# Disable stuff that does not work with musl
			CONFIG_RTE_BACKTRACE=n
			CONFIG_RTE_LIBRTE_DPAA_BUS=n
			CONFIG_RTE_LIBRTE_FSLMC_BUS=n
			CONFIG_RTE_LIBRTE_NFP_PMD=n

			# Does not work for now
			CONFIG_RTE_LIBRTE_MLX4_PMD=y
			CONFIG_RTE_LIBRTE_MLX5_PMD=y
		EOF
	}

	compile_dpdk_make()
	{
		local configurationName="$1"
		local crossCompilerPrefix="$2"

		local dpdkBuildFolderPath="$OUT_DIR"/dpdk-build
		local dpdkDestinationFolderPath="$rootOutputFolderPath"/DESTDIR
		local dpdkKernelSourcesFolderPath="$rootOutputFolderPath"/KERNEL_SOURCES

		local extraHostCFlags=''
		case "$platform" in

			Darwin)
				local libelfPrefix="$(brew --prefix libelf)"
				extraHostCFlags="-I${libelfPrefix}/include/libelf -I${libelfPrefix}/include"
			;;

		esac

		mkdir -m 0700 -p "$dpdkBuildFolderPath"
		mkdir -m 0700 -p "$dpdkDestinationFolderPath"
		mkdir -m 0700 -p "$dpdkKernelSourcesFolderPath"

		cd "$rootOutputFolderPath" 1>/dev/null 2>/dev/null

			# This is for our own patches to buildtools/check-experimental-syms.sh
			export CROSS="$crossCompilerPrefix"

			make \
				-j $numberOfMakeJobs \
				install \
				T="$configurationName" \
				V=1 \
				O="$dpdkBuildFolderPath" \
				DESTDIR="$dpdkDestinationFolderPath" \
				RTE_KERNELDIR="$dpdkKernelSourcesFolderPath" \
				prefix=/usr \
				CROSS="$crossCompilerPrefix" \
				EXTRA_CFLAGS="-msse4.2 -O3 -D_GNU_SOURCE -D_BSD_SOURCE -I$muslIncludeFolderPath -I"$rootOutputFolderPath"/extra-musl-headers -isystem${DEP_LIBNUMA_ROOT}/include -I${DEP_RDMA_CORE_ROOT}/include -Wno-pointer-to-int-cast" \
				EXTRA_LDFLAGS="-L${DEP_LIBNUMA_ROOT}/lib -L${DEP_RDMA_CORE_ROOT}/lib" \
				EXTRA_HOST_CFLAGS="$extraHostCFlags" \
				1>&2

		cd - 1>/dev/null 2>/dev/null
	}

	if [ -z "${DEP_LIBNUMA_ROOT+is_unset}" ]; then
		compile_fail 'Please specify the environment variable DEP_LIBNUMA_ROOT which must point to a sys-root folder path containing an include and a lib folder'
	fi

	if [ -z "${DEP_RDMA_CORE_ROOT+is_unset}" ]; then
		compile_fail 'Please specify the environment variable DEP_RDMA_CORE_ROOT which must point to a sys-root folder path containing an include and a lib folder'
	fi

	local configurationName='x86_64-native-linuxapp-gcc'
	local crossCompilerPrefix='x86_64-linux-musl-'

	compile_dpdk_configure "$configurationName" 2>&1

	compile_dpdk_make "$configurationName" "$crossCompilerPrefix" 2>&1
}

cargo_key_value_pairs()
{
	sed -e 's;^GROUP ( ;;g' -e 's; )$;;g' -e 's;\.a ; ;g' -e 's;\.a$;;g' -e 's;^lib;;g' -e 's; lib; ;g' "$rootOutputFolderPath"/DESTDIR/usr/lib/libdpdk.a >"$rootOutputFolderPath"/libraries.txt

	cat "$rootOutputFolderPath"/libraries.txt | xargs -n 1 printf 'cargo:rustc-link-lib=static-nobundle=%s\n'

	# Search path
	cargo_key_value_pairs_search 'native' "$OUT_DIR"/root/usr/lib

	# Not used by us, but potentially used by downstream crates.
	cargo_key_value_pairs_other 'root' "$OUT_DIR"/root
	cargo_key_value_pairs_other 'include' "$OUT_DIR"/root/usr/include/dpdk
	cargo_key_value_pairs_other 'libdir' "$OUT_DIR"/root/usr/lib
}
