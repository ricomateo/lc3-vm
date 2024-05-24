pub mod utils {
    use std::{fs::File, io::Read};

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

    pub fn get_char() -> char {
        use console::Term;
        let term = Term::stdout();
        let char = term.read_char().expect("error while reading char");
        char
    }

    pub fn read_image_file(file: &mut File, memory: &mut [usize; MEMORY_MAX]) {
        
        let mut arr: [u8; 2] = [0; 2];
        let _read_bytes = file.read(&mut arr).unwrap();
        let origin: u16 = arr[0] as u16| (arr[1] as u16) << 8;
        //let max_read = MEMORY_MAX - origin as usize;
        //let address = origin;
        let mut bytes:[u8; MEMORY_MAX * 2] = [0; MEMORY_MAX * 2];
        let _read = file.read(&mut bytes).expect("error while reading file");
        

        let mut j = 0;
        for i in (0..bytes.len()).step_by(2) {
            memory[(origin + j) as usize] = bytes[i] as usize | ((bytes[i+1] as usize) << 8);
            j += 1;
        }
    }

    pub fn read_image(image_path: &str, memory: &mut [usize; MEMORY_MAX]) -> Result<(), std::io::Error>{
        let file = File::open(image_path);
        match file {
            Err(e) => return Err(e),
            Ok(mut f) => read_image_file(&mut f, memory),
        }
        Ok(())
    }


}
