/*
    generate a gigabyte of random letters
*/

use error_chain::write_internal;
use rand::prelude::*;
use std::fs::File;
use std::io::Write;

fn main() {
    let gb_size = 1024 * 1024 * 1024;
    const WRITE_CHUNK_SIZE: usize = (1024 * 1024) as usize;

    let mut rng = thread_rng();
    let mut file = File::create(".random.txt").unwrap();

    let mut remaining = gb_size;
    while remaining > 0 {
        let mut num_letters_to_write = WRITE_CHUNK_SIZE;
        if remaining <= WRITE_CHUNK_SIZE {
            num_letters_to_write = remaining;
        }

        let mut letters: [u8; WRITE_CHUNK_SIZE] = [0; WRITE_CHUNK_SIZE];
        for i in 0..num_letters_to_write {
            let letter = rng.gen_range('a'..'z') as u8;
            letters[i] = letter;
        }

        file.write_all(&letters).unwrap();
        remaining -= WRITE_CHUNK_SIZE;
    }
}
