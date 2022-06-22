fn print_number_folds() {
  let some_numbers = vec![9, 6, 9, 10, 11];

  println!("{}", some_numbers
    .iter()
    .fold(0, |total_so_far, next_number| total_so_far + next_number));
}

fn print_string_folds() {
  let a_string = "I don't have any dashes in me.";

  let dashed = a_string
    .chars()
    .fold("-".to_string(), |mut string_so_far, next_char| {
      string_so_far.push(next_char);
      string_so_far.push('-');
      string_so_far
    });

  println!("{dashed}");
}

pub fn print_folds() {
  // print_number_folds();
  print_string_folds();
}

/*
Java Stream API reduce 와 똑같슴둥
fold(초기값, |누적값, 다음값|{}) 형태로 사용
 */