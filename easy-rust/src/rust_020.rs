fn print_country(country_name: String) {
  println!("my country is {}", country_name)
}

fn print_country_reference_version(country_name: &String) {
  println!("my country is {}", country_name)
}

pub fn print_references_in_functions() {
  let country = "Korea".to_string();
  print_country_reference_version(&country);
  print_country_reference_version(&country);
  print_country_reference_version(&country);

  print_country(country);
}

/*
```
let country = "Korea".to_string();
print_country(country);
print_country(country);
```
6 |   let country = "Korea".to_string();
  |       ------- move occurs because `country` has type `String`, which does not implement the `Copy` trait
7 |   print_country(country);
  |                 ------- value moved here
8 |   print_country(country);
  |                 ^^^^^^^ value used here after move

Use of moved value 컴파일 에러 뜸
이것은 또 무엇이냐하믄 country 라는 변수명으로 String 타입의 값을 넣어줬다
얘는 print_country() 의 인자로 쏙 들어가고 그 안에서 작업 수행 후 생을 다한다
즉 print_country() 두번 호출하지 않더라도 println!("len = {}", country.len());
요놈 같은 것 호출해도 value borrowed here after move 에러 뜬다
소유권이 print_country() 로 이전된 후에 사용됐기 때문에 에러 뿜뿜

요놈을 해결하려면 reference 로 넘겨 소유권을 넘기는 것이 아닌 값이 있는 주소를 넘기는 방식으로 푼다
받는 쪽에서 &String 으로 받고 보내는 쪽에서도 마찬가지로 reference 로 넘겨줘야 한다
아리송송..
 */