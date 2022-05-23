fn print_number(input: i32) {
  println!("my number = {}", input);
}

fn print_string(input: String) {
  println!("my country => {}", input);
}

pub fn print_copy_and_clone() {
  let my_number = 17;
  print_number(my_number);
  print_number(my_number);

  let my_country = "Korea".to_string();
  print_string(my_country.clone());
  print_string(my_country.clone());
  print_string(my_country.clone());
  
  // 소유권 넘겨버려
  print_string(my_country);
}

/*
copy types -> primitive types
non-copy types -> reference types

my_number 를 넘겼다고 해서 소유권이 넘어가는 것이 아니다
copy types 는 항상 copy 후 넘어간다
값 자체를 넘기는 건 몇번을 넘겨도 상관이 없둥

아래 코드는 Use of moved value 컴파일 에러 뜸
let my_country = "Korea".to_string();
print_string(my_country);
print_string(my_country);

input: String 으로 받는 메서드의 경우엔 소유권을 항상 넘겨버리기 때문에
호출 후 기존 스코프에서는 다시 사용할 수 없게 되는 문제가 발생한다
요럴 때 바로 clone() 떠서 복사해서 넘길 수 있다
non-copy types 경우에 사용할 수 있둥
 */