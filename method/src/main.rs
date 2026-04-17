fn main() {
  let rect = Rect {
    width: 30,
    height: 50
  };
  println!("rect area:{}", rect.area());

  // associated function
  println!("square area:{}", Rect::square(30).area());

  route(Ip::V4);
}

enum Ip { V4, V6 }

fn route(ip:Ip) {
  let num = match ip {
    Ip::V4 => 4,
    Ip::V6 => 6
  };
  println!("v{}", num);
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
