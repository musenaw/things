// https://www.codewars.com/kata/523f5d21c841566fde000009/train/rust

use std::collections::HashSet;

fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T>
where
    T: PartialEq + std::hash::Hash + Eq + std::fmt::Debug + std::clone::Clone,
{
    let mut set_to_delete: HashSet<T> = HashSet::new();
    for item in b {
        set_to_delete.insert(item);
    }
    let final_vec: Vec<_> = a
        .iter()
        .filter(|&x| !set_to_delete.contains(x))
        .cloned()
        .collect::<Vec<T>>();
    println!("{:?}", final_vec);
    final_vec
}

fn main() {
    array_diff(vec![2, 3, 2, 2, 2, 2], vec![1, 2]);
}
