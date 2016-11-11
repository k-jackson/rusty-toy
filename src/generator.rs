use tree::Node as Node;
use elfwriter;
use std::error::Error;
use std::io;
use std::fs::File as File;
use std::io::prelude::*;
use std::path::Path;


pub fn generate(ast: Node, output_file: &str) {
    // Traverse AST, output magic

    // Write to asm file
    let mut file = make_output_file(output_file);
    if file.is_ok() {
        println!("did it");
        write_elf(&mut file.unwrap());
    } else {
        panic!("Couldn't write file");
    }
}

fn make_output_file(output_file: &str) -> Result<File, io::Error> {
    let mut buff = try!(File::create(output_file));
    Ok(buff)
}

fn write_text(output_file: &mut File) {
    output_file.write_all(b"section .text\n");
    output_file.write_all(b"global _start\n\n");
    output_file.write_all(b"_start:\n");
    output_file.write_all(b"\tmov rbp, rsp\n");
    output_file.write_all(b"\tmov rax, 60\n");
    output_file.write_all(b"\tmov rdi, 0\n");
    output_file.write_all(b"\tsyscall\n");
}

fn write_elf(output_file: &mut File) {
    let elf_header = elfwriter::ElfHeaderBuilder::new().build();
    elf_header.write(output_file);
    let elf_program_header = elfwriter::ElfProgramHeaderBuilder::new().build();
    elf_program_header.write(output_file);
}

// todo: enum
fn get_register() -> String {
    return "r10".to_string()
}

fn set_identifier() {

}

fn get_instruction() {

}