use std::mem;

#[derive(Debug)]
struct Country {
  population: u32,
  capital: String,
  leader_name: String,
}

#[derive(Debug)]
struct Numbers {
  one: u8,
  two: u8,
  three: u8,
  four: u32,
}

pub fn print_size_of_struct() {
  let population = 35_000_000;
  let capital = "Ottawa".to_string();
  let leader_name = "Justin Trudeau".to_string();

  let leader = "Justin Bieber".to_string();

  // struct 에서 선언한 변수명과 할당할 변수명이 같다면 생략 가능
  let canada = Country {
    population,
    capital,
    leader_name,
  };

  println!("size of struct = {}", mem::size_of_val(&canada));
  println!("canada struct is {:#?}", canada);

  let my_numbers = Numbers {
    one: 5,
    two: 33,
    three: 96,
    four: 343144,
  };

  println!("size of numbers = {}", mem::size_of_val(&my_numbers));
  println!("numbers struct is {:#?}", my_numbers);
}

/*
할당할 변수명과 다른 경우에는 붙여줘야 함둥
let bieber = Country {
  population,
  capital,
  leader_name: leader
};

Numbers 는 1byte 짜리 3개, 4byte 짜리 1개인데 사이즈 찍어보면 왜 8이 나올까?
그게 더 효율적이기 때문이란다
일반적으로 32bit 컴퓨터의 경우 32bit -> 4byte 단위로 데이터를 다루는게 가장 효율적이라서
4byte, 1byte, 1byte, 1byte 할당 받지 않고 4byte, 4byte 할당 받고 남는 공간은 비워둔다
1byte까지 아껴쓰는 비트 구두쇠라면 변수 타입에 주의해야함둥

성능 극대화를 위해서는 하나의 struct 안에서 데이터 저장 효율을 높여야 한다
작은 바이트 수를 차지하는 애들이 여럿이고 뜬금 없게 큰 바이트를 차지하는 놈이 있다면
이 놈을 다른 struct로 옮겨 처리하는 방식도 있다
for statement 돌릴 때 데이터를 복사해서 돌려야 하는디 이 과정에 CPU, Mem 낭비 발생할 수도 있기 때문
 */