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

    pub fn write_header(&self, output_file: &mut File) {
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
            e_phoff:     0x0,                // Program Header Table offset
            e_shoff:     0x0,                // Section Header Table offset (0 == none)
            e_flags:     0x0,                // Arch flags: n/a on i386
            e_ehsize:    0x4000,             // Size of ELF header -_-
            e_phentsize: 0x4000,             // Program header size
            e_phnum:     0x0000,             // Number of program headers
            e_shentsize: 0x4000,             // Section header size
            e_shnum:     0x0000,             // Number of section headers
            e_shstrndx:  0x0000              // Section header string index
        }
    }

    pub fn set_type(&self, is_executable: bool) {

    }

    pub fn set_entry(&self, entry_point: u64) {

    }

    pub fn set_phoff(&self, phoff: u64) {

    }

    pub fn set_phentsize(&self, phentsize: u16) {

    }

    pub fn set_shoff(&self, shoff: u64) {

    }

    pub fn set_shentsize(&self, shentsize: u16) {

    }

    pub fn set_shnum(&self, shnum: u16) {

    }

    pub fn set_shstrndx(&self, shstrndx: u16) {

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

