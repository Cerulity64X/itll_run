use std::fs;

///Gets from a byte vector, replacing None with a 0 and a false value and Some with the u8 and a true value.
fn getor0(v: &Vec<u8>, i: usize) -> (u8, bool) {
    match v.get(i) { Some(r) => (*r, true), None => (0, false) }
}

///getor0, but increments i before.
fn next(v: &Vec<u8>, i: &mut usize) -> (u8, bool) {
    *i = *i + 1;
    getor0(v, *i)
}

fn main() {
    let script: Vec<u8> = match fs::read("dfds") { Ok(v) => v, Err(_) => Vec::new() };
    let mut i: usize = 0;
    while i < script.len() {
        
    }
}
