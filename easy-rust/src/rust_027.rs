pub fn print_from_into() {
  let my_str = String::from("yahoo");
  println!("{}", my_str);

  let my_str_into: String = "hooya".into();
  println!("{}", my_str_into);

  let my_panda_face = String::from('🐼');
  println!("{}", my_panda_face);

  let my_int_vec = Vec::from([1, 5, 8, 8]);
  println!("{:?}", my_int_vec);

  println!("{}", my_int_vec.capacity());
}

/*
let my_str_into = "hooya".into();
타입 안 줬다고 뭐라 하는데?! 헉 내부적으로 U::from(self) 호출함
기냥 into 갈겨버리면 앞에 문자열을 줘도 인식을 못하나벼
 */