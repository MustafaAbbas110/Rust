use Packages_and_Crates::mode1::detail as A;
use rand::Rng;

use Packages_and_Crates::HashMap;
fn main() {
	
	// Externel Crates from crate.io
	let number = rand::thread_rng().gen_range(1,11);
	println!("{}",number);
	let mut v = HashMap::new();
	v.insert("1","Abbas");
    println!("Hello, world!");
	detail();
	A();
}

fn detail(){
	println!("Hello! from Binary Crates");
}

