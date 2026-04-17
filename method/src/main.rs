fn main() {
  let rect = Rect {
    width: 30,
    height: 50
  };
  println!("rect area:{}", rect.area());

  // associated function
  println!("square area:{}", Rect::square(30).area());
}

struct Rect {
  width: u32,
  height: u32
}

impl Rect {
  fn square(len:u32) -> Rect {
    Rect {
      width:len,
      height:len
    }
  }

  fn area(&self) -> u32 {
    self.width * self.height
  }
}