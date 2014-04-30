fn main() {

	println!("January has: ");
	println!("{} days", 31);

	println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");


	println!("{subject} {verb} {predicate}",
		     predicate="over the lazy dog",
		     subject="the quick brown fox",
		     verb="jumps");

    // This made me smile :D
	println!("{} of {:t} people know binary, the other half don't", 1, 2);
}