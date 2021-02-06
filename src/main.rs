#[allow(dead_code)]
#[derive(Debug)]
enum OpCode {
    LD = 0x00,
    ST = 0x01,
    ADD = 0x02,
    XOR = 0x03,
}

fn add(op0: u16, op1: u16) -> u16 {
    op0 + op1
}

fn xor(op0: u16, op1: u16) -> u16 {
    op0 ^ op1
}

fn main() {
    // ADD R0, R1
    // instruction 0x201 decodes into:
    //   opcode: 0x2 (ADD)
    //   op0: 0 (Index 0 into the registers table, i.e. R0)
    //   op0: 1 (Index 1 into the registers table, i.e. R1)
    let insn: u16 = 0x201;
    let mut r0: u16 = 10;
    let r1: u16 = 20;

    println!(
        "do-core-1: instruction {:#x?} Initial CPU state [R0:{:#x?} R1:{:#x?}]",
        insn, r0, r1
    );

    let opcode = (insn >> 8) as u8;
    let op0 = ((insn & 0xf0) >> 4) as u8;
    let op1: u8 = (insn & 0xf) as u8;

    println!(
        "do-core-1: instruction decoded into [opcode:{:?} op0:{} op1:{}]",
        opcode, op0, op1
    );

    match opcode {
        OpCode::ADD => r0 = add(r0, r1),
        OpCode::XOR => r0 = xor(r0, r1),
        _ => panic!("Unknown opcode {:?}", opcode),
    }

    println!(
        "do-core-1: instruction {:#x?} Final CPU state [R0:{:#x?} R1:{:#x?}]",
        insn, r0, r1
    );
}
