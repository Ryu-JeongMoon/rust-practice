// closure -> anonymous function that capture their environment ?!

use crate::rust_069::print_iterator;

pub fn print_closure() {
  // 호오,, || 로 캡처링 해분다, 요 형태는 자바의 Consumer 같은 놈?
  let my_closure = || println!("this is closure!");
  my_closure();

  // 길게 만들어야 하는 경우에는 |변수 필요한 경우 넣고| curly-bracket 열어서 만들 수 있당
  // 근디 이런 식으로는 잘 안 쓰고, iterator 에서 map 사용할 때 많이 씀둥
  let long_closure = || {
    let my_number = 10;
    let other_number = 55;
    println!("two numbers are {my_number} and {other_number}");
    my_number + other_number
  };

  let my_var = long_closure();
  println!("{my_var}");
}