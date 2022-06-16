pub fn print_skip_fold() {
  let twenty_seven_chars = ('a'..).take(27).collect::<Vec<_>>();
  println!("{twenty_seven_chars:?}");

  let skip_and_then_ten_chars = ('a'..).skip(1111).take(10).collect::<Vec<_>>();
  println!("{skip_and_then_ten_chars:?}");

  let some_numbers = vec![9, 6, 9, 10, 11, 25];
  println!("{}", some_numbers
    .iter()
    .fold(0, |total_so_far, next_number| total_so_far + next_number));
}

/*
fold -> Java Stream reduce 동일?!
seed 값 주고 그 뒤에 어떻게 누적시킬지 결정?!
 */