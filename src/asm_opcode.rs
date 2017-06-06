use byteorder::{BigEndian, WriteBytesExt};

#[allow(unused)]
pub enum Register {
    RAX,
    RCX,
    RDX,
    RBX,
    RSP,
    RBP,
    RSI,
    RDI,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15
}

fn get_register_operand(reg: &Register) -> u8 {
    let operand = match reg {
        &Register::RAX => 0x00,
        &Register::RCX => 0x01,
        &Register::RDX => 0x02,
        &Register::RBX => 0x03,
        &Register::RSP => 0x04,
        &Register::RBP => 0x05,
        &Register::RSI => 0x06,
        &Register::RDI => 0x07,
        &Register::R8  => 0x00,
        &Register::R9  => 0x01,
        &Register::R10 => 0x02,
        &Register::R11 => 0x03,
        &Register::R12 => 0x04,
        &Register::R13 => 0x05,
        &Register::R14 => 0x06,
        &Register::R15 => 0x07,
    };

    operand
}

// Helper method to convert u64 values to a big endian fixed-size array
fn get_val_slice(val: u64) -> [u8; 8] {
    let mut val_vec = vec![];
    val_vec.write_u64::<BigEndian>(val).unwrap();
    let mut val_slice = [0; 8];

    for i in 0..8 {
        val_slice[i] = val_vec[i];
    }

    val_slice
}

// Helper method to convert u32 values to a big endian fixed-size array
fn get_val_slice_32(val: u32) -> [u8; 4] {
    let mut val_vec = vec![];
    val_vec.write_u32::<BigEndian>(val).unwrap();
    let mut val_slice = [0; 4];

    for i in 0..4 {
        val_slice[i] = val_vec[i];
    }

    val_slice
}

// The "REX.B"(0001) bit signals to use the higher half of the GPRs
fn get_rex_opcode_reg(reg: &Register) -> u8 {
    let gpr_select = match reg {
        &Register::R8  => 0x01,
        &Register::R9  => 0x01,
        &Register::R10 => 0x01,
        &Register::R11 => 0x01,
        &Register::R12 => 0x01,
        &Register::R13 => 0x01,
        &Register::R14 => 0x01,
        &Register::R15 => 0x01,
        _              => 0x0
    };

    gpr_select
}

pub fn mov_im(reg: Register, val: u64) -> [u8; 10] {
    let mov_op: u8 = 0xb8 + get_register_operand(&reg);
    let val_slice = get_val_slice(val);
    let mut opcode = [0; 10];

    opcode[0] = 0x48 + get_rex_opcode_reg(&reg); // REX 64-bit operand
    opcode[1] = mov_op;
    for i in 2..10 {
        opcode[i] = val_slice[i - 2];
    }

    opcode
}

pub fn mov_im_32(reg: Register, val: u32) -> [u8; 5] {
    let mov_op: u8 = 0xb8 + get_register_operand(&reg);
    let val_slice = get_val_slice_32(val);
    let mut opcode = [0; 5];

    opcode[0] = mov_op;
    for i in 1..5 {
        opcode[i] = val_slice[i - 1];
    }

    opcode
}

pub fn mov_rax_to_offset(offset: u64) -> [u8; 10] {
    let mov_op: u8 = 0xA3;
    let val_slice = get_val_slice(offset);
    let mut opcode = [0; 10];

    opcode[0] = 0x48;
    opcode[1] = mov_op;
    for i in 2..10 {
        opcode[i] = val_slice[i - 2];
    }

    opcode
}

#[allow(dead_code)]
pub fn mov_offset_to_rax(offset: u64) -> [u8; 10] {
    let mov_op: u8 = 0xA1;
    let val_slice = get_val_slice(offset);
    let mut opcode = [0; 10];

    opcode[0] = 0x48;
    opcode[1] = mov_op;
    for i in 2..10 {
        opcode[i] = val_slice[i - 2];
    }

    opcode
}

pub fn syscall() -> [u8; 2] {
    [0x0F, 0x05]
}
/*
pub fn push(val: u64) -> u64 {unimplemented!()}
pub fn pop(dest: u64) -> u64 {unimplemented!()}
pub fn inc(reg: Register, val: u64) -> u64 {unimplemented!()}
pub fn dec(reg: Register, val: u64) -> u64 {unimplemented!()}
pub fn add(reg: Register, val: u64) -> u64 {unimplemented!()}
pub fn sub(reg: Register, val: u64) -> u64 {unimplemented!()}
pub fn mul(reg: Register, val: u64) -> u64 {unimplemented!()}
pub fn div(reg: Register, val: u64) -> u64 {unimplemented!()}
pub fn and(reg: Register, val: u64) -> u64 {unimplemented!()}
pub fn or(reg: Register, val: u64) -> u64 {unimplemented!()}
pub fn cmp(reg: Register, val: u64) -> u64 {unimplemented!()}
pub fn jmp(reg: Register, val: u64) -> u64 {unimplemented!()}
pub fn je(reg: Register, val: u64) -> u64 {unimplemented!()}
pub fn jne(reg: Register, val: u64) -> u64 {unimplemented!()}
*/
