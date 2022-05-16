fn give_number(one: i16, two: i16) -> i16 {
  let multiplied_by_ten = {
    let first_number = 10;
    first_number * one * two
  };

  multiplied_by_ten
}

pub fn print_number() {
  println!("{}", give_number(7, 8));
}

/*
rust 타입 깐깐, 연산 갈길 때 타입 다르면 에러 터트려버림
복잡한 연산을 한 후 할당하고 싶으면 코드 블록을 이용하면 됨 ex) DB 접속 후 데이터 조작
 */