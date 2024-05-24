

use crate::constants::*;
use crate::utils::*;

pub fn add(instr: u16, mut reg: &mut [u16; R_COUNT]) {
    println!("add");
    /* destination register (DR) */
    let r0: u16 = (instr >> 9) & 0x7;
    /* first operand (SR1) */
    let r1: u16 = (instr >> 6) & 0x7;
    /* whether we are in immediate mode */
    let imm_flag: u16 = (instr >> 5) & 0x1;

    if imm_flag == 1 {
        let imm5: u16 = sign_extend(instr & 0x1F, 5);
        reg[r0 as usize] = reg[r1 as usize] + imm5;
    } else {
        let r2: u16 = instr & 0x7;
        reg[r0 as usize] = reg[r1 as usize] + reg[r2 as usize];
    }

    update_flags(r0, &mut reg);
}

pub fn ldi(instr: u16, mut reg: &mut [u16; R_COUNT], memory: &[u16; MEMORY_MAX]) {
    println!("ldi");
    /* destination register (DR) */
    let r0: u16 = (instr >> 9) & 0x7;
    /* PCoffset 9*/
    let pc_offset: u16 = sign_extend(instr & 0x1FF, 9);
    /* add pc_offset to the current PC, look at that memory location to get the final address */
    reg[r0 as usize] = memory[memory[(reg[R_PC] + pc_offset) as usize] as usize];
    update_flags(r0, &mut reg);
}

pub fn and(instr: u16, mut reg: &mut [u16; R_COUNT]) {
    println!("and");
    let r0: u16 = (instr >> 9) & 0x7;
    let r1: u16 = (instr >> 6) & 0x7;
    let imm_flag: u16 = (instr >> 5) & 0x1;

    if imm_flag == 1 {
        let imm5: u16 = sign_extend(instr & 0x1F, 5);
        reg[r0  as usize] = reg[r1 as usize] & imm5;
    } else {
        let r2: u16 = instr & 0x7;
        reg[r0 as usize] = reg[r1 as usize] & reg[r2 as usize];
    }
    update_flags(r0, &mut reg);
}

pub fn not(instr: u16, mut reg: &mut [u16; R_COUNT]) {
    println!("not!");
    let r0: u16 = (instr >> 9) & 0x7;
    let r1: u16 = (instr >> 6) & 0x7;

    reg[r0 as usize] = !reg[r1 as usize];
    update_flags(r0, &mut reg);
}

pub fn branch(instr: u16, mut reg: &mut [u16; R_COUNT]) {
    
    let pc_offset: u16 = sign_extend(instr & 0x1FF, 9) as u16 as u16;
    let cond_flag: u16 = (instr >> 9) & 0x7;
    if instr != 0 {
        println!("branch from {} to {}", reg[R_PC], reg[R_PC] + pc_offset);
    }
    if cond_flag & reg[R_COND] > 0 {
        reg[R_PC] += pc_offset;
    }
}

pub fn jump(instr: u16, mut reg: &mut [u16; R_COUNT]) {
    // println!("jump");
    let r1: u16 = (instr >> 6) & 0x7;
    reg[R_PC] = reg[r1 as usize];
}

/// saves the PC value onto the r7 register, and then jumps
/// to the value given on the PCoffset11 field or the one
/// contained in the register given by BaseR
pub fn jump_register(instr: u16, reg: &mut [u16; R_COUNT]) {
    println!("jump register");
    let long_flag: u16 = (instr >> 11) & 1;
    reg[R_R7] = reg[R_PC];
    if long_flag > 0 {
        let long_pc_offset: u16 = sign_extend(instr & 0x7FF, 11);
        reg[R_PC] += long_pc_offset;  /* JSR */
    } else {
        let r1: u16 = (instr >> 6) & 0x7;
        reg[R_PC] = reg[r1 as usize]; /* JSRR */
    }
}

/// Loads onto a register the value contained in the address
/// PC + PCoffset9
pub fn ld(instr: u16, reg: &mut [u16; R_COUNT], memory: &[u16; MEMORY_MAX]) {
    println!("ld");
    let r0: u16 = (instr >> 9) & 0x7;
    let pc_offset: u16 = sign_extend(instr & 0x1FF, 9);
    reg[r0 as usize] = memory[reg[R_PC] as usize + pc_offset as usize];
    update_flags(r0, reg);
}

/// Computes an address by calculating address = base register + offset6
/// and then loads the value contained in that address into a register
pub fn ldr(instr: u16, reg: &mut [u16; R_COUNT], memory: &[u16; MEMORY_MAX]) {
    println!("ldr");
    let r0: u16 = (instr >> 9) & 0x7;
    let r1: u16 = (instr >> 6) & 0x7;
    let offset: u16 = sign_extend(instr & 0x3F, 6);
    reg[r0 as usize] = memory[(reg[r1 as usize] + offset) as usize];
    update_flags(r0, reg);
}

/// Computes an address by adding PC and PCoffset9, and loading it
/// into a given register
pub fn lea(instr: u16, reg: &mut [u16; R_COUNT]) {
    println!("lea");
    let r0: u16 = (instr >> 9) & 0x7;
    let pc_offset: u16 = sign_extend(instr & 0x1FF, 9);
    reg[r0 as usize] = reg[R_PC] + pc_offset;
    update_flags(r0, reg);
}

/// Computes a memory address by adding PC and PCoffset9, and then 
/// stores the value contained in a register in that memory address
pub fn store(instr: u16, reg: &mut [u16; R_COUNT], memory: &mut [u16; MEMORY_MAX]) {
    println!("store");
    let r0: u16 = (instr >> 9) & 0x7;
    let pc_offset: u16 = sign_extend(instr & 0x1FF, 9);
    memory[(reg[R_PC] + pc_offset) as usize] = reg[r0 as usize];
}

/// Computes a memory address by adding pc + offset. This address
/// contains the address where the value given by a register is
/// going to be stored in
pub fn store_indirect(instr: u16, reg: &mut [u16; R_COUNT], memory: &mut [u16; MEMORY_MAX]) {
    println!("store indirect");
    let r0: u16 = (instr >> 9) & 0x7;
    let pc_offset: u16 = sign_extend(instr & 0x1FF, 9);
    memory[memory[(reg[R_PC] + pc_offset) as usize] as usize] = reg[r0 as usize];
}

/// Computes a memory address by adding base register + offset 6.
/// The value specified by the register SR is stored in the computed address.
pub fn store_register(instr: u16, reg: &mut [u16; R_COUNT], memory: &mut [u16; MEMORY_MAX]) {
    println!("store register");
    let r0: u16 = (instr >> 9) & 0x7;
    let r1: u16 = (instr >> 6) & 0x7;
    let offset: u16 = sign_extend(instr & 0x3F, 6) as u16 as u16;
    println!("reg[r1] = {:X}", reg[r1 as usize]);
    println!("offset = {:X}", offset);
    let index = reg[r1 as usize] + offset;
    if index as usize >= MEMORY_MAX {
        println!("-----\ninstruction = {:X}\n-----\n", instr);
        println!("index >= memory_max");
        println!("reg[r1] = {:X}, reg[r1] as u16 = {:X}", reg[r1 as usize], reg[r1 as usize] as u16);
        println!("offset = {:X}, offset as u16 = {:X}", offset, offset as u16);
    }
    memory[(reg[r1 as usize] + offset) as usize] = reg[r0 as usize];
}






#[cfg(test)]
mod tests {
    use crate::constants::*;
    use super::*;
    
    #[test]
    /// This test sets r1 = 2, r2 = 2 and then
    /// executes r3 = r1 + r2
    fn adding_two_plus_two_works_correctly() {
        let mut reg: [u16; R_COUNT] = [0; R_COUNT];
        reg[R_R1] = 2;
        reg[R_R2] = 2;
        // instr = b0001011001000010 = 0x1642
        let instr: u16 = 0x1642;
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
        let mut reg: [u16; R_COUNT] = [0; R_COUNT];
        // instr = b1010001000000001 = 0xA201
        let instr: u16 = 0xA201;
        let mut memory: [u16; MEMORY_MAX] = [0; MEMORY_MAX];
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
        let mut reg: [u16; R_COUNT] = [0; R_COUNT];
        reg[R_R1] = 1;
        reg[R_R2] = 1;
        // instr = b0101011001000010 = 0x5642
        let instr: u16 = 0x5642;
        // before the AND r3 = 0
        assert_eq!(reg[R_R3], 0);
        and(instr, &mut reg);
        // after executing the instruction, r3 = 4
        assert_eq!(reg[R_R3], 1);
    }

    #[test]
    /// This test executes r2 = NOT r1
    fn not_works_correctly() {
        let mut reg: [u16; R_COUNT] = [0; R_COUNT];
        // instr = b1001010001111111 = 0x947F
        let instr: u16 = 0x947F;
        assert_eq!(reg[R_R2], 0);
        not(instr, &mut reg);
        // the &0xffff is needed because reg contains u16, not u16
        assert_eq!(reg[R_R2] &0xffff, 65535);
    }

    #[test]
    /// Branch 16 memory positions if flag = positive
    fn branch_works_correctly() {
        let mut reg: [u16; R_COUNT] = [0; R_COUNT];
        // instr = b0000001000010000 = 0x210
        let instr: u16 = 0x210;
        reg[R_COND] = FL_POS;
        assert_eq!(reg[R_PC], 0);
        branch(instr, &mut reg);
        assert_eq!(reg[R_PC], 16);
    }

    #[test]
    /// Jumps to the position indicated by the register r1
    /// i.e,  pc = r1
    fn jump_works_correctly() {
        let mut reg: [u16; R_COUNT] = [0; R_COUNT];
        // instr = b1100000001000000 = 0xC040
        let instr: u16 = 0xC040;
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
        let mut reg: [u16; R_COUNT] = [0; R_COUNT];
        // instr = b0100100000000100 = 0x4801
        let instr: u16 = 0x4804;
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
        let mut reg: [u16; R_COUNT] = [0; R_COUNT];
        let mut memory: [u16; MEMORY_MAX] = [0; MEMORY_MAX];
        let offset: u16 = 16;
        let value: u16 = 32;
        memory[offset as usize] = value;
        // instr = b0010001000010000 = 0x2210
        let instr: u16 = 0x2210;
        assert_eq!(reg[R_R1], 0);
        ld(instr, &mut reg, &mut memory);
        assert_eq!(reg[R_R1], value)
    }
    #[test]
    /// Loads the value contained at the address
    /// contained at the address base + offset to the
    /// register r2
    fn ldr_works_correctly() {
        let mut reg: [u16; R_COUNT] = [0; R_COUNT];
        let mut memory: [u16; MEMORY_MAX] = [0; MEMORY_MAX];
        let offset: u16 = 8;
        let base: u16 = 8;
        let value: u16 = 32;
        reg[R_R1] = base;
        memory[(base + offset) as usize] = value;
        // instr = b0110010001001000 = 0x6448
        let instr: u16 = 0x6448;
        assert_eq!(reg[R_R2], 0);
        ldr(instr, &mut reg, &mut memory);
        assert_eq!(reg[R_R2], value);
    }

    #[test]
    /// Initialize PC = 2, and then load the value
    /// PC + offset = 2 + 2 = 4 onto the r1 register
    fn lea_works_correctly() {
        let mut reg: [u16; R_COUNT] = [0; R_COUNT];
        let pc_offset = 2;
        let pc = 2;
        reg[R_PC] = pc;
        // instr = b1110001000000010 = 0xE202
        let instr: u16 = 0xE202;
        assert_eq!(reg[R_R1], 0);
        lea(instr, &mut reg);
        assert_eq!(reg[R_R1], 4);
    }


    #[test]
    /// Stores the value contained by r1 (16) in the
    /// address given by pc + pc_offset = 4
    fn store_works_correctly() {
        let mut reg: [u16; R_COUNT] = [0; R_COUNT];
        let mut memory: [u16; MEMORY_MAX] = [0; MEMORY_MAX];
        reg[R_R1] = 16;
        let pc_offset: u16 = 4;
        // instr = b0110001000000100 = 0x6204;
        let instr: u16 = 0x6204;
        let address = reg[R_PC] + pc_offset;
        assert_eq!(memory[address as usize], 0);
        store(instr, &mut reg, &mut memory);
        assert_eq!(memory[address as usize], reg[R_R1]);
    }

    #[test]
    /// memory[pc + pc_offset] contains the address where
    /// the value is stored. The value is specified by the register r1.
    fn store_indirect_works_correctly() {
        let mut reg: [u16; R_COUNT] = [0; R_COUNT];
        let mut memory: [u16; MEMORY_MAX] = [0; MEMORY_MAX];
        let value = 16;
        reg[R_R1] = value;
        let address = 64;
        let pc_offset = 4;
        memory[(reg[R_PC] + pc_offset) as usize] = address;
        // instr = b1011001000000100 = 0xB204
        let instr: u16 = 0xB204;
        assert_eq!(memory[address as usize], 0);
        store_indirect(instr, &mut reg, &mut memory);
        assert_eq!(memory[address as usize], reg[R_R1]);
    }


    #[test]
    /// stores the value specified by the register r1 (16) in the
    /// address r2 + offset = 4 + 4 = 8
    fn store_register_works_correctly() {
        let mut reg: [u16; R_COUNT] = [0; R_COUNT];
        let mut memory: [u16; MEMORY_MAX] = [0; MEMORY_MAX];
        let value = 16;
        reg[R_R1] = value;
        reg[R_R2] = 4;
        let offset: u16 = 4;
        // instr = 0111 001 010 000100 = 0x7284
        let instr: u16 = 0x7284;
        assert_eq!(memory[(reg[R_R2] + offset) as usize], 0);
        store_register(instr, &mut reg, &mut memory);
        assert_eq!(memory[(reg[R_R2] + offset) as usize], value);
    }
}






