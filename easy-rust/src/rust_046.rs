fn take_dangerous_fifth(value: Vec<i32>) -> i32 {
  value[4]
}

pub(crate) fn take_safe_fifth(value: Vec<i32>) -> Option<i32> {
  if value.len() < 5 {
    None
  } else {
    Some(value[4])
  }
}

pub fn print_option_and_result() {
  let short_vec = vec![1, 2];
  let long_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

  // println!("yahoo ~~ {}", take_dangerous_fifth(short_vec));
  println!("short yahoo ~~ {:?}", take_safe_fifth(short_vec));
  println!("long yahoo ~~ {:?}", take_safe_fifth(long_vec));
}

/*
아무런 처리 없이 IndexOutOfBoundError 뜰 수 있는 코드를 작성하고 실행 시키면 아래 에러 뜸
thread 'main' panicked at 'index out of bounds: the len is 2 but the index is 4', src/rust_046.rs:2:3
fn take_dangerous_fifth(value: Vec<i32>) -> i32 {
  value[4]
}

panicked 란?
컴파일은 되는데 런타임 시 터질 수 있는 scope에 들어간 경우 화들짝 놀라서 다시 나오는 상황을 의미

자바의 Optional을 enum으로 구현 해놓음
 */