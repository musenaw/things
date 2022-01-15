fn solution(phrase: &str) -> String {
    let f: Vec<_> = phrase.split("").collect();
    let mut final_string = "".to_string();
    for letter in f.iter() {
        final_string = letter.to_string() + &final_string;
    }
    final_string
}

fn main() {
    let mut phrase = String::from("Hello, world!");
    phrase = solution(&phrase);
    println!("{}", phrase);
}
