use std::os::unix::thread;

use rand::{seq::{IndexedRandom, SliceRandom}, Rng};

use crate::opts::GenPassOpts;


const UPPER : &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWER : &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMBER : &[u8] =  b"0123456789";
const SYMBOL : &[u8] = b"!@#$%^&*_";
pub fn process_genpass(
    length : u8,
    upper: bool,
    lower: bool,
    number: bool,
    symbol: bool
) -> anyhow::Result<()> {
    let mut rng = rand::thread_rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();

    if upper{
        chars.extend_from_slice(UPPER);
        password.push(*UPPER.choose(&mut rng).expect("UPPER IS NOT EMPTY"));
    }
    if lower {
        chars.extend_from_slice(LOWER);
        password.push(*LOWER.choose(&mut rng).expect("LOWER IS NOT EMPTY"));
    }
    if number {
        chars.extend_from_slice(NUMBER);
        password.push(*NUMBER.choose(&mut rng).expect("NUMBER IS NOT EMPTY"));
    }
    if symbol {
        chars.extend_from_slice(SYMBOL);
        password.push(*SYMBOL.choose(&mut rng).expect("SYMBOL IS NOT EMPTY"));
    }
    for _ in 0..(length - password.len() as u8) {
        let c = chars.choose(&mut rng).expect("Not be empty").clone();
        password.push(c);
    }
    password.shuffle(&mut rng);
    println!("{}",String::from_utf8(password)?);
    Ok(())
}