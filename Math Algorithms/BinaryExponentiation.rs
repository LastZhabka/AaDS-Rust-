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

fn Binary_Exponentiation(mut a: i64, mut b: i64) -> i64 {	
	let mut c:i64 = 1;
	while b > 0 {
		if(b&1 == 1) {
			c = c * a;
		}
		b = b >> 1;
		a = a * a;
	}
	return c;
}

fn main() {
	let (a, b) = (cin(), cin());
	print!("{}", Binary_Exponentiation(a, b));	
}