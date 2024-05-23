pub mod constants {

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
    // pub OP_LD,     /* load */
    // pub OP_ST,     /* store */
    // pub OP_JSR,    /* jump register */
    // pub OP_AND,    /* bitwise and */
    // pub OP_LDR,    /* load register */
    // pub OP_STR,    /* store register */
    // pub OP_RTI,    /* unused */
    // pub OP_NOT,    /* bitwise not */
    pub const OP_LDI: usize = 10;    /* load indirect */
    // pub OP_STI,    /* store indirect */
    // pub OP_JMP,    /* jump */
    // pub OP_RES,    /* reserved (unused) */
    // pub OP_LEA,    /* load effective address */
    // pub OP_TRAP    /* execute trap */





    pub const FL_POS: usize = 1 << 0; /* P */
    pub const FL_ZRO: usize = 1 << 1; /* Z */
    pub const FL_NEG: usize = 1 << 2; /* N */

}