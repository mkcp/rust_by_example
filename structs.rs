// Unit struct
struct Nil;

// Tuple struct
struct Pair(int, f64);

// A struct with two fields
struct Point {
  x: f64,
  y: f64,
}

// reusing structs as fields
struct Rectangle {
  p1: Point,
  p2: Point,
}


fn main() {
  let point: Point = Point { x: 3.0, y: 4.0 };

  println!("Point coordinates: ({}, {})", point.x, point.y);

  // Destructuring using let
  let Point { x: my_x, y: my_y } = point;

  let rectangle = Rectangle {
    p1: Point { x: my_x, y: my_y },
    p2: point,
  };

  // instantiate a unit struct
  let nil = Nil;

  // instantiate a tuple struct
  let pair = Pair(1, 0.0);

  // Destructuring a tuple struct
  let Pair(integer, decimal) = pair;
}
