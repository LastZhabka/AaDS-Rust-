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

struct Stack<T> {
	Elements: Vec<T>,
	n: usize,
}


impl<T> Stack<T> {
	fn back(&self) -> &T {
		return &self.Elements[self.n - 1];
	}
	fn push(&mut self, e: T) {
		self.Elements.push(e);
		self.n = self.n + 1;
	}
	fn pop(&mut self) {
		self.Elements.pop();
		self.n = self.n - 1;
	}
	fn size(&self) -> usize {
		return self.n;
	}
}

fn main() {
	let mut a = Stack{
		Elements : vec![0i32;0],
		n : 0,	
	};
	a.push(1);
	a.push(2);
	a.push(1);
	let z = a.back();
	print!("{} {}", z, a.size());
}