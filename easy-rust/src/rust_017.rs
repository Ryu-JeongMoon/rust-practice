// OWNERSHIP, rust 의 핵심?!

// fn return_it() -> &'static String {
//   let not_my_country = String::from("America");
//   &not_my_country
// }

pub fn print_ownership() {
  let country = String::from("Korea");
  println!("my country is = {}", country);

  // let not_my_country = return_it();
  // println!("not my country is = {}", not_my_country);
}

/*
```
fn return_it() -> & String {
  let not_my_country = String::from("America");
  &not_my_country
}
```
3 | fn return_it() -> &String {
  |                   ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
return_it()에서 return type으로 &String 주고 다른 scope에서 호출해 local variable에 할당하면
'static lifetime 주라는 에러 뜸

```
fn return_it() -> &'static String {
  let not_my_country = String::from("America");
  &not_my_country
}
```
5 |   &not_my_country
  |   ^^^^^^^^^^^^^^^ returns a reference to data owned by the current function
컴파일러가 하라는 대로 하면 또 에러 뜸
return_it() 안에서만 유효한 local variable 의 reference를 밖으로 전파하는 것이고
이를 사용하는 쪽에서는 있지도 않은 값에 대한 메모리 주소를 넘겨 받은 것이다
즉, 자신이 소유할 수 없는 값에 대한 reference를 넘겨 받았기 때문에 에러 뜨는 것임둥

소유권이고 나발이고.. 왜 이렇게 하는걸까?
rust는 c, c++의 시스템 프로그래밍 위상을 대체하기 위해 나온 것이고
성능 뿐만 아니라 안전성을 향상시켜야 하는 책임을 가지고 있다 (그렇지 않으면 rust 쓸 필요가 없으니)
compiler가 알아서 malloc 해놓은 것들을 해제하는 코드를 명시적으로 박아버리기 때문에
return_it()에서 선언한 not_my_country 도 메모리 해제되야 한다
근디 이놈에 대한 reference를 넘기고 사용하면 쓰레기 값을 쓰는 것임둥
 */