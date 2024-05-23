use crate::instructions::instructions::*;
use crate::constants::constants::*;
use crate::traps::traps::*;
pub mod instructions;
pub mod constants;
pub mod utils;
pub mod traps;

fn trap_handler(instr: usize, reg: &mut [usize; R_COUNT]) {
    reg[R_R7] = reg[R_PC];
    match instr & 0xFF {
        TRAP_GETC => println!("trap_getc"),
        _ => println!("not implemented yet"),
    }
}


fn start_vm() {   
    let memory: [usize; MEMORY_MAX] = [0; MEMORY_MAX];

    let mut reg: [usize; R_COUNT] = [0; R_COUNT];

    /* since exactly one condition flag should be set at any given time, set the Z flag */
    reg[R_COND] = FL_ZRO;

    /* set the PC to starting position */
    /* 0x3000 is the default */
    let pc_start = 0x3000;
    reg[R_PC] = pc_start;

    let running = 1;
    while running == 1 {
        /* FETCH */
        let instr: usize = memory[reg[R_PC] + 1];
        let op: usize = instr >> 12;

        match op {
            OP_ADD => add(instr, &mut reg),
            //OP_LDI => ldi(instr, reg, memory),
            OP_AND => and(instr, &mut reg),
            OP_TRAP => trap_handler(instr, &mut reg),
            _ => println!("not implemented yet"),
        }
    }
}

fn main() {
    start_vm();
}
