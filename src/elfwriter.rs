use std::collections::HashMap;
use std::io::prelude::*;
use std::fs::File;
use bytewriter::ByteWriter;

pub struct ElfHeader {
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

    // Defaults for an executable
    pub fn new() -> ElfHeader {
        ElfHeader {
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
            e_shoff:     0x8000000000000000, // Section Header Table offset (0 == none)
            e_flags:     0x0,                // Arch flags: n/a on i386
            e_ehsize:    0x4000,             // Size of ELF header -_-
            e_phentsize: 0x3800,             // Program header size
            e_phnum:     0x0100,             // Number of program headers
            e_shentsize: 0x4000,             // Section header size
            e_shnum:     0x0200,             // Number of section headers
            e_shstrndx:  0x0100              // Which section header is the shstrtab
        }
    }

    pub fn set_type(&mut self, is_executable: bool) -> &mut ElfHeader {
        self.e_type = if is_executable { 0x0200 } else { 0x0100 };
        self
    }

    pub fn set_entry(&mut self, entry_point: u64) -> &mut ElfHeader {
        self.e_entry = entry_point;
        self
    }

    pub fn set_phoff(&mut self, phoff: u64) -> &mut ElfHeader {
        self.e_phoff = phoff;
        self
    }

    pub fn set_phentsize(&mut self, phentsize: u16) -> &mut ElfHeader {
        self.e_phentsize = phentsize;
        self
    }

    pub fn set_shoff(&mut self, shoff: u64) -> &mut ElfHeader {
        self.e_shoff = shoff;
        self
    }

    pub fn set_shentsize(&mut self, shentsize: u16) -> &mut ElfHeader {
        self.e_shentsize = shentsize;
        self
    }

    pub fn set_shnum(&mut self, shnum: u16) -> &mut ElfHeader {
        self.e_shnum = shnum;
        self
    }

    pub fn set_shstrndx(&mut self, shstrndx: u16) -> &mut ElfHeader {
        self.e_shstrndx = shstrndx;
        self
    }

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

pub struct ElfProgramHeader {
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

    pub fn new() -> ElfProgramHeader {
        ElfProgramHeader {
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

    pub fn set_addr(&mut self, dest_addr: u64) -> &mut ElfProgramHeader {
        self.p_vaddr = dest_addr;
        self.p_paddr = dest_addr;
        self
    }

    pub fn set_filesz(&mut self, file_size: u64) -> &mut ElfProgramHeader {
        self.p_filesz = file_size;
        self
    }

    pub fn set_memsz(&mut self, mem_size: u64) -> &mut ElfProgramHeader {
        self.p_memsz = mem_size;
        self
    }

    pub fn set_align(&mut self, alignment: u64) -> &mut ElfProgramHeader {
        self.p_align = alignment;
        self
    }

    pub fn write(&self, output_file: &mut File) {
        self.p_type.write(output_file);
        self.p_flags.write(output_file);
        self.p_offset.write(output_file);
        self.p_vaddr.write(output_file);
        self.p_paddr.write(output_file);
        self.p_filesz.write(output_file);
        self.p_memsz.write(output_file);
        self.p_align.write(output_file);
        // Pad 0s to the next 0x10, to make things easier to debug/calc for now
        let pad: u64 = 0x0;
        pad.write(output_file);
    }
}

pub struct ElfSectionHeader {
    sh_name: u32,
    sh_type: u32,
    sh_flags: u64,
    sh_addr: u64,
    sh_offset: u64,
    sh_size: u64,
    sh_link: u32,
    sh_info: u32,
    sh_addralign: u64,
    sh_entsize: u64
}
impl ElfSectionHeader {

    pub fn new() -> ElfSectionHeader {
        ElfSectionHeader {
            sh_name:      0x00000000, // Name is # entry in shstrtab
            sh_type:      0x01000000, // SHT_PROGBITS
            sh_flags:     0x0000000000000000,
            sh_addr:      0x0000000000000000,
            sh_offset:    0x0000000000000000, // Offset from start of sh to start of section contents
            sh_size:      0x4000000000000000,
            sh_link:      0x00000000,
            sh_info:      0x00000000,
            sh_addralign: 0x0000000000000000,
            sh_entsize:   0x0000000000000000,
        }
    }

    pub fn set_name(&mut self, name_offset: u32) -> &mut ElfSectionHeader {
        self.sh_name = name_offset;
        self
    }

    pub fn set_size(&mut self, section_header_size: u64) -> &mut ElfSectionHeader {
        self.sh_size = section_header_size;
        self
    }

    // todo: better params
    pub fn set_type(&mut self, header_type: u32) -> &mut ElfSectionHeader {
        self.sh_type = header_type;
        self
    }

    pub fn set_offset(&mut self, offset: u64) -> &mut ElfSectionHeader {
        self.sh_offset = offset;
        self
    }

    pub fn set_flags(&mut self, flags: u64) -> &mut ElfSectionHeader {
        self.sh_flags = flags;
        self
    }

    pub fn set_align(&mut self, align: u64) -> &mut ElfSectionHeader {
        self.sh_addralign = align;
        self
    }

    pub fn write(&self, output_file: &mut File) {
        self.sh_name.write(output_file);
        self.sh_type.write(output_file);
        self.sh_flags.write(output_file);
        self.sh_addr.write(output_file);
        self.sh_offset.write(output_file);
        self.sh_size.write(output_file);
        self.sh_link.write(output_file);
        self.sh_info.write(output_file);
        self.sh_addralign.write(output_file);
        self.sh_entsize.write(output_file);
    }
}

pub struct ElfStringTable {
    st_string_bytes: Vec<u8>,
    st_string_map: HashMap<String, u64>,
    st_string_len: u64
}
impl ElfStringTable {

    pub fn new() -> ElfStringTable {
        ElfStringTable {
            st_string_bytes: vec![0x0],
            st_string_len: 1,
            st_string_map: HashMap::new(),
        }
    }

    fn append_null_byte(&mut self) {
        self.st_string_bytes.push(0x0);
        self.st_string_len += 1;
    }

    pub fn add_string(&mut self, new_string: String) -> &mut ElfStringTable {
        let str_index = self.st_string_bytes.len() as u64;
        let str_bytes = new_string.as_bytes();

        for byte in str_bytes {
            self.st_string_bytes.push(*byte);
            self.st_string_len += 1;
        }
        self.append_null_byte();

        self.st_string_map.insert(new_string.clone(), str_index);
        self
    }

    pub fn get_string_index(&self, target: String) -> Option<u64> {

        match self.st_string_map.get(&target) {
            Some(v) => Some(*v),
            None => None
        }
    }

    pub fn get_table_size_be(&self) -> u64  {
        self.st_string_len.to_be()
    }

    pub fn write(&self, output_file: &mut File) {
        for byte in &self.st_string_bytes {
            byte.write(output_file);
        }
    }
}
