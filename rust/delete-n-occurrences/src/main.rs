// https://www.codewars.com/kata/554ca54ffa7d91b236000023/train/rust

use std::collections::HashMap;

fn delete_nth(lst: &[u8], n: usize) -> Vec<u8> {
    let mut items: HashMap<u8, i32> = HashMap::new();
    let limit: i32 = n as i32 + 1;
    let mut vec_final: Vec<u8> = Vec::new();
    for l in lst {
        let stat = items.entry(*l).or_insert(0);
        *stat += 1;
        match items.get(l) {
            Some(nn) => {
                if *nn < limit {
                    vec_final.push(*l)
                }
            }
            _ => (),
        }
    }
    println!("{:?}", vec_final);
    vec_final
}

fn main() {
    delete_nth(&[1, 1, 3, 3, 7, 2, 2, 2, 2], 3);
}
