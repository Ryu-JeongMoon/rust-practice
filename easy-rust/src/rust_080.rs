pub fn print_option() {
  let first_try = vec![Some("🐼"), None, Some("success!"), None, None];
  let second_try = vec![None, Some("!"), Some("🐼"), None, Some("🐼")];
  let third_try = vec![Some("success!"), Some("success!"), Some("🐼"), Some("🐼!"), None];

  for i in 0..first_try.len() {
    println!("{:?}", first_try[i].or(second_try[i]).or(third_try[i]));
  }
}

/*
Option에다가 and(), or() 연산을 때릴 수 있고 일반적인 논리 연산의 결과를 가진다
잘 안 쓸듯..
 */