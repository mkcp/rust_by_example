fn main() {
  for n in range(0, 101) {
    if n % 15 == 0 {
      println!("FizzBuzz");
    } else if n % 3 == 0 {
      println!("Fizz");
    } else if n % 5 == 0 {
      println!("Buzz");
    } else {
      println!("{}", n);
    }
  }
}