#[derive(Debug)]
struct Book {
  title: String,
  year: u16
}

pub fn print_formatting_str() {
  let my_book = Book {
    title: "Some title".to_string(),
    year: 1919
  };

  let other_book = Book {
    title: "Other title".to_string(),
    year: 2020
  };

  println!("my book = {my_book:?}, other book = {other_book:?}");
  println!("my book name = {my_book:/^16?}");
}

/*
rust 1.58.0 부터 등장, 더욱 간단하게 디버그를 찍어볼 수 있게 됐다만 쓸모가 어디에 있을꼬
println!("my book = {my_book:?}, other book = {other_book:?}");

:? 원래 디버그 찍던 것들 사이에 표시자를 넣어줄 수 있다
양 옆으로 (*)로 패딩주고 (^) 중앙에 띄운 후 16자리로 나타내라
println!("my book name = {my_book:*^16?}");

1. 개빠른 속도로 C++을 대체하기 위함
2. 일반 언어에서 유용하게 쓰이는 기능들을 모아서 잡탕 찌개로 만드는 것이 러스트의 목표
 */