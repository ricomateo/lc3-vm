
pub mod instructions {
    use crate::constants::constants::*;
    use crate::utils::utils::*;
    pub fn add(instr: usize, mut reg: &mut [usize; R_COUNT]) {
    
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
    
        update_flags(r0, &mut reg);
    }
    
    pub fn ldi(instr: usize, mut reg: &mut [usize; R_COUNT], memory: &[usize; MEMORY_MAX]) {
        /* destination register (DR) */
        let r0: usize = (instr >> 9) & 0x7;
        /* PCoffset 9*/
        let pc_offset: usize = sign_extend(instr & 0x1FF, 9);
        /* add pc_offset to the current PC, look at that memory location to get the final address */
        reg[r0] = memory[memory[reg[R_PC] + pc_offset]];
        update_flags(r0, &mut reg);
    }

    pub fn and(instr: usize, mut reg: &mut [usize; R_COUNT]) {
        let r0: usize = (instr >> 9) & 0x7;
        let r1: usize = (instr >> 6) & 0x7;
        let imm_flag: usize = (instr >> 5) & 0x1;

        if imm_flag == 1 {
            let imm5: usize = sign_extend(instr & 0x1F, 5);
            reg[r0] = reg[r1] & imm5;
        } else {
            let r2: usize = instr & 0x7;
            reg[r0] = reg[r1] & reg[r2];
        }
        update_flags(r0, &mut reg);
    }

    pub fn not(instr: usize, mut reg: &mut [usize; R_COUNT]) {
        let r0: usize = (instr >> 9) & 0x7;
        let r1: usize = (instr >> 6) & 0x7;

        reg[r0] = !reg[r1];
        update_flags(r0, &mut reg);
    }

    pub fn branch(instr: usize, mut reg: &mut [usize; R_COUNT]) {
        let pc_offset: usize = sign_extend(instr & 0x1FF, 9);
        let cond_flag: usize = (instr >> 9) & 0x7;
        if cond_flag & reg[R_COND] > 0 {
            reg[R_PC] += pc_offset;
        }
    }

    pub fn jump(instr: usize, mut reg: &mut [usize; R_COUNT]) {
        let r1: usize = (instr >> 6) & 0x7;
        reg[R_PC] = reg[r1];
    }

    /// saves the PC value onto the r7 register, and then jumps
    /// to the value given on the PCoffset11 field or the one
    /// contained in the register given by BaseR
    pub fn jump_register(instr: usize, reg: &mut [usize; R_COUNT]) {
        let long_flag: usize = (instr >> 11) & 1;
        reg[R_R7] = reg[R_PC];
        if long_flag > 0 {
            let long_pc_offset: usize = sign_extend(instr & 0x7FF, 11);
            reg[R_PC] += long_pc_offset;  /* JSR */
        } else {
            let r1: usize = (instr >> 6) & 0x7;
            reg[R_PC] = reg[r1]; /* JSRR */
        }
    }

    /// Loads onto a register the value contained in the address
    /// PC + PCoffset9
    pub fn ld(instr: usize, reg: &mut [usize; R_COUNT], memory: &[usize; MEMORY_MAX]) {
        let r0: usize = (instr >> 9) & 0x7;
        let pc_offset: usize = sign_extend(instr & 0x1FF, 9);
        reg[r0] = memory[reg[R_PC] + pc_offset];
        update_flags(r0, reg);
    }
}





#[cfg(test)]
mod tests {
    use crate::constants::constants::*;
    use super::instructions::*;
    
    #[test]
    /// This test sets r1 = 2, r2 = 2 and then
    /// executes r3 = r1 + r2
    fn adding_two_plus_two_works_correctly() {
        let mut reg: [usize; R_COUNT] = [0; R_COUNT];
        reg[R_R1] = 2;
        reg[R_R2] = 2;
        // instr = b0001011001000010 = 0x1642
        let instr: usize = 0x1642;
        // before executing the instruction, r3 = 0
        assert_eq!(reg[R_R3], 0);
        add(instr, &mut reg);
        // after executing the instruction, r3 = 4
        assert_eq!(reg[R_R3], 4);
    }

    #[test]
    /// This test sets the 1st memory position to 2,
    /// where 2 is the address of the memory position that holds
    /// the value we want to load into a register.
    /// Then executes ldi to load that value (the stored in memory[2])
    fn ldi_works_correctly() {
        let mut reg: [usize; R_COUNT] = [0; R_COUNT];
        // instr = b1010001000000001 = 0xA201
        let instr: usize = 0xA201;
        let mut memory: [usize; MEMORY_MAX] = [0; MEMORY_MAX];
        // memory[1] holds the address of the value
        memory[1] = 2;
        // memory[2] holds the value 64
        memory[2] = 64;
        assert_eq!(reg[R_R1], 0);
        ldi(instr, &mut reg, &memory);
        assert_eq!(reg[R_R1], 64);
    }

    #[test]
    /// This test sets r1 = 1, r2 = 2 and then
    /// executes r3 = r1 AND r2
    fn and_works_correctly() {
        let mut reg: [usize; R_COUNT] = [0; R_COUNT];
        reg[R_R1] = 1;
        reg[R_R2] = 1;
        // instr = b0101011001000010 = 0x5642
        let instr: usize = 0x5642;
        // before the AND r3 = 0
        assert_eq!(reg[R_R3], 0);
        and(instr, &mut reg);
        // after executing the instruction, r3 = 4
        assert_eq!(reg[R_R3], 1);
    }

    #[test]
    /// This test executes r2 = NOT r1
    fn not_works_correctly() {
        let mut reg: [usize; R_COUNT] = [0; R_COUNT];
        // instr = b1001010001111111 = 0x947F
        let instr: usize = 0x947F;
        assert_eq!(reg[R_R2], 0);
        not(instr, &mut reg);
        // the &0xffff is needed because reg contains usize, not u16
        assert_eq!(reg[R_R2] &0xffff, 65535);
    }

    #[test]
    /// Branch 16 memory positions if flag = positive
    fn branch_works_correctly() {
        let mut reg: [usize; R_COUNT] = [0; R_COUNT];
        // instr = b0000001000010000 = 0x210
        let instr: usize = 0x210;
        reg[R_COND] = FL_POS;
        assert_eq!(reg[R_PC], 0);
        branch(instr, &mut reg);
        assert_eq!(reg[R_PC], 16);
    }

    #[test]
    /// Jumps to the position indicated by the register r1
    /// i.e,  pc = r1
    fn jump_works_correctly() {
        let mut reg: [usize; R_COUNT] = [0; R_COUNT];
        // instr = b1100000001000000 = 0xC040
        let instr: usize = 0xC040;
        reg[R_R1] = 16;
        assert_eq!(reg[R_PC], 0);
        jump(instr, &mut reg);
        assert_eq!(reg[R_PC], 16);
    }


    #[test]
    /// The pc register starts on 8. 
    /// After the execution of jump_register it should be incremented
    /// by 4 so pc = 12 and the previous value should have been saved
    /// on register r7, so r7 = 8.
    fn jump_register_works_correctly() {
        let mut reg: [usize; R_COUNT] = [0; R_COUNT];
        // instr = b0100100000000100 = 0x4801
        let instr: usize = 0x4804;
        reg[R_PC] = 8;
        assert_eq!(reg[R_R7], 0);
        jump_register(instr, &mut reg);
        assert_eq!(reg[R_PC], 12);
        assert_eq!(reg[R_R7], 8);
    }

    #[test]
    /// Sets the 16th memory position to the value 32.
    /// Then loads that value to the r1 register.
    fn ld_works_correctly() {
        let mut reg: [usize; R_COUNT] = [0; R_COUNT];
        let mut memory: [usize; MEMORY_MAX] = [0; MEMORY_MAX];
        let offset: usize = 16;
        let value: usize = 32;
        memory[offset] = value;
        // instr = b0010001000010000 = 0x2210
        let instr: usize = 0x2210;
        assert_eq!(reg[R_R1], 0);
        ld(instr, &mut reg, &mut memory);
        assert_eq!(reg[R_R1], value)
    }
}






