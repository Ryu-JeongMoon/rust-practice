use std::collections::HashMap;

// or_insert 없으면 넣고 있으면 말고?!
fn print_useless_hashmap() {
  let book_collection = vec![
    "L'Allemagne Moderne",
    "Le Petit Prince",
    "쉐도우 오브 유어 스마일",
    "Eye of the World",
    "Eye of the World",
  ];

  let mut book_hashmap = HashMap::new();

  for book in book_collection {
    book_hashmap.entry(book).or_insert(true);
  }

  for (book, true_or_false) in book_hashmap {
    println!("do we have {}?, {}", book, true_or_false);
  }
}

fn print_useful_hashmap() {
  let book_collection = vec![
    "L'Allemagne Moderne",
    "Le Petit Prince",
    "쉐도우 오브 유어 스마일",
    "Eye of the World",
    "Eye of the World",
  ];

  let mut book_hashmap = HashMap::new();

  for book in book_collection {
    let mut number_of_books = book_hashmap.entry(book).or_insert(0);
    *number_of_books += 1;
  }

  for (book, number) in book_hashmap {
    println!("{} - {}", number, book);
  }
}

pub fn print_hashmap() {
  // print_useless_hashmap();
  print_useful_hashmap();
}