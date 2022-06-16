fn is_in_char_vec(char_vec: &Vec<char>, check: char) {
  println!("Is {check} inside? {}", char_vec.iter().any(|&character| character == check));
}

// A - z 왜 전부 alphabetic 아닌가?, 특수 문자 땜시롱 이럴 때 debug print 생활화하자
fn print_is_alphabetic() {
  let smaller_vec = ('A'..'z').collect::<Vec<char>>();
  println!("All alphabetic? {}", smaller_vec.iter().all(|character| character.is_alphabetic()));
  println!("{smaller_vec:?}");
}

pub fn print_any_all() {
  let char_vec = ('a'..'吾').collect::<Vec<_>>();
  println!("{}", char_vec.len());
  println!("{}", char_vec.iter().count());

  is_in_char_vec(&char_vec, 'z');
  is_in_char_vec(&char_vec, '흙');

  let c1 = 'C';
  println!("{}", c1.is_uppercase());
  println!("{}", c1.is_alphabetic());

  print_is_alphabetic();
}


/*
동일
char_vec.len()
char_vec.iter().count()

any, all -> short circuit 으로 평가됨둥
any -> anyMatch와 동일, any() 안에서 reference 사용해야 함
|| 안에서 & 타입으로 갈기던가 || 옆에서 *로 표시해야 함
 */