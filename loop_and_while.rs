fn main() {
  let mut count = 0;

  loop {
    count += 1;

    if count == 3 {
      continue
    }

    println!("{}", count);

    if count == 5 {
      println!("OK, that's enough");
      break
    }
  }
}
