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
	let n:usize = cin().try_into().unwrap();
	let m:usize = cin().try_into().unwrap();
	let mut G:Vec<Vec<usize>> = Vec::new();
	let mut was:Vec<bool> = vec![false; n + 1];

	for i in 0..(n + 1) {
		let mut subG:Vec<usize> = Vec::new();
		G.push(subG);
	}
	for i in 0..m {
		let tek = cinarr();
		let (a, b) : (usize, usize) = (tek[0].try_into().unwrap(), tek[1].try_into().unwrap());
		G[a].push(b);
		G[b].push(a);
	}
	let mut q = Queue{
		Elements : vec![0usize;0],
		start : 0,
		n : 0	
	};

	q.push(1usize);
	was[1] = true;
	while(q.size() > 0) {
		let v = q.front();
		let mut z:usize = *v;
		q.pop();
		for u in &G[z] {
			if was[*u] == false {
				was[*u] = true;
				q.push(*u);
			}
		}
	}
}