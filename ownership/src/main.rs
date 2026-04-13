fn main() {
  let x = String::from("hello");
  //let y = x; // will cause compile-time error
  println!("{x}");
}

/* fn main() {
  let mut x = String::from("hello");
  use_x(x);

  // compile-time error: can't use x after move (ie passed as arg)
  x.push_str(" world");
}

fn use_x(x:String) {
  println!("x is {}", x);
} */

//use std::io;
/* fn main_str() {
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
} */
