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


fn dfs(G: &Vec<Vec<usize>>, was: &mut Vec<i64>, v : usize) {
	was[v] = 1;
	println!("{v}");
	for u in &G[v] {
		if was[*u] == 0 {
			dfs(G, was, *u);
		}
	}
}

fn main() {
	let n = cin();
	let m = cin();
	let mut G:Vec<Vec<usize>> = Vec::new();
	let mut was:Vec<i64> = vec![0; (n + 1).try_into().unwrap()];

	for j in (-1..n) {
		let Gs:Vec<usize> = Vec::new();
		G.push(Gs);
	}
	
	for i in (0..m) {
		let temp = cinarr();
		let a:usize = temp[0].try_into().unwrap();
		let b:usize = temp[1].try_into().unwrap();
		G[a].push(b);
		G[b].push(a);
	}

	dfs(&G, &mut was, 1);
}