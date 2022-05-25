pub fn print_control_flow() {
  let my_number = 5;
  if my_number == 6 {
    println!("it's a six");
  } else if my_number == 8 {
    println!("it's a eight");
  } else {
    println!("it's another number");
  }

  let second_number: u8 = 155;
  let third_number = match second_number {
    0 => 1,
    1 => 2,
    2 => 3,
    3 => 4,
    _ => 255
  };
  println!("third number is {}", third_number);
}

/*
자바, 코틀린의 switch-pattern matching을 match로 사용할 수 있다
expression-based language이기 땜시 값으로 받아 처리할 수도 있다!!
 */