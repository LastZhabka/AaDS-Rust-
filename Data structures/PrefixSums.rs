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

struct PrefixSums {
	p : Vec<i64>,
	n : usize,
}

impl PrefixSums {
	fn build(&mut self, a : &Vec<i64>) {
		self.n = a.len().try_into().unwrap();
		let mut sum:i64 = 0;
		for i in a {
			sum = sum + *i;
			self.p.push(sum);
		}
	}
	fn get(&self, l : usize, r : usize) -> i64 {
		if l > 0 {	    	
			return self.p[r] - self.p[l - 1];
		}
		return self.p[r];
	}
}

fn main() {
	let a = cinarr();
	let mut b = PrefixSums{
		p : vec![0i64; 0],
		n : 0usize,
	};
	b.build(&a);
	print!("{}", b.get(1, 2));
}