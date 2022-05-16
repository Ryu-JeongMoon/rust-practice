pub fn print_reference() {
  let my_number = 15; // This is an i32
  let single_reference = &my_number; //  This is a &i32
  let double_reference = &single_reference; // This is a &&i32
  let five_references = &&&&&my_number; // This is a &&&&&i32
  println!("{} {} {} {}", my_number, single_reference, double_reference, five_references);
}

/*
rust pointer -> reference 라 한다
시스템 프로그래밍을 위한 언어이기 땜시 pointer, stack, heap 잘 알아야 함둥
println!으로 찍으면 다 값이 나오넹 뭐임둥
 */