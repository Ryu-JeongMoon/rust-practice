pub fn print_map() {
  let some_output
    = Some(vec![8, 9, 10]);
  // : Option<Vec<i32>> = None;

  let first = some_output
    .clone()
    .map(|some_vec| some_vec.iter().map(|number| number + 1).collect::<Vec<i32>>());

  let second = some_output
    .and_then(|some_vec| match some_vec.len() {
      0 => None,
      1 => Some(vec![some_vec[0]]),
      _ => Some(some_vec)
    });

  println!("{first:?}");
  println!("{second:?}");
}

/*
let some_output: Option<Vec<i32>> = None;
-> None

let second = some_output
    .map(|some_vec| match some_vec.len() {
      0 => None,
      1 => Some(vec![some_vec[0]]),
      _ => Some(some_vec)
    });
-> Some(Some([8, 9, 10])) Some of Some of Vec<i32>
map()을 때리면 Option이 나오고 그 안에서도 Option으로 반환하니 Option of Option이 되었다
요럴 때 and_then()을 써줘서 Option을 한 꺼풀 벗겨내면 된다

.and_then(|some_vec| match some_vec.len() {
-> Some([8, 9, 10])
 */