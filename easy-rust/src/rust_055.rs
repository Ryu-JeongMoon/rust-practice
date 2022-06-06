use std::collections::BinaryHeap;

fn show_remainder(input: &BinaryHeap<i32>) -> Vec<i32> { // This function shows the remainder in the BinaryHeap. Actually an iterator would be
  // faster than a function - we will learn them later.
  let mut remainder_vec = vec![];
  for number in input {
    remainder_vec.push(*number)
  }
  remainder_vec
}

pub fn print_binary_heap() {
  let many_numbers = vec![0, 5, 10, 15, 20, 25, 30];

  let mut my_heap = BinaryHeap::new();

  for number in many_numbers {
    my_heap.push(number);
  }

  while let Some(number) = my_heap.pop() {
    println!("Popped off {}. Remaining numbers are {:?}", number, show_remainder(&my_heap));
  }
}

/*
BinaryHeap은 priority-queue 구현 시에 사용한다
예를 들어 VIP 고객과 일반 고객을 나눠 처리하는 경우
가장 높은 우선 순위를 가진 놈만 후딱 처리해야할 때
BinaryHeap은 최상위만 신경 쓰고 나머지는 어떻게 되든 신경 쓰지 않는당
 */