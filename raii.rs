// Resource acquisition is initialization

struct Homework;

fn dog_eats_homework(homework: ~Homework) {
  // homework is destroyed in this scope, memory is freed from the heap
}

fn main() {
  let homework = ~Homework;

  dog_eats_homework(homework);

  // Error: hoemwork has been moved out of this scope
  //dog_eats_homework(homework);

  // Blocks have their own scope
  if true {
    // allocate integer in the heap 
    let boxed_int = ~5;

    // boxed_int goes out of scope and memory is freed from the heap
  }

  // Error: boxed_int doesn't exist in this scope
  //let another_boxed_int = boxed_int;
}
