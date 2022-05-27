fn print_while_loop() {
  let mut counter = 0;

  while counter != 5 {
    counter += 1;
    println!("counter is {}", counter);
  }
}

// exclusive range a..b
// inclusive range a..=b
fn print_for_loop() {
  // this is exclusive range
  for number in 0..3 {
    println!("number is {}", number);
  }

  for _number in 0..=3 {
    println!("yahoo");
  }
}

// loop 안에서 조건 주고 값을 반환시킬 수도 있다?!
fn print_return_value_in_loop() {
  let mut counter = 5;

  let my_number = loop {
    counter += 1;
    if counter % 56 == 3 {
      break counter;
    }
  };
  println!("my number is {}", my_number);
}

pub fn print_rust_style_loop() {
  // print_while_loop();
  // print_for_loop();
  print_return_value_in_loop();
}