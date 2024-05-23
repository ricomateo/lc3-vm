use crate::instructions::instructions::*;
use crate::constants::constants::*;
pub mod instructions;
pub mod constants;
pub mod utils;




fn start_vm() {   
    let memory: [usize; MEMORY_MAX] = [0; MEMORY_MAX];

    let mut reg: [usize; R_COUNT] = [0; R_COUNT];

    /* since exactly one condition flag should be set at any given time, set the Z flag */
    reg[R_COND] = FL_ZRO;

    /* set the PC to starting position */
    /* 0x3000 is the default */
    let PC_START = 0x3000;
    reg[R_PC] = PC_START;

    let running = 1;
    while (running == 1) {
        /* FETCH */
        let instr: usize = memory[reg[R_PC] + 1];
        let op: usize = instr >> 12;

        match op {
            OP_ADD => add(instr, reg),
            OP_LDI => ldi(instr, reg, memory),
            _ => println!("not implemented yet"),
        }
    }
}

fn main() {
    
}
