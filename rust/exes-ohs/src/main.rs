// https://www.codewars.com/kata/55908aad6620c066bc00002a/solutions/rust

fn xo(string: &'static str) -> bool {
    let mut cont = 0;
    for (_i, c) in string.chars().enumerate() {
        if c == 'x' || c == 'X' {
            cont += 1;
        } else if c == 'o' || c == 'O' {
            cont -= 1
        }
    }
    return cont == 0;
}

// best solution
fn xoSol(string: &'static str) -> bool {
    string.chars().fold(0, |a, c| match c {
        'x' | 'X' => a + 1,
        'o' | 'O' => a - 1,
        _ => a,
    }) == 0
}

fn main() {
    let result = xo("xoo");
    println!("{}", result);
}
