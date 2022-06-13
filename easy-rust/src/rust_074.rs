fn print_basic_map() {
  let num_vec = vec![2, 4, 6];

  // Type을 언더바로 써서 컴파일러한테 맡겨버림?!
  let double_vec = num_vec.iter()
    .map(|number| number * 2)
    .collect::<Vec<_>>();
  println!("{double_vec:?}");
}

// 중간 연산을 아무리 해봤자 연산을 갈겨놓지는 않고,
// 다만 맵을 연속적으로 때렸으니 Map 구조체 안에다가 집어넣기는 해놓은 상태
fn print_huge_map() {
  let num_vec = vec![2, 4, 6];
  let double_vec = num_vec.iter()
    .map(|number| number * 2)
    .map(|number| number * 3)
    .map(|number| number * 4)
    .map(|number| number * 5)
    .map(|number| number * 6);
  println!("{double_vec:?}");
}

fn print_foreach() {
  let num_vec = vec![2, 4, 6];
  num_vec
    .iter() // 2, 4, 6
    .enumerate() // (index, number) 형태 tuple로 바꿔버림
    .for_each(|(i, n)| println!("{i} : {n}"));

  // tuple 이니 .으로 쓸 수 있긴 한데 가독성이 좋지 않다
  num_vec
    .iter()
    .enumerate()
    .for_each(|tuple| println!("{} : {}", tuple.0, tuple.1));
}

pub fn print_map_and_foreach() {
  print_basic_map();
  print_huge_map();
  print_foreach();
}
