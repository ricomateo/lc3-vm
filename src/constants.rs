pub const MEMORY_MAX: usize = 2_usize.pow(16);

pub const R_R0: usize = 0;
pub const R_R1: usize = 1;
pub const R_R2: usize = 2;
pub const R_R3: usize = 3;
pub const R_R4: usize = 4;
pub const R_R5: usize = 5;
pub const R_R6: usize = 6;
pub const R_R7: usize = 7;
pub const R_PC: usize = 8; /* program counter */
pub const R_COND: usize = 9;
pub const R_COUNT: usize = 10;

pub const MR_KBSR: u16 = 0xFE00; /* keyboard status */
pub const MR_KBDR: u16 = 0xFE02;

pub const OP_BR: u16 = 0; /* branch */
pub const OP_ADD: u16 = 1; /* add  */
pub const OP_LD: u16 = 2; /* load */
pub const OP_ST: u16 = 3; /* store */
pub const OP_JSR: u16 = 4; /* jump register */
pub const OP_AND: u16 = 5; /* bitwise and */
pub const OP_LDR: u16 = 6; /* load register */
pub const OP_STR: u16 = 7; /* store register */
pub const OP_RTI: u16 = 8; /* unused */
pub const OP_NOT: u16 = 9; /* bitwise not */
pub const OP_LDI: u16 = 10; /* load indirect */
pub const OP_STI: u16 = 11; /* store indirect */
pub const OP_JMP: u16 = 12; /* jump */
pub const OP_RES: u16 = 13; /* reserved (unused) */
pub const OP_LEA: u16 = 14; /* load effective address */
pub const OP_TRAP: u16 = 15; /* execute trap */


pub const FL_POS: u16 = 1 << 0; /* P */
pub const FL_ZRO: u16 = 1 << 1; /* Z */
pub const FL_NEG: u16 = 1 << 2; /* N */

pub const TRAP_GETC: u16 = 0x20; /* get character from keyboard, not echoed onto the terminal */
pub const TRAP_OUT: u16 = 0x21; /* output a character */
pub const TRAP_PUTS: u16 = 0x22; /* output a word string */
pub const TRAP_IN: u16 = 0x23; /* get character from keyboard, echoed onto the terminal */
pub const TRAP_PUTSP: u16 = 0x24; /* output a byte string */
pub const TRAP_HALT: u16 = 0x25; /* halt the program */
