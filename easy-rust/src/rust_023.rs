pub fn print_uninitialized_variables() {
  // not rust style
  let my_number;
  {
    my_number = 9;
  }
  println!("my number = {}", my_number);

  // rust style
  let my_number_in_rust = {
    // complicated logic
    let x = 32;
    x + 9
  };
  println!("my number in rust = {}", my_number_in_rust);
}

pub fn print_loop() {
  let mut counter = 0;
  loop {
    counter += 1;
    // rust 에서는 if 뒤 {} 강제됨
    if counter % 50 == 0 {
      break;
    }
  }
  println!("my number in loop = {}", counter);
}

/*
uninitialized variables, loop
java, c 계열의 사람들이 가지고 있는 코드 스타일을 위해 만들어는 놨는데
러스트에서는 이런 방식을 사용하지 않는다 함
함수형 스타일로 돌리는 듯?
 */