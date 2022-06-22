use std::cmp::max;

use crate::rust_085::print_folds;

fn print_max() {
  let my_vec = vec![-878, 897973, 94134, 0, -123123];

  let biggest = my_vec
    .into_iter()
    .fold(i32::MIN, |num1, num2|
      max(num1, num2),
    );

  println!("biggest is {biggest}");
}

fn print_chunks_and_windows() {
  let my_vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

  for chunk in my_vec.chunks(3) {
    println!("Chunk is {chunk:?}");
  }

  for window in my_vec.windows(3) {
    println!("Window is {window:?}");
  }
}

pub fn print_chunks_windows() {
  // print_max();
  print_chunks_and_windows();
}

/*
Math.max()가 아니라 std::cmp::max

Chunk -> parameter 넘기는 size 크기로 잘라서 반환
Window -> 한칸씩 밀려서 size 크기로 잘라서 반환
 */