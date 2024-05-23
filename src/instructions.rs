
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

}






