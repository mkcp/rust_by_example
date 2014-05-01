// reimplemntation of division (/)
fn division(dividend: int, divisor: int) -> int {
  if divisor == 0 {
    fail!("Division by zero");
  } else {
    dividend / divisor
  }
}

fn checked_division(dividend: int, divisor: int) -> Option<int> {
  // failure represented as None
  if divisor == 0 {
    None
  } else {
    // result wrapped in Some tuple struct
    Some(dividend / divisor)
  }
}

fn try_division(dividend: int, divisor: int) {
  // Option values can pattern matched
  match checked_division(dividend, divisor) {
    None => println!("{} / {}", dividend, divisor),
    Some(quotient) => println!("{} / {} =  {}",
                               dividend, divisor, quotient),
  }
}

fn main() {
  try_division(4, 2);
  try_division(1, 0);

  // Binding None to a variable needs to be type annotated
  let none: Option<int> = None;

  let optional_float = Some(0.0);

  // The unwrap() method will extract the wrapped value from Some()
  // and will fail! if called on None\
  let unwrapped_float = optional_float.unwrap();
  let runtime_failure = none.unwrap();
}
