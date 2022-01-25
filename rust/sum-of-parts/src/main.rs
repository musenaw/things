// https://www.codewars.com/kata/5ce399e0047a45001c853c2b/train/rust

fn parts_sums(ls: &[u64]) -> Vec<u64> {
    let mut result: Vec<u64> = Vec::new();
    let mut acc = 0;
    result.push(acc);
    let c: Vec<&u64> = ls.iter().rev().collect();
    let mut r: Vec<u64> = c
        .iter()
        .map(|&&x| {
            acc = acc + x;
            acc
        })
        .collect();
    r.insert(0, 0);
    r.into_iter().rev().collect()
}

fn main() {
    let test = vec![0, 1, 3, 6, 10];
    parts_sums(&test);
}
