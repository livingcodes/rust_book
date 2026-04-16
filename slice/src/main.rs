fn main() {
  let s = String::from("hello world");
  let word = first_word(&s);
  println!("the first word is {}", word);
}

fn first_word(s :&String) -> &str {
  for (i, &ch) in s.as_bytes().iter().enumerate() {
    if ch == b' ' {
      return &s[..i];
    }
  }
  return &s[..];
}
