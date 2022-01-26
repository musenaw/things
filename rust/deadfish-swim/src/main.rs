// https://www.codewars.com/kata/51e0007c1f9378fa810002a9/train/rust

fn parse(code: &str) -> Vec<i32> {
    let mut result = 0;
    let mut rvec: Vec<i32> = vec![];
    for cod in code.chars() {
        match cod {
            'd' => result -= 1,
            'i' => result += 1,
            's' => result = result * result,
            'o' => rvec.push(result),
            _ => continue,
        }
    }
    return rvec;
}

fn main() {
    assert_eq!(parse("iiisdoso"), vec![8, 64]);
    assert_eq!(parse("iiisdosodddddiso"), vec![8, 64, 3600]);
}
