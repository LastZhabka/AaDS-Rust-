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

struct Queue<T> {
	Elements: Vec<T>,
	start: usize,
	n: usize,
}


impl<T> Queue<T> {
	fn front(&self) -> &T {
		return &self.Elements[self.start];
	}
	fn push(&mut self, e: T) {
		self.Elements.push(e);
		self.n = self.n + 1;
	}
	fn pop(&mut self) {
		self.start = self.start + 1;
		self.n = self.n - 1;
	}
	fn size(&self) -> usize{
		return self.n;		
	}
}

fn main() {
	let mut a = Queue{
		Elements : vec![0i32;0],
		start : 0,
		n : 0	
	};
	a.push(1);
	a.push(2);
	println!("{} {}", a.front(), a.size());
	a.pop();
	println!("{} {}", a.front(), a.size());
}