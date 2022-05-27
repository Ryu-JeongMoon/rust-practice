pub fn print_not_rust_style_loop() {
  let mut counter = 0;
  let mut second_counter = 0;

  'first_loop: loop {
    counter += 1;
    println!("counter is {}", counter);

    if counter > 5 {
      'second_loop: loop {
        second_counter += 1;
        println!("second counter is {}", second_counter);
        if second_counter > 3 {
          break 'first_loop;
        }
      }
    }
  }
}

/*
loop 는 냅다 반복 돌려버리고 시작, 종료 조건은 개발자가 알아서 처리해야 함
named loop 를 만들어서 유지보수하기 굉장히 좋은 콜백 지옥 형태로도 만들수도 있듬 ㅋ
 */