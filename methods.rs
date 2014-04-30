use std::num::abs;

struct Point {
  x: f64,
  y: f64,
}

impl Point {
  fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
  }

  fn new(x: f64, y: f64) -> Point {
    Point { x: x, y: y }
  }
}

struct Rectangle {
  p1: Point,
  p2: Point,
}

impl Rectangle {
  fn area(&self) -> f64 {
    let Point { x: x1, y: y1 } = self.p1;
    let Point { x: x2, y: y2 } = self.p2;

    abs((x1 - x2) * (y1 - y2))
  }

  fn perimeter(&self) -> f64 {
    let Point { x: x1, y: y1 } = self.p1;
    let Point { x: x2, y: y2 } = self.p2;

    2.0 * abs(x1 - x2) + 2.0 * abs(y1 - y2)
  }

  fn move(&mut self, x: f64, y: f64) {
    self.p1.x += x;
    self.p2.x += x;

    self.p1.y += y;
    self.p2.y += y;
  }
}

struct Bomb {
  name: ~str,
}

impl Bomb {
  fn boom(self) {
    println!("{} goes boom", self.name);
  }
}

fn main() {
  let rectangle = Rectangle {
    p1: Point::origin(),
    p2: Point::new(3.0, 4.0),
  };

  // instance methods are called using the dot operator
  // note that the first argument &self is implicitly passed
  println!("Rectangle perimeter: {}", rectangle.perimeter());
  println!("Rectangle area: {}", rectangle.area());

  let mut square = Rectangle {
    p1: Point::origin(),
    p2: Point::new(1.0, 1.0),
  };

  // Error: object is immutable but method requires a mutable object
  //rectangle.move(1.0, 1.0);

  square.move(1.0, 1.0);

  let bomb = Bomb { name: ~"C4" };

  bomb.boom();

  // Error: previous boom() call moved the bomb out of scope
  //bomb.boom();
}
