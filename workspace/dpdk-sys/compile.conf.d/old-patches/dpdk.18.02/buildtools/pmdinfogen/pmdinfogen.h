
/* Postprocess pmd object files to export hw support
 *
 * Copyright 2016 Neil Horman <nhorman@tuxdriver.com>
 * Based in part on modpost.c from the linux kernel
 *
 * This software may be used and distributed according to the terms
 * of the GNU General Public License V2, incorporated herein by reference.
 *
 */

#include <stdio.h>
#include <stdlib.h>
#include <stdarg.h>
#include <string.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <sys/mman.h>
#ifdef __linux__
#include <endian.h>
#elif __APPLE__
#include <libkern/OSByteOrder.h>
#define htobe16(x) OSSwapHostToBigInt16(x)
#define htole16(x) OSSwapHostToLittleInt16(x)
#define be16toh(x) OSSwapBigToHostInt16(x)
#define le16toh(x) OSSwapLittleToHostInt16(x)
#define htobe32(x) OSSwapHostToBigInt32(x)
#define htole32(x) OSSwapHostToLittleInt32(x)
#define be32toh(x) OSSwapBigToHostInt32(x)
#define le32toh(x) OSSwapLittleToHostInt32(x)
#define htobe64(x) OSSwapHostToBigInt64(x)
#define htole64(x) OSSwapHostToLittleInt64(x)
#define be64toh(x) OSSwapBigToHostInt64(x)
#define le64toh(x) OSSwapLittleToHostInt64(x)
#else
#include <sys/endian.h>
#endif
#include <fcntl.h>
#include <unistd.h>
#ifdef __APPLE__
#define __LIBELF_INTERNAL__ 0
#define __LIBELF_NEED_LINK_H 0
#define __LIBELF_NEED_SYS_LINK_H 0
#include <libelf/gelf.h>
#define R_386_NONE 0
#define R_386_32 1
#define R_386_PC32 2
#define R_ARM_NONE 0
#define R_ARM_PC24 1
#define R_ARM_ABS32 2
#define R_MIPS_NONE 0
#define R_MIPS_16 1
#define R_MIPS_32 2
#define R_MIPS_REL32 3
#define R_MIPS_26 4
#define R_MIPS_HI16 5
#define R_MIPS_LO16 6
#define R_IA64_IMM64 0x23 /* symbol + addend, mov imm64 */
#define R_PPC_ADDR32 1 /* 32bit absolute address */
#define R_PPC64_ADDR64 38 /* doubleword64 S + A */
#define R_SH_DIR32 1
#define R_SPARC_64 32 /* Direct 64 bit */
#define R_X86_64_64 1 /* Direct 64 bit */
#define R_390_32 4 /* Direct 32 bit. */
#define R_390_64 22 /* Direct 64 bit. */
#define R_MIPS_64 18
#else
#include <elf.h>
#endif
#include <rte_config.h>
#include <rte_pci.h>

/* On BSD-alike OSes elf.h defines these according to host's word size */
#undef ELF_ST_BIND
#undef ELF_ST_TYPE
#undef ELF_R_SYM
#undef ELF_R_TYPE

/*
 * Define ELF64_* to ELF_*, the latter being defined in both 32 and 64 bit
 * flavors in elf.h.  This makes our code a bit more generic between arches
 * and allows us to support 32 bit code in the future should we ever want to
 */
#ifdef RTE_ARCH_64
#define Elf_Ehdr    Elf64_Ehdr
#define Elf_Shdr    Elf64_Shdr
#define Elf_Sym     Elf64_Sym
#define Elf_Addr    Elf64_Addr
#define Elf_Sword   Elf64_Sxword
#define Elf_Section Elf64_Half
#define ELF_ST_BIND ELF64_ST_BIND
#define ELF_ST_TYPE ELF64_ST_TYPE

#define Elf_Rel     Elf64_Rel
#define Elf_Rela    Elf64_Rela
#define ELF_R_SYM   ELF64_R_SYM
#define ELF_R_TYPE  ELF64_R_TYPE
#else
#define Elf_Ehdr    Elf32_Ehdr
#define Elf_Shdr    Elf32_Shdr
#define Elf_Sym     Elf32_Sym
#define Elf_Addr    Elf32_Addr
#define Elf_Sword   Elf32_Sxword
#define Elf_Section Elf32_Half
#define ELF_ST_BIND ELF32_ST_BIND
#define ELF_ST_TYPE ELF32_ST_TYPE

#define Elf_Rel     Elf32_Rel
#define Elf_Rela    Elf32_Rela
#define ELF_R_SYM   ELF32_R_SYM
#define ELF_R_TYPE  ELF32_R_TYPE
#endif


/*
 * Note, it seems odd that we have both a CONVERT_NATIVE and a TO_NATIVE macro
 * below.  We do this because the values passed to TO_NATIVE may themselves be
 * macros and need both macros here to get expanded.  Specifically its the width
 * variable we are concerned with, because it needs to get expanded prior to
 * string concatenation
 */
#define CONVERT_NATIVE(fend, width, x) ({ \
typeof(x) ___x; \
if ((fend) == ELFDATA2LSB) \
	___x = le##width##toh(x); \
else \
	___x = be##width##toh(x); \
	___x; \
})

#define TO_NATIVE(fend, width, x) CONVERT_NATIVE(fend, width, x)

enum opt_params {
	PMD_PARAM_STRING = 0,
	PMD_KMOD_DEP,
	PMD_OPT_MAX
};

struct pmd_driver {
	Elf_Sym *name_sym;
	const char *name;
	struct rte_pci_id *pci_tbl;
	struct pmd_driver *next;

	const char *opt_vals[PMD_OPT_MAX];
};

struct elf_info {
	unsigned long size;
	Elf_Ehdr     *hdr;
	Elf_Shdr     *sechdrs;
	Elf_Sym      *symtab_start;
	Elf_Sym      *symtab_stop;
	char         *strtab;

	/* support for 32bit section numbers */

	unsigned int num_sections; /* max_secindex + 1 */
	unsigned int secindex_strings;
	/* if Nth symbol table entry has .st_shndx = SHN_XINDEX,
	 * take shndx from symtab_shndx_start[N] instead
	 */
	Elf32_Word   *symtab_shndx_start;
	Elf32_Word   *symtab_shndx_stop;

	struct pmd_driver *drivers;
};

