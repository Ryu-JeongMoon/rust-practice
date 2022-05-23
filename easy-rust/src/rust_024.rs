pub fn print_collections() {
  let array2 = ["panda", "bear"];
  let array3 = ["panda", "bear", "yahoo"];

  // println!("is array2 same as array3? = {}", array2 == array3);

  // 0으로 채워서 32개 넣어주세유
  let buffer = [0; 32];
  println!("{:?}", buffer);

  let calendar = ["1월", "2월", "3월"];
  println!("{}", calendar[0]);
  println!("{}", calendar[1]);
  // println!("{}", calendar[3]);
  println!("{:?}", calendar.get(1));
  println!("{:?}", calendar.get(4));
}

/*
println!("is array2 same as array3? = {}", array2 == array3);
같은 array 타입이어도 요소의 개수가 다르면 다른 타입이라 비교가 불가하다고 뜸!
== 비교는 타입 비교는 아니고 요소까지 완전히 같아야 true 뜸

5 |   println!("is array2 same as array3? = {}", array2 == array3);
  |                                                     ^^ no implementation for `[&str; 2] == [&str; 3]`
  |
  = help: the trait `PartialEq<[&str; 3]>` is not implemented for `[&str; 2]`

println!("{}") 요 형태에 바로 찍고 싶으면 넣어주는 변수가 implements Display 여야 함둥
그게 아니라면 "{:?}" 로 debug print 해줘야 함
array 는 다른 언어와 유사하게 빠른데 편의성이 부족하다, byte buffer 용도로 주로 사용

println!("{}", calendar[3]); -> index out of bounds: the length is 3 but the index is 3 에러 뿡
println!("{:?}", calendar.get(4)); -> 값이 없는 경우에 None 뿡, 있으면 Some(value) 형태로 뿡
 */