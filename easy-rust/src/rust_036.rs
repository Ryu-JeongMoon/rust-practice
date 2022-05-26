use Star::*;

use crate::rust_036::Number::{I32, U32};

// 명시적으로 수를 줄 수도 있다, 요 때 값을 안 준 놈은 이전 값에 +1한 걸로 들어감
enum Star {
  BrownDwarf = 10,
  YellowStar = 100,
  DeadStar = 1030,
  RedDwarf = 50,
  RedGiant = 666,
  ColdPlay = 500,
  Bibi,
}

enum Number {
  U32(u32),
  I32(i32),
}

fn get_number(input: i32) -> Number {
  match input.is_positive() {
    true => U32(input as u32),
    _ => Number::I32(input)
  }
}

pub fn print_stars() {
  let star_vec = vec![BrownDwarf, RedDwarf, YellowStar, ColdPlay, RedGiant, DeadStar, Bibi];

  for star in star_vec {
    match star as u32 {
      size if size < 400 => println!("not so much big = {}", size),
      size if size > 499 => println!("pretty big = {}", size),
      _ => println!("it's hugeeee")
    }
  }

  let num_vec = vec![get_number(-8393), get_number(404)];
  for num in num_vec {
    match num {
      U32(number) => println!("it's an u32 with the value of {}", number),
      I32(number) => println!("it's a i32 with the value of {}", number)
    }
  }
}