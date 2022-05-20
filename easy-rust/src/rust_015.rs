pub fn print_string_type() {
  let mut my_name = String::with_capacity(29);
  println!("Length is {} and Capacity is {}", my_name.len(), my_name.capacity());

  my_name.push_str("panda bear");
  println!("Length is {} and Capacity is {}", my_name.len(), my_name.capacity());

  my_name.push_str("and i live in seoul");
  println!("Length is {} and Capacity is {}", my_name.len(), my_name.capacity());

  my_name.push('!');
  println!("Length is {} and Capacity is {}", my_name.len(), my_name.capacity());

  let other_name = "panda";
  let mut other_name_string = other_name.to_string();
  println!("Length is {} and Capacity is {}", other_name_string.len(), other_name_string.capacity());

  other_name_string.push('!');
  println!("Length is {} and Capacity is {}", other_name_string.len(), other_name_string.capacity());
}

/*
reallocation 방지하고자 String::with_capacity(size)로 만들었을 때
설정한 크기 초과 시 doubling (진짜 두배) 쳐버림
자바 쓰다가 러스트 쓰니까 개귀찮지만 이건 모두 성능을 위한 것?!
자바 String도 불변이지만 더하기 연산 때리면 내부적으로 새로운 String 만들어 반환한다
러스트에서는 이 귀찮은 작업을 직접 해야한다, 그래야 더 안전하고 빠릉게

&str -> to_string() 갈겨줘야 문자열 연산을 할 수 있당
왜냐?! &str -> reference, String -> value 이기 때문이지
 */