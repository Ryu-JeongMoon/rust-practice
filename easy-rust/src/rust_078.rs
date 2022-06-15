use std::fmt::format;

use crate::rust_077;
use crate::rust_077::Company;

fn print_numbers() {
  let user_inputs = vec!["8.9", "Nine point nine five", "8.0", "7.6", "elventy-twelve"];

  let actual_numbers = user_inputs
    .into_iter()
    .filter_map(|input| input.parse().ok())
    .collect::<Vec<f32>>();

  println!("{actual_numbers:?}");
}

fn print_company() {
  let company_vec = vec![
    Company::new("Umbrella Corporation", "Unknown"),
    Company::new("Ovintiv", "Doug Suttles"),
    Company::new("The Red-Headed League", ""),
    Company::new("Stark Enterprise", ""),
    Company::new("Apple", "Tim Cook"),
    Company::new("Samsung", "Lee"),
  ];

  let mut results_vec = vec![];
  company_vec
    .iter()
    .for_each(|company| {
      results_vec.push(company.get_ceo().ok_or("No CEO Found"));
    });
  println!("{results_vec:?}\n");
}

pub fn print_filter_map() {
  // print_numbers();
  print_company();
}

/*
.collect::<Vec<_>>();
요거를 추론 못 해부리네

4 |   let actual_numbers = user_inputs
  |       -------------- consider giving `actual_numbers` the explicit type `Vec<T>`, where the type parameter `B` is specified
5 |     .into_iter()
6 |     .filter_map(|input| input.parse().ok())
  |      ^^^^^^^^^^ cannot infer type for type parameter `B` declared on the associated function `filter_map`

ok(), Result to Option
ex) 5.ok() -> Some(5)
ok_or(), Result or Err
ok_or_else(),

format!("No CEO Found for {}", company.name);
format!이 원래는 &str 반환했다가 String 반환으로 바뀐듯, 컴파일 에러 터짐
 | |________^ expected `&str`, found struct `String`

개 같은 러스트 계속 터진다
company_vec
  .iter()
  .for_each(|company| {
    results_vec.push(company.get_ceo().ok_or_else(|| {
      let err_message: &str = &format!("No CEO Found for {}", company.name);
      println!("{err_message}");
      err_message.into()
    }));
  });
 */