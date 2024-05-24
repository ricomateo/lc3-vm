

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


pub const MR_KBSR: usize = 0xFE00; /* keyboard status */
pub const MR_KBDR: usize = 0xFE02;


pub const OP_BR: usize = 0; /* branch */
pub const OP_ADD: usize = 1;    /* add  */
pub const OP_LD: usize =  2;     /* load */
pub const OP_ST: usize =  3;     /* store */
pub const OP_JSR: usize =  4;    /* jump register */
pub const OP_AND: usize = 5;  /* bitwise and */
pub const OP_LDR: usize = 6;    /* load register */
pub const OP_STR: usize = 7;    /* store register */
pub const OP_RTI: usize = 8;    /* unused */
pub const OP_NOT: usize = 9;    /* bitwise not */
pub const OP_LDI: usize = 10;    /* load indirect */
pub const OP_STI: usize = 11;    /* store indirect */
pub const OP_JMP: usize = 12;    /* jump */
pub const OP_RES: usize = 13;    /* reserved (unused) */
pub const OP_LEA: usize = 14;    /* load effective address */
pub const OP_TRAP: usize = 15;    /* execute trap */


// 0111000110111111


pub const FL_POS: usize = 1 << 0; /* P */
pub const FL_ZRO: usize = 1 << 1; /* Z */
pub const FL_NEG: usize = 1 << 2; /* N */

pub const TRAP_GETC: usize = 0x20;  /* get character from keyboard, not echoed onto the terminal */
pub const TRAP_OUT: usize = 0x21;   /* output a character */
pub const TRAP_PUTS: usize = 0x22;  /* output a word string */
pub const TRAP_IN: usize = 0x23;    /* get character from keyboard, echoed onto the terminal */
pub const TRAP_PUTSP: usize = 0x24; /* output a byte string */
pub const TRAP_HALT: usize = 0x25;   /* halt the program */


