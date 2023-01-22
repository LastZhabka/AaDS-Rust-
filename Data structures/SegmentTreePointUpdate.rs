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

struct SegmentTree {
	tree: Vec<i64>,
	n: usize,
}

impl SegmentTree {
	fn build(&mut self, a : &Vec<i64>, v: usize, l:usize, r:usize) {
		if l == r {
			self.tree[v] = a[l - 1];
			return;
		}
		let m = (l + r) / 2;
		self.build(a, v * 2, l, m);
		self.build(a, v * 2 + 1, m + 1, r);
		self.tree[v] = self.tree[v * 2] + self.tree[v * 2 + 1];
	}
	fn Create(&mut self, n: usize, a : &Vec<i64>) {
		self.n = n;
		self.tree = vec![0i64; 4 * (n + 1)];
		self.build(a, 1, 1, n);
	}
	fn upd(&mut self, p:usize, z:i64, v:usize, tl:usize, tr:usize) {
		if tl == tr {
			self.tree[v] = self.tree[v] + z;
			return;
		}
		let m = (tl + tr) / 2;
		if p <= m {
			self.upd(p, z, v * 2, tl, m);
		}
		else {
			self.upd(p, z, v * 2 + 1, m + 1, tr);
		}
		self.tree[v] = self.tree[v * 2] + self.tree[v * 2 + 1];
	}
	fn get(&mut self, l:usize, r:usize, v:usize, tl:usize, tr:usize) -> i64 {
		if l > tr || tl > r {
			return 0;
		}
		if l <= tl && tr <= r { 
			return self.tree[v];
		}
		let m = (tl + tr) / 2;
		return self.get(l, r, v * 2, tl, m) + self.get(l, r, v * 2 + 1, m + 1, tr);
	}
}                   

fn main() {
	let nq = cinarr();
	let (n, q) : (usize, usize) = (nq[0].try_into().unwrap(), nq[1].try_into().unwrap());
	let a = cinarr();
	let mut rt = SegmentTree{
		tree: vec![0i64 ;0],
		n: 0,
	};
	rt.Create(n, &a);
	for i in 0..q {
		let tlr = cinarr();
		let (mut t, mut l, mut r) : (usize, usize, usize) = (
			tlr[0].try_into().unwrap(), 
			tlr[1].try_into().unwrap(),
			tlr[2].try_into().unwrap()
		);
		l = l + 1;
		if t == 1 {
			println!("{}", rt.get(l, r, 1, 1, rt.n));
		}
		else {
			rt.upd(l, r.try_into().unwrap(), 1, 1, rt.n);
		}
	}
}

// https://judge.yosupo.jp/submission/122616