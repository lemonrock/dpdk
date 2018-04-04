// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern crate dpdk_sys;
extern crate libc;
use ::core::ptr::null;
use ::dpdk_sys::RTE_CACHE_LINE_MIN_SIZE;
use ::libc::c_uint;
use ::libc::c_void;


const DefaultAlignment: c_uint = 0;
const MinimumAlignment: usize = RTE_CACHE_LINE_MIN_SIZE;

#[inline(always)]
#[no_mangle]
pub extern fn __rust_allocate(size: usize, _align: usize) -> *mut u8
{
	let alignment = if _align < MinimumAlignment
	{
		DefaultAlignment
	}
	else
	{
		_align as c_uint
	};
	
	unsafe
	{
		::dpdk_sys::rte_malloc(null(), size, alignment) as *mut u8
	}
}

#[inline(always)]
#[no_mangle]
pub extern fn __rust_deallocate(ptr: *mut u8, _old_size: usize, _align: usize)
{
	unsafe
	{
		::dpdk_sys::rte_free(ptr as *mut c_void)
	}
}

#[inline(always)]
#[no_mangle]
pub extern fn __rust_reallocate(ptr: *mut u8, _old_size: usize, size: usize, _align: usize) -> *mut u8
{
	let alignment = if _align < MinimumAlignment
	{
		DefaultAlignment
	}
	else
	{
		_align as c_uint
	};
	
	unsafe
	{
		::dpdk_sys::rte_realloc(ptr as *mut c_void, size, alignment) as *mut u8
	}
}

// Not supported by DPDK
#[inline(always)]
#[no_mangle]
pub extern fn __rust_reallocate_inplace(_ptr: *mut u8, old_size: usize, _size: usize, _align: usize) -> usize
{
	old_size
}

// Not supported by DPDK
#[inline(always)]
#[no_mangle]
pub extern fn __rust_usable_size(size: usize, _align: usize) -> usize
{
	size
}
