use std::io;
use std::convert::TryInto;

fn cin() -> i64 {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().unwrap()
}

fn cinstring() -> String { 
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    return s; 
}

fn cinarr() -> Vec<i64> {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn main() {
	let z = cin();
	let mut x:i32 = 0;
	let mut y:i64 = 0;
	x = y.try_into().unwrap();
}