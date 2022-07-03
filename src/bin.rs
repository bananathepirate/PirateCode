use pirate_lib::*;

fn main() {
	let b = pirate_translate(&"Gold".to_string());
	println!("{} {}", &b, pirate_translate(&b));
}
