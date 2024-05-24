use std::mem;

use utils::read_image;

use crate::instructions::*;
use crate::constants::*;
use crate::traps::*;
pub mod instructions;
pub mod constants;
pub mod utils;
pub mod traps;

fn trap_handler(instr: usize, mut reg: &mut [usize; R_COUNT], mut memory: &mut [usize; MEMORY_MAX], mut running: &mut usize) {
    println!("trap_handler");
    reg[R_R7] = reg[R_PC];
    match instr & 0xFF {
        TRAP_GETC => getc(&mut reg),
        TRAP_OUT => output_character(&mut reg),
        TRAP_IN => input_character(&mut reg),
        TRAP_PUTS => puts(&mut reg, &mut memory),
        TRAP_PUTSP => putsp(&mut reg, &mut memory),
        TRAP_HALT => halt(&mut running),
        _ => println!("trap {:X} not implemented yet", instr & 0xFF),
    }
}


fn start_vm() {   
    let mut memory: [usize; MEMORY_MAX] = [0; MEMORY_MAX];

    let mut reg: [usize; R_COUNT] = [0; R_COUNT];

    /* since exactly one condition flag should be set at any given time, set the Z flag */
    reg[R_COND] = FL_ZRO;
    read_image("../2048.obj", &mut memory).expect("error while reading obj file");
    /* set the PC to starting position */
    /* 0x3000 is the default */
    let pc_start = 0x3000;
    reg[R_PC] = pc_start;

    for i in 0..memory.len() {
        //println!("memory[{i}] = {}", memory[i]);
    }

    let mut running = 1;
    while running == 1 {
        /* FETCH */
        reg[R_PC] += 1;
        let instr: usize = memory[(reg[R_PC] as u16) as usize];
        let op: usize = instr >> 12;
        match op {
            OP_ADD => add(instr, &mut reg),
            OP_LDI => ldi(instr, &mut reg, &memory),
            OP_AND => and(instr, &mut reg),
            OP_LD => ld(instr, &mut reg, &memory),
            OP_ST => store(instr, &mut reg, &mut memory),
            OP_JSR => jump_register(instr, &mut reg),
            OP_LDR => ldr(instr, &mut reg, &mut memory),
            OP_STR => store_register(instr, &mut reg, &mut memory),
            OP_NOT => not(instr, &mut reg),
            OP_STI => store_indirect(instr, &mut reg, &mut memory),
            OP_JMP => jump(instr, &mut reg),
            OP_LEA => lea(instr, &mut reg),
            OP_BR => branch(instr, &mut reg),
            OP_TRAP => trap_handler(instr, &mut reg, &mut memory, &mut running),
            _ => println!("not implemented yet"),
        }
        // if reg[R_PC] == MEMORY_MAX - 1 {
        //     reg[R_PC] = 0;
        // }
    }
}

fn main() {
    start_vm();
}
