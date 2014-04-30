struct Pair<T> {
  first: T,
  second: T,
}

fn swap<T>(pair: Pair<T>) -> Pair<T> {
  let Pair { first: first, second: second } = pair;

  Pair { first: second, second: first }
}

// Reimplemntation of a 2 element tuple
struct Tuple2<T, U>(T, U);

fn main() {
  let pair_of_chars: Pair<char> = Pair { first: 'a', second: 'b' };
  let pair_of_ints = Pair { first: 1, second: 2 };

  let tuple = Tuple2("one", 2.0);

  let swapped_out_chars = swap(pair_of_chars);
  let swapped_out_ints = swap(pair_of_ints);
}
