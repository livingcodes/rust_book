// can't mod ref, unless mut
// if mut ref, then no other refs
// ref scope begins on when introduced and ends after last usage
fn main() {
  let mut s = String::from("hello");
  let r1 = &s;
  let r2 = &s;
  println!("{r1} and {r2}");

  let r3 = &mut s;
  println!("{r3}");
}

/* fn main() {
  let s1 = String::from("hello");
  let len = calc_len(&s1);
  println!("Length of {s1} is {len}");
}

// use ref instead of taking ownership
fn calc_len(s: &String) -> usize {
  s.len();
} */

/* fn main() {
  let x = String::from("hello");
  //let y = x; // will cause compile-time error
  println!("{x}");
} */

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
