pub fn print_option() {
  let first_try = vec![Some("๐ผ"), None, Some("success!"), None, None];
  let second_try = vec![None, Some("!"), Some("๐ผ"), None, Some("๐ผ")];
  let third_try = vec![Some("success!"), Some("success!"), Some("๐ผ"), Some("๐ผ!"), None];

  for i in 0..first_try.len() {
    println!("{:?}", first_try[i].or(second_try[i]).or(third_try[i]));
  }
}

/*
Option์๋ค๊ฐ and(), or() ์ฐ์ฐ์ ๋๋ฆด ์ ์๊ณ  ์ผ๋ฐ์ ์ธ ๋ผ๋ฆฌ ์ฐ์ฐ์ ๊ฒฐ๊ณผ๋ฅผ ๊ฐ์ง๋ค
์ ์ ์ธ๋ฏ..
 */