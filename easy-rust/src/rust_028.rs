pub fn print_tuples() {
  let my_tuple = (1, 'π€¬', "hehe", vec![1, 3, 4, 5, 1], [5, 7, 45]);
  println!("my tuple is {:?}", my_tuple);

  // μ°Έ λͺ»λ¬κ΅°
  println!("
inside the tuple is
first item = {}
second item = {}
third item = {}
fourth item = {:?}
fifth item = {:?}",
           my_tuple.0,
           my_tuple.1,
           my_tuple.2,
           my_tuple.3,
           my_tuple.4
  );

  let my_flexible_vec: Vec<(String, i32)> = vec!(("μ°ν€ν€".to_string(), 39), ("μ°μΊ¬μΊ¬".to_string(), 83));
  println!("my flexible vec is {:?}", my_flexible_vec);

  // μ€μ° destructuring κΉλ¦¬ν μΈμ΄κ΅¬λ§~~, a & b λ§κ³ λ κ΄μ¬ μμ!
  let (a, b, _, _, _) = my_tuple;
  println!("a = {}, b = {}", a, b);
}

/*
[] -> vec
() -> tuple
tuple μ νμ μκ΄ μμ΄ λ€ λλ €λ°μ μ μλ€
μ μ°νκ² μ¬μ©ν λΌλ©΄ μλ₯Ό μ°λ©΄ λκ² μ§λ§ rust λ κΉκΉμ΄κΈ° λλ¬Έμ
μμ λ€μ΄κ°λ μμλ‘ νμμ νλ³νκΈ°μ λΆνΈν  μ μλΉ
λν νμμ΄ μ ν΄μ Έμμ§ μμΌλ―λ‘ μ¬μ΄μ¦κ° λ€μ₯ λ μ₯νλ€λ λ¨μ μ΄ μλΉ

κ·ΈλΌ μ±λ₯μ μν΄ ν­μ Vec<λ¨μΌ νμ>μΌλ‘ μ¨μΌ νλκ°?
μΈμ΄μ κ·μΉμ κΉ¨μ§ μμΌλ©΄μ κΌΌμλ₯Ό μ°λ©΄ λλ€
Vec<Tuple<T, U>> ννλ‘ μ§μ΄λ£λλ° Tuple<T, U> νμμ΄ λ€μ΄κ° κ²μλ₯
 */