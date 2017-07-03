fn dangle() -> String {
	let s = String::from("hello");
	s
}

fn main() {
	let reference_to_nothing = dangle();
	print!("{:?}",reference_to_nothing);
}
