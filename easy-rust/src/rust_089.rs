pub fn print_inspect() {
  let my_vec = vec![8, 9, 10];

  let double_vec = my_vec.iter()
    .inspect(|n| {
      dbg!(n);
    })
    .map(|n| n * 2)
    .inspect(|n| {
      dbg!(n);
    })
    .filter(|n| *n >= 17)
    .collect::<Vec<_>>();
}

/*
inspect == peek ?!

.inspect(|x| dbg!(x))
mismatched types [E0308] expected `()`, found `&i32`
왜 안 되는가?, dbg!(value) 리턴 값이 &i32로 나와서 임둥
 */