
//pub mod constants;
//use crate::utils::utils::*;
//pub mod crate::utils::utils;
//pub mod utils;

pub mod instructions {
    //use utils::*;
    use crate::constants::constants::*;
    use crate::utils::utils::*;
    pub fn add(instr: usize, mut reg: [usize; R_COUNT]) {
    
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
    
    pub fn ldi(instr: usize, mut reg: [usize; R_COUNT], memory: [usize; MEMORY_MAX]) {
        /* destination register (DR) */
        let r0: usize = (instr >> 9) & 0x7;
        /* PCoffset 9*/
        let pc_offset: usize = sign_extend(instr & 0x1FF, 9);
        /* add pc_offset to the current PC, look at that memory location to get the final address */
        reg[r0] = memory[memory[reg[R_PC] + pc_offset]];
        update_flags(r0, reg);
    }

    pub fn and(instr: usize, mut reg: [usize; R_COUNT]) {
        let r0: usize = (instr >> 9) & 0x7;
        let r1: usize = (instr >> 6) & 0x7;
        let imm_flag: usize = (instr >> 5) & 0x1;

        if (imm_flag == 1) {
            let imm5: usize = sign_extend(instr & 0x1F, 5);
            reg[r0] = reg[r1] & imm5;
        } else {
            let r2: usize = instr & 0x7;
            reg[r0] = reg[r1] & reg[r2];
        }
        update_flags(r0, reg);
    }
}





