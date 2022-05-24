pub fn print_vec() {
  let name1 = String::from("windoy");
  let name2 = String::from("gomchi");

  // 타입을 모르는 상태, Vec<?>로 추론됨 요놈은 반드시 push 해줘야 타입을 알 수 있게 된다
  let mut my_vec = Vec::new();
  println!("my_vec capacity = {}", my_vec.capacity());

  my_vec.push(&name1);
  println!("my_vec capacity = {}", my_vec.capacity());

  my_vec.push(&name2);
  println!("my_vec capacity = {}", my_vec.capacity());

  println!("my cats are {:?}", my_vec);

  let my_vec_vec = vec![name1.clone(), name2.clone(), name1.clone(), name2.clone()];
  println!("my cats are {:?}", my_vec_vec);
}

/*
타입 명시를 통해 명시적으로 알려줌
let mut my_vec2: Vec<String> = Vec::new();

Vec 도 사이즈 0으로 초기화된 후, doubling 친다
넣을 요소의 사이즈를 대강이라도 안다면 사이즈 주고 초기화하는 것이 좋음둥

vec! vec 매크로라고 하는 요 방식을 많이 사용한다고 함
String 중복으로 사용하려하니 used value 어쩌구 떠서 요럴 때 clone 써줄 것
 */