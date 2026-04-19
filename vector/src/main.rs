fn main() {
  let mut v:Vec<i32> = Vec::new();
  v.push(1);
  v.push(3);
  println!("hello {}x", v[1]);

  let v2 = vec![1, 2, 3];
  println!("hello count:{}", v2.len());

  let &second = &v2[1];
  println!("second ref:{}", second);
}