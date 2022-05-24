pub fn print_tuples() {
  let my_tuple = (1, '🤬', "hehe", vec![1, 3, 4, 5, 1], [5, 7, 45]);
  println!("my tuple is {:?}", my_tuple);

  // 참 못났군
  println!("
inside the tuple is
first item = {}
second item = {}
third item = {}
fourth item = {:?}
fifth item = {:?}",
           my_tuple.0,
           my_tuple.1,
           my_tuple.2,
           my_tuple.3,
           my_tuple.4
  );

  let my_flexible_vec: Vec<(String, i32)> = vec!(("우헤헤".to_string(), 39), ("우캬캬".to_string(), 83));
  println!("my flexible vec is {:?}", my_flexible_vec);

  // 오우 destructuring 까리한 언어구만~~, a & b 말고는 관심 없엉!
  let (a, b, _, _, _) = my_tuple;
  println!("a = {}, b = {}", a, b);
}

/*
[] -> vec
() -> tuple
tuple 은 타입 상관 없이 다 때려박을 수 있다
유연하게 사용할라면 얘를 쓰면 되겠지만 rust 는 깐깐이기 때문에
안에 들어가는 요소로 타입을 판별하기에 불편할 수 있당
또한 타입이 정해져있지 않으므로 사이즈가 들쑥 날쑥하다는 단점이 있당

그럼 성능을 위해 항상 Vec<단일 타입>으로 써야 하는가?
언어의 규칙을 깨지 않으면서 꼼수를 쓰면 된다
Vec<Tuple<T, U>> 형태로 집어넣는데 Tuple<T, U> 타입이 들어간 것임둥
 */