fn main() {
  let mut v:Vec<i32> = Vec::new();
  v.push(1);
  v.push(3);
  println!("hello {}x", v[1]);

  let v2 = vec![1, 2, 3];
  println!("hello count:{}", v2.len());

  let &second = &v2[1];
  println!("second ref:{}", second);

  let vget = v2.get(100);
  println!("vget: {:?}", vget); // Option None

  // let panic = &v2[100]; // at runtime
  // println!("panic:{}", panic);

  let v3 = vec![1, 2, 3];
  for i in &v3 {
    println!("{i}");
  }

  let mut v4 = vec![1, 2, 3];
  for i in &mut v4 {
    *i += 50;
    println!("{i}");
  }
}