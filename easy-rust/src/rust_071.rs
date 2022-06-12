use crate::rust_069::print_iterator;

#[derive(Debug)]
struct Library {
  library_type: LibraryType,
  books: Vec<String>
}

#[derive(Debug)]
enum LibraryType {
  City,
  Country
}

impl Library {
  // 매번 to_string() 호출 시키지 않고 받는 쪽에서 바꿔주면 편안-
  fn add_book(&mut self, book: &str) {
    self.books.push(book.to_string());
  }

  fn new() -> Self {
    Self {
      library_type: LibraryType::City,
      books: Vec::new()
    }
  }
}

// 내가 정의한 타입을 돌려버리고 싶을 때, impl Iterator 때려준다
impl Iterator for Library {
  type Item = String;

  fn next(&mut self) -> Option<Self::Item> {
    match self.books.pop() {
      // String + &str 정도까지는 가능하다고 한다, 정말 대단하구만
      Some(book_title) => Some(book_title + "is found!"),
      None => None
    }
  }
}

pub fn print_impl_iterator() {
  let mut my_library = Library::new();
  my_library.add_book("The Doom of the Darksword"); // add some books
  my_library.add_book("Demian - die Geschichte einer Jugend");
  my_library.add_book("구운몽");
  my_library.add_book("吾輩は猫である");

  println!("{my_library:?}");

  for item in my_library {
    println!("{item}");
  }
}