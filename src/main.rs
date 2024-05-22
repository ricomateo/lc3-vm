const MEMORY_MAX: usize = 2_usize.pow(16);



fn sign_extend(mut x: usize, bit_count: usize) -> usize {
    if (x >> (bit_count - 1)) & 1 == 1 {
        x |= 0xFFFF << bit_count;
    }
    x
}

fn update_flags(r: usize, mut reg: [usize; R_COUNT]) {
    if reg[r] == 0 {
        reg[R_COND] = FL_ZRO;
    } else if reg[r] >> 15 == 1 /* a 1 in the left-most bit indicates negative */{
        reg[R_COND] = FL_NEG;
    } else {
        reg[R_COND] = FL_POS;
    }
}


fn add(instr: usize, mut reg: [usize; R_COUNT]) {
    
    /* destination register (DR) */
    let r0: usize = (instr >> 9) & 0x7;
    /* first operand (SR1) */
    let r1: usize = (instr >> 6) & 0x7;
    /* whether we are in immediate mode */
    let imm_flag: usize = (instr >> 5) & 0x1;

    if imm_flag == 1 {
        let imm5: usize = sign_extend(instr & 0x1F, 5);
        reg[r0] = reg[r1] + imm5;
    } else {
        let r2: usize = instr & 0x7;
        reg[r0] = reg[r1] + reg[r2];
    }

    update_flags(r0, reg);
}

fn main()
{   
    let memory: [usize; MEMORY_MAX];

    let mut reg: [usize; R_COUNT];
    //@{Load Arguments}

    /* since exactly one condition flag should be set at any given time, set the Z flag */
    reg[R_COND] = FL_ZRO;

    /* set the PC to starting position */
    /* 0x3000 is the default */
    let PC_START = 0x3000;
    reg[R_PC] = PC_START;

    let running = 1;
    while (running == 1)
    {
        /* FETCH */
        let instr: usize = memory[(reg[R_PC] + 1)];
        let op: usize = instr >> 12;

        match op {
            OP_ADD => add(instr, reg),
            _ => println!("not implemented yet"),
        }
    }
    @{Shutdown}
}



const R_R0: usize = 0;
const R_R1: usize = 1;
const R_R2: usize = 2;
const R_R3: usize = 3;
const R_R4: usize = 4;
const R_R5: usize = 5;
const R_R6: usize = 6;
const R_R7: usize = 7;
const R_PC: usize = 8; /* program counter */
const R_COND: usize = 9;
const R_COUNT: usize = 10;



const OP_BR: usize = 0; /* branch */
const OP_ADD: usize = 1;    /* add  */
// OP_LD,     /* load */
// OP_ST,     /* store */
// OP_JSR,    /* jump register */
// OP_AND,    /* bitwise and */
// OP_LDR,    /* load register */
// OP_STR,    /* store register */
// OP_RTI,    /* unused */
// OP_NOT,    /* bitwise not */
// OP_LDI,    /* load indirect */
// OP_STI,    /* store indirect */
// OP_JMP,    /* jump */
// OP_RES,    /* reserved (unused) */
// OP_LEA,    /* load effective address */
// OP_TRAP    /* execute trap */





const FL_POS: usize = 1 << 0; /* P */
const FL_ZRO: usize = 1 << 1; /* Z */
const FL_NEG: usize = 1 << 2; /* N */
