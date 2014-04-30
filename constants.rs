static LANGUAGE: &'static str = "Rust";
static THRESHOLD: int = 10;

fn is_big(n: int) -> bool {
  // access contant in some function
  n > THRESHOLD
}

fn main() {
  let n = 16;

  // access content in the main loop
  println!("This is {}", LANGUAGE);
  println!("The threshold is {}", THRESHOLD);
  println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

  // Error: cannot modify static item
  //THRESHOLD = 5;
}
