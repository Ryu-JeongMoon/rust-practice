// find - Option<Self::Item>
// position - Option<usize>
// cycle - limitless iterator

pub fn print_find_position_cycle() {
  let num_vec = vec![10, 20, 30, 40, 50, 60, 70, 80, 90];
  // 있으면 Some(value)
  println!("{:?}", num_vec.iter().find(|&n| n % 3 == 0));
  // 없으면 None
  println!("{:?}", num_vec.iter().find(|&n| n % 11 == 0));

  let mut even_odd = vec!["even", "odd"].into_iter().cycle();
  let even_odd_vec = (0..7)
    .zip(even_odd)
    .collect::<Vec<(i32, &str)>>();
  println!("{even_odd_vec:?}");

  even_odd = vec!["even", "odd"].into_iter().cycle();
  let sixteen_items = even_odd.take(16).collect::<Vec<_>>();
  println!("{sixteen_items:?}");
}

/*
cycle로 끝나지 않는 iterator 만들고 zip, take 등을 이용해 연산을 갈긴다
 */