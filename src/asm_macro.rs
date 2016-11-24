use asm_opcode as op;

pub fn exit(output: &mut Vec<u8>) -> u64 {
    output.extend(op::mov_im(op::Register::RAX, 0x3c00000000000000).iter());
    output.extend(op::mov_im(op::Register::RDI, 0x0).iter());
    output.extend(op::syscall().iter());
    22
}

