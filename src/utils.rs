extern crate byteorder;
use byteorder::{BigEndian, ReadBytesExt};
use std::io::Read;
use std::{fs::File, io::BufReader};

use crate::constants::*;
pub fn sign_extend(mut x: u16, bit_count: u16) -> u16 {
    if (x >> (bit_count - 1)) & 1 == 1 {
        x |= 0xFFFF << bit_count;
        //x |= 0xFF << bit_count;
    }
    x
}

pub fn update_flags(r: u16, reg: &mut [u16; R_COUNT]) {
    if reg[r as usize] == 0 {
        reg[R_COND] = FL_ZRO;
    } else if reg[r as usize] >> 15 == 1
    /* a 1 in the left-most bit indicates negative */
    {
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

pub fn read_image_file(file: &mut File, memory: &mut [u16; MEMORY_MAX]) {
    let mut rdr = BufReader::new(file);

    let base_address = rdr
        .read_u16::<BigEndian>()
        .expect("error while reading base_address");
    let mut address = base_address as u16;
    loop {
        match rdr.read_u16::<BigEndian>() {
            Ok(instruction) => {
                //println!("instruction = {:X}", instruction);
                memory[address as usize] = instruction as u16;
                address += 1;
            }
            Err(_e) => {
                return;
            }
        }
    }
}

pub fn read_image(image_path: &str, memory: &mut [u16; MEMORY_MAX]) -> Result<(), std::io::Error> {
    let file = File::open(image_path);
    match file {
        Err(e) => return Err(e),
        Ok(mut f) => read_image_file(&mut f, memory),
    }
    Ok(())
}

pub fn memory_read(memory: &mut [u16; MEMORY_MAX], address: u16) -> u16 {
    if address == MR_KBSR as u16 {
        handle_keyboard(memory);
    }
    memory[address as usize]
}

fn handle_keyboard(memory: &mut [u16; MEMORY_MAX]) {
    let mut buffer = [0; 1];
    std::io::stdin().read_exact(&mut buffer).unwrap();
    if buffer[0] != 0 {
        memory[MR_KBSR as usize] = 1 << 15;
        memory[MR_KBDR as usize] = buffer[0] as u16;
    } else {
        memory[MR_KBSR as usize] = 0;
    }
}
