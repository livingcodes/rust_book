fn main() {
  let s = String::from("hello world");
  let space_after_first_word_index = first_word(&s);
  println!("end {}", &space_after_first_word_index);
}

fn first_word(s: &String) -> usize {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return i;
    }
  }

  return s.len();
}
