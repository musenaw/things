use std::collections::HashSet;

fn run_main<T>(thing: T, things: Vec<T>, contains_item: T, remove_item: T)
where
    T: std::fmt::Display + std::cmp::Eq + std::hash::Hash,
{
    let mut books = HashSet::new();

    books.insert(thing);
    for th in things {
        books.insert(th);
    }

    // Check for a specific one.
    if !books.contains(&contains_item) {
        println!(
            "We have {} books, but {} ain't one.",
            books.len(),
            contains_item
        );
    }

    // Remove a book.
    books.remove(&remove_item);

    // Iterate over everything.
    for book in &books {
        println!("{}", book);
    }
}

fn main() {
    run_main(
        "aa".to_string(),
        ["bbb".to_string()].to_vec(),
        "cc".to_string(),
        "bb".to_string(),
    );
    run_main(123, [1, 2, 3].to_vec(), 4, 1);
}
