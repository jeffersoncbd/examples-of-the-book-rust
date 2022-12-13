#[derive(Debug)]
struct Rectangle {
  length: u32,
  width: u32,
}

impl Rectangle {
  fn new(length: u32, width: u32) -> Rectangle {
    Rectangle { length, width }
  }

  fn area(&self) -> u32 {
    self.length * self.width
  }
  fn can_hold(&self, other: &Rectangle) -> bool {
    self.length > other.length && self.width > other.width
  }
}

fn main() {
  let rect1 = Rectangle::new(50, 30);
  let rect2 = Rectangle::new(40, 10);
  let rect3 = Rectangle::new(45, 60);

  println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
  println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

  println!("Area of the rect1 is {}", rect1.area());
}
