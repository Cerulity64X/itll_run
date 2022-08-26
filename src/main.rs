#![allow(arithmetic_overflow)]
#![allow(overflowing_literals)]
use std::{fs, env::{args, Args}, vec, cmp::Ordering};

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
    let argv: Args = args();
    let mut path: String;
    let mut argl: Vec<String> = vec![];
    let mut no_break: bool = false;
    for a in argv {
        argl.push(a);
    }
    path = argl.get(1).expect("Please input a file!").clone();
    match argl.get(2) {
        Some(s) => {
            match s.as_str() {
                "no_break" => {no_break = true;},
                _ => {}
            }
        },
        None => {}
    }
    let script: Vec<u8> = match fs::read(path) { Ok(v) => v, Err(_) => Vec::new() };
    let mut stack: Vec<i32> = vec![];
    let mut i: usize = 0;
    while i < script.len() {
        match next(&script, &mut i) {
            0 => {if !no_break { break }},
            1 => {stack.push(get_i32(&script, &mut i))},
            2 => {popor0(&mut stack);},
            3 => {
                let left: i32 = popor0(&mut stack);
                let right: i32 = popor0(&mut stack);
                stack.push(left + right);
            }
            4 => {
                let left: i32 = popor0(&mut stack);
                let right: i32 = popor0(&mut stack);
                stack.push(left - right);
            }
            5 => {
                let left: i32 = popor0(&mut stack);
                let right: i32 = popor0(&mut stack);
                stack.push(left * right);
            }
            6 => {
                let left: i32 = popor0(&mut stack);
                let right: i32 = popor0(&mut stack);
                stack.push(left / right);
            },
            7 => {
                match popor0(&mut stack).cmp(&popor0(&mut stack)) {
                    Ordering::Less => {stack.push(-1)},
                    Ordering::Equal => {stack.push(0)},
                    Ordering::Greater => {stack.push(1)},
                }
            },
            8 => {
                i = wrap(&script, get_i32(&script, &mut i));
            },
            9 => {
                if popor0(&mut stack) < 0 {i = wrap(&script, get_i32(&script, &mut i))};
            },
            10 => {
                if popor0(&mut stack) == 0 {i = wrap(&script, get_i32(&script, &mut i))};
            },
            11 => {
                if popor0(&mut stack) > 0 {i = wrap(&script, get_i32(&script, &mut i))};
            }
            _ => continue
        }
    }
}
