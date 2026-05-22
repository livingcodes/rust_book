fn main() {
  println!("hello world");

  let nums = vec![10, 2, 50, 1];

  let _largest = largest(&nums);

  println!("largest: {}", _largest);
}

fn largest<T>(list: &[T]) -> &T
where T: PartialOrd {
  let mut largest = &list[0];

  for item in list {
    if item > largest {
      largest = item;
    }
  }

  largest
}