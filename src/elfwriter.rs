use std::io::prelude::*;
use std::fs::File;
use bytewriter::ByteWriter;

struct ElfHeader {
    e_ident: [u8; 8],
    e_ident_pad: u64,
    e_type: u16,
    e_machine: u16,
    e_version: u32,
    e_entry: u64,
    e_phoff: u64,
    e_shoff: u64,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16,
}
impl ElfHeader {

    pub fn write(&self, output_file: &mut File) {
        self.e_ident.write(output_file);
        self.e_ident_pad.write(output_file);
        self.e_type.write(output_file);
        self.e_machine.write(output_file);
        self.e_version.write(output_file);
        self.e_entry.write(output_file);
        self.e_phoff.write(output_file);
        self.e_shoff.write(output_file);
        self.e_flags.write(output_file);
        self.e_ehsize.write(output_file);
        self.e_phentsize.write(output_file);
        self.e_phnum.write(output_file);
        self.e_shentsize.write(output_file);
        self.e_shnum.write(output_file);
        self.e_shstrndx.write(output_file);
    }
}

pub struct ElfHeaderBuilder {
    e_ident: [u8; 8],
    e_ident_pad: u64,
    e_type: u16,
    e_machine: u16,
    e_version: u32,
    e_entry: u64,
    e_phoff: u64,
    e_shoff: u64,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16,
}

impl ElfHeaderBuilder {
    // Defaults for an executable
    pub fn new() -> ElfHeaderBuilder {
        ElfHeaderBuilder {
            e_ident: [
                0x7f, 0x45, 0x4c, 0x46, // 0x7f E L F
                0x02, // 64 bit
                0x01, // Little Endian data
                0x01, // ELF format v1
                0x00, // OS ABI v0
            ],
            e_ident_pad: 0x0,
            e_type:      0x0200,             // Executable file
            e_machine:   0x3e00,             // AMD64
            e_version:   0x01000000,         // V1
            e_entry:     0xb000400000000000, // IP entry point
            e_phoff:     0x4000000000000000, // Program Header Table offset
            e_shoff:     0x0,                // Section Header Table offset (0 == none)
            e_flags:     0x0,                // Arch flags: n/a on i386
            e_ehsize:    0x4000,             // Size of ELF header -_-
            e_phentsize: 0x3800,             // Program header size
            e_phnum:     0x0100,             // Number of program headers
            e_shentsize: 0x4000,             // Section header size
            e_shnum:     0x0000,             // Number of section headers
            e_shstrndx:  0x0000              // Section header string index
        }
    }

    pub fn set_type(&mut self, is_executable: bool) {
        self.e_type = if is_executable { 0x0200 } else { 0x0100 }
    }

    pub fn set_entry(&mut self, entry_point: u64) {
        self.e_entry = entry_point;
    }

    pub fn set_phoff(&mut self, phoff: u64) {
        self.e_phoff = phoff;
    }

    pub fn set_phentsize(&mut self, phentsize: u16) {
        self.e_phentsize = phentsize;
    }

    pub fn set_shoff(&mut self, shoff: u64) {
        self.e_shoff = shoff;
    }

    pub fn set_shentsize(&mut self, shentsize: u16) {
        self.e_shentsize = shentsize;
    }

    pub fn set_shnum(&mut self, shnum: u16) {
        self.e_shnum = shnum;
    }

    pub fn set_shstrndx(&mut self, shstrndx: u16) {
        self.e_shstrndx = shstrndx;
    }

    pub fn build(&self) -> ElfHeader {
        ElfHeader {
            e_ident: self.e_ident,
            e_ident_pad: self.e_ident_pad,
            e_type: self.e_type,
            e_machine: self.e_machine,
            e_version: self.e_version,
            e_entry: self.e_entry,
            e_phoff: self.e_phoff,
            e_shoff: self.e_shoff,
            e_flags: self.e_flags,
            e_ehsize: self.e_ehsize,
            e_phentsize: self.e_phentsize,
            e_phnum: self.e_phnum,
            e_shentsize: self.e_shentsize,
            e_shnum: self.e_shnum,
            e_shstrndx: self.e_shstrndx
        }
    }
}

struct ElfProgramHeader {
    p_type: u32,
    p_flags: u32,
    p_offset: u64,
    p_vaddr: u64,
    p_paddr: u64,
    p_filesz: u64,
    p_memsz: u64,
    p_align: u64
}
impl ElfProgramHeader {
    pub fn write(&self, output_file: &mut File) {
        self.p_type.write(output_file);
        self.p_flags.write(output_file);
        self.p_offset.write(output_file);
        self.p_vaddr.write(output_file);
        self.p_paddr.write(output_file);
        self.p_filesz.write(output_file);
        self.p_memsz.write(output_file);
        self.p_align.write(output_file);
    }
}

pub struct ElfProgramHeaderBuilder {
    p_type: u32,
    p_flags: u32,
    p_offset: u64,
    p_vaddr: u64,
    p_paddr: u64,
    p_filesz: u64,
    p_memsz: u64,
    p_align: u64
}

impl ElfProgramHeaderBuilder {
    pub fn new() -> ElfProgramHeaderBuilder {
        ElfProgramHeaderBuilder {
            p_type:   0x01000000,         // Loadable segment
            p_flags:  0x05000000,         // r+x segment
            p_offset: 0x0,                // offset of segment's first byte from start of segment?
            p_vaddr:  0x0000400000000000, // Virtual memory destination address
            p_paddr:  0x0000400000000000, // Physical memory destination address (typically N/A)
            p_filesz: 0xFF00000000000000, // Size of file image for segment
            p_memsz:  0xFF00000000000000, // Size of memory image for segment
            p_align:  0x0000200000000000  // Value to which segments are aligned in memory + file
        }
    }

    // todo
    pub fn set_type() {

    }

    // todo
    pub fn set_flags() {

    }

    pub fn set_addr(&mut self, dest_addr: u64) {
        self.p_vaddr = dest_addr;
        self.p_paddr = dest_addr;
    }

    pub fn set_filesz(&mut self, file_size: u64) {
        self.p_filesz = file_size;
    }

    pub fn set_memsz(&mut self, mem_size: u64) {
        self.p_memsz = mem_size;
    }

    pub fn set_align(&mut self, alignment: u64) {
        self.p_align = alignment;
    }

    pub fn build(&self) -> ElfProgramHeader {
        ElfProgramHeader {
            p_type: self.p_type,
            p_flags: self.p_flags,
            p_offset: self.p_offset,
            p_vaddr: self.p_vaddr,
            p_paddr: self.p_paddr,
            p_filesz: self.p_filesz,
            p_memsz: self.p_memsz,
            p_align: self.p_align
        }
    }
}
