use std::mem::size_of;

pub fn print_chars() {
  println!("Size of char : {} bytes", size_of::<char>());
  println!("Size of string containing 'a': {}", "a".len()); // .len() gives the size of the string in bytes
  println!("Size of string containing 'ß': {}", "ß".len());
  println!("Size of string containing '国': {}", "国".len());
  println!("Size of string containing '𓅱': {}", "𓅱".len());
}

/*
len() gives size in bytes
chars().count() gives size
 */