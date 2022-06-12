// iterator -> a collection of things that you can call .next() on
// .iter() -> iterator of references &T
// .iter_mut() -> iterator of mutable references &mut T
// .into_iter() -> consuming iterator

pub fn print_iterator() {
  let vector1 = vec![1, 2, 3];

  // turbo-fish 형태로 작성하던가 타입을 명시해줘야 한다, 안 해주면 컴파일에 터짐
  // turbo-fish 형태가 물 흐르듯이 읽히기 땜시 사람이 이해하기가 더 쉬울 수 있다?!
  let vector2 = vector1.iter().map(|x| x + 1).collect::<Vec<i32>>();
  let vector3: Vec<i32> = vector1.into_iter().map(|x| x * 10).collect();
  let vector4 = vector2.into_iter().map(|x| x * 15).collect::<Vec<i32>>();

  // iter_mut()는 왜 쓰는 것인가?, 기존의 컬렉션의 소유권을 땡겨오지 않고 값만 쓰기 위함임둥
  let mut vector5 = vec![10, 20, 30];
  // reference 니께 값을 쓰려면 *을 붙여야 한다?!
  let vector6 = vector5.iter_mut().map(|x| *x / 5).collect::<Vec<i32>>();

  // println!("{vector1:?}");
  // println!("{vector2:?}");
  println!("{vector3:?}");
  println!("{vector4:?}");
  println!("{vector5:?}");
  println!("{vector6:?}");
}

/*
into_iter()로 소비해버렸응게 소유권이 없다!!
println!("{vector1:?}");
println!("{vector2:?}");
note: this function takes ownership of the receiver `self`, which moves `vector1`
    |
254 |     fn into_iter(self) -> Self::IntoIter;
    |                  ^^^^
    = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

 */