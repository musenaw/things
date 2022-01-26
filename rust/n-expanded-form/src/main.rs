// https://www.codewars.com/kata/5842df8ccbd22792a4000245/train/rust
fn expanded_form(n: u64) -> String {
    let mut result = "".to_string();
    let mut dd: i64 = 1;
    for nn in n.to_string().chars().rev() {
        println!("{}", nn.to_digit(10).unwrap());
        if nn.to_digit(10).unwrap() > 0 {
            result = format!(
                "{}{}{}",
                " + ",
                nn.to_digit(10).unwrap() as i64 * dd,
                result
            );
        };
        dd = dd * 10;
    }
    result[3..].to_string()
}

fn main() {
    println!("Hello, world!");
    expanded_form(12);
    expanded_form(42);
    expanded_form(70304);
}
