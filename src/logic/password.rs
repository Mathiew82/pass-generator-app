use crate::logic::state::PasswordOptions;
use rand::prelude::*;

const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRTUVWXYZ";
const LOWER: &[u8] = b"abcdefghjkmnpqrstuvwxyz";
const NUM: &[u8] = b"2346789";
const SYM: &[u8] = b"!@#$%&*()-_=+[]{}?/";

pub fn generate_password(opts: PasswordOptions) -> String {
    if opts.length == 0 {
        return String::new();
    }

    let mut sets: Vec<&[u8]> = Vec::new();
    if opts.uppercase {
        sets.push(UPPER);
    }
    if opts.lowercase {
        sets.push(LOWER);
    }
    if opts.numbers {
        sets.push(NUM);
    }
    if opts.symbols {
        sets.push(SYM);
    }

    if sets.is_empty() {
        return String::new();
    }

    let mut rng = rand::rng();
    let mut chars: Vec<u8> = Vec::with_capacity(opts.length);

    if opts.length >= sets.len() {
        for set in &sets {
            let &c = set.choose(&mut rng).unwrap();
            chars.push(c);
        }
    }

    let pool: Vec<u8> = sets.concat();

    while chars.len() < opts.length {
        let &c = pool.choose(&mut rng).unwrap();
        chars.push(c);
    }

    chars.shuffle(&mut rng);

    String::from_utf8(chars).unwrap()
}
