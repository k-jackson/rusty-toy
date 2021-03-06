use tree::Node as Node;
use tree::ASTNodeKind as ASTNodeKind;
use elfwriter;
use std::io;
use std::fs::File as File;
use bytewriter::ByteWriter;
use asm::Assembler as Assembler;
use constdata::ConstData as ConstData;
use std::str::FromStr;

pub fn generate(ast: Vec<Node>, const_data: &ConstData, output_file: &str) {
    // Traverse AST, output magic

    // Write to asm file
    let file = make_output_file(output_file);
    if file.is_ok() {

        write_elf(&mut file.unwrap(), ast, const_data);
    } else {
        panic!("Couldn't write file");
    }
}

fn make_output_file(output_file: &str) -> Result<File, io::Error> {
    let buff = File::create(output_file)?;
    Ok(buff)
}

fn build_asm(ast: Vec<Node>, const_data: &ConstData) -> Assembler {
    let mut asm = Assembler {output: Vec::new(), length: 0, const_data: const_data};
    for nodes in ast {
        walk_ast(nodes, &mut asm);
    }
    asm.exit();

    asm
}

fn walk_ast(ast: Node, asm: &mut Assembler) {
    let mut operands: Vec<&Node> = Vec::new();
    let traversal = ast.traverse_postorder();
    for n in traversal {
        match n.kind {
            ASTNodeKind::FunctionCall => {
                let func_name: String = n.val.clone().unwrap();
                let func_param: String = operands.pop().unwrap().clone().val.unwrap();

                if is_function_builtin(&func_name) {
                    asm.builtin_function(&func_name, &func_param);
                }
            },
            // Assignment statements have the variable on the left (thus lowest of two stack entries)
            ASTNodeKind::Assignment => {
                let value: &str = &operands.pop().unwrap().clone().val.unwrap();
                let variable: String = operands.pop().unwrap().clone().val.unwrap();
                asm.assignment_statement(&variable, u64::from_str(value).unwrap());

            },
            ASTNodeKind::Integer => {
                operands.push(n)
            },
            ASTNodeKind::Variable => {
                operands.push(n)
            }
        }
    }
}

fn is_function_builtin(function_name: &str) -> bool {
    match function_name {
        "print" => true,
        _ => false
    }
}

fn write_elf(output_file: &mut File, ast: Vec<Node>, const_data: &ConstData) {
    let const_section_data = const_data.get_data();

    let mut elf_header = elfwriter::ElfHeader::new();
    let elf_text_program_header = elfwriter::ElfProgramHeader::new();
    let mut elf_data_program_header = elfwriter::ElfProgramHeader::new();

    // ELF header (64) + .text phead (56) + .data phead (56)
    let sh_data_offset: u64 = 176;
    let sh_data_length: u64 = const_section_data.len() as u64;

    let section_header_count: u16 =  4;
    let section_header_size: u16 = 64;

    let asm_offset = sh_data_offset + sh_data_length;
    let assembler = build_asm(ast, &const_data);
    let asm_length = assembler.get_length();
    let asm_data = assembler.get_output();

    let section_header_offset: u64 = asm_offset + asm_length;

    // Instructions start at end of .data
    elf_header.set_entry(0xb000400000000000 + sh_data_length.to_be());
    elf_header.set_shnum(section_header_count.to_be());
    elf_header.set_shentsize(section_header_size.to_be());
    elf_header.set_shoff(section_header_offset.to_be());

    // shstrtab contents
    let mut elf_string_table = elfwriter::ElfStringTable::new();
    elf_string_table.add_string(String::from(".shstrtab"));
    elf_string_table.add_string(String::from(".text"));
    elf_string_table.add_string(String::from(".data"));
    let sh_strtab_length = elf_string_table.get_table_size();

    // mandatory "null" section header first
    let mut sh_null = elfwriter::ElfSectionHeader::new();
    sh_null
        .set_flags(0x0)
        .set_align(0x0)
        .set_size(0x0)
        .set_type(0x00000000);

    let mut sh_data = elfwriter::ElfSectionHeader::new();
    sh_data
        .set_flags(0x0300000000000000)
        .set_name(0x11000000)
        .set_size(sh_data_length.to_be())
        .set_addr(0x0000800000000000)
        .set_align(0x0400000000000000)
        .set_offset(sh_data_offset.to_be())
        .set_type(0x01000000);

    let sh_text_offset: u64 = asm_offset;
    let sh_text_length: u64 = asm_length;

    let mut sh_text = elfwriter::ElfSectionHeader::new();
    sh_text
        .set_flags(0x0600000000000000)
        .set_name(0x0b000000)
        .set_align(0x1000000000000000)
        .set_addr(0x8000400000000000)
        .set_size(sh_text_length.to_be())
        .set_offset(sh_text_offset.to_be())
        .set_type(0x01000000);

    // shstrtab
    let sh_strtab_offset: u64 = section_header_offset + (section_header_count as u64 * section_header_size as u64);
    let mut sh_strtab = elfwriter::ElfSectionHeader::new();
    sh_strtab
        .set_flags(0)
        .set_name(0x01000000)
        .set_size(sh_strtab_length.to_be())
        .set_offset(sh_strtab_offset.to_be())
        .set_align(0x0100000000000000)
        .set_type(0x03000000);
    // add the little-endian virtual offset to the data offset, to get the virtual address of .data
    let sh_data_virtual_offset: u64 =  0x0000000000800000 + sh_data_offset;
    elf_data_program_header
        .set_addr(sh_data_virtual_offset.to_be())
        .set_offset(sh_data_offset.to_be())
        .set_size(sh_data_length.to_be())
        .set_flags(0x06000000); // R+W

    // generate madness
    elf_header.write(output_file);
    elf_text_program_header.write(output_file);
    elf_data_program_header.write(output_file);
    const_section_data.as_slice().write(output_file);
    asm_data.as_slice().write(output_file);
    sh_null.write(output_file);
    sh_data.write(output_file);
    sh_text.write(output_file);
    sh_strtab.write(output_file);
    elf_string_table.write(output_file);
}
