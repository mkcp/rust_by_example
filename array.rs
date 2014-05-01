use std::mem::size_of_val;

fn analyze_slice(slice: &[int]) {
  println!("first element of the slice: {}", slice[0]);
  println!("the slice has {} elements", slice.len());
}

fn main() {
  let xs: [int, ..5] = [1, 2, 3, 4, 5];

  println!("first element of the array: {}", xs[0]);
  println!("second element of the array: {}", xs[1]);

  // len() returns the size of the array
  println!("array size: {}", xs.len());

  println!("array occupies {} bytes", size_of_val(&xs));

  // out of bound indexing yields a runtime failure
  //println!("{}", size_of_val(&xs));

  // arrays can be automtically converted to slices
  println!("borrow the whole array as a slice");
  analyze_slice(xs);

  // slices can point to a section of an array
  println!("borrow a section of the array as a slice");
  analyze_slice(xs.slice(1, 4));
}
