// unit struct
struct FileDirectory;

fn takes_file_directory(input: FileDirectory) {
  println!("i got a file directory");
}

// attribute struct
#[derive(Debug)]
struct Colour(u8, u8, u8);

// named struct
#[derive(Debug)]
struct Country {
  population: u32,
  capital: String,
  leader_name: String,
}

pub fn print_struct() {
  let f = FileDirectory;
  println!("the size of FileDirectory is {}", std::mem::size_of_val(&f));
  takes_file_directory(f);

  let c = Colour(15, 50, 95);
  println!("the size of Colour is {}", std::mem::size_of_val(&c));
  println!("Colour is {:?}", c);

  let canada = Country {
    population: 35_000_000,
    capital: "Ottawa".to_string(),
    leader_name: "Justin Trudeau".to_string(),
  };
  println!("country is {:#?}", canada);
}

/*
unit struct -> 리얼리 바이트가 0인 애, 이걸 쓰는 경우가 있을까?
attribute struct -> 간단하게 속성만 사용할 때 사용
named struct -> 가장 struct스러운 녀석
struct는 초기화 시키지 않으면 생성 조차 안 된다리

직접 만든 struct를 print하고 싶다면 #[derive(Debug)] 주고 디버그 프린트 찍던가 직접 Display 구현해야 한당
{:#?} 형태로 pretty print 할 수 있덩
 */