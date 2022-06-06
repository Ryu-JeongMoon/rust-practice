use std::collections::VecDeque;

pub fn print_vec_deque() {
  // 빨리 처리됨둥
  let mut huge_vec = VecDeque::from(vec![0; 600_000]);
  for i in 0..600_000 {
    huge_vec.remove(0);
  }
  println!("yahoo!")
}

/*
VecDeque, 요놈은 언제 쓰는가?
일반적인 Vec은 앞쪽에 위치한 요소를 지울 경우 shift to the left 발생한다
대부분 일반 Vec이 빠르지만 인덱스 상관 없이 요소의 변경이 자주 일어나는 경우에는 VecDeque을 써야함둥

오래 걸림둥
let mut huge_vec = vec![0; 600_000];
for i in 0..600_000 {
  huge_vec.remove(0);
}
 */