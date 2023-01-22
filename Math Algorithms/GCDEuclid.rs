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

fn gcd(a: i64, b: i64) -> i64 {	
	if b == 0 { 
		return a; 
	}
	return gcd(b, a%b);
}

fn main() {
	let (a, b) = (cin(), cin());
	print!("{}", gcd(a, b));	
}