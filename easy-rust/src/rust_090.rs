// fn returns_reference<'a>() -> &'a str {
//   let my_string = "panda".to_string();
//   &my_string
// }

struct Book<'lifetime> {
  name: &'lifetime str
}

pub fn print_lifetime() {
  // println!("{:?}", returns_reference());

  let my_book = Book {
    name: "my_book!"
  };
  println!("{}", my_book.name);
}

/*
fn returns_reference<'a>() -> &'a str {
  let my_string = "panda".to_string();
  &my_string
}
3 |   &my_string
  |   ^^^^^^^^^^ returns a reference to data owned by the current function

struct Book<'lifetime> {
  name: &'lifetime str
}
'lifetime 이란 Generics와 같은 것, 임의의 문자로 주면 되고
Book에다가 걸면 <Book struct 살아있는 동안 유지될 것이다> 라는 의미임둥

c, c++ 에서의 call after free 문제를 해결하기 위해 나온 것으로
rust 는 컴파일 타임에 라이프 타임에 맞춰 메모리 해제 코드까지 다 박아버린다
긍게 scope 를 벗어나서 반환하거나 호출하는 문제를 컴파일 타임에 잡아줄 수 있는 것
 */