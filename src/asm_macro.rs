use std::fs::File as File;
use bytewriter::ByteWriter;

pub fn exit(f: &mut File) -> u64 {
    let mov_rax_60: [u8; 5] = [0xB8, 0x3c, 0x00, 0x00, 0x00];
    let mov_rdi_0: [u8; 5]  = [0xBF, 0x00, 0x00, 0x00, 0x00];
    let syscall: [u8; 2]    = [0x0F, 0x05];
    let pad: [u8; 4]        = [0,0,0,0];
    mov_rax_60.write(f);
    mov_rdi_0.write(f);
    syscall.write(f);
    pad.write(f);
    16
}

