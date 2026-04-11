use std::io;
fn main() {
  // string literal
  let s1 = "hello world";
  println!("string literal: {}", s1);

  // String type
  let s2 = String::from("hello world");
  println!("String type: {}", s2);

  // mutable String
  let mut s3 = String::from("casey");
  let mut ln = String::new();
  println!("Enter last name:");
  io::stdin()
    .read_line(&mut ln)
    .expect("User to enter last name:");
  s3.push_str(" ");
  s3.push_str(&ln);
  println!("mutable string: hello {}", s3);
}
