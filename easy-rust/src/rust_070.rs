pub fn print_iterators_assert_eq() {
  let my_vec = vec!['a', 'b', '우', '헤', '헤'];
  let mut my_vec_iter = my_vec.iter();

  // junit - assertEquals() ?!, 테스트용으로 사용한다고 함둥
  println!("{my_vec_iter:?}");
  assert_eq!(my_vec_iter.next(), Some(&'a'));
  assert_eq!(my_vec_iter.next(), Some(&'b'));
  assert_eq!(my_vec_iter.next(), Some(&'우'));
  assert_eq!(my_vec_iter.next(), Some(&'헤'));
  assert_eq!(my_vec_iter.next(), Some(&'헤'));
}

/*
assert_eq!(my_vec_iter.next(), Some('a'));
8 |   assert_eq!(my_vec_iter.next(), Some('a'));
  |   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `&char`, found `char`
으잉 char 넣은 것 같은데 next()로 가져오면 &char 되나봄?!
공식 문서에 예시로 assert_eq()를 많이 사용하기 땜시 얘로 검색해서 보면 좋다고 함둥
 */