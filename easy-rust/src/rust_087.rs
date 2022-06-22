/*
match_indices -> random input 이 들어오는 상황에 유익?!
 */
use crate::rust_085::print_folds;

fn print_match_indices() {
  let rules = "Rule number 1: No fighting.
Rule number 2: Go to bed at 8pm.
Rule number 3: Wake up at 6 am.";

  let rule_locations = rules.match_indices("Rule").collect::<Vec<_>>();
  println!("Rule locations = {rule_locations:?}");
}

fn print_peekable() {
  let just_numbers = vec![1, 5, 100];
  let mut number_iter = just_numbers.iter().peekable();

  for _ in 0..3 {
    println!("I love the number {}", number_iter.peek().unwrap());
    println!("I really love the number {}", number_iter.peek().unwrap());
    println!("Next is {:?}", number_iter.next());
  }
}

pub fn print_match_indices_peekable() {
  print_match_indices();
  print_peekable();
}

