#![allow(arithmetic_overflow)]
use std::fs;

///Gets from a byte vector, replacing None with a 0 and a false value and Some with the u8 and a true value.
fn getor0(v: &Vec<u8>, i: usize) -> u8 {
    match v.get(i) { Some(r) => *r, None => 0 }
}

///getor0, but increments i before.
fn next(v: &Vec<u8>, i: &mut usize) -> u8 {
    *i = *i + 1;
    getor0(v, *i)
}

///Pops an i32 stack, returning 0 if the stack is empty.
fn popor0(v: &mut Vec<i32>) -> i32 {
    match v.pop() { Some(r) => r, None => 0 }
}

///Wraps an i32 within the bounds of a byte vector, returning a valid index for the byte array.
fn wrap(v: &Vec<u8>, i: i32) -> usize {
    (i.abs() as usize % v.len()) as usize
}

fn get_i32(v: &Vec<u8>, i: &mut usize) -> i32 {
    ((next(v, i) as i32) << 24) + ((next(v, i) as i32) << 16) + ((next(v, i) as i32) << 8) + (next(v, i) as i32)
}

fn main() {
    let script: Vec<u8> = match fs::read("dfds") { Ok(v) => v, Err(_) => Vec::new() };
    let stack: Vec<i32> = Vec::new();
    let mut i: usize = 0;
    while i < script.len() {
        match next(&script, &mut i) {
            0 => break,
            _ => continue
        }
    }
}