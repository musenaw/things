// https://www.codewars.com/kata/5842df8ccbd22792a4000245/train/rust
fn string_iterator() {
    let test = "thisisanexample";
    let mut testit = test.chars();

    /* loop {
        match testit.next() {
            Some(x) => println!("{}", x),
            None => break,
        }
    } */

    let test2 = "dsds".to_string();
    let test23 = test2;
    println!("{}", test2);
    println!("{}", test23)
}

fn parse(code: &str) -> Vec<i32> {
    let mut value: i32 = 0;
    let mut res = vec![];
    code.chars().for_each(|x| {
        if x == 'i' {
            value += 1;
        }
        if x == 'd' {
            value -= 1;
        }
        if x == 's' {
            value *= value;
        }
        if x == 'o' {
            res.push(value);
            println!("{} added to output", value);
        }
    });
    res
}

// https://doc.rust-lang.org/book/ch13-00-functional-features.html

fn main() {
    string_iterator()
}
