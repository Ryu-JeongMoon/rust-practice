pub fn print_strings() {
  // this is &str
  let first_name = "panda";

  let second_name = String::from("panda");

  // to_string() 안해놓으면 str로 인식되어 push 연산을 갈길 수 없다
  let mut third_name = "panda".to_string();
  third_name.push('!');

  println!("{} {} {}", first_name, second_name, third_name);
}

/*
String -> Sized Type
&str(ref str) -> Dynamic Type

무엇을 써야하는고?
String은 그 자체로 값을 가지고 있는 타입이어서 다양한 연산을 갈길 수 있다
얘는 Heap에 위치하고 몇 바이트인지 정확히 알 수 있다 (값이므로)

&str은 reference 이니까 가르키는 그 값이 존재하는지 아닌지도 신경 쓰면서 사용해야 하니 귀찮당
또한 가르키는 것이 값의 위치를 나타내는 메모리 주소이기 땜시 몇 바이트인지 모른당
 */