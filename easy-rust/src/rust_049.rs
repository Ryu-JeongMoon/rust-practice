use std::num::ParseIntError;

fn check_if_five(number: i32) -> Result<i32, String> {
  match number {
    5 => Ok(number),
    _ => Err("It's not number 5, idiot".to_string())
  }
}

fn parse_number(number: &str) -> Result<i32, ParseIntError> {
  number.parse()
}

pub fn print_result() {
  let mut result_vec = Vec::new();
  for number in 2..=7 {
    result_vec.push(check_if_five(number));
  }
  println!("the result is {:#?}", result_vec);

  let mut result_vec = vec![];
  result_vec.push(parse_number("8"));
  result_vec.push(parse_number("yahoo"));
  result_vec.push(parse_number("8"));
  for result in result_vec {
    println!("the result is {:?}", result);
  }
}