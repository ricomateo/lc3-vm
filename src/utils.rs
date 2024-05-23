pub mod utils {
    use crate::constants::constants::*;
    pub fn sign_extend(mut x: usize, bit_count: usize) -> usize {
        if (x >> (bit_count - 1)) & 1 == 1 {
            x |= 0xFFFF << bit_count;
        }
        x
    }
    
    pub fn update_flags(r: usize, reg: &mut [usize; R_COUNT]) {
        if reg[r] == 0 {
            reg[R_COND] = FL_ZRO;
        } else if reg[r] >> 15 == 1 /* a 1 in the left-most bit indicates negative */{
            reg[R_COND] = FL_NEG;
        } else {
            reg[R_COND] = FL_POS;
        }
    }
}