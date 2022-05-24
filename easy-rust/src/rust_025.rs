pub fn print_slices() {
  let seasons = ["spring", "summer", "fall", "winter", "spring", "summer", "fall", "winter"];

  // closed range 표현
  println!("{:?}", &seasons[0..3]);

  // open range 표현
  println!("{:?}", &seasons[1..=4]);

  //
  println!("{:?}", &seasons[..4]);

  //
  println!("{:?}", &seasons[5..]);
}

/*
아래와 같이 사용하면 slices 타입으로 나오는데 slices 는 dynamically sized type 이란다
seasons[5..]

그게 뭐꼬
말 그대로 동적으로 사이즈가 정해진다는 것이고 컴파일러는 그 크기를 알 수 없다
레퍼런스로 넘겨야 하는데 레퍼런스로 넘기면 어떻게 아는 것일까?
레퍼런스 찾아가서 사이즈 확인하남?

Array : Vec(Vector) -> &str : String
두 관계가 유사하다
Array 는 개빠른 대신 기능이 별로 없고 &str 도 마찬가지
Vec 은 편의성은 뛰어나지만 Array 에 비해 성능이 떨어진다, String 도 마찬가지
String 이 내부적으로 Vec 으로 만들어져 있기 때문!
 */