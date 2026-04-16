fn main() {
  let s_literal : &str = "hello world"; // string literal is a slice
  let word = first_word(&s_literal);
  println!("the first word is {word}");

  let s_string : String = String::from("hello world");
  let word = first_word(&s_string); // reference to String is also a slice
  println!("the first word is {word}");
}

fn first_word(s :&str) -> &str {
  for (i, &ch) in s.as_bytes().iter().enumerate() {
    if ch == b' ' {
      return &s[..i];
    }
  }
  return &s[..];
}
