pub mod traps {
    use crate::constants::constants::*;
    use crate::utils::utils::*;

    pub fn puts(reg: &mut [usize; R_COUNT], memory: &mut [usize; MEMORY_MAX]) { //-> Vec<char> {
        let mut address = reg[R_R0];
        let mut c: usize = memory[address] as usize;
        let mut chars: Vec<char> = Vec::new();
        while c != 0 {
            c = memory[address];
            if c >> 16 > 0 {
                println!("invalid character (non utf16)");
                break;
            }
            let char = char::from_u32(c as u32).expect("invalid char conversion");
            chars.push(char);
            address += 1;
        }
        chars.iter().for_each(|c| {
            print!("{}", c);
        });
        // remove the \0
        //chars.remove(chars.len() - 1);
        //chars
    }

    pub fn getc(reg: &mut [usize; R_COUNT]) {
        let char: usize = (get_char() as usize) << 16 >> 16;
        reg[R_R0] = char;
        update_flags(R_R0, reg);
    }

    /// Prints the character stored in r0 register.
    pub fn output_character(reg: &mut [usize; R_COUNT]) {
        print!("{}", (reg[R_R0] as u8) as char);
    }

    pub fn input_character(reg: &mut [usize; R_COUNT]) {
        println!("Enter a character: ");
        let c: char = get_char();
        print!("{c}");
        reg[R_R0] = c as usize;
        update_flags(R_R0, reg);
    }

    pub fn putsp(reg: &mut [usize; R_COUNT], memory: &mut [usize; MEMORY_MAX]) {//-> Vec<char> {
        /* one char per byte (two bytes per word)
            here we need to swap back to
            big endian format */
        let mut address = reg[R_R0];
        let mut c: usize = memory[address];
        let mut chars: Vec<char> = Vec::new();
        while c as u8 != 0 {
            c = memory[address];
            let char1: usize = (c) & 0xFF;
            if char1 == 0 {
                break;
            }
            let char1 = char::from_u32(char1 as u32).expect("error while converting char");
            chars.push(char1);
            print!("{}", char1);
            let char2: usize = (c) >> 8;
            if char2 as u8 != 0 {
                let char2 = char::from_u32(char2 as u32).expect("error while converting char");
                print!("{}",char2);
                chars.push(char2);
                address += 1;
                c = memory[address];
            } else {
                break;
            }
        }
        //chars
    }


    pub fn halt(running: &mut usize) {
        println!("HALT");
        *running = 0;
    }
}



#[cfg(test)]
mod tests {
    use crate::constants::constants::*;
    use super::traps::*;
    #[test]
    /// This test sets r1 = 2, r2 = 2 and then
    /// executes r3 = r1 + r2
    fn puts_works_correctly() {
        let mut reg: [usize; R_COUNT] = [0; R_COUNT];
        let mut memory: [usize; MEMORY_MAX] = [0; MEMORY_MAX];
        let chars = vec!['h', 'e', 'l', 'l', 'o'];
        memory[0] = 0x68; // 'h'
        memory[1] = 0x65; // 'e'
        memory[2] = 0x6c; // 'l'
        memory[3] = 0x6c; // 'l'
        memory[4] = 0x6f; // 'o'

        //assert_eq!(chars, puts(&mut reg, &mut memory));
    }

    #[test]
    /// Blocks until a key is pressed. Checks if the
    /// r0 value changes after pressing a key.
    fn getc_works_correctly() {
        let mut reg: [usize; R_COUNT] = [0; R_COUNT];
        assert_eq!(reg[R_R0], 0);
        getc(&mut reg);
        assert_ne!(reg[R_R0], 0);
    }
    

    #[test]
    fn putsp_works_correctly() {
        let chars = vec!['h', 'e', 'l', 'l', 'o'];
        let mut reg: [usize; R_COUNT] = [0; R_COUNT];
        let mut memory: [usize; MEMORY_MAX] = [0; MEMORY_MAX];

        let mut j = 0;
        
        // set a character in each of the two bytes of memory, for each address
        // for example
        // 0:  'e'  'h'
        // 1:  'l'  'l'
        // 2:  '0'  'o'
        // note that the characters are in little endian
        for i in 0..(chars.len() / 2) {
            memory[i] = (chars[j] as usize | ((chars[j + 1] as usize) << 8)) as usize;
            j += 2;
        }

        // in case the number of chars are odd, we will use half of
        // a memory address
        if chars.len() % 2 != 0 {
            memory[chars.len() / 2] = chars[chars.len() - 1] as usize;
        }
        //assert_eq!(chars, putsp(&mut reg, &mut memory));
    }
}