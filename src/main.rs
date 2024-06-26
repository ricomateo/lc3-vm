use crate::constants::*;
use crate::instructions::*;
use crate::traps::*;
use std::env;
use utils::read_image;
pub mod constants;
pub mod instructions;
pub mod traps;
pub mod utils;

fn trap_handler(
    instr: u16,
    mut reg: &mut [u16; R_COUNT],
    mut memory: &mut [u16; MEMORY_MAX],
    mut running: &mut usize,
) {
    //println!("trap_handler");
    reg[R_R7] = reg[R_PC];
    match instr & 0xFF {
        TRAP_GETC => getc(&mut reg),
        TRAP_OUT => output_character(&mut reg),
        TRAP_IN => input_character(&mut reg),
        TRAP_PUTS => puts(&mut reg, &mut memory),
        TRAP_PUTSP => putsp(&mut reg, &mut memory),
        TRAP_HALT => halt(&mut running),
        _ => panic!("invalid trap code!"),
    }
}

fn start_vm(image_path: String) {
    let mut memory: [u16; MEMORY_MAX] = [0; MEMORY_MAX];

    let mut reg: [u16; R_COUNT] = [0; R_COUNT];

    /* since exactly one condition flag should be set at any given time, set the Z flag */
    reg[R_COND] = FL_ZRO;
    read_image(&image_path, &mut memory).expect("error while reading obj file");
    /* set the PC to starting position */
    /* 0x3000 is the default */
    let pc_start = 0x3000;
    reg[R_PC] = pc_start;

    let mut running = 1;
    let mut _i = 0;
    while running == 1 {
        /* FETCH */
        //reg[R_PC] += 1;
        let instr: u16 = memory[reg[R_PC] as usize];
        let op: u16 = instr >> 12;
        reg[R_PC] += 1;
        _i += 1;
        match op {
            OP_ADD => add(instr, &mut reg),
            OP_LDI => ldi(instr, &mut reg, &mut memory),
            OP_AND => and(instr, &mut reg),
            OP_LD => ld(instr, &mut reg, &mut memory),
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
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        panic!("File argument is missing");
    }
    let filename: String = args[1].clone();
    let image_path = "../".to_string() + &filename;
    start_vm(image_path);
}
