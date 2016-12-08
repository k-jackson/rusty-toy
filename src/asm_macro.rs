use asm_opcode as op;

pub fn exit(output: &mut Vec<u8>) -> u64 {
    output.extend(op::mov_im(op::Register::RAX, 0x3c00000000000000).iter());
    output.extend(op::mov_im(op::Register::RDI, 0x0).iter());
    output.extend(op::syscall().iter());
    22
}

pub fn print_str(output: &mut Vec<u8>, str_offset: u64, str_len: u64) -> u64 {
    output.extend(op::mov_im_32(op::Register::RAX, 0x01000000).iter()); // write
    output.extend(op::mov_im_32(op::Register::RDI, 0x01000000).iter()); // to stdout
    output.extend(op::mov_im(op::Register::RSI, str_offset).iter());
    output.extend(op::mov_im(op::Register::RDX, str_len).iter());
    output.extend(op::syscall().iter());
    32
}
