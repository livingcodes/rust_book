fn main() {
  let s = String::from("hello world");
  let hello = &s[0..5];
  let world = &s[6..11];
  println!("{hello} {world}");

  let x = String::from("hello");
  let lo = &x[3..];
  println!("{lo}");

  let len = x.len();
  let lo = &x[3..len];
  println!("{lo} {len}");

  let lo = &x[3..5];
  println!("{lo}");
}
