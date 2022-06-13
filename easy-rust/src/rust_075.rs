use std::collections::HashMap;

pub fn print_zip() {
  let some_numbers = vec![0, 1, 2, 3, 4, 5]; // a Vec<i32>
  let some_words = vec!["zero", "one", "two", "three", "four", "five"];

  //
  let my_hashmap: HashMap<i32, &str> = some_numbers.into_iter()
    .zip(some_words.into_iter())
    .collect();
  println!("{my_hashmap:?}");

  let result = my_hashmap.get(&10).unwrap_or_else(|| {
    println!("Beep!");
    &"no number"
  });

  println!("{result}");
}

/*
.zip()으로 다른 두 iterator 를 합쳐버릴 수가 있다?!
a.iter().zip(b.iter()) 형태로 만들면 되고 a, b 둘 모두 iter 를 열어둔 상태여야 한다
a & b 자체가 소멸되도 상관 없다면 into_iter(), 아니라면 iter() 사용

a, b 길이가 다르면 길이를 맞춰서 짜른다 panic 안 뜸둥
 */