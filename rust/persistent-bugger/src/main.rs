// https://www.codewars.com/kata/55bf01e5a717a0d57e0000ec/train/rust

fn persistence(num: u64) -> u64 {
    let mut num_str = num.to_string();
    let mut result = 0;
    loop {
        let str_num: u64 = num_str.parse().unwrap();
        if str_num < 10 {
            break;
        }

        let r: u64 = num_str
            .chars()
            .into_iter()
            .fold(1, |acc, x| x.to_digit(10).unwrap() as u64 * acc);

        result = result + 1;
        num_str = r.to_string();
    }
    result as u64
}

fn main() {
    persistence(999);
}
