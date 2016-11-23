pub fn exit(output: &mut Vec<u8>) -> u64 {
    let mov_eax_60: [u8; 5] = [0xB8, 0x3c, 0, 0, 0];
    let mov_edi_0: [u8; 5]  = [0xBF, 0, 0, 0, 0];
    let syscall: [u8; 2]    = [0x0F, 0x05];
    output.extend(&mov_eax_60);
    output.extend(&mov_edi_0);
    output.extend(&syscall);
    12
}

