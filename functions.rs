fn is_divisible_by(this: uint, that: uint) -> bool {
  if that == 0 {
    return false
  }

  this % that == 0
}

fn fizzbuzz(n: uint) {
  if is_divisible_by(n, 15) {
    println!("FizzBuzz");
  } else if is_divisible_by(n, 3) {
    println!("Fizz");
  } else if is_divisible_by(n, 5) {
    println!("Buzz");
  } else {
    println!("{}", n);
  }
}

fn main() {
  for n in range(1u, 101) {
    fizzbuzz(n);
  }
}
