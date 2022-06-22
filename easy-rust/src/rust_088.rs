pub fn print_debug_macro() {
  let mut my_number = dbg!(9);
  dbg!(my_number += 10);

  let new_vec = dbg!(vec![8,9,10]);

  dbg!();

  let double_vec = dbg!(new_vec
    .iter()
    .map(|x| x * 2)
    .collect::<Vec<i32>>());
}

/*
dbg! 연산, 할당문에 다 감싸버릴 수가 있다?!
빈 라인에다 찍고 거기까지는 수행되는지 파악할 수도 있다?!
 */