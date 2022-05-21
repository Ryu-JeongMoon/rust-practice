// Cannot borrow immutable local variable `my_number` as mutable
// fn print_mutable_first() {
//   let mut my_number = 10;
//   let my_ref = &my_number;
//   let mut my_change = &mut my_number;
//   *my_change += 10;
//
//   println!("my number is ?! => {}", my_ref)
// }

fn print_immutable_first() {
  let mut my_number = 10;
  let mut my_change = &mut my_number;
  *my_change += 10;

  let my_ref = &my_number;
  println!("my number is ?! => {}", my_ref);
}

/*
이게 대체 무슨 짓거리인가?
my_ref 는 my_country 에 대해 immutable borrow 떴다
그 다음 같은 이름으로 새로 변수를 선언하면 ?
shadowing 이라 해서 기존의 값은 접근할 수 없으며 뒤에 선언한 값으로 엎어친다
근디 선언 전에 reference 만들어 두었으니 shadowing 된 상태로도 접근이 가능하다리
기존의 my_country 로는 접근할 수 없으나 my_ref 로는 가능, 참으로 신기하구먼
 */
fn print_shadowing() {
  let my_country = "Korea";
  let my_ref = &my_country;
  let my_country = 8;
  println!("my country is {}, {}", my_ref, my_country)
}

pub fn print_unique_reference() {
  // print_mutable_first();
  print_immutable_first();
  print_shadowing();
}

/*
print_mutable_first 호출하면 ?
reference 만드는 것을 immutable borrow 라 하는구만?!
요거는 immutable 맨들어놓구 mutable 또 맨들어서 값을 바꿔버린 상황
immutable 로 땡겨왔는데 값이 바뀐다? 컴파일러 개빡침
4 |   let my_ref = &my_number;
  |                ---------- immutable borrow occurs here
5 |   let mut my_change = &mut my_number;
  |                       ^^^^^^^^^^^^^^ mutable borrow occurs here
...
8 |   println!("my number is ?! => {}", my_ref)
  |                                     ------ immutable borrow later used here

print_immutable_first 호출하면?
오께이, 이건 왜 오께이인고
유연성을 더하다보니 복잡성이 증가한 케이스인데
mutable reference 생성하고 얘로 값을 바꿔버린 후 (mutable borrow)
바뀐 값에 대한 immutable reference 땡긴 것이다 (immutable borrow)
같은 scope 내에 mutable, immutable 공존한다고 해서 무조건 에러 뿜이 아니라
실질적으로 immutable 이 지켜지는 상황이라면 에러 뿜 노노

단 immutable 해놓구 그 이후에도 mutable reference 로 조작하면 바로 에러 뿜
아래와 같이 바꾸지 않고 사용하는 코드가 있기만 해도 뿜뿜
```
println!("my number is ?! => {}", my_ref);
println!("my number is ?! => {}", my_change);
```
13 |   let mut my_change = &mut my_number;
   |                       -------------- mutable borrow occurs here
...
16 |   let my_ref = &my_number;
   |                ^^^^^^^^^^ immutable borrow occurs here
17 |   println!("my number is ?! => {}", my_ref);
18 |   println!("my number is ?! => {}", my_change);
   |                                     --------- mutable borrow later used here
 */