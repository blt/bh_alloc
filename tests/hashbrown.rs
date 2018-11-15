extern crate bh_alloc;
extern crate hashbrown;

#[global_allocator]
static ALLOC: bh_alloc::BumpAlloc = bh_alloc::BumpAlloc::INIT;

use hashbrown::HashMap;

#[test]
fn doc_example() {
    let mut book_reviews = HashMap::new();

    // review some books.
    book_reviews.insert("Adventures of Huckleberry Finn", "My favorite book.");
    book_reviews.insert("Grimms' Fairy Tales", "Masterpiece.");
    book_reviews.insert("Pride and Prejudice", "Very enjoyable.");
    book_reviews.insert("The Adventures of Sherlock Holmes", "Eye lyked it alot.");

    // check for a specific one.
    assert!(!book_reviews.contains_key("Les Misérables"));

    // oops, this review has a lot of spelling mistakes, let's delete it.
    assert!(book_reviews.contains_key("The Adventures of Sherlock Holmes"));
    book_reviews.remove("The Adventures of Sherlock Holmes");
    assert!(!book_reviews.contains_key("The Adventures of Sherlock Holmes"));

    // look up the values associated with some keys.
    assert_eq!(
        book_reviews.get("Pride and Prejudice"),
        Some(&"Very enjoyable.")
    );
    assert_eq!(book_reviews.get("Alice's Adventure in Wonderland"), None);
}
