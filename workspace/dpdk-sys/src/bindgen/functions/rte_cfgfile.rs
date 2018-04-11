// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn rte_cfgfile_add_entry(cfg: *mut rte_cfgfile, sectionname: *const c_char, entryname: *const c_char, entryvalue: *const c_char) -> c_int;
	pub fn rte_cfgfile_add_section(cfg: *mut rte_cfgfile, sectionname: *const c_char) -> c_int;
	pub fn rte_cfgfile_close(cfg: *mut rte_cfgfile) -> c_int;
	pub fn rte_cfgfile_create(flags: c_int) -> *mut rte_cfgfile;
	pub fn rte_cfgfile_get_entry(cfg: *mut rte_cfgfile, sectionname: *const c_char, entryname: *const c_char) -> *const c_char;
	pub fn rte_cfgfile_has_entry(cfg: *mut rte_cfgfile, sectionname: *const c_char, entryname: *const c_char) -> c_int;
	pub fn rte_cfgfile_has_section(cfg: *mut rte_cfgfile, sectionname: *const c_char) -> c_int;
	pub fn rte_cfgfile_load(filename: *const c_char, flags: c_int) -> *mut rte_cfgfile;
	pub fn rte_cfgfile_load_with_params(filename: *const c_char, flags: c_int, params: *const rte_cfgfile_parameters) -> *mut rte_cfgfile;
	pub fn rte_cfgfile_num_sections(cfg: *mut rte_cfgfile, sec_name: *const c_char, length: usize) -> c_int;
	pub fn rte_cfgfile_save(cfg: *mut rte_cfgfile, filename: *const c_char) -> c_int;
	pub fn rte_cfgfile_section_entries(cfg: *mut rte_cfgfile, sectionname: *const c_char, entries: *mut rte_cfgfile_entry, max_entries: c_int) -> c_int;
	pub fn rte_cfgfile_section_entries_by_index(cfg: *mut rte_cfgfile, index: c_int, sectionname: *mut c_char, entries: *mut rte_cfgfile_entry, max_entries: c_int) -> c_int;
	pub fn rte_cfgfile_section_num_entries(cfg: *mut rte_cfgfile, sectionname: *const c_char) -> c_int;
	pub fn rte_cfgfile_section_num_entries_by_index(cfg: *mut rte_cfgfile, sectionname: *mut c_char, index: c_int) -> c_int;
	pub fn rte_cfgfile_sections(cfg: *mut rte_cfgfile, sections: *mut *mut c_char, max_sections: c_int) -> c_int;
	pub fn rte_cfgfile_set_entry(cfg: *mut rte_cfgfile, sectionname: *const c_char, entryname: *const c_char, entryvalue: *const c_char) -> c_int;
}
