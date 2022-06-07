use std::num::ParseIntError;

// 마지막 줄에 값을 리턴시킬 때는 세미콜론을 빼라고 나온다
fn parse_str(input: &str) -> Result<i32, ParseIntError> {
  // early return, gives the error to the caller
  let parsed_number = input.parse::<i32>()?;

  println!("the number is {}", parsed_number);
  Ok(parsed_number)
}

pub fn print_question_mark_operator() {
  for item in vec!["seven", "6", "8.0", "99구구", "13413"] {
    println!("result = {:?}", parse_str(item));
  }
}

/*
? 는 orElseThrow 같은 역할을 하나봄둥
결과가 있으면 다음 라인으로 진행하고 없으면 바로 터트려서 Err() 반환시킴둥
 */