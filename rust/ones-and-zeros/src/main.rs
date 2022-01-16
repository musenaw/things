// https://www.codewars.com/kata/578553c3a1b8d5c40300037c/train/rust

fn binary_slice_to_number(slice: &[u32]) -> u32 {
    let mut binary_string = "".to_string();
    for numb in slice.iter() {
        binary_string = binary_string + &numb.to_string();
    }
    let rr = u32::from_str_radix(binary_string.as_str(), 2).unwrap();
    println!("{:?}", rr);
    rr
}

fn main() {
    binary_slice_to_number(&vec![1, 1, 1, 1]);
}
