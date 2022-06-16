fn print_rev() {
  let mut big_vec = vec![6; 1000];
  big_vec.push(5);

  println!("{}", big_vec.iter().rev().any(|number| *number == 5));
}

pub fn print_reverse_vec() {
  // print_rev();
  let mut big_vec = vec![6; 1000];
  big_vec.push(5);

  let mut big_count = 0;
  let mut big_iter = big_vec.clone().into_iter();

  let mut small_count = 0;
  let mut small_iter = big_vec.into_iter().rev();

  loop {
    big_count += 1;
    if big_iter.next() == Some(5) {
      break;
    }
  }

  loop {
    small_count += 1;
    if small_iter.next() == Some(5) {
      break;
    }
  }

  println!("big_count is {}", big_count);
  println!("small_count is {}", small_count);
}


/*
6으로 1000개 채워라, vec!['value', 'the number of value']

1000개 검사 후 마지막 하나에서 오께이 -> 1001번의 연산
big_vec.iter().any(|number| *number == 5);

1개 검사 후 바로 오께이 -> 1번의 연산
big_vec.iter().rev().any(|number| *number == 5);

대충 예상해서 찾으려는 값이 어느 쪽에서 가까울지를 판단하고 rev() 쓸지 말지 결정!
 */