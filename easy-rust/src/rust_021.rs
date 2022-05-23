fn add_is_great(country_name: &mut String) {
  country_name.push_str(" is great");
  println!("{}", country_name);
}

/*
얘는 왜 되는 걸까?
그냥 mut 쓰면 이 빵숀 스코프 안에서 설정을 하는 것, 소유권이 얘로 완전 넘어온 것
take by value, declare as mutable
 */
fn add_is_great_mut_version(mut country_name: String) -> String {
  country_name.push_str(" is great");
  println!("{}", country_name);
  country_name
}

pub fn print_mutable_reference_in_functions() {
  let mut my_country = "korea".to_string();
  add_is_great(&mut my_country);

  let return_value = add_is_great_mut_version(my_country);
  add_is_great_mut_version(return_value);
  println!("{}", return_value);
}

/*
아래 코드는 런타임 에러가 나는데 mut String 을 받는 빵숀으로 소유권을 넘기고 또 쓸라고 하기 때문임둥
이 개념을 위에서도 썼듯 'take by value, declare as mutable' 라고 하는데
변수를 값으로 받아버린 것이고 이 빵숀에서 반환하더라도
my_country 로 받은 애의 lifetime은 끝났고 반환 값으로 재할당 해줘야 쓸 수 있듬

```
let mut my_country = "korea".to_string();
let return_value = add_is_great_mut_version(my_country);
add_is_great_mut_version(return_value);
println!("{}", return_value);
```
 */